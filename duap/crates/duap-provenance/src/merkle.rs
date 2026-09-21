//! Binary Merkle tree over an append-only sequence, with inclusion and
//! consistency proofs.
//!
//! STATUS: PRODUCTION.
//!
//! The construction follows RFC 6962 section 2 (Certificate Transparency):
//!
//! ```text
//! MTH({})       = H("")                     with the empty-tree domain
//! MTH({d0})     = H(0x00 || d0)             leaf hash
//! MTH(D[0:n])   = H(0x01 || MTH(D[0:k]) || MTH(D[k:n]))
//!                 where k is the largest power of two strictly below n
//! ```
//!
//! The `0x00`/`0x01` prefixes are what stop a leaf being passed off as an
//! interior node -- without them an attacker can present a two-leaf subtree's
//! hash as a single leaf and produce a second preimage for the root. DUAP adds
//! a domain label on top, so a DUAP tree head cannot be confused with a
//! Certificate Transparency one.
//!
//! # Why a transparency log rather than a blockchain
//!
//! The property the protocol needs is *append-only, globally consistent
//! history that a third party can audit* -- not *agreement among mutually
//! distrusting validators on a total order of transactions*. A Merkle
//! transparency log gives the former for the cost of one hash per entry and
//! O(log n) proofs, with no consensus, no tokens and no fork-choice rule. A
//! log operator that equivocates -- shows different histories to different
//! parties -- is caught by consistency proofs plus gossip, which is a
//! detection guarantee rather than a prevention guarantee. `docs/adr/
//! 0005-transparency-log-not-blockchain.md` works through the alternatives,
//! and `THREAT_MODEL.md` T-15 states exactly what equivocation costs.

use duap_canon::digest::{Digest, HashAlg};

/// Domain label for leaf hashes.
pub const LEAF_DOMAIN: &str = "duap.log.leaf.v1";
/// Domain label for interior node hashes.
pub const NODE_DOMAIN: &str = "duap.log.node.v1";
/// Domain label for the empty tree.
pub const EMPTY_DOMAIN: &str = "duap.log.empty.v1";

/// Hash of the empty tree.
pub fn empty_root(alg: HashAlg) -> Digest {
    Digest::of(alg, EMPTY_DOMAIN, b"")
}

/// Hash of a leaf holding `data`.
pub fn leaf_hash(alg: HashAlg, data: &[u8]) -> Digest {
    let mut buf = Vec::with_capacity(1 + data.len());
    buf.push(0x00);
    buf.extend_from_slice(data);
    Digest::of(alg, LEAF_DOMAIN, &buf)
}

/// Hash of an interior node with the given children.
pub fn node_hash(alg: HashAlg, left: &Digest, right: &Digest) -> Digest {
    let mut buf = Vec::with_capacity(1 + 64);
    buf.push(0x01);
    buf.extend_from_slice(&left.bytes);
    buf.extend_from_slice(&right.bytes);
    Digest::of(alg, NODE_DOMAIN, &buf)
}

/// Largest power of two strictly less than `n`, for `n > 1`.
fn split_point(n: usize) -> usize {
    debug_assert!(n > 1);
    let mut k = 1usize;
    while k << 1 < n {
        k <<= 1;
    }
    k
}

/// Merkle tree head over `leaves` (already hashed).
pub fn root_of(alg: HashAlg, leaves: &[Digest]) -> Digest {
    match leaves.len() {
        0 => empty_root(alg),
        1 => leaves[0],
        n => {
            let k = split_point(n);
            node_hash(alg, &root_of(alg, &leaves[..k]), &root_of(alg, &leaves[k..]))
        }
    }
}

/// An append-only Merkle tree.
///
/// Leaf hashes are retained so that proofs can be produced for any past
/// index. A production log stores them in a durable tile-based layout; the
/// in-memory form here is the reference against which that is tested.
#[derive(Debug, Clone)]
pub struct MerkleLog {
    alg: HashAlg,
    leaves: Vec<Digest>,
}

impl MerkleLog {
    pub fn new(alg: HashAlg) -> MerkleLog {
        MerkleLog {
            alg,
            leaves: Vec::new(),
        }
    }

    pub fn alg(&self) -> HashAlg {
        self.alg
    }

    pub fn len(&self) -> u64 {
        self.leaves.len() as u64
    }

    pub fn is_empty(&self) -> bool {
        self.leaves.is_empty()
    }

    /// Append `data`, returning its index and leaf hash.
    pub fn append(&mut self, data: &[u8]) -> (u64, Digest) {
        let h = leaf_hash(self.alg, data);
        self.leaves.push(h);
        (self.leaves.len() as u64 - 1, h)
    }

    /// Append a pre-computed leaf hash.
    pub fn append_hash(&mut self, h: Digest) -> u64 {
        self.leaves.push(h);
        self.leaves.len() as u64 - 1
    }

    pub fn leaf(&self, index: u64) -> Option<Digest> {
        self.leaves.get(index as usize).copied()
    }

    /// Current root.
    pub fn root(&self) -> Digest {
        root_of(self.alg, &self.leaves)
    }

    /// Root as of the first `size` entries.
    pub fn root_at(&self, size: u64) -> Option<Digest> {
        if size as usize > self.leaves.len() {
            return None;
        }
        Some(root_of(self.alg, &self.leaves[..size as usize]))
    }

