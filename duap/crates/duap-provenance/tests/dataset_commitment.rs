//! Dataset commitments: what can actually be proven about training data.
//!
//! This test is the production counterpart to the experiment in
//! `research/ai-attribution/`. It demonstrates the one claim in the AI
//! attribution space that is cryptographic rather than statistical:
//!
//!   **inclusion** -- that a specific record was a member of the dataset a
//!   model was trained on, where the trainer committed to the dataset
//!   before or at training time.
//!
//! and it demonstrates, by construction, that this says nothing about
//! **influence** or **economic contribution**.

use duap_canon::{Digest, HashAlg};
use duap_model::prelude::*;
use duap_provenance::*;

fn record_digest(i: usize) -> Digest {
    Digest::of(
        HashAlg::Sha2_256,
        "duap.object.v1",
        format!("synthetic:record-{i}").as_bytes(),
    )
}

/// A dataset commitment is the Merkle root over its members' content
/// identifiers, in a declared order. Publishing it fixes the membership.
fn dataset_commitment(members: &[Digest]) -> (Digest, MerkleLog) {
    let mut log = MerkleLog::new(HashAlg::Sha2_256);
    for m in members {
        log.append_hash(merkle::leaf_hash(HashAlg::Sha2_256, &m.bytes));
    }
    (log.root(), log)
}

#[test]
fn a_dataset_commitment_proves_membership_and_nothing_else() {
    let members: Vec<Digest> = (0..1_000).map(record_digest).collect();
    let (commitment, log) = dataset_commitment(&members);

    // A subject holding record 437 can prove it was in the training set.
    let index = 437u64;
    let proof = log
        .inclusion_proof(index, log.len())
        .expect("index is in range");
    let leaf = merkle::leaf_hash(HashAlg::Sha2_256, &members[index as usize].bytes);
    assert!(
        proof.verify(leaf, commitment),
        "inclusion in the committed dataset must be provable"
    );

    // A record that was not a member cannot be shown to be one: no proof
    // exists, and the one for a different index does not transfer.
    let outsider = record_digest(99_999);
    let outsider_leaf = merkle::leaf_hash(HashAlg::Sha2_256, &outsider.bytes);
    assert!(
        !proof.verify(outsider_leaf, commitment),
        "a non-member must not verify against another member's path"
    );

    // Adding a record after publication changes the commitment, so a
    // trainer cannot extend the dataset behind a published root.
    let mut extended = members.clone();
    extended.push(record_digest(1_000));
    let (extended_root, _) = dataset_commitment(&extended);
    assert_ne!(
        commitment, extended_root,
        "the commitment must not survive a change of membership"
    );
}

/// The provenance graph records that a model derived from a dataset that
/// derived from a subject's records. That is lineage. The weight on the
/// edge is a *declared* attribution share, and `WeightBasis` records where
/// the number came from so nobody can mistake an assumption for a
/// measurement.
#[test]
fn lineage_is_recorded_separately_from_influence() {
    let org: OrgId = "org:duap/acme-example-corp".parse().expect("valid");
    let at = Timestamp::from_secs(1_758_412_800);
    let mut g = ProvenanceGraph::new();

    let subjects: Vec<SubjectRef> = (0..4).map(|i| SubjectRef([i as u8; 16])).collect();
    let sources: Vec<ContentId> = (0..4)
        .map(|i| ContentId::of_bytes("duap.object.v1", format!("synthetic:src-{i}").as_bytes()))
        .collect();
    for (s, cid) in subjects.iter().zip(&sources) {
        g.insert(ProvNode::source(*cid, org.clone(), *s, at))
            .expect("source inserts");
    }

    // The dataset edge weights are uniform, and say so.
    let dataset = ContentId::of_bytes("duap.dataset.v1", b"synthetic:training-set-v1");
    let quarter = Ratio::new(1, 4).expect("non-zero");
    g.insert(ProvNode::derived(
        dataset,
        NodeKind::Dataset,
        org.clone(),
        at,
        sources
            .iter()
            .map(|c| Edge {
                from: *c,
                weight: quarter,
                basis: WeightBasis::Uniform,
                severed: false,
                note: Some("equal split; no influence estimate was performed".into()),
            })
            .collect(),
        Operation::ProcessAggregate,
    ))
    .expect("dataset inserts");

    let model = ContentId::of_bytes("duap.object.v1", b"synthetic:model-v1");
    g.insert(ProvNode::derived(
        model,
        NodeKind::Model,
        org.clone(),
        at,
        vec![Edge {
            from: dataset,
            weight: Ratio::ONE,
            basis: WeightBasis::Uniform,
            severed: false,
            note: None,
        }],
        Operation::AiFinetune,
    ))
    .expect("model inserts");

    let (shares, dropped) = g
        .attribution(&model, &DerivationPolicy::default())
        .expect("attribution computes");

    // Every contributor is attributed, exactly, and the shares sum to one.
    assert_eq!(shares.len(), 4);
    for s in &subjects {
        assert_eq!(shares[s], quarter);
    }
    assert_eq!(dropped, Ratio::ZERO);

    // The basis of every weight is recorded, and here it is `Uniform`:
    // an assumption, not a measurement. A consumer of this graph can tell
    // the difference without reading the code that produced it.
    let node = g.get(&dataset).expect("dataset present");
    assert!(
        node.inputs.iter().all(|e| e.basis == WeightBasis::Uniform),
        "the weights are declared uniform and must say so"
    );
    assert!(
        node.inputs.iter().all(|e| e.basis != WeightBasis::Measured),
        "nothing here was measured, and nothing may claim to have been"
    );
}

/// An adjudicated or agreed transformation can sever attribution while
/// keeping the lineage visible for audit. This is how a dispute outcome or
/// a contractual term enters the graph without rewriting history.
#[test]
fn severing_keeps_lineage_and_stops_attribution() {
    let org: OrgId = "org:duap/acme-example-corp".parse().expect("valid");
    let at = Timestamp::from_secs(1_758_412_800);
    let mut g = ProvenanceGraph::new();
    let subject = SubjectRef([7u8; 16]);
    let src = ContentId::of_bytes("duap.object.v1", b"synthetic:src");
    g.insert(ProvNode::source(src, org.clone(), subject, at))
        .expect("source inserts");

    let released = ContentId::of_bytes("duap.object.v1", b"synthetic:dp-release");
    g.insert(ProvNode::derived(
        released,
        NodeKind::DpRelease,
        org.clone(),
        at,
        vec![Edge {
            from: src,
            weight: Ratio::ONE,
            basis: WeightBasis::Adjudicated,
            severed: true,
            note: Some("attribution extinguished by agreement of 2026-09-01".into()),
        }],
        Operation::ProcessDpRelease,
    ))
    .expect("release inserts");

    let (shares, dropped) = g
        .attribution(&released, &DerivationPolicy::default())
        .expect("attribution computes");
    assert!(shares.is_empty(), "a severed edge carries no attribution");
    assert_eq!(
        dropped,
        Ratio::ONE,
        "and the whole weight is accounted as terminated"
    );

    // The lineage is still there. An auditor can see what it came from.
    assert_eq!(g.get(&released).expect("present").inputs[0].from, src);
    assert_eq!(g.descendants(&src).len(), 1);
}
