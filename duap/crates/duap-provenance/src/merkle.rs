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
            node_hash(
                alg,
                &root_of(alg, &leaves[..k]),
                &root_of(alg, &leaves[k..]),
            )
        }
    }
}

/// An append-only Merkle tree.
///
/// Leaf hashes are retained so that proofs can be produced for any past
/// index. A production log stores them in a durable tile-based layout; the
/// in-memory form here is the reference against which that is tested.
///
/// # Why there is a cache
///
/// Proof generation without one is O(n): [`subproof`] pushes the root of
/// the sibling subtree at every level, and computing that root from leaves
/// walks the whole subtree. Measured, that was 14.99 ms to produce one
/// audit path in a 100,000-entry tree against 2.5 us to verify it -- a
/// denial-of-service lever on any endpoint that serves proofs, recorded as
/// PERF-01 in `security/findings.md`.
///
/// `nodes` caches the roots of *complete* subtrees, level by level:
/// `nodes[L][j]` is the root over leaves `[j << (L+1), (j+1) << (L+1))`,
/// present only when that range is entirely filled. Appending maintains it
/// in amortised O(1), and it doubles the memory the log holds -- n leaf
/// digests plus n-1 interior ones at most.
///
/// The cache is an optimisation and never a second definition of the tree.
/// [`root_of`] remains the reference construction, and
/// `cached_and_recursive_agree_for_every_size` in the crate's tests asserts
/// they produce identical roots, proofs and consistency paths.
#[derive(Debug, Clone)]
pub struct MerkleLog {
    alg: HashAlg,
    leaves: Vec<Digest>,
    /// `nodes[L][j]`: root of the complete subtree of `1 << (L + 1)` leaves
    /// starting at `j << (L + 1)`. Level 0 holds pairs of leaves.
    nodes: Vec<Vec<Digest>>,
}

impl MerkleLog {
    pub fn new(alg: HashAlg) -> MerkleLog {
        MerkleLog {
            alg,
            leaves: Vec::new(),
            nodes: Vec::new(),
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
        let i = self.append_hash(h);
        (i, h)
    }

    /// Append a pre-computed leaf hash.
    pub fn append_hash(&mut self, h: Digest) -> u64 {
        self.leaves.push(h);
        self.extend_cache();
        self.leaves.len() as u64 - 1
    }

    /// Fill in every complete subtree the new leaf just closed.
    ///
    /// A leaf completes a subtree at level `L` exactly when the new length
    /// is a multiple of `1 << (L + 1)`, so this loop runs once per trailing
    /// one-bit of the previous length: O(1) amortised over appends.
    fn extend_cache(&mut self) {
        let n = self.leaves.len();
        let mut level = 0usize;
        loop {
            let span = 1usize << (level + 1);
            if n % span != 0 {
                break;
            }
            let j = n / span - 1;
            let (left, right) = if level == 0 {
                (self.leaves[j * 2], self.leaves[j * 2 + 1])
            } else {
                let below = &self.nodes[level - 1];
                (below[j * 2], below[j * 2 + 1])
            };
            let h = node_hash(self.alg, &left, &right);
            if self.nodes.len() == level {
                self.nodes.push(Vec::new());
            }
            debug_assert_eq!(self.nodes[level].len(), j);
            self.nodes[level].push(h);
            level += 1;
        }
    }

    /// Root over `leaves[a..b]`, using cached complete subtrees.
    ///
    /// Equivalent to `root_of(alg, &leaves[a..b])` and asymptotically
    /// cheaper: the RFC 6962 split makes every left child of this recursion
    /// an aligned complete subtree, so each level costs one cache lookup
    /// and the recursion only descends the right spine.
    fn cached_root(&self, a: usize, b: usize) -> Digest {
        debug_assert!(a <= b && b <= self.leaves.len());
        let n = b - a;
        match n {
            0 => empty_root(self.alg),
            1 => self.leaves[a],
            _ => {
                if n.is_power_of_two() && a % n == 0 {
                    let level = n.trailing_zeros() as usize - 1;
                    return self.nodes[level][a / n];
                }
                let k = split_point(n);
                node_hash(
                    self.alg,
                    &self.cached_root(a, a + k),
                    &self.cached_root(a + k, b),
                )
            }
        }
    }

    /// Audit path for `index` within the prefix of length `size`.
    fn cached_subproof(&self, m: usize, a: usize, b: usize, out: &mut Vec<Digest>) {
        let n = b - a;
        if n <= 1 {
            return;
        }
        let k = split_point(n);
        if m < k {
            self.cached_subproof(m, a, a + k, out);
            out.push(self.cached_root(a + k, b));
        } else {
            self.cached_subproof(m - k, a + k, b, out);
            out.push(self.cached_root(a, a + k));
        }
    }

    fn cached_consistency_subproof(
        &self,
        m: usize,
        a: usize,
        b: usize,
        is_complete: bool,
        out: &mut Vec<Digest>,
    ) {
        let n = b - a;
        if m == n {
            if !is_complete {
                out.push(self.cached_root(a, b));
            }
            return;
        }
        let k = split_point(n);
        if m <= k {
            self.cached_consistency_subproof(m, a, a + k, is_complete, out);
            out.push(self.cached_root(a + k, b));
        } else {
            self.cached_consistency_subproof(m - k, a + k, b, false, out);
            out.push(self.cached_root(a, a + k));
        }
    }

    pub fn leaf(&self, index: u64) -> Option<Digest> {
        self.leaves.get(index as usize).copied()
    }

    /// Current root.
    pub fn root(&self) -> Digest {
        self.cached_root(0, self.leaves.len())
    }

    /// Root as of the first `size` entries.
    pub fn root_at(&self, size: u64) -> Option<Digest> {
        if size as usize > self.leaves.len() {
            return None;
        }
        Some(self.cached_root(0, size as usize))
    }

    /// Audit path proving that leaf `index` is in the tree of size `size`.
    pub fn inclusion_proof(&self, index: u64, size: u64) -> Option<InclusionProof> {
        if index >= size || size as usize > self.leaves.len() {
            return None;
        }
        let mut path = Vec::new();
        self.cached_subproof(index as usize, 0, size as usize, &mut path);
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
            self.cached_consistency_subproof(old as usize, 0, new as usize, true, &mut path);
        }
        Some(ConsistencyProof {
            old,
            new,
            path,
            alg: self.alg,
        })
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