    /// Audit path proving that leaf `index` is in the tree of size `size`.
    pub fn inclusion_proof(&self, index: u64, size: u64) -> Option<InclusionProof> {
        if index >= size || size as usize > self.leaves.len() {
            return None;
        }
        let mut path = Vec::new();
        subproof(
            self.alg,
            index as usize,
            &self.leaves[..size as usize],
            &mut path,
        );
        Some(InclusionProof {
            index,
            size,
            path,
            alg: self.alg,
        })
    }

    /// Proof that the tree of size `old` is a prefix of the tree of size
    /// `new`.
    pub fn consistency_proof(&self, old: u64, new: u64) -> Option<ConsistencyProof> {
        if old > new || new as usize > self.leaves.len() {
            return None;
        }
        let mut path = Vec::new();
        if old > 0 && old < new {
            consistency_subproof(
                self.alg,
                old as usize,
                &self.leaves[..new as usize],
                true,
                &mut path,
            );
        }
        Some(ConsistencyProof {
            old,
            new,
            path,
            alg: self.alg,
        })
    }
}

fn subproof(alg: HashAlg, m: usize, leaves: &[Digest], out: &mut Vec<Digest>) {
    let n = leaves.len();
    if n <= 1 {
        return;
    }
    let k = split_point(n);
    if m < k {
        subproof(alg, m, &leaves[..k], out);
        out.push(root_of(alg, &leaves[k..]));
    } else {
        subproof(alg, m - k, &leaves[k..], out);
        out.push(root_of(alg, &leaves[..k]));
    }
}

fn consistency_subproof(
    alg: HashAlg,
    m: usize,
    leaves: &[Digest],
    is_complete: bool,
    out: &mut Vec<Digest>,
) {
    let n = leaves.len();
    if m == n {
        if !is_complete {
            out.push(root_of(alg, leaves));
        }
        return;
    }
    let k = split_point(n);
    if m <= k {
        consistency_subproof(alg, m, &leaves[..k], is_complete, out);
        out.push(root_of(alg, &leaves[k..]));
    } else {
        consistency_subproof(alg, m - k, &leaves[k..], false, out);
        out.push(root_of(alg, &leaves[..k]));
    }
}

/// A proof that an entry is in a tree of a stated size.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InclusionProof {
    #[serde(rename = "i")]
    pub index: u64,
    #[serde(rename = "n")]
    pub size: u64,
    #[serde(rename = "p")]
    pub path: Vec<Digest>,
    #[serde(rename = "a")]
    pub alg: HashAlg,
}

impl InclusionProof {
    /// Recompute the root implied by `leaf` and this path.
    ///
    /// Standalone: a client verifies with nothing but the proof, the leaf
    /// hash and the tree head it already trusts.
    pub fn compute_root(&self, leaf: Digest) -> Option<Digest> {
        if self.index >= self.size {
            return None;
        }
        let mut fn_ = self.index;
        let mut sn = self.size - 1;
        let mut r = leaf;
        for p in &self.path {
            if sn == 0 {
                return None; // path longer than the tree is deep
            }
            if fn_ % 2 == 1 || fn_ == sn {
                r = node_hash(self.alg, p, &r);
                while fn_ % 2 == 0 && fn_ != 0 {
                    fn_ /= 2;
                    sn /= 2;
                }
            } else {
                r = node_hash(self.alg, &r, p);
            }
            fn_ /= 2;
            sn /= 2;
        }
        if sn != 0 {
            return None; // path shorter than the tree is deep
        }
        Some(r)
    }

    /// Check the proof against a known root.
    pub fn verify(&self, leaf: Digest, root: Digest) -> bool {
        if self.size == 1 {
            return self.index == 0 && self.path.is_empty() && leaf == root;
        }
        self.compute_root(leaf) == Some(root)
    }
}

/// A proof that one tree is a prefix of another.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConsistencyProof {
    #[serde(rename = "o")]
    pub old: u64,
    #[serde(rename = "n")]
    pub new: u64,
    #[serde(rename = "p")]
    pub path: Vec<Digest>,
    #[serde(rename = "a")]
    pub alg: HashAlg,
}

impl ConsistencyProof {
    /// Verify that `old_root` (size `old`) is a prefix of `new_root`
    /// (size `new`).
    ///
    /// This is the check that catches a log operator who rewrites history: a
    /// removed or altered entry makes every later consistency proof fail.
    /// The algorithm is RFC 6962 section 2.1.2 `VERIFY_CONSISTENCY`.
    pub fn verify(&self, old_root: Digest, new_root: Digest) -> bool {
        if self.old > self.new {
            return false;
        }
        if self.old == self.new {
            return self.path.is_empty() && old_root == new_root;
        }
        if self.old == 0 {
            // Every tree is consistent with the empty tree; the proof is
            // empty and carries no information.
            return self.path.is_empty();
        }

        let mut fnode = self.old - 1;
        let mut snode = self.new - 1;
        while fnode & 1 == 1 {
            fnode >>= 1;
            snode >>= 1;
        }

        let mut iter = self.path.iter();
        let (mut fr, mut sr) = if fnode != 0 {
            match iter.next() {
                Some(d) => (*d, *d),
                None => return false,
            }
        } else {
            (old_root, old_root)
        };

        for c in iter {
            if snode == 0 {
                return false;
            }
            if fnode & 1 == 1 || fnode == snode {
                fr = node_hash(self.alg, c, &fr);
                sr = node_hash(self.alg, c, &sr);
                while fnode != 0 && fnode & 1 == 0 {
                    fnode >>= 1;
                    snode >>= 1;
                }
            } else {
                sr = node_hash(self.alg, &sr, c);
            }
            fnode >>= 1;
            snode >>= 1;
        }

        snode == 0 && fr == old_root && sr == new_root
    }
}
