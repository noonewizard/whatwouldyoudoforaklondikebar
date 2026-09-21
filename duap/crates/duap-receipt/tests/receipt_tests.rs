//! Tests for the Data Usage Receipt, including its negative claims.

use duap_canon::{Digest, HashAlg};
use duap_crypto::*;
use duap_model::prelude::*;
use duap_provenance::{EntryKind, LogEntry, TransparencyLog};
use duap_receipt::*;

const T0: u64 = 1_750_000_000;

fn org(s: &str) -> OrgId {
    s.parse().unwrap()
}

fn event_digests(n: usize) -> Vec<Digest> {
    (0..n)
        .map(|i| Digest::of(HashAlg::Sha2_256, "duap.event.v1", &(i as u64).to_be_bytes()))
        .collect()
}

fn coverage(n: usize) -> Coverage {
    let ds = event_digests(n);
    Coverage {
        data_class: DataClass::LocationCoarse,
        operation: Operation::AccessQuery,
        purpose: Purpose::MarketingAdvertisingBehavioral,
        country: "DE".into(),
        quantity: Quantity::new(Unit::Query, n as u64),
        event_count: n as u64,
        events_root: events_root(HashAlg::Sha2_256, &ds),
    }
}

fn base_receipt(n: usize) -> Receipt {
    ReceiptBuilder::new(
        org("org:duap/clearing-eu"),
        org("org:duap/acme"),
        SubjectScope::Subject { subject: SubjectRef([1u8; 16]) },
        TimeRange::new(Timestamp::from_secs(T0), Timestamp::from_secs(T0 + 86_400)).unwrap(),
        coverage(n),
        AuthorizationRef {
            grant: GrantId([2u8; 16]),
            grant_digest: Digest::of(HashAlg::Sha2_256, "duap.grant.v1", b"g"),
            epoch: 1,
        },
        DecisionSummary {
            permitting_terms: vec![1],
            obligations: vec!["max_retention".into()],
            deferred: vec!["delete_by".into()],
        },
        Money::new(Currency::EUR, 42),
        Digest::of(HashAlg::Sha2_256, "duap.price.v1", b"p"),
        Timestamp::from_secs(T0 + 86_400),
    )
    .subject_share(Money::new(Currency::EUR, 21))
    .build()
    .unwrap()
}

fn registry(key: &SecretKey, role: KeyRole) -> KeyRegistry {
    let mut r = KeyRegistry::new();
    r.enroll(KeyRecord::new(
        key.public_key(),
        "org:duap/clearing-eu",
        vec![role],
        0,
        None,
    ))
    .unwrap();
    r
}

#[test]
fn a_receipt_round_trips_and_names_itself_by_content() {
    let r = base_receipt(10);
    let bytes = r.to_canonical().unwrap();
    assert!(duap_canon::is_canonical(&bytes));
    let back = Receipt::from_canonical(&bytes).unwrap();
    assert_eq!(back, r);
    assert_eq!(back.id().unwrap(), r.id().unwrap());
    assert!(r.id().unwrap().starts_with("rcpt1:"));

    let mut other = r.clone();
    other.charge = Money::new(Currency::EUR, 43);
    assert_ne!(other.id().unwrap(), r.id().unwrap());
}

#[test]
fn coverage_is_checkable_against_the_event_set() {
    let r = base_receipt(7);
    let ds = event_digests(7);
    r.verify_coverage(&ds).unwrap();

    // A different set of the same size must fail.
    let mut altered = ds.clone();
    altered[3] = Digest::of(HashAlg::Sha2_256, "duap.event.v1", b"substituted");
    assert!(matches!(
        r.verify_coverage(&altered),
        Err(ReceiptError::RootMismatch { .. })
    ));

    // A different size must fail.
    assert!(matches!(
        r.verify_coverage(&ds[..6]),
        Err(ReceiptError::CoverageMismatch { .. })
    ));

    // Reordering must fail: the root is over the acceptance order.
    let mut reordered = ds.clone();
    reordered.swap(0, 1);
    assert!(r.verify_coverage(&reordered).is_err());
}

