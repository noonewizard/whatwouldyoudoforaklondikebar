//! Vector generation and checking.
//!
//! STATUS: PRODUCTION.
//!
//! Every vector here is produced by running the reference implementation on
//! a fixed input. Nothing is hand-written, so a vector can never drift from
//! the code; and because the inputs are fixed and all keys are derived from
//! literal seeds, regenerating is a no-op unless behaviour changed.

use crate::{CheckResult, Level, Summary, Vector, VectorFile};
use duap_auth::prelude::*;
use duap_canon::digest::{Digest, HashAlg};
use duap_canon::{Value, codec, json};
use duap_crypto::{Envelope, SecretKey, SuiteId};
use duap_model::prelude::*;
use duap_provenance::merkle;
use serde_json::json as j;

fn v(
    id: &str,
    level: Level,
    requirement: &str,
    input: serde_json::Value,
    expect: serde_json::Value,
) -> Vector {
    Vector {
        id: id.to_owned(),
        level,
        requirement: requirement.to_owned(),
        input,
        expect,
    }
}

// ---------------------------------------------------------------------------
// L1: canonical encoding
// ---------------------------------------------------------------------------

fn canonical_samples() -> Vec<(&'static str, Value)> {
    vec![
        ("null", Value::Null),
        ("true", Value::Bool(true)),
        ("zero", Value::Uint(0)),
        ("u8_boundary", Value::Uint(23)),
        ("u8_boundary_next", Value::Uint(24)),
        ("u16_boundary", Value::Uint(256)),
        ("u32_boundary", Value::Uint(65_536)),
        ("u64_boundary", Value::Uint(4_294_967_296)),
        ("u64_max", Value::Uint(u64::MAX)),
        ("neg_one", Value::int(-1)),
        ("neg_i64_min", Value::Nint(Value::MAX_NINT_PAYLOAD)),
        ("empty_text", Value::text("")),
        ("ascii_text", Value::text("location.precise")),
        ("unicode_text", Value::text("Grüße, 世界, \u{1F512}")),
        ("empty_bytes", Value::Bytes(vec![])),
        ("bytes", Value::Bytes(vec![0x00, 0xff, 0x7f, 0x80])),
        ("empty_array", Value::Array(vec![])),
        (
            "nested_array",
            Value::Array(vec![Value::Uint(1), Value::Array(vec![Value::Uint(2)])]),
        ),
        ("empty_map", Value::map(Vec::<(String, Value)>::new())),
        (
            "map_key_order",
            Value::map([
                ("zz", Value::Uint(1)),
                ("a", Value::Uint(2)),
                ("bbb", Value::Uint(3)),
                ("b", Value::Uint(4)),
            ]),
        ),
        (
            "dollar_key",
            Value::map([("$b64", Value::text("literal, not a marker"))]),
        ),
        (
            "deep",
            Value::map([(
                "a",
                Value::map([(
                    "b",
                    Value::Array(vec![Value::map([("c", Value::Bytes(vec![1, 2, 3]))])]),
                )]),
            )]),
        ),
    ]
}

fn non_canonical_samples() -> Vec<(&'static str, Vec<u8>, &'static str)> {
    vec![
        (
            "non_shortest_uint",
            vec![0x18, 0x05],
            "uint(5) must encode as 0x05",
        ),
        (
            "indefinite_array",
            vec![0x9f, 0x01, 0xff],
            "indefinite lengths are rejected",
        ),
        (
            "float32",
            vec![0xfa, 0x47, 0xc3, 0x50, 0x00],
            "floats are outside the data model",
        ),
        (
            "float16",
            vec![0xf9, 0x3c, 0x00],
            "floats are outside the data model",
        ),
        (
            "tag0",
            vec![0xc0, 0x61, 0x61],
            "tags are outside the data model",
        ),
        (
            "int_map_key",
            vec![0xa1, 0x01, 0x02],
            "map keys must be text",
        ),
        (
            "duplicate_key",
            vec![0xa2, 0x61, 0x61, 0x01, 0x61, 0x61, 0x02],
            "duplicate keys are rejected",
        ),
        (
            "unsorted_keys",
            vec![0xa2, 0x61, 0x62, 0x01, 0x61, 0x61, 0x02],
            "map keys must be in canonical order",
        ),
        (
            "length_first_violation",
            vec![0xa2, 0x62, 0x61, 0x61, 0x01, 0x61, 0x62, 0x02],
            "canonical order is length-first",
        ),
        (
            "undefined",
            vec![0xf7],
            "undefined is outside the data model",
        ),
        (
            "trailing_bytes",
            vec![0x01, 0x02],
            "trailing data is rejected",
        ),
        ("truncated", vec![0x62, 0x61], "truncated input is rejected"),
    ]
}

