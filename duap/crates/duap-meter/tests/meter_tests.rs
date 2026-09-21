//! Tests for metering.

use duap_canon::HashAlg;
use duap_meter::*;
use duap_model::prelude::*;
use proptest::prelude::*;

const T0: u64 = 1_750_000_000;

fn controller() -> OrgId {
    "org:duap/acme".parse().unwrap()
}

fn auth() -> AuthorizationRef {
    AuthorizationRef {
        grant: GrantId([1u8; 16]),
        grant_digest: duap_canon::Digest::of(HashAlg::Sha2_256, "g", b"g"),
        epoch: 1,
    }
}

fn ev_at(id: u8, secs: u64) -> DataUsageEvent {
    EventBuilder::new(
        EventId([id; 16]),
        AgentRef::new("test", "0"),
        controller(),
        SubjectScope::Subject {
            subject: SubjectRef([7u8; 16]),
        },
        Jurisdiction::new("DE").unwrap(),
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
        auth(),
        1,
        Timestamp::from_secs(secs),
    )
    .build()
    .unwrap()
}

fn now() -> Timestamp {
    Timestamp::from_secs(T0 + 60)
}

// ---------------------------------------------------------------------------
// Dedup
// ---------------------------------------------------------------------------

#[test]
fn retransmission_is_idempotent() {
    let mut idx = DedupIndex::new(WindowConfig::default());
    let e = ev_at(1, T0);
    let d = e.digest().unwrap();
    assert_eq!(idx.admit(&e, d, now()), Admission::Accepted);
    match idx.admit(&e, d, now()) {
        Admission::Rejected(RejectReason::Duplicate { .. }) => {}
        other => panic!("expected duplicate, got {other:?}"),
    }
    assert_eq!(idx.len(), 1);
}

#[test]
fn replay_outside_the_window_is_rejected() {
    let mut idx = DedupIndex::new(WindowConfig {
        max_age: 3600 * 1_000_000,
        max_skew: 60 * 1_000_000,
    });
    let old = ev_at(1, T0 - 7200);
    assert!(matches!(
        idx.admit(&old, old.digest().unwrap(), Timestamp::from_secs(T0)),
        Admission::Rejected(RejectReason::TooOld { .. })
    ));
    let future = ev_at(2, T0 + 3600);
    assert!(matches!(
        idx.admit(&future, future.digest().unwrap(), Timestamp::from_secs(T0)),
        Admission::Rejected(RejectReason::TooNew { .. })
    ));
}

#[test]
fn sequence_gaps_are_detected() {
    let mut idx = DedupIndex::new(WindowConfig::default());
    let stream = ContentId::of_bytes("duap.stream.v1", b"agent-1");
    for i in [0u64, 1, 2, 5, 6] {
        let e = EventBuilder::new(
            EventId([i as u8; 16]),
            AgentRef::new("test", "0"),
            controller(),
            SubjectScope::Subject {
                subject: SubjectRef([7u8; 16]),
            },
            Jurisdiction::new("DE").unwrap(),
            DataClass::LocationCoarse,
            Operation::AccessQuery,
            Purpose::ServiceCore,
            auth(),
            1,
            Timestamp::from_secs(T0 + i),
        )
        .sequence(EventSequence { stream, index: i })
        .build()
        .unwrap();
        assert_eq!(
            idx.admit(&e, e.digest().unwrap(), now()),
            Admission::Accepted
        );
    }
    let gaps = idx.gaps();
    assert_eq!(gaps.len(), 1);
    assert_eq!(gaps[0].missing, vec![3, 4]);
    assert_eq!(gaps[0].high_water, 6);
}

#[test]
fn a_late_arrival_closes_its_gap() {
    let mut idx = DedupIndex::new(WindowConfig::default());
    let stream = ContentId::of_bytes("duap.stream.v1", b"agent-1");
    let mk = |i: u64, tag: u8| {
        EventBuilder::new(
            EventId([tag; 16]),
            AgentRef::new("test", "0"),
            controller(),
            SubjectScope::Subject {
                subject: SubjectRef([7u8; 16]),
            },
            Jurisdiction::new("DE").unwrap(),
            DataClass::LocationCoarse,
            Operation::AccessQuery,
            Purpose::ServiceCore,
            auth(),
            1,
            Timestamp::from_secs(T0 + i),
        )
        .sequence(EventSequence { stream, index: i })
        .build()
        .unwrap()
    };
    for (i, tag) in [(0u64, 10u8), (2, 12)] {
        let e = mk(i, tag);
        idx.admit(&e, e.digest().unwrap(), now());
    }
    assert_eq!(idx.gaps()[0].missing, vec![1]);
    let late = mk(1, 11);
    assert_eq!(
        idx.admit(&late, late.digest().unwrap(), now()),
        Admission::Accepted
    );
    assert!(idx.gaps().is_empty());
}