#[test]
fn validation_rejects_structurally_impossible_receipts() {
    let mut r = base_receipt(3);
    r.coverage.quantity.unit = Unit::Token;
    assert!(r.validate().is_err());

    let mut r = base_receipt(3);
    r.subject_share = Some(Money::new(Currency::EUR, 1_000));
    assert!(r.validate().is_err(), "the subject share cannot exceed the charge");

    let mut r = base_receipt(3);
    r.decision.permitting_terms.clear();
    assert!(r.validate().is_err(), "a permission must be explained");

    let mut r = base_receipt(3);
    r.issued_at = Timestamp::from_secs(T0 - 1);
    assert!(r.validate().is_err());

    let mut r = base_receipt(3);
    r.protocol = "OTHER/9".into();
    assert!(r.validate().is_err());
}

#[test]
fn a_deriving_operation_must_commit_to_its_output() {
    let mut c = coverage(2);
    c.operation = Operation::ProcessProfile;
    c.quantity = Quantity::new(Unit::Inference, 2);
    let b = ReceiptBuilder::new(
        org("org:duap/clearing-eu"),
        org("org:duap/acme"),
        SubjectScope::Subject { subject: SubjectRef([1u8; 16]) },
        TimeRange::new(Timestamp::from_secs(T0), Timestamp::from_secs(T0 + 10)).unwrap(),
        c.clone(),
        AuthorizationRef {
            grant: GrantId([2u8; 16]),
            grant_digest: Digest::of(HashAlg::Sha2_256, "duap.grant.v1", b"g"),
            epoch: 1,
        },
        DecisionSummary { permitting_terms: vec![1], obligations: vec![], deferred: vec![] },
        Money::new(Currency::EUR, 1),
        Digest::of(HashAlg::Sha2_256, "duap.price.v1", b"p"),
        Timestamp::from_secs(T0 + 10),
    );
    assert!(b.build().is_err());
}

#[test]
fn verification_requires_a_receipt_signer_key() {
    let key = SecretKey::from_seed(SuiteId::Ed25519, [3u8; 32]);
    let r = base_receipt(4);
    let mut env = r.seal().unwrap();
    env.sign(&key, T0 * 1_000_000, None).unwrap();

    let ok = Receipt::verify(
        &env,
        &registry(&key, KeyRole::ReceiptSigner),
        &SuitePolicy::draft_default(),
        &VerificationContext::archival(T0 * 1_000_000 + 1),
    )
    .unwrap();
    assert_eq!(ok.receipt, r);
    assert_eq!(ok.signers, vec![key.key_id()]);
    assert!(!ok.anchored, "no anchor was attached");

    // The same key enrolled only as an event signer must not do.
    let bad = Receipt::verify(
        &env,
        &registry(&key, KeyRole::EventSigner),
        &SuitePolicy::draft_default(),
        &VerificationContext::archival(T0 * 1_000_000 + 1),
    );
    assert!(matches!(bad, Err(ReceiptError::UnauthorisedSigner(_))));
}

#[test]
fn tampering_with_a_sealed_receipt_is_detected() {
    let key = SecretKey::from_seed(SuiteId::Ed25519, [4u8; 32]);
    let r = base_receipt(4);
    let mut env = r.seal().unwrap();
    env.sign(&key, T0 * 1_000_000, None).unwrap();
    let reg = registry(&key, KeyRole::ReceiptSigner);
    let ctx = VerificationContext::archival(T0 * 1_000_000 + 1);

    for i in (0..env.payload.len()).step_by(7) {
        let mut bad = env.clone();
        bad.payload[i] ^= 0x80;
        assert!(
            Receipt::verify(&bad, &reg, &SuitePolicy::draft_default(), &ctx).is_err(),
            "tamper at byte {i} went undetected"
        );
    }
}