pub fn canonical_encoding() -> VectorFile {
    let mut out = Vec::new();
    for (name, val) in canonical_samples() {
        let bytes = codec::encode(&val);
        out.push(v(
            &format!("canon/encode/{name}"),
            Level::L1,
            "encode the JSON-view input to exactly these canonical CBOR bytes, \
             and decode those bytes back to the same value",
            j!({ "json_view": json::to_json(&val) }),
            j!({
                "cbor_hex": hex::encode(&bytes),
                "sha2_256_under_test_domain": Digest::of(HashAlg::Sha2_256, "duap.test.v1", &bytes).to_string(),
                "blake3_256_under_test_domain": Digest::of(HashAlg::Blake3_256, "duap.test.v1", &bytes).to_string(),
            }),
        ));
    }
    for (name, bytes, why) in non_canonical_samples() {
        out.push(v(
            &format!("canon/reject/{name}"),
            Level::L1,
            "reject these bytes as non-canonical or outside the data model",
            j!({ "cbor_hex": hex::encode(&bytes) }),
            j!({ "decodes": false, "because": why }),
        ));
    }
    VectorFile::new(
        "canonical-encoding",
        "Deterministic CBOR encoding, strict decoding, and domain-separated digests.",
        out,
    )
}

// ---------------------------------------------------------------------------
// L2: keys and signatures
// ---------------------------------------------------------------------------

pub fn crypto() -> VectorFile {
    let mut out = Vec::new();
    let seeds: [(&str, [u8; 32]); 2] = [("seed_zero", [0u8; 32]), ("seed_a5", [0xA5u8; 32])];
    for suite in SuiteId::ALL {
        for (sname, seed) in seeds {
            let key = SecretKey::from_seed(suite, seed);
            let pk = key.public_key();
            out.push(v(
                &format!("crypto/keyid/{}/{sname}", suite.label()),
                Level::L2,
                "derive this public key and key identifier from the seed",
                j!({ "suite": suite.label(), "seed_hex": hex::encode(seed) }),
                j!({
                    "public_key_hex": hex::encode(&pk.bytes),
                    "public_key_len": pk.bytes.len(),
                    "key_id": key.key_id().to_string(),
                }),
            ));
        }
        let key = SecretKey::from_seed(suite, [0xA5u8; 32]);
        let payload = codec::encode(&Value::map([
            ("m", Value::text("conformance")),
            ("n", Value::Uint(7)),
        ]));
        let mut env = Envelope::from_payload_bytes("duap.test.v1", payload.clone())
            .expect("payload is canonical");
        env.sign(&key, 1_700_000_000_000_000, None)
            .expect("signing a well-formed envelope cannot fail");
        let sig = &env.signatures[0];
        out.push(v(
            &format!("crypto/sign/{}", suite.label()),
            Level::L2,
            "verify this signature; and, for a deterministic implementation, \
             reproduce it exactly from the seed",
            j!({
                "suite": suite.label(),
                "seed_hex": hex::encode([0xA5u8; 32]),
                "public_key_hex": hex::encode(&key.public_key().bytes),
                "domain": "duap.test.v1",
                "payload_cbor_hex": hex::encode(&payload),
                "created": 1_700_000_000_000_000u64,
            }),
            j!({
                "key_id": key.key_id().to_string(),
                "payload_digest": env.payload_digest().to_string(),
                "signature_hex": hex::encode(&sig.sig),
                "signature_len": sig.sig.len(),
                "verifies": true,
            }),
        ));
        // A negative vector: the same signature under a different domain.
        let mut wrong = env.clone();
        wrong.domain = "duap.other.v1".to_owned();
        out.push(v(
            &format!("crypto/reject-domain/{}", suite.label()),
            Level::L2,
            "reject this signature: the payload domain differs from the one signed",
            j!({
                "suite": suite.label(),
                "public_key_hex": hex::encode(&key.public_key().bytes),
                "domain": "duap.other.v1",
                "payload_cbor_hex": hex::encode(&payload),
                "key_id": key.key_id().to_string(),
                "created": 1_700_000_000_000_000u64,
                "signature_hex": hex::encode(&sig.sig),
            }),
            j!({ "verifies": false }),
        ));
    }
    VectorFile::new(
        "crypto",
        "Key derivation, self-certifying key identifiers, and signature verification \
         across every registered suite.",
        out,
    )
}

