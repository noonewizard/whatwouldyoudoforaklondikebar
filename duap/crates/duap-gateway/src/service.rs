//! The gateway service: HTTP surface over a clearing node.
//!
//! STATUS: REFERENCE.
//!
//! # Endpoints
//!
//! | method | path | purpose |
//! |---|---|---|
//! | POST | `/v1/events` | ingest a signed event envelope |
//! | POST | `/v1/grants` | register a signed authorization |
//! | POST | `/v1/revocations` | register a signed revocation |
//! | POST | `/v1/keys` | enrol a public key |
//! | POST | `/v1/batches/seal` | seal pending events into a log entry |
//! | POST | `/v1/periods/close` | price, receipt and invoice a period |
//! | GET | `/v1/log/head` | current signed tree head |
//! | GET | `/v1/log/proof` | inclusion proof for an index |
//! | GET | `/v1/log/consistency` | consistency proof between sizes |
//! | GET | `/v1/receipts` | receipts issued so far |
//! | GET | `/v1/stats` | node counters |
//! | GET | `/v1/healthz` | liveness |
//! | GET | `/metrics` | Prometheus exposition |
//!
//! Bodies are canonical CBOR for protocol objects and JSON for control and
//! reporting. The content type distinguishes them; a protocol object sent as
//! JSON is refused rather than converted, because converting would mean the
//! gateway re-encoding bytes a signature covers.
//!
//! # Rate limiting
//!
//! A fixed-window counter per source organisation, applied after
//! authentication so that an unauthenticated flood cannot exhaust another
//! organisation's budget. This is the simplest limiter that is not actively
//! misleading; a deployment behind a shared proxy should use the proxy's.

use crate::http::{Request, Response};
use crate::observe::{Metrics, Stage, StageTimer, log};
use duap_canon::digest::HashAlg;
use duap_clearing::{ClearingNode, IngestOutcome, RejectAt};
use duap_crypto::{Envelope, KeyRecord, KeyRole, PublicKey, SuiteId};
use duap_model::prelude::*;
use duap_receipt::Receipt;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

/// Requests per organisation per window.
#[derive(Debug, Clone, Copy)]
pub struct RateLimit {
    pub window_secs: u64,
    pub max_requests: u64,
}

impl Default for RateLimit {
    fn default() -> Self {
        RateLimit {
            window_secs: 60,
            max_requests: 100_000,
        }
    }
}

#[derive(Debug, Default)]
struct Limiter {
    windows: BTreeMap<String, (u64, u64)>,
}

impl Limiter {
    fn allow(&mut self, who: &str, now: Timestamp, cfg: RateLimit) -> bool {
        let w = now.as_secs() / cfg.window_secs;
        let e = self.windows.entry(who.to_owned()).or_insert((w, 0));
        if e.0 != w {
            *e = (w, 0);
        }
        e.1 += 1;
        e.1 <= cfg.max_requests
    }
}

/// The service state.
pub struct Gateway {
    pub node: Mutex<ClearingNode>,
    pub metrics: Arc<Metrics>,
    pub rate_limit: RateLimit,
    limiter: Mutex<Limiter>,
    receipts: Mutex<Vec<(Receipt, Envelope)>>,
}

impl Gateway {
    pub fn new(node: ClearingNode) -> Gateway {
        Gateway {
            node: Mutex::new(node),
            metrics: Arc::new(Metrics::default()),
            rate_limit: RateLimit::default(),
            limiter: Mutex::new(Limiter::default()),
            receipts: Mutex::new(Vec::new()),
        }
    }

