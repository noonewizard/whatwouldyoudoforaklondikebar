//! A minimal append-only Merkle accumulator for aggregation evidence.
//!
//! STATUS: PRODUCTION.
//!
//! This is the same construction as `duap-provenance::merkle` but maintains
//! only the running root, in O(log n) memory, because an aggregation bucket
//! never needs to produce an inclusion proof from memory -- the durable log
//! does that. Keeping it here also keeps `duap-meter` free of a dependency on
//! the provenance crate, so an embedded agent can meter without linking the
//! log machinery.
//!
//! The digest construction is byte-identical to the transparency log's, which
//! is checked by `tests/evidence_matches_log.rs` in `duap-clearing`: an
//! evidence root computed by an agent must equal the root the clearing node
//! computes over the same events.

use duap_canon::digest::{Digest, HashAlg};

/// Incremental Merkle accumulator holding one digest per level.
#[derive(Debug, Clone)]
pub struct MerkleLog {
    alg: HashAlg,
    /// Roots of complete subtrees, smallest last.
    stack: Vec<(u64, Digest)>,
    n: u64,
}

impl MerkleLog {
    pub fn new(alg: HashAlg) -> MerkleLog {
        MerkleLog {
            alg,
            stack: Vec::new(),
            n: 0,
        }
    }

    pub fn len(&self) -> u64 {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Append data, folding complete sibling subtrees as they form.
    pub fn append(&mut self, data: &[u8]) -> Digest {
        let mut h = leaf_hash(self.alg, data);
        let mut size = 1u64;
        while let Some((top_size, top)) = self.stack.last().copied() {
            if top_size != size {
                break;
            }
            self.stack.pop();
            h = node_hash(self.alg, &top, &h);
            size *= 2;
        }
        self.stack.push((size, h));
        self.n += 1;
        h
    }

    /// Current root, folding the incomplete right edge.
    pub fn root(&self) -> Digest {
        match self.stack.split_last() {
            None => empty_root(self.alg),
            Some((&(_, last), rest)) => {
                let mut acc = last;
                for (_, d) in rest.iter().rev() {
                    acc = node_hash(self.alg, d, &acc);
                }
                acc
            }
        }
    }
}

pub fn empty_root(alg: HashAlg) -> Digest {
    Digest::of(alg, "duap.log.empty.v1", b"")
}

pub fn leaf_hash(alg: HashAlg, data: &[u8]) -> Digest {
    let mut buf = Vec::with_capacity(1 + data.len());
    buf.push(0x00);
    buf.extend_from_slice(data);
    Digest::of(alg, "duap.log.leaf.v1", &buf)
}

pub fn node_hash(alg: HashAlg, left: &Digest, right: &Digest) -> Digest {
    let mut buf = Vec::with_capacity(65);
    buf.push(0x01);
    buf.extend_from_slice(&left.bytes);
    buf.extend_from_slice(&right.bytes);
    Digest::of(alg, "duap.log.node.v1", &buf)
}