#[test]
fn a_reused_sequence_index_with_different_content_is_a_conflict() {
    let mut idx = DedupIndex::new(WindowConfig::default());
    let stream = ContentId::of_bytes("duap.stream.v1", b"agent-1");
    let mk = |tag: u8| {
        EventBuilder::new(
            EventId([tag; 16]),
            AgentRef::new("test", "0"),
            controller(),
            SubjectScope::Subject {
                subject: SubjectRef([7u8; 16]),
            },
            Jurisdiction::new("DE").unwrap(),
            DataClass::LocationCoarse,
            Operation::AccessQuery,
            Purpose::ServiceCore,
            auth(),
            1,
            Timestamp::from_secs(T0),
        )
        .sequence(EventSequence { stream, index: 4 })
        .build()
        .unwrap()
    };
    let a = mk(1);
    idx.admit(&a, a.digest().unwrap(), now());
    let b = mk(2);
    assert!(matches!(
        idx.admit(&b, b.digest().unwrap(), now()),
        Admission::Rejected(RejectReason::SequenceConflict { .. })
    ));
}

#[test]
fn eviction_bounds_the_index() {
    let mut idx = DedupIndex::new(WindowConfig::default());
    for i in 0..50u8 {
        let e = ev_at(i, T0 + i as u64);
        idx.admit(&e, e.digest().unwrap(), Timestamp::from_secs(T0 + i as u64));
    }
    assert_eq!(idx.len(), 50);
    let dropped = idx.evict_before(Timestamp::from_secs(T0 + 25));
    assert_eq!(dropped, 25);
    assert_eq!(idx.len(), 25);
}

// ---------------------------------------------------------------------------
// Double counting
// ---------------------------------------------------------------------------

fn same_operation_two_reporters() -> (DataUsageEvent, DataUsageEvent) {
    let base = |processor: Option<OrgId>, agent: &str| {
        let mut b = EventBuilder::new(
            EventId([if processor.is_some() { 2 } else { 1 }; 16]),
            AgentRef::new(agent, "0"),
            controller(),
            SubjectScope::Subject {
                subject: SubjectRef([7u8; 16]),
            },
            Jurisdiction::new("DE").unwrap(),
            DataClass::LocationCoarse,
            Operation::AccessQuery,
            Purpose::ServiceCore,
            auth(),
            1,
            Timestamp::from_secs(T0),
        );
        if let Some(p) = processor {
            b = b.processor(p);
        }
        b.build().unwrap()
    };
    (
        base(None, "controller-agent"),
        base(Some("org:duap/vendor".parse().unwrap()), "vendor-agent"),
    )
}

#[test]
fn two_reporters_of_one_operation_collide() {
    let (a, b) = same_operation_two_reporters();
    assert_eq!(fingerprint(&a, 1_000), fingerprint(&b, 1_000));
    assert_ne!(a.digest().unwrap(), b.digest().unwrap());
}

#[test]
fn controller_wins_reverses_the_processor_report() {
    let (controller_ev, processor_ev) = same_operation_two_reporters();
    let mut d = DoubleCountDetector::new(ResolutionPolicy::ControllerWins);
    assert_eq!(
        d.offer(&processor_ev, processor_ev.digest().unwrap()),
        CountDecision::Count
    );
    match d.offer(&controller_ev, controller_ev.digest().unwrap()) {
        CountDecision::CountAndReverse { reversed, .. } => {
            assert_eq!(reversed, processor_ev.digest().unwrap())
        }
        other => panic!("expected reversal, got {other:?}"),
    }
    // The other order: the controller's report is already the incumbent, so
    // the processor's is suppressed.
    let mut d = DoubleCountDetector::new(ResolutionPolicy::ControllerWins);
    d.offer(&controller_ev, controller_ev.digest().unwrap());
    assert!(matches!(
        d.offer(&processor_ev, processor_ev.digest().unwrap()),
        CountDecision::Suppress(_)
    ));
}

