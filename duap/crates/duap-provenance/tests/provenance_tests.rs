//! Tests for the transparency log and derivation graph.

use duap_canon::HashAlg;
use duap_crypto::{KeyRecord, KeyRegistry, KeyRole, SecretKey, SuiteId, SuitePolicy, VerificationContext};
use duap_model::prelude::*;
use duap_provenance::*;
use proptest::prelude::*;

// ---------------------------------------------------------------------------
// Merkle tree
// ---------------------------------------------------------------------------

fn build(n: usize) -> MerkleLog {
    let mut l = MerkleLog::new(HashAlg::Sha2_256);
    for i in 0..n {
        l.append(format!("entry-{i}").as_bytes());
    }
    l
}

#[test]
fn empty_and_single_trees() {
    let l = MerkleLog::new(HashAlg::Sha2_256);
    assert_eq!(l.root(), duap_provenance::merkle::empty_root(HashAlg::Sha2_256));
    let l = build(1);
    assert_eq!(l.root(), l.leaf(0).unwrap());
}

/// Leaf and interior hashes must be domain separated, or a subtree root can
/// be presented as a leaf (second-preimage attack on the tree).
#[test]
fn leaf_and_node_hashes_are_separated() {
    let a = leaf_hash(HashAlg::Sha2_256, b"x");
    let b = leaf_hash(HashAlg::Sha2_256, b"y");
    let parent = node_hash(HashAlg::Sha2_256, &a, &b);
    // The parent must not equal the leaf hash of the concatenation of the
    // children, which is the classic confusion.
    let mut cat = Vec::new();
    cat.extend_from_slice(&a.bytes);
    cat.extend_from_slice(&b.bytes);
    assert_ne!(parent, leaf_hash(HashAlg::Sha2_256, &cat));
}

#[test]
fn inclusion_proofs_verify_for_every_index() {
    for n in [1usize, 2, 3, 4, 5, 7, 8, 9, 16, 17, 100, 257] {
        let l = build(n);
        let root = l.root();
        for i in 0..n as u64 {
            let p = l.inclusion_proof(i, n as u64).expect("proof exists");
            let leaf = l.leaf(i).unwrap();
            assert!(p.verify(leaf, root), "n={n} i={i}");
        }
    }
}

#[test]
fn inclusion_proofs_reject_wrong_leaf_and_root() {
    let l = build(64);
    let root = l.root();
    let p = l.inclusion_proof(7, 64).unwrap();
    assert!(p.verify(l.leaf(7).unwrap(), root));
    assert!(!p.verify(l.leaf(8).unwrap(), root));
    let other = build(65).root();
    assert!(!p.verify(l.leaf(7).unwrap(), other));

    // A tampered path element must fail.
    let mut bad = p.clone();
    bad.path[0].bytes[0] ^= 0xff;
    assert!(!bad.verify(l.leaf(7).unwrap(), root));

    // A truncated or extended path must fail.
    let mut short = p.clone();
    short.path.pop();
    assert!(!short.verify(l.leaf(7).unwrap(), root));
    let mut long = p.clone();
    long.path.push(l.leaf(0).unwrap());
    assert!(!long.verify(l.leaf(7).unwrap(), root));
}

#[test]
fn historical_roots_are_recoverable() {
    let l = build(100);
    let l30 = build(30);
    assert_eq!(l.root_at(30).unwrap(), l30.root());
}

#[test]
fn consistency_proofs_verify() {
    for (old, new) in [(1u64, 2u64), (1, 8), (3, 8), (4, 8), (6, 8), (7, 8), (2, 100), (50, 100), (99, 100), (100, 100)] {
        let l = build(new as usize);
        let p = l.consistency_proof(old, new).expect("proof exists");
        let old_root = l.root_at(old).unwrap();
        let new_root = l.root_at(new).unwrap();
        assert!(p.verify(old_root, new_root), "old={old} new={new}");
    }
}

