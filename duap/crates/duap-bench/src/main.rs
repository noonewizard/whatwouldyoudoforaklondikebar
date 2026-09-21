//! DUAP system-level benchmark harness.
//!
//! STATUS: PRODUCTION (measurement tool).
//!
//! # What this measures, and what it does not
//!
//! Criterion benchmarks in each crate measure individual operations. This
//! harness measures the paths a deployment actually cares about: an event
//! through the whole ingest pipeline, a receipt verified by a third party,
//! the bytes a stored event costs.
//!
//! It reports what it measured and the machine it ran on, and it does not
//! extrapolate. A per-core figure multiplied by a core count is a model, and
//! `.claude/rules/02-no-fake-benchmarks.md` forbids presenting one as a
//! measurement, so this program does not produce one.
//!
//! Every scenario is deterministic: fixed seeds, fixed inputs. Run to run
//! the *work* is identical, so a change in the number is a change in the
//! code or the machine.

use clap::Parser;
use duap_auth::prelude::*;
use duap_canon::HashAlg;
use duap_clearing::{ClearingConfig, ClearingNode, IngestOutcome};
use duap_crypto::{
    Envelope, KeyRecord, KeyRegistry, KeyRole, SecretKey, SuiteId, SuitePolicy, VerificationContext,
};
use duap_model::prelude::*;
use duap_sdk::prelude::*;
use serde::Serialize;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "duap-bench", about = "DUAP system benchmarks")]
struct Cli {
    /// Events to push through the ingest pipeline.
    #[arg(long, default_value_t = 20_000)]
    events: usize,
    /// Iterations for signature and verification scenarios.
    #[arg(long, default_value_t = 2_000)]
    crypto_iters: usize,
    /// Entries for the transparency-log scenarios.
    #[arg(long, default_value_t = 100_000)]
    log_entries: usize,
    /// Emit JSON instead of a markdown table.
    #[arg(long)]
    json: bool,
}

#[derive(Serialize, Clone)]
struct Measurement {
    scenario: String,
    unit: String,
    n: u64,
    total_micros: u64,
    per_op_nanos: u64,
    ops_per_sec: u64,
    note: String,
}

fn measure(scenario: &str, unit: &str, n: u64, note: &str, f: impl FnOnce()) -> Measurement {
    let t = Instant::now();
    f();
    let micros = t.elapsed().as_micros().max(1) as u64;
    let per_op = (micros as f64 * 1000.0 / n as f64) as u64;
    let ops = (n as f64 / (micros as f64 / 1e6)) as u64;
    Measurement {
        scenario: scenario.to_owned(),
        unit: unit.to_owned(),
        n,
        total_micros: micros,
        per_op_nanos: per_op,
        ops_per_sec: ops,
        note: note.to_owned(),
    }
}

#[derive(Serialize)]
struct Sizes {
    event_canonical_bytes: usize,
    envelope_ed25519_bytes: usize,
    envelope_hybrid_bytes: usize,
    receipt_canonical_bytes: usize,
    receipt_envelope_hybrid_bytes: usize,
    grant_canonical_bytes: usize,
    log_entry_bytes: usize,
    dedup_index_bytes_per_event_estimate: usize,
}

#[derive(Serialize)]
struct Report {
    protocol: String,
    measurements: Vec<Measurement>,
    sizes: Sizes,
    environment: Environment,
}

#[derive(Serialize)]
struct Environment {
    cores: usize,
    rustc: String,
    profile: String,
    date: String,
    note: String,
}

fn env_block() -> Environment {
    Environment {
        cores: std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(0),
        rustc: option_env!("DUAP_RUSTC")
            .unwrap_or("unknown; set DUAP_RUSTC")
            .to_owned(),
        profile: if cfg!(debug_assertions) {
            "debug (NOT a valid performance measurement)".to_owned()
        } else {
            "release".to_owned()
        },
        date: Timestamp::now().to_rfc3339(),
        note: "single process, single thread unless the scenario says otherwise".to_owned(),
    }
}

fn org(s: &str) -> OrgId {
    s.parse().expect("valid org id")
}