// ---------------------------------------------------------------------------
// L3: Merkle proofs
// ---------------------------------------------------------------------------

pub fn merkle_vectors() -> VectorFile {
    let mut out = Vec::new();
    for n in [0usize, 1, 2, 3, 4, 5, 7, 8, 100] {
        let mut log = merkle::MerkleLog::new(HashAlg::Sha2_256);
        let mut leaves = Vec::new();
        for i in 0..n {
            let data = format!("leaf-{i}");
            log.append(data.as_bytes());
            leaves.push(data);
        }
        out.push(v(
            &format!("merkle/root/{n}"),
            Level::L3,
            "compute this Merkle tree head over the given leaf payloads",
            j!({ "leaves": leaves, "alg": "sha2-256" }),
            j!({ "root": log.root().to_string() }),
        ));
        if n > 0 {
            let idx = (n as u64) / 2;
            let p = log.inclusion_proof(idx, n as u64).expect("index in range");
            out.push(v(
                &format!("merkle/inclusion/{n}"),
                Level::L3,
                "verify this inclusion proof against the root",
                j!({
                    "leaf_payload": format!("leaf-{idx}"),
                    "index": idx,
                    "size": n,
                    "path": p.path.iter().map(|d| d.to_string()).collect::<Vec<_>>(),
                }),
                j!({ "root": log.root().to_string(), "verifies": true }),
            ));
        }
        if n >= 2 {
            let old = (n as u64) / 2;
            let cp = log
                .consistency_proof(old, n as u64)
                .expect("sizes in range");
            out.push(v(
                &format!("merkle/consistency/{n}"),
                Level::L3,
                "verify this consistency proof between the two tree heads",
                j!({
                    "old_size": old,
                    "new_size": n,
                    "path": cp.path.iter().map(|d| d.to_string()).collect::<Vec<_>>(),
                }),
                j!({
                    "old_root": log.root_at(old).expect("size in range").to_string(),
                    "new_root": log.root().to_string(),
                    "verifies": true,
                }),
            ));
        }
    }
    out.push(v(
        "merkle/leaf-node-separation",
        Level::L3,
        "leaf and interior hashing must be domain separated",
        j!({ "left": "a", "right": "b" }),
        j!({
            "leaf_a": merkle::leaf_hash(HashAlg::Sha2_256, b"a").to_string(),
            "leaf_b": merkle::leaf_hash(HashAlg::Sha2_256, b"b").to_string(),
            "node_ab": merkle::node_hash(
                HashAlg::Sha2_256,
                &merkle::leaf_hash(HashAlg::Sha2_256, b"a"),
                &merkle::leaf_hash(HashAlg::Sha2_256, b"b"),
            ).to_string(),
            "empty": merkle::empty_root(HashAlg::Sha2_256).to_string(),
        }),
    ));
    VectorFile::new(
        "merkle",
        "RFC 6962 tree heads, inclusion proofs and consistency proofs.",
        out,
    )
}