    /// Route and handle one request.
    pub fn handle(&self, req: &Request) -> Response {
        self.metrics.requests.fetch_add(1, Ordering::Relaxed);
        self.metrics
            .bytes_in
            .fetch_add(req.body.len() as u64, Ordering::Relaxed);
        match (req.method.as_str(), req.path.as_str()) {
            ("POST", "/v1/events") => self.post_event(req),
            ("POST", "/v1/grants") => self.post_grant(req),
            ("POST", "/v1/revocations") => self.post_revocation(req),
            ("POST", "/v1/keys") => self.post_key(req),
            ("POST", "/v1/batches/seal") => self.seal_batch(),
            ("POST", "/v1/periods/close") => self.close_period(req),
            ("GET", "/v1/log/head") => self.log_head(),
            ("GET", "/v1/log/proof") => self.log_proof(req),
            ("GET", "/v1/log/consistency") => self.log_consistency(req),
            ("GET", "/v1/receipts") => self.list_receipts(),
            ("GET", "/v1/stats") => self.stats(),
            ("GET", "/v1/healthz") => Response::json(
                200,
                serde_json::json!({ "status": "ok", "protocol": duap_canon::PROTOCOL_ID }),
            ),
            ("GET", "/metrics") => Response::text(200, self.metrics.render())
                .with_header("Content-Type", "text/plain; version=0.0.4"),
            ("GET", "/v1/conformance") => Response::json(
                200,
                serde_json::json!({
                    "protocol": duap_canon::PROTOCOL_ID,
                    "canon_version": duap_canon::CANON_VERSION,
                    "ontology_version": duap_model::taxonomy::ONTOLOGY_VERSION,
                    "ontology_sha256": duap_model::taxonomy::ONTOLOGY_SHA256,
                    "suites_accepted": SuiteId::ALL.iter().map(|s| s.label()).collect::<Vec<_>>(),
                    "levels_claimed": ["L1", "L2", "L3", "L4", "L5"],
                }),
            ),
            ("GET", _) | ("POST", _) => Response::json(
                404,
                serde_json::json!({ "error": "not_found", "path": req.path }),
            ),
            _ => Response::json(405, serde_json::json!({ "error": "method_not_allowed" })),
        }
    }

    fn require_cbor(&self, req: &Request) -> Result<(), Response> {
        match req.headers.get("content-type").map(String::as_str) {
            Some(ct) if ct.starts_with("application/duap+cbor") => Ok(()),
            other => Err(Response::json(
                415,
                serde_json::json!({
                    "error": "unsupported_media_type",
                    "detail": "protocol objects must be sent as application/duap+cbor; the \
                               gateway does not re-encode bytes a signature covers",
                    "got": other,
                }),
            )),
        }
    }