struct Scenario {
    node: ClearingNode,
    agent: ControllerAgent,
    subject: SubjectRef,
    envelopes: Vec<Envelope>,
}

fn build(n: usize) -> Scenario {
    let acme = org("org:duap/acme-example-corp");
    let clearing = org("org:duap/clearing-bench");
    let root = SubjectRoot::from_secret([0x11; 32]);
    let pseudonym = root.pseudonym_for(&acme);
    let subject_key = root.key_for(&acme, SuiteId::Ed25519);
    let acme_key = SecretKey::from_seed(SuiteId::Ed25519, [0x22; 32]);

    let mut node = ClearingNode::new(
        clearing.clone(),
        SecretKey::from_seed(SuiteId::Ed25519MlDsa44, [0x44; 32]),
        ClearingConfig::reference(Currency::EUR),
    );
    for (pk, holder, roles) in [
        (
            acme_key.public_key(),
            acme.to_string(),
            vec![KeyRole::EventSigner],
        ),
        (
            subject_key.public_key(),
            format!("subject:{pseudonym}"),
            vec![KeyRole::AuthorizationSigner],
        ),
    ] {
        node.registry
            .enroll(KeyRecord::new(pk, holder, roles, 0, None))
            .expect("enrolment succeeds");
    }

    let mut sa = SubjectAgent::new(root, subject_key);
    let (grant, _env) = sa
        .authorize(
            &acme,
            GrantId([0x33; 16]),
            vec![
                Term::permit(
                    1,
                    Matcher::any().purposes(PurposeSelector::Under {
                        roots: vec![Purpose::Service],
                    }),
                )
                .with_pricing(PricingRule::per_unit(
                    Unit::Query,
                    Precise::new(Currency::EUR, 2_000_000),
                )),
                Term::deny(
                    9,
                    Matcher::any().purposes(PurposeSelector::Commercial { value: true }),
                ),
            ],
            Timestamp::from_secs(1_758_412_800),
            Currency::EUR,
        )
        .expect("subject authorises");
    node.authorizations
        .insert_grant(grant.clone())
        .expect("grant stores");

    let mut agent = ControllerAgent::new(
        acme.clone(),
        AgentRef::new("duap-bench", "0.1.0"),
        Jurisdiction::new("DE").expect("valid"),
        acme_key,
    );
    agent.use_authorization(grant.reference().expect("reference"));

    // Pre-build the envelopes so the ingest measurement excludes agent-side
    // signing, which is measured separately.
    let base = 1_758_412_800u64;
    let mut envelopes = Vec::with_capacity(n);
    for i in 0..n {
        let (_e, _d, env) = agent
            .record_usage(
                UsageRecord::new(
                    SubjectScope::Subject { subject: pseudonym },
                    DataClass::LocationCoarse,
                    Operation::AccessQuery,
                    Purpose::ServiceCore,
                    1 + (i as u64 % 7),
                    Timestamp::from_secs(base + (i as u64 % 3600)),
                ),
                Timestamp::from_secs(base + (i as u64 % 3600)),
            )
            .expect("agent records usage");
        envelopes.push(env);
    }

    Scenario {
        node,
        agent,
        subject: pseudonym,
        envelopes,
    }
}

