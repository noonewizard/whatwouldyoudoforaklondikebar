//! The complete-subtree cache must be an optimisation, never a second
//! definition of the tree.
//!
//! `root_of` is the reference construction, written straight from RFC 6962
//! section 2.1. `MerkleLog` now answers from a cache of complete subtree
//! roots so that proof generation is not O(n) (PERF-01). These tests assert
//! the two agree on every root, every audit path and every consistency path,
//! for every tree size in a range that covers all the shapes the split rule
//! can produce -- powers of two, one either side of them, and everything
//! between.
//!
//! If these ever disagree, the cache is wrong and the reference is right.

use duap_canon::HashAlg;
use duap_provenance::merkle::{InclusionProof, MerkleLog, leaf_hash, root_of};

const ALG: HashAlg = HashAlg::Sha2_256;

fn log_of(n: usize) -> (MerkleLog, Vec<duap_canon::Digest>) {
    let mut log = MerkleLog::new(ALG);
    let mut leaves = Vec::with_capacity(n);
    for i in 0..n {
        let h = leaf_hash(ALG, format!("synthetic:entry-{i}").as_bytes());
        log.append_hash(h);
        leaves.push(h);
    }
    (log, leaves)
}

/// Reference audit path, computed from leaves with no cache. This is a
/// transcription of the recursive construction the cache replaced.
fn reference_subproof(m: usize, leaves: &[duap_canon::Digest], out: &mut Vec<duap_canon::Digest>) {
    let n = leaves.len();
    if n <= 1 {
        return;
    }
    let mut k = 1usize;
    while k << 1 < n {
        k <<= 1;
    }
    if m < k {
        reference_subproof(m, &leaves[..k], out);
        out.push(root_of(ALG, &leaves[k..]));
    } else {
        reference_subproof(m - k, &leaves[k..], out);
        out.push(root_of(ALG, &leaves[..k]));
    }
}

#[test]
fn cached_and_recursive_agree_for_every_size() {
    // 0..=130 covers every shape around 1, 2, 4, 8, 16, 32, 64 and 128.
    for n in 0..=130usize {
        let (log, leaves) = log_of(n);
        assert_eq!(
            log.root(),
            root_of(ALG, &leaves),
            "cached root disagrees with the reference at size {n}"
        );
        for size in 0..=n {
            assert_eq!(
                log.root_at(size as u64).expect("size is within the log"),
                root_of(ALG, &leaves[..size]),
                "cached root_at({size}) disagrees at size {n}"
            );
        }
    }
}

#[test]
fn cached_inclusion_proofs_match_the_reference_and_verify() {
    for n in 1..=66usize {
        let (log, leaves) = log_of(n);
        let root = root_of(ALG, &leaves);
        for index in 0..n {
            let proof = log
                .inclusion_proof(index as u64, n as u64)
                .expect("index is in range");

            let mut expected = Vec::new();
            reference_subproof(index, &leaves, &mut expected);
            assert_eq!(
                proof.path, expected,
                "cached audit path for leaf {index} of {n} differs from the reference"
            );

            assert!(
                proof.verify(leaves[index], root),
                "audit path for leaf {index} of {n} does not verify"
            );
        }
    }
}

#[test]
fn cached_consistency_proofs_verify_for_every_pair() {
    for n in 1..=40usize {
        let (log, leaves) = log_of(n);
        for old in 1..=n {
            let proof = log
                .consistency_proof(old as u64, n as u64)
                .expect("old <= new <= len");
            assert!(
                proof.verify(root_of(ALG, &leaves[..old]), root_of(ALG, &leaves)),
                "consistency proof {old} -> {n} does not verify"
            );
        }
    }
}

/// The cache must not change what a *verifier* sees. A verifier holds no
/// cache and no leaves beyond its own, so this checks the proof still stands
/// on its own.
#[test]
fn a_proof_still_verifies_without_the_log() {
    let (log, leaves) = log_of(1000);
    let root = log.root();
    let proof = log.inclusion_proof(617, 1000).expect("index is in range");

    let encoded = serde_json::to_vec(&proof).expect("a proof serialises");
    let decoded: InclusionProof = serde_json::from_slice(&encoded).expect("and round-trips");

    drop(log);
    assert!(decoded.verify(leaves[617], root));
}

/// Appending after a proof was taken must not invalidate it: the proof names
/// the tree size it was taken at, and that prefix does not change.
#[test]
fn a_proof_survives_later_appends() {
    let (mut log, leaves) = log_of(300);
    let root_at_300 = log.root();
    let proof = log.inclusion_proof(42, 300).expect("index is in range");

    for i in 300..500 {
        log.append_hash(leaf_hash(ALG, format!("synthetic:entry-{i}").as_bytes()));
    }

    assert!(proof.verify(leaves[42], root_at_300));
    assert_eq!(
        log.root_at(300).expect("the prefix is still addressable"),
        root_at_300,
        "appending changed a historical root"
    );
}