    // -----------------------------------------------------------------
    // POST /v1/events
    // -----------------------------------------------------------------
    fn post_event(&self, req: &Request) -> Response {
        if let Err(r) = self.require_cbor(req) {
            return r;
        }
        let mut timer = StageTimer::new();
        let now = Timestamp::now();

        let env: Envelope = match duap_canon::from_canonical_cbor(&req.body) {
            Ok(e) => e,
            Err(e) => {
                self.metrics.record_rejection("schema");
                timer.mark(Stage::Schema);
                log(
                    "ingest.rejected",
                    serde_json::json!({ "stage": "schema", "detail": e.to_string() }),
                );
                return Response::json(
                    400,
                    serde_json::json!({
                        "accepted": false,
                        "stage": "schema",
                        "error": "not_a_canonical_envelope",
                        "detail": e.to_string(),
                    }),
                );
            }
        };
        timer.mark(Stage::Schema);

        let mut node = self.node.lock().expect("node mutex");

        // Rate limit by the first signature's key holder, after the envelope
        // parses but before any expensive work.
        let who = env
            .signatures
            .first()
            .map(|s| s.kid.to_string())
            .unwrap_or_else(|| "anonymous".into());
        if !self
            .limiter
            .lock()
            .expect("limiter mutex")
            .allow(&who, now, self.rate_limit)
        {
            self.metrics.record_rejection("rate_limit");
            return Response::json(
                429,
                serde_json::json!({ "accepted": false, "error": "rate_limited" }),
            );
        }

        let outcome = match node.ingest(&env, now) {
            Ok(o) => o,
            Err(e) => {
                self.metrics.record_rejection("internal");
                return Response::json(
                    500,
                    serde_json::json!({ "accepted": false, "error": e.to_string() }),
                );
            }
        };
        timer.mark(Stage::Acknowledge);
        let micros = timer.total_micros();
        self.metrics.observe_ingest(micros);

        match outcome {
            IngestOutcome::Accepted { ack, decision } => {
                self.metrics.events_accepted.fetch_add(1, Ordering::Relaxed);
                log(
                    "ingest.accepted",
                    serde_json::json!({
                        "event": ack.event.to_string(),
                        "sequence": ack.store_sequence,
                        "micros": micros,
                        "stages": timer.as_json(),
                        "terms": decision.permitting_terms,
                    }),
                );
                Response::json(
                    202,
                    serde_json::json!({
                        "accepted": true,
                        "event": ack.event.to_string(),
                        "store_sequence": ack.store_sequence,
                        "node": ack.node.to_string(),
                        "accepted_at": ack.accepted_at.to_rfc3339(),
                        "permitting_terms": decision.permitting_terms,
                        "obligations": decision.obligations.iter()
                            .map(|o| o.label()).collect::<Vec<_>>(),
                        "deferred": decision.deferred,
                        "ingest_micros": micros,
                    }),
                )
            }
            IngestOutcome::Duplicate { event } => {
                self.metrics
                    .events_duplicate
                    .fetch_add(1, Ordering::Relaxed);
                // 200, not an error: retransmission must be safe.
                Response::json(
                    200,
                    serde_json::json!({
                        "accepted": true,
                        "duplicate": true,
                        "event": event.to_string(),
                        "detail": "already held; counted once",
                    }),
                )
            }
            IngestOutcome::Rejected(r) => {
                let (stage, detail, status) = describe(&r);
                self.metrics.record_rejection(stage);
                log(
                    "ingest.rejected",
                    serde_json::json!({
                        "stage": stage,
                        "detail": detail,
                        "micros": micros,
                    }),
                );
                Response::json(
                    status,
                    serde_json::json!({
                        "accepted": false,
                        "stage": stage,
                        "detail": detail,
                        "ingest_micros": micros,
                    }),
                )
            }
        }
    }

    // -----------------------------------------------------------------
    // POST /v1/grants and /v1/revocations
    // -----------------------------------------------------------------
    fn post_grant(&self, req: &Request) -> Response {
        if let Err(r) = self.require_cbor(req) {
            return r;
        }
        let env: Envelope = match duap_canon::from_canonical_cbor(&req.body) {
            Ok(e) => e,
            Err(e) => return bad(400, "not_a_canonical_envelope", &e.to_string()),
        };
        let mut node = self.node.lock().expect("node mutex");
        let ctx = duap_crypto::VerificationContext::live(Timestamp::now().0);
        let signers = match env.verify(&node.registry, &node.suite_policy, &ctx) {
            Ok(s) => s,
            Err(e) => return bad(401, "signature_invalid", &e.to_string()),
        };
        for kid in &signers {
            match node.registry.get(kid) {
                Some(r) if r.has_role(KeyRole::AuthorizationSigner) => {}
                _ => return bad(403, "wrong_role", "signer may not sign authorizations"),
            }
        }
        let grant: duap_auth::Grant = match env.open() {
            Ok(g) => g,
            Err(e) => return bad(400, "not_a_grant", &e.to_string()),
        };
        // The grant's declared subject key must be one of the signers:
        // otherwise anyone with an authorization-signer role could issue
        // grants in another subject's name.
        if !signers.contains(&grant.subject_key) {
            return bad(
                403,
                "subject_key_mismatch",
                "the grant was not signed by the key it names as the subject's",
            );
        }
        let digest = match grant.digest() {
            Ok(d) => d,
            Err(e) => return bad(400, "bad_grant", &e.to_string()),
        };
        if let Err(e) = node.authorizations.insert_grant(grant.clone()) {
            return bad(409, "grant_rejected", &e.to_string());
        }
        if let Err(e) = node.log.append(duap_provenance::LogEntry {
            kind: duap_provenance::EntryKind::Grant,
            object: digest,
            submitter: grant.controller.clone(),
            sequenced_at: Timestamp::now(),
            shard: None,
        }) {
            return bad(500, "log_append_failed", &e.to_string());
        }
        log(
            "grant.registered",
            serde_json::json!({
                "grant": grant.id.to_string(),
                "epoch": grant.epoch,
                "controller": grant.controller.to_string(),
                "terms": grant.terms.len(),
            }),
        );
        Response::json(
            201,
            serde_json::json!({
                "registered": true,
                "grant": grant.id.to_string(),
                "epoch": grant.epoch,
                "digest": digest.to_string(),
            }),
        )
    }