#[test]
fn consistency_proof_catches_a_rewritten_history() {
    // Two logs that agree on the first 40 entries, then diverge.
    let mut a = build(40);
    let mut b = build(40);
    assert_eq!(a.root(), b.root());
    a.append(b"honest-41");
    b.append(b"tampered-41");
    assert_ne!(a.root(), b.root());

    // A proof from the honest log does not validate the tampered root.
    let p = a.consistency_proof(40, 41).unwrap();
    assert!(p.verify(a.root_at(40).unwrap(), a.root()));
    assert!(!p.verify(a.root_at(40).unwrap(), b.root()));
}

#[test]
fn log_monitor_rejects_regression_and_forks() {
    let mut log = TransparencyLog::new("log:test", HashAlg::Sha2_256);
    let org: OrgId = "org:duap/acme".parse().unwrap();
    let mut mon = LogMonitor::new("log:test");

    for i in 0..10u64 {
        log.append(LogEntry {
            kind: EntryKind::Receipt,
            object: duap_canon::Digest::of(HashAlg::Sha2_256, "duap.receipt.v1", &i.to_be_bytes()),
            submitter: org.clone(),
            sequenced_at: Timestamp::from_secs(1_700_000_000 + i),
            shard: None,
        })
        .unwrap();
    }
    let h10 = log.head(Timestamp::from_secs(1_700_000_100));
    mon.advance(h10.clone(), None).unwrap();

    for i in 10..20u64 {
        log.append(LogEntry {
            kind: EntryKind::Receipt,
            object: duap_canon::Digest::of(HashAlg::Sha2_256, "duap.receipt.v1", &i.to_be_bytes()),
            submitter: org.clone(),
            sequenced_at: Timestamp::from_secs(1_700_000_000 + i),
            shard: None,
        })
        .unwrap();
    }
    let h20 = log.head(Timestamp::from_secs(1_700_000_200));
    let proof = log.consistency_proof(10, 20).unwrap();
    mon.advance(h20.clone(), Some(&proof)).unwrap();

    // Going backwards is refused.
    assert!(matches!(
        mon.advance(h10.clone(), None),
        Err(LogError::Regression { .. })
    ));

    // A same-size head with a different root is a fork.
    let mut forked = h20.clone();
    forked.root.bytes[0] ^= 0xff;
    assert!(matches!(
        mon.advance(forked, None),
        Err(LogError::Inconsistent { .. })
    ));

    // Advancing without a proof is refused.
    let h21 = TreeHead { size: 21, ..h20.clone() };
    assert!(matches!(
        mon.advance(h21, None),
        Err(LogError::Inconsistent { .. })
    ));
}

#[test]
fn inclusion_witness_verifies_end_to_end() {
    let key = SecretKey::from_seed(SuiteId::Ed25519, [3u8; 32]);
    let mut registry = KeyRegistry::new();
    registry
        .enroll(KeyRecord::new(
            key.public_key(),
            "org:duap/log",
            vec![KeyRole::LogSigner],
            0,
            None,
        ))
        .unwrap();

    let mut log = TransparencyLog::new("log:eu-1", HashAlg::Sha2_256);
    let org: OrgId = "org:duap/acme".parse().unwrap();
    let mut target = None;
    for i in 0..25u64 {
        let e = LogEntry {
            kind: EntryKind::Receipt,
            object: duap_canon::Digest::of(HashAlg::Sha2_256, "duap.receipt.v1", &i.to_be_bytes()),
            submitter: org.clone(),
            sequenced_at: Timestamp::from_secs(1_700_000_000 + i),
            shard: Some("eu".into()),
        };
        let idx = log.append(e.clone()).unwrap();
        if i == 13 {
            target = Some((idx, e));
        }
    }
    let (idx, entry) = target.unwrap();
    let at = Timestamp::from_secs(1_700_001_000);
    let env = log.sign_head(&key, at).unwrap();
    let witness = InclusionWitness {
        entry,
        proof: log.inclusion_proof(idx, log.size()).unwrap(),
        head: log.head(at),
    };
    let signers = witness
        .verify(
            &env,
            &registry,
            &SuitePolicy::draft_default(),
            &VerificationContext::archival(at.0),
        )
        .unwrap();
    assert_eq!(signers, vec![key.key_id()]);

    // A witness whose entry has been altered fails.
    let mut bad = witness.clone();
    bad.entry.sequenced_at = Timestamp::from_secs(1);
    assert!(matches!(
        bad.verify(&env, &registry, &SuitePolicy::draft_default(), &VerificationContext::archival(at.0)),
        Err(LogError::NotIncluded { .. })
    ));
}

