//! Integration tests that drive the gateway over real HTTP.
//!
//! These are the tests that show the ingest pipeline behaving as a service
//! rather than as a library call: the server is bound to an ephemeral port,
//! requests are written as bytes on a socket, and the assertions are about
//! status codes and JSON bodies.

use duap_auth::prelude::*;
use duap_clearing::{ClearingConfig, ClearingNode};
use duap_crypto::{SecretKey, SuiteId};
use duap_gateway::{Gateway, Server};
use duap_model::prelude::*;
use duap_sdk::prelude::*;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

struct Harness {
    server: Option<Server>,
    port: u16,
}

impl Harness {
    fn start() -> (Harness, Fixtures) {
        let fx = Fixtures::new();
        let node = fx.build_node();
        let gw = Arc::new(Gateway::new(node));
        let g = Arc::clone(&gw);
        let server = Server::start("127.0.0.1:0", 8, move |req| g.handle(req))
            .expect("the server binds an ephemeral port");
        let port = server.port;
        (
            Harness {
                server: Some(server),
                port,
            },
            fx,
        )
    }

    fn request(&self, method: &str, path: &str, ctype: &str, body: &[u8]) -> (u16, String) {
        let mut s = TcpStream::connect(("127.0.0.1", self.port)).expect("connects");
        let head = format!(
            "{method} {path} HTTP/1.1\r\nHost: localhost\r\nContent-Type: {ctype}\r\n\
             Content-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        s.write_all(head.as_bytes()).expect("writes head");
        s.write_all(body).expect("writes body");
        s.flush().expect("flushes");
        let mut out = String::new();
        s.read_to_string(&mut out).expect("reads response");
        let status: u16 = out
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let body = out.split("\r\n\r\n").nth(1).unwrap_or("").to_owned();
        (status, body)
    }

    fn get(&self, path: &str) -> (u16, serde_json::Value) {
        let (s, b) = self.request("GET", path, "application/json", b"");
        (s, serde_json::from_str(&b).unwrap_or(serde_json::Value::Null))
    }

    fn post_cbor(&self, path: &str, body: &[u8]) -> (u16, serde_json::Value) {
        let (s, b) = self.request("POST", path, "application/duap+cbor", body);
        (s, serde_json::from_str(&b).unwrap_or(serde_json::Value::Null))
    }

    fn post_json(&self, path: &str, v: serde_json::Value) -> (u16, serde_json::Value) {
        let body = serde_json::to_vec(&v).expect("serialises");
        let (s, b) = self.request("POST", path, "application/json", &body);
        (s, serde_json::from_str(&b).unwrap_or(serde_json::Value::Null))
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        if let Some(s) = self.server.take() {
            s.stop();
        }
    }
}

struct Fixtures {
    acme: OrgId,
    clearing: OrgId,
    acme_key: SecretKey,
    node_key: SecretKey,
    subject: SubjectAgent,
    subject_pseudonym: SubjectRef,
}

impl Fixtures {
    fn new() -> Fixtures {
        let acme: OrgId = "org:duap/acme".parse().expect("valid");
        let clearing: OrgId = "org:duap/clearing-test".parse().expect("valid");
        let root = SubjectRoot::from_secret([0x31; 32]);
        let pseudonym = root.pseudonym_for(&acme);
        let skey = root.key_for(&acme, SuiteId::Ed25519);
        Fixtures {
            acme,
            clearing,
            acme_key: SecretKey::from_seed(SuiteId::Ed25519, [0x32; 32]),
            node_key: SecretKey::from_seed(SuiteId::Ed25519, [0x33; 32]),
            subject: SubjectAgent::new(root, skey),
            subject_pseudonym: pseudonym,
        }
    }

    fn build_node(&self) -> ClearingNode {
        let mut node = ClearingNode::new(
            self.clearing.clone(),
            SecretKey::from_seed(SuiteId::Ed25519, [0x33; 32]),
            ClearingConfig::reference(Currency::EUR),
        );
        for (key, holder, roles) in [
            (
                self.acme_key.public_key(),
                self.acme.to_string(),
                vec![KeyRole::EventSigner],
            ),
            (
                self.node_key.public_key(),
                self.clearing.to_string(),
                vec![KeyRole::ReceiptSigner, KeyRole::LogSigner],
            ),
            (
                self.subject.public_key(),
                format!("subject:{}", self.subject_pseudonym),
                vec![KeyRole::AuthorizationSigner],
            ),
        ] {
            node.registry
                .enroll(KeyRecord::new(key, holder, roles, 0, None))
                .expect("enrolment succeeds");
        }
        node
    }