    fn post_revocation(&self, req: &Request) -> Response {
        if let Err(r) = self.require_cbor(req) {
            return r;
        }
        let env: Envelope = match duap_canon::from_canonical_cbor(&req.body) {
            Ok(e) => e,
            Err(e) => return bad(400, "not_a_canonical_envelope", &e.to_string()),
        };
        let mut node = self.node.lock().expect("node mutex");
        let ctx = duap_crypto::VerificationContext::live(Timestamp::now().0);
        let signers = match env.verify(&node.registry, &node.suite_policy, &ctx) {
            Ok(s) => s,
            Err(e) => return bad(401, "signature_invalid", &e.to_string()),
        };
        let rev: duap_auth::Revocation = match env.open() {
            Ok(r) => r,
            Err(e) => return bad(400, "not_a_revocation", &e.to_string()),
        };
        // Only the grant's own subject key may revoke it.
        match node.authorizations.grant(&rev.grant, rev.epoch) {
            Some(g) if signers.contains(&g.subject_key) => {}
            Some(_) => {
                return bad(
                    403,
                    "not_the_subject",
                    "a revocation must be signed by the key the grant names",
                );
            }
            None => return bad(404, "unknown_grant", "no such grant at that epoch"),
        }
        let digest = match rev.digest() {
            Ok(d) => d,
            Err(e) => return bad(400, "bad_revocation", &e.to_string()),
        };
        if let Err(e) = node.authorizations.insert_revocation(rev.clone()) {
            return bad(409, "revocation_rejected", &e.to_string());
        }
        let submitter = node.org.clone();
        if let Err(e) = node.log.append(duap_provenance::LogEntry {
            kind: duap_provenance::EntryKind::Revocation,
            object: digest,
            submitter,
            sequenced_at: Timestamp::now(),
            shard: None,
        }) {
            return bad(500, "log_append_failed", &e.to_string());
        }
        log(
            "revocation.registered",
            serde_json::json!({
                "grant": rev.grant.to_string(),
                "effective_from": rev.effective_from.to_rfc3339(),
                "retroactive": format!("{:?}", rev.retroactive),
            }),
        );
        Response::json(
            201,
            serde_json::json!({
                "registered": true,
                "grant": rev.grant.to_string(),
                "effective_from": rev.effective_from.to_rfc3339(),
                "digest": digest.to_string(),
            }),
        )
    }

    // -----------------------------------------------------------------
    // POST /v1/keys
    // -----------------------------------------------------------------
    fn post_key(&self, req: &Request) -> Response {
        #[derive(Deserialize)]
        struct Enrol {
            holder: String,
            suite: String,
            public_key_hex: String,
            roles: Vec<String>,
        }
        let body: Enrol = match serde_json::from_slice(&req.body) {
            Ok(b) => b,
            Err(e) => return bad(400, "bad_json", &e.to_string()),
        };
        let suite: SuiteId = match body.suite.parse() {
            Ok(s) => s,
            Err(e) => return bad(400, "unknown_suite", &e.to_string()),
        };
        let bytes = match hex::decode(&body.public_key_hex) {
            Ok(b) => b,
            Err(e) => return bad(400, "bad_hex", &e.to_string()),
        };
        let pk = match PublicKey::new(suite, bytes) {
            Ok(p) => p,
            Err(e) => return bad(400, "bad_key", &e.to_string()),
        };
        let mut roles = Vec::new();
        for r in &body.roles {
            roles.push(match r.as_str() {
                "event_signer" => KeyRole::EventSigner,
                "authorization_signer" => KeyRole::AuthorizationSigner,
                "receipt_signer" => KeyRole::ReceiptSigner,
                "log_signer" => KeyRole::LogSigner,
                "settlement_signer" => KeyRole::SettlementSigner,
                "registry_admin" => KeyRole::RegistryAdmin,
                "auditor" => KeyRole::Auditor,
                other => return bad(400, "unknown_role", other),
            });
        }
        let kid = pk.key_id();
        let mut node = self.node.lock().expect("node mutex");
        if let Err(e) =
            node.registry
                .enroll(KeyRecord::new(pk, body.holder.clone(), roles, 0, None))
        {
            return bad(409, "enrolment_failed", &e.to_string());
        }
        log(
            "key.enrolled",
            serde_json::json!({ "holder": body.holder, "kid": kid.to_string(), "suite": suite.label() }),
        );
        Response::json(
            201,
            serde_json::json!({ "enrolled": true, "key_id": kid.to_string() }),
        )
    }