#[test]
fn a_key_without_the_log_role_cannot_sign_heads() {
    let key = SecretKey::from_seed(SuiteId::Ed25519, [4u8; 32]);
    let mut registry = KeyRegistry::new();
    registry
        .enroll(KeyRecord::new(
            key.public_key(),
            "org:duap/acme",
            vec![KeyRole::EventSigner], // not LogSigner
            0,
            None,
        ))
        .unwrap();
    let mut log = TransparencyLog::new("log:eu-1", HashAlg::Sha2_256);
    let e = LogEntry {
        kind: EntryKind::Receipt,
        object: duap_canon::Digest::of(HashAlg::Sha2_256, "duap.receipt.v1", b"x"),
        submitter: "org:duap/acme".parse().unwrap(),
        sequenced_at: Timestamp::from_secs(1),
        shard: None,
    };
    log.append(e.clone()).unwrap();
    let at = Timestamp::from_secs(10);
    let env = log.sign_head(&key, at).unwrap();
    let w = InclusionWitness {
        entry: e,
        proof: log.inclusion_proof(0, 1).unwrap(),
        head: log.head(at),
    };
    assert!(matches!(
        w.verify(&env, &registry, &SuitePolicy::draft_default(), &VerificationContext::archival(at.0)),
        Err(LogError::BadSignature(_))
    ));
}

// ---------------------------------------------------------------------------
// Derivation graph
// ---------------------------------------------------------------------------

fn cid(s: &str) -> ContentId {
    ContentId::of_bytes("duap.object.v1", s.as_bytes())
}

fn org() -> OrgId {
    "org:duap/acme".parse().unwrap()
}

fn t() -> Timestamp {
    Timestamp::from_secs(1_700_000_000)
}

/// Build: three subjects' source records -> a dataset -> a model -> an output.
fn sample_graph() -> (ProvenanceGraph, Vec<SubjectRef>) {
    let mut g = ProvenanceGraph::new();
    let subjects: Vec<SubjectRef> = (0..3).map(|i| SubjectRef([i as u8; 16])).collect();
    for (i, s) in subjects.iter().enumerate() {
        g.insert(ProvNode::source(cid(&format!("src{i}")), org(), *s, t()))
            .unwrap();
    }
    let third = Ratio::new(1, 3).unwrap();
    g.insert(ProvNode::derived(
        cid("dataset"),
        NodeKind::Dataset,
        org(),
        t(),
        (0..3)
            .map(|i| Edge::uniform(cid(&format!("src{i}")), third))
            .collect(),
        Operation::ProcessAggregate,
    ))
    .unwrap();
    g.insert(ProvNode::derived(
        cid("model"),
        NodeKind::Model,
        org(),
        t(),
        vec![Edge::uniform(cid("dataset"), Ratio::ONE)],
        Operation::AiFinetune,
    ))
    .unwrap();
    g.insert(ProvNode::derived(
        cid("output"),
        NodeKind::Output,
        org(),
        t(),
        vec![Edge::uniform(cid("model"), Ratio::ONE)],
        Operation::AiInferenceInput,
    ))
    .unwrap();
    (g, subjects)
}

#[test]
fn attribution_flows_through_the_graph() {
    let (g, subjects) = sample_graph();
    let (shares, dropped) = g
        .attribution(&cid("output"), &DerivationPolicy::default())
        .unwrap();
    assert_eq!(shares.len(), 3);
    for s in &subjects {
        assert_eq!(shares[s], Ratio::new(1, 3).unwrap());
    }
    assert_eq!(dropped, Ratio::ZERO);
    // Shares sum to exactly 1.
    let total = shares.values().fold(Ratio::ZERO, |a, b| a.add(*b).unwrap());
    assert_eq!(total, Ratio::ONE);
}