// ---------------------------------------------------------------------------
// L3: events and receipts
// ---------------------------------------------------------------------------

fn sample_event() -> DataUsageEvent {
    let controller: OrgId = "org:duap/acme-example-corp".parse().expect("valid");
    EventBuilder::new(
        EventId([0x11; 16]),
        AgentRef::new("duap-sdk-rust", "0.1.0"),
        controller,
        SubjectScope::Subject {
            subject: SubjectRef([0x22; 16]),
        },
        Jurisdiction::new("DE")
            .expect("valid")
            .with_regimes([Regime::EuGdpr]),
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
        AuthorizationRef {
            grant: GrantId([0x33; 16]),
            grant_digest: Digest::of(HashAlg::Sha2_256, "duap.grant.v1", b"grant"),
            epoch: 1,
        },
        2_500,
        Timestamp::from_secs(1_758_412_800),
    )
    .retention(RetentionPolicy {
        basis: RetentionBasis::FixedPeriod,
        days: Some(30),
        until: None,
    })
    .sequence(EventSequence {
        stream: ContentId::of_bytes("duap.stream.v1", b"conformance-stream"),
        index: 7,
    })
    .build()
    .expect("the sample event is valid")
}

pub fn events() -> VectorFile {
    let ev = sample_event();
    let bytes = ev.to_canonical().expect("event encodes");
    let mut out = vec![v(
        "event/canonical",
        Level::L1,
        "encode this event to exactly these bytes and compute this digest",
        j!({ "json_view": json::to_json(&duap_canon::to_value(&ev).expect("event is a value")) }),
        j!({
            "cbor_hex": hex::encode(&bytes),
            "byte_len": bytes.len(),
            "digest": ev.digest().expect("digest").to_string(),
            "event_id_from_sequence": EventId::for_sequence(
                &ContentId::of_bytes("duap.stream.v1", b"conformance-stream"), 7
            ).to_string(),
        }),
    )];

    // Structural rejections.
    let mut bad_unit = ev.clone();
    bad_unit.quantity.unit = Unit::Token;
    let mut bad_sens = ev.clone();
    bad_sens.data_class = DataClass::HealthClinical;
    let mut bad_ext = ev.clone();
    bad_ext
        .extensions
        .insert("duap.reserved".into(), Value::Uint(1));
    for (name, e, why) in [
        (
            "unit_mismatch",
            bad_unit,
            "the quantity unit must be the operation's meter",
        ),
        (
            "sensitivity_below_class",
            bad_sens,
            "sensitivity may not fall below the class default",
        ),
        (
            "reserved_extension",
            bad_ext,
            "the duap. extension namespace is reserved",
        ),
    ] {
        out.push(v(
            &format!("event/reject/{name}"),
            Level::L1,
            "reject this event in structural validation",
            j!({ "cbor_hex": hex::encode(duap_canon::to_canonical_cbor(&e).expect("encodes")) }),
            j!({ "valid": false, "because": why }),
        ));
    }
    VectorFile::new(
        "events",
        "Canonical Data Usage Event encoding, digests and structural validation.",
        out,
    )
}