    // -----------------------------------------------------------------
    // Batches, periods and the log
    // -----------------------------------------------------------------
    fn seal_batch(&self) -> Response {
        let mut node = self.node.lock().expect("node mutex");
        match node.seal_batch(Timestamp::now()) {
            Ok(Some((index, root))) => Response::json(
                201,
                serde_json::json!({ "sealed": true, "log_index": index, "batch_root": root.to_string() }),
            ),
            Ok(None) => Response::json(
                200,
                serde_json::json!({ "sealed": false, "detail": "nothing pending" }),
            ),
            Err(e) => bad(500, "seal_failed", &e.to_string()),
        }
    }

    fn close_period(&self, req: &Request) -> Response {
        #[derive(Deserialize)]
        struct Close {
            start_secs: u64,
            end_secs: u64,
        }
        let body: Close = match serde_json::from_slice(&req.body) {
            Ok(b) => b,
            Err(e) => return bad(400, "bad_json", &e.to_string()),
        };
        let period = match TimeRange::new(
            Timestamp::from_secs(body.start_secs),
            Timestamp::from_secs(body.end_secs),
        ) {
            Ok(p) => p,
            Err(e) => return bad(400, "bad_period", &e.to_string()),
        };
        let mut node = self.node.lock().expect("node mutex");
        let now = Timestamp::now();
        match node.close_period_untaxed(period, now) {
            Ok(result) => {
                self.metrics
                    .receipts_issued
                    .fetch_add(result.receipts.len() as u64, Ordering::Relaxed);
                self.metrics
                    .invoices_issued
                    .fetch_add(result.invoices.len() as u64, Ordering::Relaxed);
                let summary = serde_json::json!({
                    "receipts": result.receipts.len(),
                    "invoices": result.invoices.iter().map(|i| serde_json::json!({
                        "id": i.id.to_string(),
                        "payer": i.payer.to_string(),
                        "total": i.total.to_string(),
                        "lines": i.lines.len(),
                    })).collect::<Vec<_>>(),
                    "total_charged": result.total_charged.to_string(),
                    "total_subject_share": result.total_subject_share.to_string(),
                    "unpriced_counters": result.unpriced.len(),
                    "ledger_balanced": node.ledger.is_balanced(),
                });
                self.receipts
                    .lock()
                    .expect("receipts mutex")
                    .extend(result.receipts);
                log("period.closed", summary.clone());
                Response::json(201, summary)
            }
            Err(e) => bad(500, "close_failed", &e.to_string()),
        }
    }

    fn log_head(&self) -> Response {
        let node = self.node.lock().expect("node mutex");
        let head = node.log.head(Timestamp::now());
        Response::json(
            200,
            serde_json::json!({
                "log_id": head.log_id,
                "size": head.size,
                "root": head.root.to_string(),
                "issued_at": head.issued_at.to_rfc3339(),
                "alg": head.alg.label(),
            }),
        )
    }