#[test]
fn attribution_sums_over_multiple_paths() {
    // One subject feeds two branches that recombine.
    let mut g = ProvenanceGraph::new();
    let s = SubjectRef([9u8; 16]);
    g.insert(ProvNode::source(cid("src"), org(), s, t())).unwrap();
    for b in ["a", "b"] {
        g.insert(ProvNode::derived(
            cid(b),
            NodeKind::Derived,
            org(),
            t(),
            vec![Edge::uniform(cid("src"), Ratio::ONE)],
            Operation::ProcessTransform,
        ))
        .unwrap();
    }
    let half = Ratio::new(1, 2).unwrap();
    g.insert(ProvNode::derived(
        cid("join"),
        NodeKind::Derived,
        org(),
        t(),
        vec![Edge::uniform(cid("a"), half), Edge::uniform(cid("b"), half)],
        Operation::ProcessMatch,
    ))
    .unwrap();
    let (shares, _) = g.attribution(&cid("join"), &DerivationPolicy::default()).unwrap();
    assert_eq!(shares[&s], Ratio::ONE);
}

#[test]
fn severed_edges_stop_attribution_but_keep_lineage() {
    let (mut g, subjects) = sample_graph();
    let mut node = g.get(&cid("model")).unwrap().clone();
    node.inputs[0].severed = true;
    node.inputs[0].basis = WeightBasis::Adjudicated;
    // Re-insert under a new id, since the old one is already fixed.
    let severed_id = cid("model-severed");
    node.id = severed_id;
    g.insert(node).unwrap();
    let (shares, dropped) = g.attribution(&severed_id, &DerivationPolicy::default()).unwrap();
    assert!(shares.is_empty());
    assert_eq!(dropped, Ratio::ONE);
    // Lineage is still visible even though attribution does not flow.
    assert_eq!(g.get(&severed_id).unwrap().inputs[0].from, cid("dataset"));
    assert_eq!(subjects.len(), 3);
}

#[test]
fn dp_release_terminates_attribution_when_policy_says_so() {
    let mut g = ProvenanceGraph::new();
    let s = SubjectRef([1u8; 16]);
    g.insert(ProvNode::source(cid("src"), org(), s, t())).unwrap();
    let mut dp = ProvNode::derived(
        cid("dp"),
        NodeKind::DpRelease,
        org(),
        t(),
        vec![Edge::uniform(cid("src"), Ratio::ONE)],
        Operation::ProcessDpRelease,
    );
    dp.epsilon_micro = Some(100_000); // epsilon 0.1
    g.insert(dp).unwrap();

    // Default policy: DP does not terminate, so the subject is attributed.
    let (shares, _) = g.attribution(&cid("dp"), &DerivationPolicy::default()).unwrap();
    assert_eq!(shares[&s], Ratio::ONE);

    // Policy that terminates at epsilon <= 0.5.
    let policy = DerivationPolicy {
        dp_terminates_at_or_below_micro: Some(500_000),
        ..Default::default()
    };
    let (shares, dropped) = g.attribution(&cid("dp"), &policy).unwrap();
    assert!(shares.is_empty());
    assert_eq!(dropped, Ratio::ONE);
}

#[test]
fn tiny_shares_are_dropped_and_accounted() {
    let mut g = ProvenanceGraph::new();
    let s = SubjectRef([1u8; 16]);
    g.insert(ProvNode::source(cid("src"), org(), s, t())).unwrap();
    g.insert(ProvNode::derived(
        cid("d"),
        NodeKind::Derived,
        org(),
        t(),
        vec![Edge::uniform(cid("src"), Ratio::new(1, 1_000_000_000).unwrap())],
        Operation::ProcessTransform,
    ))
    .unwrap();
    let (shares, dropped) = g.attribution(&cid("d"), &DerivationPolicy::default()).unwrap();
    assert!(shares.is_empty(), "a nanoshare must not create a claim");
    assert_eq!(dropped, Ratio::new(1, 1_000_000_000).unwrap());
}

