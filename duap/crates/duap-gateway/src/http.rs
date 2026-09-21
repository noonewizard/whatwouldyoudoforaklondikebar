//! A minimal blocking HTTP/1.1 server.
//!
//! STATUS: REFERENCE. This is not a production HTTP stack and does not
//! pretend to be one: no TLS, no HTTP/2, no keep-alive tuning, no
//! connection limits beyond a fixed thread pool.
//!
//! # Why not a framework
//!
//! `docs/adr/0011-gateway-http-stack.md` records the decision. In short: the
//! ingest endpoint is the protocol's trust boundary, and the reference
//! implementation's job is to be *auditable* rather than fast. A hand-written
//! HTTP/1.1 reader in 200 lines can be read end to end by a reviewer in an
//! afternoon; an async framework plus its runtime is a few hundred thousand
//! lines of dependency that the reviewer must take on trust. A production
//! deployment is expected to terminate TLS and HTTP at a mature reverse proxy
//! and speak to this service over a loopback or mesh connection --
//! `DEPLOYMENT.md` sets that out.

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

/// Maximum request body accepted, in bytes.
pub const MAX_BODY: usize = 4 * 1024 * 1024;
/// Maximum size of the request line plus headers.
pub const MAX_HEAD: usize = 16 * 1024;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query: BTreeMap<String, String>,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn query_u64(&self, key: &str) -> Option<u64> {
        self.query.get(key).and_then(|v| v.parse().ok())
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub content_type: String,
    pub body: Vec<u8>,
    pub extra_headers: Vec<(String, String)>,
}

impl Response {
    pub fn json(status: u16, value: serde_json::Value) -> Response {
        Response {
            status,
            content_type: "application/json".into(),
            body: serde_json::to_vec_pretty(&value).unwrap_or_else(|_| b"{}".to_vec()),
            extra_headers: Vec::new(),
        }
    }

    pub fn text(status: u16, body: impl Into<String>) -> Response {
        Response {
            status,
            content_type: "text/plain; charset=utf-8".into(),
            body: body.into().into_bytes(),
            extra_headers: Vec::new(),
        }
    }

    pub fn cbor(status: u16, body: Vec<u8>) -> Response {
        Response {
            status,
            content_type: "application/duap+cbor".into(),
            body,
            extra_headers: Vec::new(),
        }
    }

    pub fn with_header(mut self, k: &str, v: &str) -> Response {
        self.extra_headers.push((k.to_owned(), v.to_owned()));
        self
    }
}

fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        413 => "Payload Too Large",
        422 => "Unprocessable Content",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "Status",
    }
}

/// A running server. Dropping it stops accepting new connections.
pub struct Server {
    pub port: u16,
    shutdown: mpsc::Sender<()>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Server {
    /// Bind and serve on `addr`, dispatching to `handler`.
    pub fn start<H>(addr: &str, workers: usize, handler: H) -> std::io::Result<Server>
    where
        H: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        let port = listener.local_addr()?.port();
        let (tx, rx) = mpsc::channel::<()>();
        let handler = Arc::new(handler);
        let pool = Arc::new(Mutex::new(0usize));

        let handle = thread::spawn(move || {
            loop {
                if rx.try_recv().is_ok() {
                    return;
                }
                match listener.accept() {
                    Ok((stream, _)) => {
                        // A fixed ceiling on concurrent connections: the
                        // simplest defence against a slow-loris flood that
                        // does not require an async runtime.
                        let busy = {
                            let mut g = pool.lock().expect("pool mutex");
                            if *g >= workers {
                                true
                            } else {
                                *g += 1;
                                false
                            }
                        };
                        if busy {
                            let _ = write_response(
                                &mut { stream },
                                &Response::json(
                                    503,
                                    serde_json::json!({
                                        "error": "server_busy",
                                        "detail": "connection limit reached"
                                    }),
                                ),
                            );
                            continue;
                        }
                        let h = Arc::clone(&handler);
                        let p = Arc::clone(&pool);
                        thread::spawn(move || {
                            let mut s = stream;
                            let _ = s.set_nonblocking(false);
                            let _ = serve_one(&mut s, h.as_ref());
                            let mut g = p.lock().expect("pool mutex");
                            *g = g.saturating_sub(1);
                        });
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(std::time::Duration::from_millis(5));
                    }
                    Err(_) => return,
                }
            }
        });

        Ok(Server {
            port,
            shutdown: tx,
            handle: Some(handle),
        })
    }

    pub fn stop(mut self) {
        let _ = self.shutdown.send(());
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.shutdown.send(());
    }
}

fn serve_one<H>(stream: &mut TcpStream, handler: &H) -> std::io::Result<()>
where
    H: Fn(&Request) -> Response,
{
    let resp = match read_request(stream) {
        Ok(req) => handler(&req),
        Err(e) => Response::json(
            400,
            serde_json::json!({ "error": "bad_request", "detail": e.to_string() }),
        ),
    };
    write_response(stream, &resp)
}

fn read_request(stream: &mut TcpStream) -> std::io::Result<Request> {
    stream.set_read_timeout(Some(std::time::Duration::from_secs(15)))?;
    let mut reader = BufReader::new(stream.try_clone()?);

    let mut head = String::new();
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        head.push_str(&line);
        if head.len() > MAX_HEAD {
            return Err(std::io::Error::other("request head too large"));
        }
        if line == "\r\n" || line == "\n" {
            break;
        }
    }

    let mut lines = head.lines();
    let start = lines
        .next()
        .ok_or_else(|| std::io::Error::other("empty request"))?;
    let mut parts = start.split_whitespace();
    let method = parts.next().unwrap_or_default().to_owned();
    let target = parts.next().unwrap_or_default().to_owned();

    let mut headers = BTreeMap::new();
    for l in lines {
        if let Some((k, v)) = l.split_once(':') {
            headers.insert(k.trim().to_ascii_lowercase(), v.trim().to_owned());
        }
    }

    let (path, query) = match target.split_once('?') {
        Some((p, q)) => (p.to_owned(), parse_query(q)),
        None => (target, BTreeMap::new()),
    };

    let len: usize = headers
        .get("content-length")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    if len > MAX_BODY {
        return Err(std::io::Error::other("request body too large"));
    }
    let mut body = vec![0u8; len];
    if len > 0 {
        reader.read_exact(&mut body)?;
    }

    Ok(Request {
        method,
        path,
        query,
        headers,
        body,
    })
}

fn parse_query(q: &str) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    for pair in q.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        m.insert(percent_decode(k), percent_decode(v));
    }
    m
}

fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let Ok(v) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(if b[i] == b'+' { b' ' } else { b[i] });
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn write_response(stream: &mut TcpStream, r: &Response) -> std::io::Result<()> {
    let mut head = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\
         X-Content-Type-Options: nosniff\r\nCache-Control: no-store\r\n",
        r.status,
        reason(r.status),
        r.content_type,
        r.body.len()
    );
    for (k, v) in &r.extra_headers {
        head.push_str(&format!("{k}: {v}\r\n"));
    }
    head.push_str("\r\n");
    stream.write_all(head.as_bytes())?;
    stream.write_all(&r.body)?;
    stream.flush()
}