    fn grant(&mut self) -> (Grant, Vec<u8>) {
        let (g, env) = self
            .subject
            .authorize(
                &self.acme.clone(),
                GrantId([0x34; 16]),
                vec![
                    Term::permit(
                        1,
                        Matcher::any().purposes(PurposeSelector::Under {
                            roots: vec![Purpose::Service],
                        }),
                    )
                    .with_pricing(PricingRule::unit_table([
                        (Unit::Query, Precise::new(Currency::EUR, 2_000_000)),
                        (Unit::Record, Precise::new(Currency::EUR, 30_000_000)),
                    ])),
                    Term::deny(
                        9,
                        Matcher::any().purposes(PurposeSelector::Commercial { value: true }),
                    ),
                ],
                Timestamp::now(),
                Currency::EUR,
            )
            .expect("the subject can authorise");
        let bytes = duap_canon::to_canonical_cbor(&env).expect("envelope encodes");
        (g, bytes)
    }

    fn agent(&self, grant: &Grant) -> ControllerAgent {
        let mut a = ControllerAgent::new(
            self.acme.clone(),
            AgentRef::new("gateway-test", "0.1.0"),
            Jurisdiction::new("DE").expect("valid"),
            SecretKey::from_seed(SuiteId::Ed25519, [0x32; 32]),
        );
        a.use_authorization(grant.reference().expect("reference"));
        a
    }
}

#[test]
fn health_and_conformance_advertise_the_protocol() {
    let (h, _fx) = Harness::start();
    let (s, v) = h.get("/v1/healthz");
    assert_eq!(s, 200);
    assert_eq!(v["protocol"], "DUAP/1");

    let (s, v) = h.get("/v1/conformance");
    assert_eq!(s, 200);
    assert_eq!(v["ontology_sha256"], duap_model::taxonomy::ONTOLOGY_SHA256);
    assert!(v["suites_accepted"].as_array().expect("array").len() >= 4);
}

#[test]
fn the_full_ingest_path_works_over_http() {
    let (h, mut fx) = Harness::start();
    let (grant, grant_bytes) = fx.grant();

    let (s, v) = h.post_cbor("/v1/grants", &grant_bytes);
    assert_eq!(s, 201, "grant registration: {v}");
    assert_eq!(v["epoch"], 1);

    let mut agent = fx.agent(&grant);
    let (_e, digest, env) = agent
        .record_usage(
            UsageRecord::new(
                SubjectScope::Subject {
                    subject: fx.subject_pseudonym,
                },
                DataClass::LocationCoarse,
                Operation::AccessQuery,
                Purpose::ServiceCore,
                1_500,
                Timestamp::now(),
            ),
            Timestamp::now(),
        )
        .expect("agent records usage");
    let body = duap_canon::to_canonical_cbor(&env).expect("encodes");

    let (s, v) = h.post_cbor("/v1/events", &body);
    assert_eq!(s, 202, "ingest: {v}");
    assert_eq!(v["accepted"], true);
    assert_eq!(v["event"], digest.to_string());
    assert_eq!(v["permitting_terms"][0], 1);
    assert!(v["ingest_micros"].as_u64().is_some());

    // Retransmission is idempotent and is not an error.
    let (s, v) = h.post_cbor("/v1/events", &body);
    assert_eq!(s, 200, "retransmission: {v}");
    assert_eq!(v["duplicate"], true);

    // Seal and close.
    let (s, v) = h.post_json("/v1/batches/seal", serde_json::json!({}));
    assert_eq!(s, 201, "seal: {v}");
    assert!(v["log_index"].as_u64().is_some());

    let now = Timestamp::now().as_secs();
    let (s, v) = h.post_json(
        "/v1/periods/close",
        serde_json::json!({ "start_secs": now - 86_400, "end_secs": now + 86_400 }),
    );
    assert_eq!(s, 201, "close: {v}");
    assert_eq!(v["ledger_balanced"], true);
    assert!(v["receipts"].as_u64().expect("count") >= 1);

    let (s, v) = h.get("/v1/receipts");
    assert_eq!(s, 200);
    assert!(v["count"].as_u64().expect("count") >= 1);
    assert_eq!(v["receipts"][0]["anchored"], true);

    // The log exposes verifiable proofs.
    let (s, head) = h.get("/v1/log/head");
    assert_eq!(s, 200);
    let size = head["size"].as_u64().expect("size");
    assert!(size >= 2);
    let (s, proof) = h.get(&format!("/v1/log/proof?index=0&size={size}"));
    assert_eq!(s, 200, "proof: {proof}");
    assert!(proof["path"].as_array().is_some());
    let (s, cons) = h.get(&format!("/v1/log/consistency?old=1&new={size}"));
    assert_eq!(s, 200, "consistency: {cons}");
}

#[test]
fn every_rejection_names_the_stage_it_failed_at() {
    let (h, mut fx) = Harness::start();
    let (grant, grant_bytes) = fx.grant();
    h.post_cbor("/v1/grants", &grant_bytes);

    // Schema: not canonical CBOR at all.
    let (s, v) = h.post_cbor("/v1/events", b"\x18\x05");
    assert_eq!(s, 400);
    assert_eq!(v["stage"], "schema");

    // Authentication: signed by a key the registry does not know.
    let stranger = SecretKey::from_seed(SuiteId::Ed25519, [0x99; 32]);
    let mut agent = ControllerAgent::new(
        fx.acme.clone(),
        AgentRef::new("stranger", "0"),
        Jurisdiction::new("DE").expect("valid"),
        stranger,
    );
    agent.use_authorization(grant.reference().expect("reference"));
    let (_e, _d, env) = agent
        .record_usage(
            UsageRecord::new(
                SubjectScope::Subject { subject: fx.subject_pseudonym },
                DataClass::LocationCoarse,
                Operation::AccessQuery,
                Purpose::ServiceCore,
                1,
                Timestamp::now(),
            ),
            Timestamp::now(),
        )
        .expect("records");
    let (s, v) = h.post_cbor(
        "/v1/events",
        &duap_canon::to_canonical_cbor(&env).expect("encodes"),
    );
    assert_eq!(s, 401, "{v}");
    assert_eq!(v["stage"], "authenticate");

    // Authorization: a commercial purpose the grant denies.
    let mut agent = fx.agent(&grant);
    let (_e, _d, env) = agent
        .record_usage(
            UsageRecord::new(
                SubjectScope::Subject { subject: fx.subject_pseudonym },
                DataClass::BehaviorWebBrowsing,
                Operation::CommercialAdvertise,
                Purpose::MarketingAdvertisingBehavioral,
                10,
                Timestamp::now(),
            ),
            Timestamp::now(),
        )
        .expect("records");
    let (s, v) = h.post_cbor(
        "/v1/events",
        &duap_canon::to_canonical_cbor(&env).expect("encodes"),
    );
    assert_eq!(s, 403, "{v}");
    assert_eq!(v["stage"], "authorize");
    assert!(v["detail"].as_str().expect("detail").contains("denied_by_term"));
}

#[test]
fn protocol_objects_must_be_sent_as_cbor() {
    let (h, _fx) = Harness::start();
    let (s, v) = h.post_json("/v1/events", serde_json::json!({ "not": "cbor" }));
    assert_eq!(s, 415);
    assert_eq!(v["error"], "unsupported_media_type");
}

#[test]
fn a_grant_must_be_signed_by_the_key_it_names() {
    let (h, fx) = Harness::start();
    // Build a grant naming the subject's key but sign it with Acme's.
    let grant = GrantBuilder::new(
        GrantId([0x35; 16]),
        fx.subject_pseudonym,
        fx.subject.public_key().key_id(),
        fx.acme.clone(),
        Timestamp::now(),
        Currency::EUR,
    )
    .term(Term::permit(1, Matcher::any()))
    .build()
    .expect("builds");
    let mut env = duap_crypto::Envelope::seal(duap_auth::GRANT_DOMAIN, &grant).expect("seals");
    env.sign(&fx.acme_key, Timestamp::now().0, None).expect("signs");
    let (s, v) = h.post_cbor(
        "/v1/grants",
        &duap_canon::to_canonical_cbor(&env).expect("encodes"),
    );
    // Acme's key has no authorization-signer role, so this stops at the role
    // check; with the role it would stop at the subject-key check.
    assert!(s == 403, "expected a refusal, got {s}: {v}");
}

#[test]
fn metrics_are_exposed_in_prometheus_format() {
    let (h, mut fx) = Harness::start();
    let (_g, grant_bytes) = fx.grant();
    h.post_cbor("/v1/grants", &grant_bytes);
    let (s, body) = h.request("GET", "/metrics", "text/plain", b"");
    assert_eq!(s, 200);
    assert!(body.contains("duap_gateway_requests_total"));
    assert!(body.contains("# TYPE duap_gateway_ingest_micros histogram"));
    assert!(body.contains("duap_gateway_ingest_micros_bucket{le=\"+Inf\"}"));
}

#[test]
fn stats_report_sequence_gaps() {
    let (h, _fx) = Harness::start();
    let (s, v) = h.get("/v1/stats");
    assert_eq!(s, 200);
    assert_eq!(v["ledger_balanced"], true);
    assert!(v["gaps"].as_array().expect("array").is_empty());
}

#[test]
fn unknown_routes_are_refused_cleanly() {
    let (h, _fx) = Harness::start();
    let (s, v) = h.get("/v1/nope");
    assert_eq!(s, 404);
    assert_eq!(v["error"], "not_found");
}