#[test]
fn weight_sum_is_checked_not_normalised() {
    let mut g = ProvenanceGraph::new();
    g.insert(ProvNode::source(cid("a"), org(), SubjectRef([1u8; 16]), t())).unwrap();
    g.insert(ProvNode::source(cid("b"), org(), SubjectRef([2u8; 16]), t())).unwrap();
    let half = Ratio::new(1, 2).unwrap();
    g.insert(ProvNode::derived(
        cid("ok"),
        NodeKind::Derived,
        org(),
        t(),
        vec![Edge::uniform(cid("a"), half), Edge::uniform(cid("b"), half)],
        Operation::ProcessAggregate,
    ))
    .unwrap();
    assert!(g.check_weights(&cid("ok")).is_ok());

    g.insert(ProvNode::derived(
        cid("inflated"),
        NodeKind::Derived,
        org(),
        t(),
        vec![Edge::uniform(cid("a"), Ratio::ONE), Edge::uniform(cid("b"), Ratio::ONE)],
        Operation::ProcessAggregate,
    ))
    .unwrap();
    assert!(
        matches!(g.check_weights(&cid("inflated")), Err(GraphError::WeightsDoNotSum { .. })),
        "doubling the weights must be reported, not silently normalised"
    );
}

#[test]
fn depth_and_descendants() {
    let (g, _) = sample_graph();
    assert_eq!(g.depth(&cid("src0")).unwrap(), 0);
    assert_eq!(g.depth(&cid("dataset")).unwrap(), 1);
    assert_eq!(g.depth(&cid("model")).unwrap(), 2);
    assert_eq!(g.depth(&cid("output")).unwrap(), 3);
    let d = g.descendants(&cid("src0"));
    assert_eq!(d.len(), 3);
}

#[test]
fn depth_limit_is_enforced() {
    let mut g = ProvenanceGraph::new();
    g.insert(ProvNode::source(cid("s"), org(), SubjectRef([1u8; 16]), t())).unwrap();
    let mut prev = cid("s");
    for i in 0..20 {
        let id = cid(&format!("n{i}"));
        g.insert(ProvNode::derived(
            id,
            NodeKind::Derived,
            org(),
            t(),
            vec![Edge::uniform(prev, Ratio::ONE)],
            Operation::ProcessTransform,
        ))
        .unwrap();
        prev = id;
    }
    let policy = DerivationPolicy { max_depth: 5, ..Default::default() };
    assert!(matches!(
        g.attribution(&prev, &policy),
        Err(GraphError::DepthExceeded(5))
    ));
}

#[test]
fn unknown_inputs_and_self_edges_are_refused() {
    let mut g = ProvenanceGraph::new();
    let n = ProvNode::derived(
        cid("x"),
        NodeKind::Derived,
        org(),
        t(),
        vec![Edge::uniform(cid("missing"), Ratio::ONE)],
        Operation::ProcessTransform,
    );
    assert!(matches!(g.insert(n), Err(GraphError::Unknown(_))));

    let n = ProvNode::derived(
        cid("y"),
        NodeKind::Derived,
        org(),
        t(),
        vec![Edge::uniform(cid("y"), Ratio::ONE)],
        Operation::ProcessTransform,
    );
    assert!(matches!(g.insert(n), Err(GraphError::Cycle { .. })));
}

#[test]
fn conflicting_content_for_the_same_id_is_refused() {
    let mut g = ProvenanceGraph::new();
    let a = ProvNode::source(cid("s"), org(), SubjectRef([1u8; 16]), t());
    let mut b = a.clone();
    b.subject = Some(SubjectRef([2u8; 16]));
    g.insert(a.clone()).unwrap();
    g.insert(a).unwrap(); // idempotent
    assert!(matches!(g.insert(b), Err(GraphError::Conflict(_))));
}