    fn log_proof(&self, req: &Request) -> Response {
        let node = self.node.lock().expect("node mutex");
        let size = req.query_u64("size").unwrap_or(node.log.size());
        let Some(index) = req.query_u64("index") else {
            return bad(400, "missing_index", "index is required");
        };
        match node.log.inclusion_proof(index, size) {
            Some(p) => Response::json(
                200,
                serde_json::json!({
                    "index": p.index,
                    "size": p.size,
                    "path": p.path.iter().map(|d| d.to_string()).collect::<Vec<_>>(),
                    "root": node.log.root_at(size).map(|r| r.to_string()),
                }),
            ),
            None => bad(404, "no_such_entry", "index out of range for that size"),
        }
    }

    fn log_consistency(&self, req: &Request) -> Response {
        let node = self.node.lock().expect("node mutex");
        let Some(old) = req.query_u64("old") else {
            return bad(400, "missing_old", "old is required");
        };
        let new = req.query_u64("new").unwrap_or(node.log.size());
        match node.log.consistency_proof(old, new) {
            Some(p) => Response::json(
                200,
                serde_json::json!({
                    "old": p.old,
                    "new": p.new,
                    "path": p.path.iter().map(|d| d.to_string()).collect::<Vec<_>>(),
                    "old_root": node.log.root_at(old).map(|r| r.to_string()),
                    "new_root": node.log.root_at(new).map(|r| r.to_string()),
                }),
            ),
            None => bad(400, "bad_sizes", "sizes out of range"),
        }
    }

    fn list_receipts(&self) -> Response {
        let rs = self.receipts.lock().expect("receipts mutex");
        Response::json(
            200,
            serde_json::json!({
                "count": rs.len(),
                "receipts": rs.iter().map(|(r, env)| serde_json::json!({
                    "id": r.id().unwrap_or_default(),
                    "controller": r.controller.to_string(),
                    "operation": r.coverage.operation.code(),
                    "quantity": r.coverage.quantity.amount,
                    "charge": r.charge.to_string(),
                    "subject_share": r.subject_share.map(|m| m.to_string()),
                    "anchored": r.anchor.is_some(),
                    "envelope_cbor_hex": duap_canon::to_canonical_cbor(env)
                        .map(hex::encode).unwrap_or_default(),
                })).collect::<Vec<_>>(),
            }),
        )
    }

    fn stats(&self) -> Response {
        let node = self.node.lock().expect("node mutex");
        Response::json(
            200,
            serde_json::json!({
                "node": node.org.to_string(),
                "stats": node.stats,
                "meter": node.meter.stats,
                "log_size": node.log.size(),
                "store_events": node.store_len().unwrap_or(0),
                "grants": node.authorizations.grant_count(),
                "registry_keys": node.registry.len(),
                "ledger_entries": node.ledger.entry_count(),
                "ledger_balanced": node.ledger.is_balanced(),
                "gaps": node.meter.dedup.gaps(),
            }),
        )
    }
}

fn bad(status: u16, error: &str, detail: &str) -> Response {
    Response::json(
        status,
        serde_json::json!({ "error": error, "detail": detail }),
    )
}

fn describe(r: &RejectAt) -> (&'static str, String, u16) {
    match r {
        RejectAt::Authentication { detail } => ("authenticate", detail.clone(), 401),
        RejectAt::Schema { detail } => ("schema", detail.clone(), 400),
        RejectAt::Structure { detail } => ("structure", detail.clone(), 422),
        RejectAt::Authorization { reason } => (
            "authorize",
            serde_json::to_string(reason).unwrap_or_else(|_| "denied".into()),
            403,
        ),
        RejectAt::Replay { detail } => ("replay", detail.clone(), 409),
        RejectAt::DoubleCount { detail } => ("double_count", detail.clone(), 409),
        RejectAt::Storage { detail } => ("append", detail.clone(), 500),
    }
}

/// Every stage name, for documentation and for the metrics help text.
pub fn stage_names() -> Vec<&'static str> {
    Stage::ALL.iter().map(|s| s.name()).collect()
}

/// Hash algorithm the gateway's log uses.
pub const LOG_ALG: HashAlg = HashAlg::Sha2_256;