#[test]
fn an_anchored_receipt_verifies_its_log_position() {
    let signing = SecretKey::from_seed(SuiteId::Ed25519, [5u8; 32]);
    let r = base_receipt(5);

    let mut log = TransparencyLog::new("log:eu-1", HashAlg::Sha2_256);
    // Some unrelated entries, then ours, then more.
    for i in 0..9u64 {
        log.append(LogEntry {
            kind: EntryKind::EventBatch,
            object: Digest::of(HashAlg::Sha2_256, "duap.batch.v1", &i.to_be_bytes()),
            submitter: org("org:duap/acme"),
            sequenced_at: Timestamp::from_secs(T0),
            shard: None,
        })
        .unwrap();
    }
    let head_time = Timestamp::from_secs(T0 + 90_000);
    let entry = LogEntry {
        kind: EntryKind::Receipt,
        object: r.digest().unwrap(),
        submitter: r.issuer.clone(),
        sequenced_at: head_time,
        shard: None,
    };
    let index = log.append(entry).unwrap();
    for i in 0..4u64 {
        log.append(LogEntry {
            kind: EntryKind::EventBatch,
            object: Digest::of(HashAlg::Sha2_256, "duap.batch.v1", &(100 + i).to_be_bytes()),
            submitter: org("org:duap/acme"),
            sequenced_at: Timestamp::from_secs(T0),
            shard: None,
        })
        .unwrap();
    }
    let head = log.head(head_time);
    let anchored = ReceiptBuilder::new(
        r.issuer.clone(),
        r.controller.clone(),
        r.subject.clone(),
        r.period,
        r.coverage.clone(),
        r.authorization.clone(),
        r.decision.clone(),
        r.charge,
        r.price_digest,
        r.issued_at,
    )
    .subject_share(r.subject_share.unwrap())
    .anchor(LogAnchor {
        log_id: "log:eu-1".into(),
        index,
        proof: log.inclusion_proof(index, log.size()).unwrap(),
        head: head.clone(),
    })
    .build()
    .unwrap();

    // The anchor commits to the *unanchored* receipt's digest, which is how
    // a log entry can reference a receipt that then carries the proof.
    assert_eq!(r.digest().unwrap(), {
        let mut stripped = anchored.clone();
        stripped.anchor = None;
        stripped.digest().unwrap()
    });

    let mut env = stripped_seal(&anchored);
    env.sign(&signing, T0 * 1_000_000, None).unwrap();
    let v = Receipt::verify(
        &env,
        &registry(&signing, KeyRole::ReceiptSigner),
        &SuitePolicy::draft_default(),
        &VerificationContext::archival(T0 * 1_000_000 + 1),
    )
    .unwrap();
    assert!(v.anchored, "the inclusion proof must be checked");

    // A forged anchor pointing at a different index must fail.
    let mut forged = anchored.clone();
    if let Some(a) = &mut forged.anchor {
        a.index += 1;
    }
    let mut env = stripped_seal(&forged);
    env.sign(&signing, T0 * 1_000_000, None).unwrap();
    assert!(matches!(
        Receipt::verify(
            &env,
            &registry(&signing, KeyRole::ReceiptSigner),
            &SuitePolicy::draft_default(),
            &VerificationContext::archival(T0 * 1_000_000 + 1)
        ),
        Err(ReceiptError::AnchorInvalid)
    ));
}

fn stripped_seal(r: &Receipt) -> Envelope {
    r.seal().unwrap()
}

/// The negative claims are part of the artefact, not a footnote.
#[test]
fn claims_distinguish_what_is_and_is_not_established() {
    let r = base_receipt(3);
    let claims = r.claims();
    let established: Vec<&Claim> = claims.iter().filter(|c| c.established).collect();
    let not: Vec<&Claim> = claims.iter().filter(|c| !c.established).collect();

    assert_eq!(established.len(), 4, "an unanchored receipt establishes four things");
    assert!(not.len() >= 6);
    assert!(
        not.iter().any(|c| c.statement.contains("actually took place")),
        "the receipt must say that it does not prove the operation happened"
    );
    assert!(not.iter().all(|c| c.basis.contains("NOT ESTABLISHED")));

    // Anchoring converts exactly one claim.
    let mut anchored = r.clone();
    anchored.anchor = Some(LogAnchor {
        log_id: "log:eu-1".into(),
        index: 0,
        proof: duap_provenance::InclusionProof {
            index: 0,
            size: 1,
            path: vec![],
            alg: HashAlg::Sha2_256,
        },
        head: duap_provenance::TreeHead {
            log_id: "log:eu-1".into(),
            size: 1,
            root: Digest::of(HashAlg::Sha2_256, "x", b"y"),
            issued_at: Timestamp::from_secs(T0),
            alg: HashAlg::Sha2_256,
        },
    });
    assert_eq!(anchored.claims().iter().filter(|c| c.established).count(), 5);
}