fn main() {
    let cli = Cli::parse();
    let mut ms: Vec<Measurement> = Vec::new();

    if cfg!(debug_assertions) {
        eprintln!(
            "duap-bench: WARNING -- built without optimisation. These numbers are not a \
             performance measurement. Run with --release."
        );
    }

    // ---------------- ingest pipeline ----------------
    let mut sc = build(cli.events);
    let envelopes = std::mem::take(&mut sc.envelopes);
    let now = Timestamp::from_secs(1_758_412_800 + 4_000);
    let mut accepted = 0u64;
    ms.push(measure(
        "ingest.pipeline",
        "event",
        cli.events as u64,
        "authenticate, schema, structure, authorize, dedup, double-count, store, provenance",
        || {
            for env in &envelopes {
                if let Ok(IngestOutcome::Accepted { .. }) = sc.node.ingest(env, now) {
                    accepted += 1;
                }
            }
        },
    ));
    assert_eq!(
        accepted as usize, cli.events,
        "every benchmark event must be accepted"
    );

    // ---------------- agent-side event production ----------------
    let mut produced = 0u64;
    ms.push(measure(
        "agent.record_usage",
        "event",
        cli.crypto_iters as u64,
        "build, validate, canonically encode and sign one event (ed25519)",
        || {
            for i in 0..cli.crypto_iters {
                let _ = sc
                    .agent
                    .record_usage(
                        UsageRecord::new(
                            SubjectScope::Subject {
                                subject: sc.subject,
                            },
                            DataClass::LocationCoarse,
                            Operation::AccessQuery,
                            Purpose::ServiceCore,
                            1,
                            Timestamp::from_secs(1_900_000_000 + i as u64),
                        ),
                        Timestamp::from_secs(1_900_000_000 + i as u64),
                    )
                    .expect("records");
                produced += 1;
            }
        },
    ));
    assert_eq!(produced as usize, cli.crypto_iters);

    // ---------------- signature verification by suite ----------------
    for suite in SuiteId::ALL {
        let key = SecretKey::from_seed(suite, [0x55; 32]);
        let pk = key.public_key();
        let payload = duap_canon::to_canonical_cbor(&"bench payload").expect("encodes");
        let mut env = Envelope::from_payload_bytes("duap.test.v1", payload).expect("canonical");
        env.sign(&key, 1_700_000_000_000_000, None).expect("signs");
        let sig = env.signatures[0].clone();
        ms.push(measure(
            &format!("verify.signature.{}", suite.label()),
            "signature",
            cli.crypto_iters as u64,
            "envelope signature verification including the signing-input reconstruction",
            || {
                for _ in 0..cli.crypto_iters {
                    env.verify_with_key(&sig, &pk).expect("verifies");
                }
            },
        ));
        ms.push(measure(
            &format!("sign.{}", suite.label()),
            "signature",
            (cli.crypto_iters / 4).max(1) as u64,
            "envelope signing including canonical encoding of the signature input",
            || {
                for i in 0..(cli.crypto_iters / 4).max(1) {
                    let mut e = Envelope::from_payload_bytes(
                        "duap.test.v1",
                        duap_canon::to_canonical_cbor(&"bench payload").expect("encodes"),
                    )
                    .expect("canonical");
                    e.sign(&key, 1_700_000_000_000_000 + i as u64, None)
                        .expect("signs");
                }
            },
        ));
    }

    // ---------------- canonical codec ----------------
    let sample_event = {
        let (e, _d, _env) = sc
            .agent
            .record_usage(
                UsageRecord::new(
                    SubjectScope::Subject {
                        subject: sc.subject,
                    },
                    DataClass::LocationCoarse,
                    Operation::AccessQuery,
                    Purpose::ServiceCore,
                    1,
                    Timestamp::from_secs(1_950_000_000),
                ),
                Timestamp::from_secs(1_950_000_000),
            )
            .expect("records");
        e
    };
    let event_bytes = sample_event.to_canonical().expect("encodes");
    ms.push(measure(
        "canon.encode.event",
        "event",
        (cli.crypto_iters * 10) as u64,
        "serialise and canonicalise one event",
        || {
            for _ in 0..(cli.crypto_iters * 10) {
                let _ = sample_event.to_canonical().expect("encodes");
            }
        },
    ));
    ms.push(measure(
        "canon.decode.event",
        "event",
        (cli.crypto_iters * 10) as u64,
        "strict canonical decode of one event",
        || {
            for _ in 0..(cli.crypto_iters * 10) {
                let _ = DataUsageEvent::from_canonical(&event_bytes).expect("decodes");
            }
        },
    ));
    ms.push(measure(
        "canon.digest.event",
        "event",
        (cli.crypto_iters * 20) as u64,
        "domain-separated SHA-256 over the canonical bytes",
        || {
            for _ in 0..(cli.crypto_iters * 20) {
                let _ = duap_canon::Digest::of(
                    HashAlg::Sha2_256,
                    duap_model::event::EVENT_DOMAIN,
                    &event_bytes,
                );
            }
        },
    ));

    // ---------------- authorization evaluation ----------------
    let grant = sc
        .node
        .authorizations
        .latest_epoch(&GrantId([0x33; 16]))
        .expect("grant present")
        .clone();
    let ctx = EvalContext::verified();
    ms.push(measure(
        "authorize.evaluate",
        "decision",
        (cli.crypto_iters * 10) as u64,
        "two-term grant, deny-overrides with obligation union",
        || {
            for _ in 0..(cli.crypto_iters * 10) {
                let _ = evaluate(&grant, &[], &sample_event, &ctx);
            }
        },
    ));

    // ---------------- transparency log ----------------
    let mut log = duap_provenance::MerkleLog::new(HashAlg::Sha2_256);
    ms.push(measure(
        "log.append",
        "entry",
        cli.log_entries as u64,
        "incremental Merkle append of a 32-byte digest",
        || {
            for i in 0..cli.log_entries {
                log.append(&(i as u64).to_be_bytes());
            }
        },
    ));
    let size = log.len();
    let mid = size / 2;
    ms.push(measure(
        "log.inclusion_proof",
        "proof",
        (cli.crypto_iters) as u64,
        &format!("audit path generation in a tree of {size} entries"),
        || {
            for _ in 0..cli.crypto_iters {
                let _ = log.inclusion_proof(mid, size).expect("in range");
            }
        },
    ));
    let proof = log.inclusion_proof(mid, size).expect("in range");
    let leaf = log.leaf(mid).expect("leaf");
    let root = log.root();
    ms.push(measure(
        "log.inclusion_verify",
        "proof",
        (cli.crypto_iters * 5) as u64,
        &format!("audit path verification in a tree of {size} entries"),
        || {
            for _ in 0..(cli.crypto_iters * 5) {
                assert!(proof.verify(leaf, root));
            }
        },
    ));

    // ---------------- period close, receipts, invoicing ----------------
    let period = TimeRange::new(
        Timestamp::from_secs(1_758_412_800),
        Timestamp::from_secs(1_758_412_800 + 7 * 86_400),
    )
    .expect("valid range");
    let close_at = Timestamp::from_secs(1_758_412_800 + 8 * 86_400);
    let t = Instant::now();
    let result = sc
        .node
        .close_period_untaxed(period, close_at)
        .expect("period closes");
    let close_micros = t.elapsed().as_micros().max(1) as u64;
    let receipts = result.receipts.len().max(1) as u64;
    ms.push(Measurement {
        scenario: "clearing.close_period".into(),
        unit: "receipt".into(),
        n: receipts,
        total_micros: close_micros,
        per_op_nanos: (close_micros as f64 * 1000.0 / receipts as f64) as u64,
        ops_per_sec: (receipts as f64 / (close_micros as f64 / 1e6)) as u64,
        note: format!(
            "price, receipt, anchor and invoice {} counters covering {} events",
            receipts, cli.events
        ),
    });

    // ---------------- third-party receipt verification ----------------
    let (receipt, receipt_env) = result
        .receipts
        .first()
        .cloned()
        .expect("at least one receipt");
    let mut verifier = KeyRegistry::new();
    verifier
        .enroll(KeyRecord::new(
            sc.node.public_key(),
            sc.node.org.to_string(),
            vec![KeyRole::ReceiptSigner, KeyRole::LogSigner],
            0,
            None,
        ))
        .expect("enrols");
    let policy = SuitePolicy::draft_default();
    let vctx = VerificationContext::archival(close_at.0 + 1);
    ms.push(measure(
        "receipt.verify",
        "receipt",
        (cli.crypto_iters / 2).max(1) as u64,
        "hybrid signature, structural validation and log inclusion proof",
        || {
            for _ in 0..(cli.crypto_iters / 2).max(1) {
                duap_receipt::Receipt::verify(&receipt_env, &verifier, &policy, &vctx)
                    .expect("verifies");
            }
        },
    ));

    // ---------------- sizes ----------------
    let ed_key = SecretKey::from_seed(SuiteId::Ed25519, [0x66; 32]);
    let hy_key = SecretKey::from_seed(SuiteId::Ed25519MlDsa44, [0x67; 32]);
    let mut ed_env =
        Envelope::from_payload_bytes(duap_model::event::EVENT_DOMAIN, event_bytes.clone())
            .expect("canonical");
    ed_env
        .sign(&ed_key, 1_700_000_000_000_000, None)
        .expect("signs");
    let mut hy_env =
        Envelope::from_payload_bytes(duap_model::event::EVENT_DOMAIN, event_bytes.clone())
            .expect("canonical");
    hy_env
        .sign(&hy_key, 1_700_000_000_000_000, None)
        .expect("signs");

    let sizes = Sizes {
        event_canonical_bytes: event_bytes.len(),
        envelope_ed25519_bytes: duap_canon::to_canonical_cbor(&ed_env)
            .expect("encodes")
            .len(),
        envelope_hybrid_bytes: duap_canon::to_canonical_cbor(&hy_env)
            .expect("encodes")
            .len(),
        receipt_canonical_bytes: receipt.to_canonical().expect("encodes").len(),
        receipt_envelope_hybrid_bytes: duap_canon::to_canonical_cbor(&receipt_env)
            .expect("encodes")
            .len(),
        grant_canonical_bytes: grant.to_canonical().expect("encodes").len(),
        log_entry_bytes: duap_canon::to_canonical_cbor(&duap_provenance::LogEntry {
            kind: duap_provenance::EntryKind::EventBatch,
            object: duap_canon::Digest::of(HashAlg::Sha2_256, "x", b"y"),
            submitter: org("org:duap/acme-example-corp"),
            sequenced_at: Timestamp::from_secs(1),
            shard: None,
        })
        .expect("encodes")
        .len(),
        // A dedup entry is a digest string key plus a timestamp; the map
        // overhead is implementation-specific, so this is the payload only
        // and is labelled as such in the report.
        dedup_index_bytes_per_event_estimate: 71 + 8,
    };

    let report = Report {
        protocol: duap_canon::PROTOCOL_ID.to_owned(),
        measurements: ms,
        sizes,
        environment: env_block(),
    };

    if cli.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("serialises")
        );
        return;
    }

    println!("# DUAP benchmark run\n");
    println!("protocol: {}", report.protocol);
    println!("profile: {}", report.environment.profile);
    println!("cores available: {}", report.environment.cores);
    println!("date: {}\n", report.environment.date);
    println!("| scenario | unit | n | per op | ops/s | what it covers |");
    println!("|---|---|---:|---:|---:|---|");
    for m in &report.measurements {
        println!(
            "| `{}` | {} | {} | {} | {} | {} |",
            m.scenario,
            m.unit,
            m.n,
            fmt_ns(m.per_op_nanos),
            m.ops_per_sec,
            m.note
        );
    }
    println!("\n## Sizes\n");
    println!("| object | bytes |");
    println!("|---|---:|");
    println!(
        "| event, canonical | {} |",
        report.sizes.event_canonical_bytes
    );
    println!(
        "| event envelope, ed25519 | {} |",
        report.sizes.envelope_ed25519_bytes
    );
    println!(
        "| event envelope, ed25519+ml-dsa-44 | {} |",
        report.sizes.envelope_hybrid_bytes
    );
    println!(
        "| grant, canonical | {} |",
        report.sizes.grant_canonical_bytes
    );
    println!(
        "| receipt, canonical | {} |",
        report.sizes.receipt_canonical_bytes
    );
    println!(
        "| receipt envelope with anchor, hybrid | {} |",
        report.sizes.receipt_envelope_hybrid_bytes
    );
    println!(
        "| log entry, canonical | {} |",
        report.sizes.log_entry_bytes
    );
    println!(
        "| dedup index payload per event | {} |",
        report.sizes.dedup_index_bytes_per_event_estimate
    );
}

fn fmt_ns(ns: u64) -> String {
    if ns < 1_000 {
        format!("{ns} ns")
    } else if ns < 1_000_000 {
        format!("{:.1} us", ns as f64 / 1000.0)
    } else {
        format!("{:.2} ms", ns as f64 / 1e6)
    }
}
