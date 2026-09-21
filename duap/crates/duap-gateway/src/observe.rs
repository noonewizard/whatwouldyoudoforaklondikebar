//! Observability: structured logs, metrics, and per-stage timings.
//!
//! STATUS: PRODUCTION (reference).
//!
//! # What is and is not recorded
//!
//! Every ingest attempt produces one structured log line naming the stage it
//! reached and how long each stage took. The line carries the event digest,
//! the controller, the operation and the outcome.
//!
//! It does NOT carry the subject pseudonym, the authorization digest, the
//! commitment, or any event field beyond class, operation and purpose.
//! Pseudonyms in logs are the classic way a privacy-preserving system leaks
//! a join key to whoever runs log aggregation; `PRIVACY.md` section
//! "Operational exposure" states the rule and this module enforces it.
//!
//! The metrics format is the Prometheus text exposition format, chosen
//! because it is a stable, documented, plain-text format that needs no
//! client library.

use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// The nine ingest stages, for per-stage timing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stage {
    Authenticate,
    Schema,
    Structure,
    Authorize,
    Replay,
    DoubleCount,
    Identity,
    Append,
    Acknowledge,
}

impl Stage {
    pub const ALL: [Stage; 9] = [
        Stage::Authenticate,
        Stage::Schema,
        Stage::Structure,
        Stage::Authorize,
        Stage::Replay,
        Stage::DoubleCount,
        Stage::Identity,
        Stage::Append,
        Stage::Acknowledge,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Stage::Authenticate => "authenticate",
            Stage::Schema => "schema",
            Stage::Structure => "structure",
            Stage::Authorize => "authorize",
            Stage::Replay => "replay",
            Stage::DoubleCount => "double_count",
            Stage::Identity => "identity",
            Stage::Append => "append",
            Stage::Acknowledge => "acknowledge",
        }
    }
}

/// A timer that records how far a request got.
pub struct StageTimer {
    start: Instant,
    marks: Vec<(Stage, u64)>,
    last: Instant,
}

impl Default for StageTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl StageTimer {
    pub fn new() -> StageTimer {
        let now = Instant::now();
        StageTimer {
            start: now,
            marks: Vec::with_capacity(9),
            last: now,
        }
    }

    /// Record the completion of a stage.
    pub fn mark(&mut self, s: Stage) {
        let now = Instant::now();
        self.marks
            .push((s, now.duration_since(self.last).as_micros() as u64));
        self.last = now;
    }

    pub fn total_micros(&self) -> u64 {
        self.start.elapsed().as_micros() as u64
    }

    pub fn as_json(&self) -> serde_json::Value {
        let mut m = serde_json::Map::new();
        for (s, us) in &self.marks {
            m.insert(s.name().to_owned(), serde_json::json!(us));
        }
        serde_json::Value::Object(m)
    }

    pub fn reached(&self) -> Option<Stage> {
        self.marks.last().map(|(s, _)| *s)
    }
}

/// Process-wide counters.
#[derive(Debug, Default)]
pub struct Metrics {
    pub requests: AtomicU64,
    pub events_accepted: AtomicU64,
    pub events_duplicate: AtomicU64,
    pub events_rejected: AtomicU64,
    pub receipts_issued: AtomicU64,
    pub invoices_issued: AtomicU64,
    pub bytes_in: AtomicU64,
    rejections_by_stage: Mutex<BTreeMap<&'static str, u64>>,
    ingest_micros: Mutex<Histogram>,
}

/// A fixed-bucket latency histogram, in microseconds.
#[derive(Debug)]
pub struct Histogram {
    bounds: Vec<u64>,
    counts: Vec<u64>,
    sum: u64,
    n: u64,
}

impl Default for Histogram {
    fn default() -> Self {
        Histogram {
            bounds: vec![
                50, 100, 250, 500, 1_000, 2_500, 5_000, 10_000, 50_000, 250_000,
            ],
            counts: vec![0; 11],
            sum: 0,
            n: 0,
        }
    }
}

impl Histogram {
    pub fn observe(&mut self, v: u64) {
        let mut i = self.bounds.len();
        for (k, b) in self.bounds.iter().enumerate() {
            if v <= *b {
                i = k;
                break;
            }
        }
        self.counts[i] += 1;
        self.sum += v;
        self.n += 1;
    }