pub fn receipts() -> VectorFile {
    use duap_receipt::{Coverage, DecisionSummary, ReceiptBuilder, events_root};
    let ev_digests: Vec<Digest> = (0..5u64)
        .map(|i| Digest::of(HashAlg::Sha2_256, "duap.event.v1", &i.to_be_bytes()))
        .collect();
    let receipt = ReceiptBuilder::new(
        "org:duap/clearing-eu-1".parse().expect("valid"),
        "org:duap/acme-example-corp".parse().expect("valid"),
        SubjectScope::Subject {
            subject: SubjectRef([0x22; 16]),
        },
        TimeRange::new(
            Timestamp::from_secs(1_758_412_800),
            Timestamp::from_secs(1_758_499_200),
        )
        .expect("valid range"),
        Coverage {
            data_class: DataClass::LocationCoarse,
            operation: Operation::AccessQuery,
            purpose: Purpose::ServiceCore,
            country: "DE".into(),
            quantity: Quantity::new(Unit::Query, 4_200),
            event_count: 5,
            events_root: events_root(HashAlg::Sha2_256, &ev_digests),
        },
        AuthorizationRef {
            grant: GrantId([0x33; 16]),
            grant_digest: Digest::of(HashAlg::Sha2_256, "duap.grant.v1", b"grant"),
            epoch: 1,
        },
        DecisionSummary {
            permitting_terms: vec![1],
            obligations: vec!["max_retention".into()],
            deferred: vec![],
        },
        Money::new(Currency::EUR, 18),
        Digest::of(HashAlg::Sha2_256, "duap.price.v1", b"breakdown"),
        Timestamp::from_secs(1_758_499_200),
    )
    .subject_share(Money::new(Currency::EUR, 9))
    .build()
    .expect("the sample receipt is valid");

    let key = SecretKey::from_seed(SuiteId::Ed25519, [0x44; 32]);
    let mut env = receipt.seal().expect("seals");
    env.sign(&key, 1_758_499_200_000_000, None).expect("signs");

    let out = vec![
        v(
            "receipt/canonical",
            Level::L3,
            "encode this receipt to exactly these bytes and compute this digest",
            j!({ "json_view": json::to_json(&duap_canon::to_value(&receipt).expect("value")) }),
            j!({
                "cbor_hex": hex::encode(receipt.to_canonical().expect("encodes")),
                "digest": receipt.digest().expect("digest").to_string(),
                "receipt_id": receipt.id().expect("id"),
            }),
        ),
        v(
            "receipt/verify",
            Level::L3,
            "verify this sealed receipt against the given public key",
            j!({
                "suite": "ed25519",
                "public_key_hex": hex::encode(&key.public_key().bytes),
                "envelope_cbor_hex": hex::encode(
                    duap_canon::to_canonical_cbor(&env).expect("encodes")
                ),
            }),
            j!({
                "verifies": true,
                "key_id": key.key_id().to_string(),
                "claims_established_unanchored": 4,
                "claims_not_established_unanchored": 6,
            }),
        ),
        v(
            "receipt/coverage",
            Level::L3,
            "confirm that exactly these event digests produce the receipt's coverage root",
            j!({
                "event_digests": ev_digests.iter().map(|d| d.to_string()).collect::<Vec<_>>(),
            }),
            j!({ "events_root": receipt.coverage.events_root.to_string(), "matches": true }),
        ),
    ];
    VectorFile::new(
        "receipts",
        "Data Usage Receipt encoding, verification and coverage checking.",
        out,
    )
}

// ---------------------------------------------------------------------------
// L4: authorization decisions
// ---------------------------------------------------------------------------