#[test]
fn first_wins_and_suspend_both_policies() {
    let (a, b) = same_operation_two_reporters();
    let mut d = DoubleCountDetector::new(ResolutionPolicy::FirstWins);
    d.offer(&a, a.digest().unwrap());
    assert!(matches!(
        d.offer(&b, b.digest().unwrap()),
        CountDecision::Suppress(_)
    ));

    let mut d = DoubleCountDetector::new(ResolutionPolicy::SuspendBoth);
    d.offer(&a, a.digest().unwrap());
    assert!(matches!(
        d.offer(&b, b.digest().unwrap()),
        CountDecision::SuspendBoth(_)
    ));
}

#[test]
fn distinct_operations_do_not_collide() {
    let a = ev_at(1, T0);
    let mut b = ev_at(2, T0);
    b.purpose = Purpose::SecurityFraud;
    assert_ne!(fingerprint(&a, 1_000), fingerprint(&b, 1_000));

    let c = ev_at(3, T0 + 10);
    assert_ne!(fingerprint(&a, 1_000), fingerprint(&c, 1_000));
}

// ---------------------------------------------------------------------------
// Aggregation
// ---------------------------------------------------------------------------

#[test]
fn aggregation_groups_by_pricing_dimensions() {
    let mut agg = Aggregator::new(WindowSize::Hour);
    for i in 0..10u8 {
        let e = ev_at(i, T0 + i as u64);
        agg.add(&e, e.digest().unwrap()).unwrap();
    }
    assert_eq!(agg.len(), 1);
    let key = agg.key_for(&ev_at(0, T0));
    let c = agg.get(&key).unwrap();
    assert_eq!(c.quantity, 10);
    assert_eq!(c.event_count, 10);
    assert_eq!(c.evidence_size, 10);

    // A different purpose lands in a different bucket.
    let mut other = ev_at(20, T0);
    other.purpose = Purpose::SecurityFraud;
    agg.add(&other, other.digest().unwrap()).unwrap();
    assert_eq!(agg.len(), 2);
}

#[test]
fn aggregation_refuses_non_additive_units() {
    let mut agg = Aggregator::new(WindowSize::Hour);
    let mut e = ev_at(1, T0);
    e.quantity = Quantity {
        unit: Unit::Share,
        amount: 1,
    };
    assert!(matches!(
        agg.add(&e, e.digest().unwrap()),
        Err(AggregateError::NonAdditive(_))
    ));
}

#[test]
fn window_boundaries_split_counters() {
    let mut agg = Aggregator::new(WindowSize::Hour);
    let a = ev_at(1, T0);
    let b = ev_at(2, T0 + 3600 * 2);
    agg.add(&a, a.digest().unwrap()).unwrap();
    agg.add(&b, b.digest().unwrap()).unwrap();
    assert_eq!(agg.len(), 2);

    let closed = agg.drain_closed(Timestamp::from_secs(T0 + 3600 * 2));
    assert_eq!(closed.len(), 1, "only the earlier window has closed");
    assert_eq!(agg.len(), 1);
}

#[test]
fn reversal_reduces_the_count_but_advances_the_evidence() {
    let mut agg = Aggregator::new(WindowSize::Hour);
    let a = ev_at(1, T0);
    let b = ev_at(2, T0);
    agg.add(&a, a.digest().unwrap()).unwrap();
    agg.add(&b, b.digest().unwrap()).unwrap();
    let key = agg.key_for(&a);
    let before = agg.get(&key).unwrap().clone();
    agg.reverse(&b, b.digest().unwrap()).unwrap();
    let after = agg.get(&key).unwrap();
    assert_eq!(after.quantity, before.quantity - 1);
    assert_eq!(after.event_count, before.event_count - 1);
    assert_ne!(
        after.evidence_root, before.evidence_root,
        "a reversal must be visible in the evidence, not erase history"
    );
    assert_eq!(after.evidence_size, before.evidence_size + 1);
}

/// The agent's incremental accumulator must agree with the log's recursive
/// construction, or an agent and the clearing node would compute different
/// evidence roots for the same events.
#[test]
fn evidence_root_matches_the_transparency_log_construction() {
    for n in [0usize, 1, 2, 3, 5, 8, 13, 64, 100] {
        let mut inc = duap_meter::evidence::MerkleLog::new(HashAlg::Sha2_256);
        let mut leaves = Vec::new();
        for i in 0..n {
            let data = format!("leaf-{i}");
            inc.append(data.as_bytes());
            leaves.push(duap_provenance::merkle::leaf_hash(
                HashAlg::Sha2_256,
                data.as_bytes(),
            ));
        }
        let recursive = duap_provenance::merkle::root_of(HashAlg::Sha2_256, &leaves);
        assert_eq!(inc.root(), recursive, "mismatch at n={n}");
    }
}