#[test]
fn paths_from_subject_are_reported_for_disputes() {
    let (g, subjects) = sample_graph();
    let paths = g.paths_from_subject(&cid("output"), subjects[1], 10);
    assert_eq!(paths.len(), 1);
    assert_eq!(
        paths[0],
        vec![cid("src1"), cid("dataset"), cid("model"), cid("output")]
    );
}

// ---------------------------------------------------------------------------
// Property tests
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn every_inclusion_proof_verifies(n in 1usize..200, i in 0usize..200) {
        prop_assume!(i < n);
        let l = build(n);
        let p = l.inclusion_proof(i as u64, n as u64).unwrap();
        prop_assert!(p.verify(l.leaf(i as u64).unwrap(), l.root()));
    }

    #[test]
    fn every_consistency_proof_verifies(new in 1usize..200, old in 0usize..200) {
        prop_assume!(old <= new);
        let l = build(new);
        let p = l.consistency_proof(old as u64, new as u64).unwrap();
        prop_assert!(p.verify(l.root_at(old as u64).unwrap(), l.root()));
    }

    /// An entry appended later must never verify as included in an earlier
    /// tree: this is the property that makes backdating detectable.
    #[test]
    fn later_entries_are_not_in_earlier_trees(n in 2usize..100) {
        let l = build(n);
        let earlier_root = l.root_at((n - 1) as u64).unwrap();
        let p = l.inclusion_proof((n - 1) as u64, n as u64).unwrap();
        prop_assert!(!p.verify(l.leaf((n - 1) as u64).unwrap(), earlier_root));
    }

    /// Insertion order cannot create a cycle, because every input must
    /// already be present.
    #[test]
    fn insertion_order_prevents_cycles(k in 1usize..24) {
        let mut g = ProvenanceGraph::new();
        g.insert(ProvNode::source(cid("root"), org(), SubjectRef([1u8; 16]), t())).unwrap();
        let mut prev = cid("root");
        for i in 0..k {
            let id = cid(&format!("n{i}"));
            g.insert(ProvNode::derived(id, NodeKind::Derived, org(), t(),
                vec![Edge::uniform(prev, Ratio::ONE)], Operation::ProcessTransform)).unwrap();
            prev = id;
        }
        prop_assert_eq!(g.depth(&prev).unwrap() as usize, k);
    }

    /// Attribution over a tree of uniform splits always totals to one.
    #[test]
    fn uniform_attribution_totals_one(fanout in 1u64..8, depth in 1usize..4) {
        let mut g = ProvenanceGraph::new();
        let mut layer: Vec<ContentId> = Vec::new();
        let leaves = fanout.pow(depth as u32);
        for i in 0..leaves {
            let id = cid(&format!("s{i}"));
            g.insert(ProvNode::source(id, org(), SubjectRef([(i % 251) as u8; 16]), t())).unwrap();
            layer.push(id);
        }
        let mut level = 0;
        while layer.len() > 1 {
            let w = Ratio::new(1, fanout).unwrap();
            let mut next = Vec::new();
            for (j, chunk) in layer.chunks(fanout as usize).enumerate() {
                let id = cid(&format!("l{level}-{j}"));
                let edges: Vec<Edge> = chunk.iter().map(|c| Edge::uniform(*c, w)).collect();
                // The last chunk may be short; re-weight it so the sum is 1.
                let edges = if chunk.len() as u64 != fanout {
                    let w2 = Ratio::new(1, chunk.len() as u64).unwrap();
                    chunk.iter().map(|c| Edge::uniform(*c, w2)).collect()
                } else { edges };
                g.insert(ProvNode::derived(id, NodeKind::Derived, org(), t(), edges,
                    Operation::ProcessAggregate)).unwrap();
                next.push(id);
            }
            layer = next;
            level += 1;
        }
        let policy = DerivationPolicy { min_share: Ratio::ZERO, ..Default::default() };
        let (shares, dropped) = g.attribution(&layer[0], &policy).unwrap();
        let total = shares.values().fold(Ratio::ZERO, |a, b| a.add(*b).unwrap());
        prop_assert_eq!(total.add(dropped).unwrap(), Ratio::ONE);
    }
}