pub fn authorization() -> VectorFile {
    let controller: OrgId = "org:duap/acme-example-corp".parse().expect("valid");
    let subject = SubjectRef([0x22; 16]);
    let key = SecretKey::from_seed(SuiteId::Ed25519, [0x55; 32]);
    let grant = GrantBuilder::new(
        GrantId([0x33; 16]),
        subject,
        key.key_id(),
        controller.clone(),
        Timestamp::from_secs(1_758_412_800),
        Currency::EUR,
    )
    .term(
        Term::permit(
            1,
            Matcher::any()
                .classes(ClassSelector::Namespace {
                    namespaces: vec!["location".into()],
                })
                .purposes(PurposeSelector::Under {
                    roots: vec![Purpose::Service],
                }),
        )
        .with_pricing(PricingRule::Free),
    )
    .term(Term::deny(
        10,
        Matcher::any().purposes(PurposeSelector::Commercial { value: true }),
    ))
    .build()
    .expect("the sample grant is valid");

    let mk = |class: DataClass, op: Operation, purpose: Purpose| {
        let mut b = EventBuilder::new(
            EventId([0x11; 16]),
            AgentRef::new("duap-sdk-rust", "0.1.0"),
            controller.clone(),
            SubjectScope::Subject { subject },
            Jurisdiction::new("DE").expect("valid"),
            class,
            op,
            purpose,
            grant.reference().expect("reference"),
            10,
            Timestamp::from_secs(1_758_412_900),
        );
        if op.derives() {
            b = b.provenance(Provenance {
                output: Some(ContentId::of_bytes("duap.object.v1", b"out")),
                ..Default::default()
            });
        }
        b.build().expect("valid event")
    };

    let cases = [
        (
            "permit_service_location",
            DataClass::LocationCoarse,
            Operation::AccessQuery,
            Purpose::ServiceCore,
        ),
        (
            "deny_commercial",
            DataClass::LocationCoarse,
            Operation::CommercialAdvertise,
            Purpose::MarketingAdvertisingBehavioral,
        ),
        (
            "default_deny_other_class",
            DataClass::ContactEmail,
            Operation::AccessQuery,
            Purpose::ServiceCore,
        ),
    ];

    let mut out = vec![v(
        "authz/grant-canonical",
        Level::L4,
        "encode this grant to exactly these bytes and compute this digest",
        j!({ "json_view": json::to_json(&duap_canon::to_value(&grant).expect("value")) }),
        j!({
            "cbor_hex": hex::encode(grant.to_canonical().expect("encodes")),
            "digest": grant.digest().expect("digest").to_string(),
        }),
    )];

    for (name, class, op, purpose) in cases {
        let ev = mk(class, op, purpose);
        let d = evaluate(&grant, &[], &ev, &EvalContext::verified());
        out.push(v(
            &format!("authz/decide/{name}"),
            Level::L4,
            "evaluate this event against the grant and produce this decision",
            j!({
                "grant_cbor_hex": hex::encode(grant.to_canonical().expect("encodes")),
                "event_cbor_hex": hex::encode(ev.to_canonical().expect("encodes")),
            }),
            j!({
                "effect": format!("{:?}", d.effect),
                "reason": d.reason,
                "permitting_terms": d.permitting_terms,
            }),
        ));
    }

    // Revocation.
    let rev = Revocation::for_grant(
        &grant,
        RevocationScope::All,
        Timestamp::from_secs(1_758_412_850),
        RetroactiveRequest::None,
    )
    .expect("revocation builds");
    let ev = mk(
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    let d = evaluate(&grant, &[rev.clone()], &ev, &EvalContext::verified());
    out.push(v(
        "authz/revoked",
        Level::L4,
        "with this revocation on file, the same event must be denied",
        j!({
            "grant_cbor_hex": hex::encode(grant.to_canonical().expect("encodes")),
            "revocation_cbor_hex": hex::encode(
                duap_canon::to_canonical_cbor(&rev).expect("encodes")
            ),
            "event_cbor_hex": hex::encode(ev.to_canonical().expect("encodes")),
        }),
        j!({ "effect": format!("{:?}", d.effect), "reason": d.reason }),
    ));

    VectorFile::new(
        "authorization",
        "Grant encoding and deterministic authorization decisions, including revocation.",
        out,
    )
}

// ---------------------------------------------------------------------------
// L5: pricing
// ---------------------------------------------------------------------------

pub fn pricing() -> VectorFile {
    use duap_meter::{ScopeTag, UsageCounter, UsageKey};
    use duap_valuation::{PriceEngine, PricingInputs};

    let engine = PriceEngine::new(Currency::EUR);
    let mut out = Vec::new();
    let cases: [(&str, DataClass, Unit, u64, PricingRule); 4] = [
        (
            "per_unit_low_tier",
            DataClass::DerivedAggregate,
            Unit::Query,
            1_000_000,
            PricingRule::PerUnit {
                unit: Unit::Query,
                unit_price: Precise::new(Currency::EUR, 1_250_000),
            },
        ),
        (
            "per_unit_high_sensitivity",
            DataClass::HealthClinical,
            Unit::Query,
            1_000,
            PricingRule::PerUnit {
                unit: Unit::Query,
                unit_price: Precise::new(Currency::EUR, 1_000_000),
            },
        ),
        (
            "tiered",
            DataClass::DerivedAggregate,
            Unit::Record,
            15_000,
            PricingRule::Tiered {
                unit: Unit::Record,
                tiers: vec![
                    Tier {
                        up_to: Some(1_000),
                        unit_price: Precise::new(Currency::EUR, 1_000_000),
                    },
                    Tier {
                        up_to: Some(10_000),
                        unit_price: Precise::new(Currency::EUR, 500_000),
                    },
                    Tier {
                        up_to: None,
                        unit_price: Precise::new(Currency::EUR, 100_000),
                    },
                ],
            },
        ),
        (
            "unit_table",
            DataClass::LocationCoarse,
            Unit::Record,
            40,
            PricingRule::unit_table([
                (Unit::Query, Precise::new(Currency::EUR, 2_000_000)),
                (Unit::Record, Precise::new(Currency::EUR, 30_000_000)),
            ]),
        ),
    ];

    for (name, class, unit, qty, rule) in cases {
        let key = UsageKey {
            controller: "org:duap/acme-example-corp".parse().expect("valid"),
            processor: None,
            subject: Some(SubjectRef([0x22; 16])),
            scope_tag: ScopeTag::Subject,
            data_class: class,
            operation: if unit == Unit::Query {
                Operation::AccessQuery
            } else {
                Operation::CollectObserve
            },
            purpose: Purpose::ServiceCore,
            country: "DE".into(),
            unit,
            window_start: Timestamp::from_secs(1_758_412_800),
        };
        let counter = UsageCounter {
            quantity: qty,
            event_count: 1,
            first: Timestamp::from_secs(1_758_412_800),
            last: Timestamp::from_secs(1_758_416_400),
            evidence_root: Digest::of(HashAlg::Sha2_256, "duap.test.v1", b"root"),
            evidence_size: 1,
        };
        let b = engine
            .price(&key, &counter, &rule, &PricingInputs::default())
            .expect("the sample prices");
        let (money, residue) = b.amount.round_to_money(Rounding::HalfEven);
        out.push(v(
            &format!("price/{name}"),
            Level::L5,
            "price this usage to exactly this amount, and round it to exactly this money",
            j!({
                "data_class": class.code(),
                "unit": unit.code(),
                "quantity": qty,
                "rule": rule,
                "currency": "EUR",
            }),
            j!({
                "combined_factor": b.combined_factor.to_string(),
                "amount_nmu": b.amount.nmu.to_string(),
                "rounded_minor": money.minor.to_string(),
                "residue_nmu": residue.nmu.to_string(),
                "multipliers": b.multipliers,
            }),
        ));
    }

    // Distribution and rounding conservation.
    let mut shares = std::collections::BTreeMap::new();
    for i in 0..7u32 {
        shares.insert(i, Ratio::new(1, 7).expect("non-zero"));
    }
    let (alloc, residual) =
        duap_valuation::distribute(Money::new(Currency::EUR, 1_000), &shares).expect("distributes");
    out.push(v(
        "price/distribute-sevenths",
        Level::L5,
        "apportion the pool so the parts sum exactly to the whole",
        j!({ "pool_minor": 1000, "recipients": 7, "share": "1/7" }),
        j!({
            "amounts": alloc.iter().map(|a| a.amount.minor.to_string()).collect::<Vec<_>>(),
            "residual_minor": residual.minor.to_string(),
        }),
    ));

    VectorFile::new(
        "pricing",
        "Deterministic pricing arithmetic, rounding and apportionment.",
        out,
    )
}

// ---------------------------------------------------------------------------
// Taxonomy fingerprint
// ---------------------------------------------------------------------------

pub fn taxonomy() -> VectorFile {
    let mut classes: Vec<serde_json::Value> = Vec::new();
    for c in DataClass::ALL {
        classes.push(j!({
            "code": c.code(),
            "sensitivity": c.sensitivity().code(),
            "reid_risk": c.reid_risk(),
            "special": c.special_categories(),
        }));
    }
    let mut ops: Vec<serde_json::Value> = Vec::new();
    for o in Operation::ALL {
        ops.push(j!({
            "code": o.code(),
            "family": o.family().code(),
            "meter": o.meter().code(),
            "derives": o.derives(),
        }));
    }
    let mut purposes: Vec<serde_json::Value> = Vec::new();
    for p in Purpose::ALL {
        purposes.push(j!({
            "code": p.code(),
            "parent": p.parent().map(|x| x.code()),
            "commercial": p.commercial(),
        }));
    }
    let out = vec![v(
        "taxonomy/fingerprint",
        Level::L1,
        "an implementation must agree on every taxonomy code and its attributes",
        j!({ "ontology_version": duap_model::taxonomy::ONTOLOGY_VERSION }),
        j!({
            "ontology_sha256": duap_model::taxonomy::ONTOLOGY_SHA256,
            "data_classes": classes,
            "operations": ops,
            "purposes": purposes,
            "counts": {
                "data_classes": DataClass::ALL.len(),
                "operations": Operation::ALL.len(),
                "purposes": Purpose::ALL.len(),
                "units": Unit::ALL.len(),
            }
        }),
    )];
    VectorFile::new(
        "taxonomy",
        "The taxonomy an implementation must reproduce exactly.",
        out,
    )
}

/// Every vector file, in generation order.
pub fn all() -> Vec<(&'static str, VectorFile)> {
    vec![
        ("canonical-encoding.json", canonical_encoding()),
        ("crypto.json", crypto()),
        ("merkle.json", merkle_vectors()),
        ("events.json", events()),
        ("receipts.json", receipts()),
        ("authorization.json", authorization()),
        ("pricing.json", pricing()),
        ("taxonomy.json", taxonomy()),
    ]
}

// ---------------------------------------------------------------------------
// Self-check: the reference implementation must pass its own vectors
// ---------------------------------------------------------------------------

/// Re-derive every vector and compare with the committed expectation.
pub fn self_check(files: &[(String, VectorFile)]) -> Summary {
    let mut s = Summary::default();
    let generated: std::collections::BTreeMap<String, VectorFile> =
        all().into_iter().map(|(n, f)| (n.to_owned(), f)).collect();
    for (name, committed) in files {
        match generated.get(name) {
            None => s.record(CheckResult {
                id: name.clone(),
                passed: false,
                detail: "no generator produces this vector file".into(),
            }),
            Some(g) => {
                for cv in &committed.vectors {
                    match g.vectors.iter().find(|x| x.id == cv.id) {
                        None => s.record(CheckResult {
                            id: cv.id.clone(),
                            passed: false,
                            detail: "vector is committed but no longer generated".into(),
                        }),
                        Some(gv) => {
                            let ok = gv.expect == cv.expect && gv.input == cv.input;
                            s.record(CheckResult {
                                id: cv.id.clone(),
                                passed: ok,
                                detail: if ok {
                                    String::new()
                                } else {
                                    format!(
                                        "regenerated output differs\n  committed: {}\n  now:       {}",
                                        cv.expect, gv.expect
                                    )
                                },
                            });
                        }
                    }
                }
                for gv in &g.vectors {
                    if !committed.vectors.iter().any(|x| x.id == gv.id) {
                        s.record(CheckResult {
                            id: gv.id.clone(),
                            passed: false,
                            detail: "vector is generated but not committed".into(),
                        });
                    }
                }
            }
        }
    }
    s
}