    fn render(&self, name: &str, out: &mut String) {
        let mut cumulative = 0u64;
        for (i, b) in self.bounds.iter().enumerate() {
            cumulative += self.counts[i];
            out.push_str(&format!("{name}_bucket{{le=\"{b}\"}} {cumulative}\n"));
        }
        cumulative += self.counts[self.bounds.len()];
        out.push_str(&format!("{name}_bucket{{le=\"+Inf\"}} {cumulative}\n"));
        out.push_str(&format!("{name}_sum {}\n", self.sum));
        out.push_str(&format!("{name}_count {}\n", self.n));
    }
}

impl Metrics {
    pub fn record_rejection(&self, stage: &'static str) {
        self.events_rejected.fetch_add(1, Ordering::Relaxed);
        let mut g = self.rejections_by_stage.lock().expect("metrics mutex");
        *g.entry(stage).or_insert(0) += 1;
    }

    pub fn observe_ingest(&self, micros: u64) {
        self.ingest_micros
            .lock()
            .expect("metrics mutex")
            .observe(micros);
    }

    /// Prometheus text exposition format.
    pub fn render(&self) -> String {
        let mut out = String::with_capacity(2048);
        let g = |name: &str, help: &str, kind: &str, v: u64, out: &mut String| {
            out.push_str(&format!(
                "# HELP {name} {help}\n# TYPE {name} {kind}\n{name} {v}\n"
            ));
        };
        g(
            "duap_gateway_requests_total",
            "HTTP requests handled.",
            "counter",
            self.requests.load(Ordering::Relaxed),
            &mut out,
        );
        g(
            "duap_gateway_events_accepted_total",
            "Events accepted and counted.",
            "counter",
            self.events_accepted.load(Ordering::Relaxed),
            &mut out,
        );
        g(
            "duap_gateway_events_duplicate_total",
            "Events recognised as already held.",
            "counter",
            self.events_duplicate.load(Ordering::Relaxed),
            &mut out,
        );
        g(
            "duap_gateway_events_rejected_total",
            "Events rejected at some pipeline stage.",
            "counter",
            self.events_rejected.load(Ordering::Relaxed),
            &mut out,
        );
        g(
            "duap_gateway_receipts_issued_total",
            "Receipts issued at period close.",
            "counter",
            self.receipts_issued.load(Ordering::Relaxed),
            &mut out,
        );
        g(
            "duap_gateway_invoices_issued_total",
            "Invoices issued at period close.",
            "counter",
            self.invoices_issued.load(Ordering::Relaxed),
            &mut out,
        );
        g(
            "duap_gateway_bytes_in_total",
            "Request body bytes received.",
            "counter",
            self.bytes_in.load(Ordering::Relaxed),
            &mut out,
        );
        out.push_str(
            "# HELP duap_gateway_rejections_by_stage_total Rejections by pipeline stage.\n\
             # TYPE duap_gateway_rejections_by_stage_total counter\n",
        );
        for (k, v) in self
            .rejections_by_stage
            .lock()
            .expect("metrics mutex")
            .iter()
        {
            out.push_str(&format!(
                "duap_gateway_rejections_by_stage_total{{stage=\"{k}\"}} {v}\n"
            ));
        }
        out.push_str(
            "# HELP duap_gateway_ingest_micros Ingest latency in microseconds.\n\
             # TYPE duap_gateway_ingest_micros histogram\n",
        );
        self.ingest_micros
            .lock()
            .expect("metrics mutex")
            .render("duap_gateway_ingest_micros", &mut out);
        out
    }
}

/// Emit one structured log line to stderr.
///
/// The caller is responsible for passing only non-identifying fields; this
/// function does not inspect them, but the call sites in `service.rs` are
/// the only ones in the crate and are reviewed against the rule above.
pub fn log(event: &str, fields: serde_json::Value) {
    let line = serde_json::json!({
        "ts": duap_model::time::Timestamp::now().to_rfc3339(),
        "event": event,
        "fields": fields,
    });
    eprintln!("{line}");
}