// ---------------------------------------------------------------------------
// Pipeline
// ---------------------------------------------------------------------------

#[test]
fn pipeline_counts_once_per_event() {
    let mut p = MeterPipeline::new(
        WindowConfig::default(),
        WindowSize::Hour,
        ResolutionPolicy::ControllerWins,
    );
    let e = ev_at(1, T0);
    assert!(p.offer(&e, now()).unwrap().counted());
    assert!(!p.offer(&e, now()).unwrap().counted());
    assert_eq!(p.stats.counted, 1);
    assert_eq!(p.stats.duplicates, 1);
    assert_eq!(p.aggregator.total(Unit::Query), 1);
}

#[test]
fn pipeline_reverses_on_controller_wins() {
    let (controller_ev, processor_ev) = same_operation_two_reporters();
    let mut p = MeterPipeline::new(
        WindowConfig::default(),
        WindowSize::Hour,
        ResolutionPolicy::ControllerWins,
    );
    assert!(p.offer(&processor_ev, now()).unwrap().counted());
    assert_eq!(p.aggregator.total(Unit::Query), 1);
    let out = p.offer(&controller_ev, now()).unwrap();
    assert!(matches!(out, MeterOutcome::CountedWithReversal { .. }));
    assert_eq!(
        p.aggregator.total(Unit::Query),
        1,
        "the operation must be counted exactly once after the reversal"
    );
    assert_eq!(p.stats.reversals, 1);
}

#[test]
fn pipeline_rejects_invalid_events() {
    let mut p = MeterPipeline::new(
        WindowConfig::default(),
        WindowSize::Hour,
        ResolutionPolicy::FirstWins,
    );
    let mut e = ev_at(1, T0);
    e.quantity.unit = Unit::Token; // wrong meter for access.query
    assert!(matches!(
        p.offer(&e, now()).unwrap(),
        MeterOutcome::Invalid(_)
    ));
    assert_eq!(p.stats.invalid, 1);
}

// ---------------------------------------------------------------------------
// Properties
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    /// However many times an event is offered, it is counted once.
    #[test]
    fn counting_is_idempotent(times in 1usize..12) {
        let mut p = MeterPipeline::new(WindowConfig::default(), WindowSize::Hour, ResolutionPolicy::FirstWins);
        let e = ev_at(1, T0);
        for _ in 0..times {
            p.offer(&e, now()).unwrap();
        }
        prop_assert_eq!(p.stats.counted, 1);
        prop_assert_eq!(p.aggregator.total(Unit::Query), 1);
    }

    /// The aggregate total equals the number of distinct accepted events.
    #[test]
    fn totals_match_distinct_events(n in 1u8..40) {
        let mut p = MeterPipeline::new(WindowConfig::default(), WindowSize::Day, ResolutionPolicy::FirstWins);
        for i in 0..n {
            let e = ev_at(i, T0 + i as u64);
            p.offer(&e, Timestamp::from_secs(T0 + 100)).unwrap();
        }
        prop_assert_eq!(p.aggregator.total(Unit::Query), n as u128);
        prop_assert_eq!(p.stats.counted, n as u64);
    }

    /// Fingerprints ignore who reported, and nothing else.
    #[test]
    fn fingerprint_ignores_reporter_only(secs in 0u64..100) {
        let a = ev_at(1, T0 + secs);
        let mut b = a.clone();
        b.processor = Some("org:duap/vendor".parse().unwrap());
        b.agent = AgentRef::new("other", "9");
        b.id = EventId([99u8; 16]);
        prop_assert_eq!(fingerprint(&a, 1_000), fingerprint(&b, 1_000));

        let mut c = a.clone();
        c.quantity.amount = a.quantity.amount + 1;
        prop_assert_ne!(fingerprint(&a, 1_000), fingerprint(&c, 1_000));
    }

    /// The incremental and recursive Merkle constructions always agree.
    #[test]
    fn evidence_roots_agree(n in 0usize..80) {
        let mut inc = duap_meter::evidence::MerkleLog::new(HashAlg::Sha2_256);
        let mut leaves = Vec::new();
        for i in 0..n {
            let d = i.to_be_bytes();
            inc.append(&d);
            leaves.push(duap_provenance::merkle::leaf_hash(HashAlg::Sha2_256, &d));
        }
        prop_assert_eq!(inc.root(), duap_provenance::merkle::root_of(HashAlg::Sha2_256, &leaves));
    }
}
