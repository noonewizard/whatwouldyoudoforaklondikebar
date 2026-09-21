//! The DUAP transparency log.
//!
//! STATUS: PRODUCTION (in-memory reference; the durable backend lives in
//! `duap-clearing`).
//!
//! The log records *commitments*, never content. A log entry is the digest of
//! a protocol object plus enough metadata to find it, so publishing the log
//! publishes no personal data. This is the reason DUAP can have a public,
//! third-party-auditable history at all: the thing being made public is a
//! hash.
//!
//! What the log provides:
//!
//! * **Existence with an upper bound on time.** An entry in a signed tree
//!   head dated T proves the object existed no later than T. Combined with
//!   the signature's `created` field it bounds backdating: a controller
//!   cannot claim an authorization it only obtained afterwards.
//! * **Append-only history.** Consistency proofs between tree heads make
//!   deletion and reordering detectable.
//! * **Non-equivocation, conditionally.** A log that shows different
//!   histories to different auditors is detected when they compare tree
//!   heads. Detection needs gossip; the log cannot prevent equivocation on
//!   its own, and DUAP does not pretend otherwise. `THREAT_MODEL.md` T-15.
//!
//! What it does not provide: it says nothing about whether the logged object
//! is *true*. A controller can log a receipt for an event that never
//! happened. Detecting that is the job of cross-checks against the subject's
//! own records and of audit sampling, not of the log.

use crate::merkle::{ConsistencyProof, InclusionProof, MerkleLog};
use duap_canon::digest::{Digest, HashAlg};
use duap_crypto::{Envelope, KeyId, KeyRegistry, SuitePolicy, VerificationContext};
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};

/// Domain label for signed tree heads.
pub const STH_DOMAIN: &str = "duap.sth.v1";
/// Domain label for log entries.
pub const ENTRY_DOMAIN: &str = "duap.log-entry.v1";

/// What kind of object an entry commits to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryKind {
    /// A batch of data usage events, committed by their batch root.
    EventBatch,
    /// An authorization grant.
    Grant,
    /// A revocation.
    Revocation,
    /// A usage receipt issued by the clearing node.
    Receipt,
    /// A key registry checkpoint.
    KeyRegistry,
    /// A pricing schedule publication.
    PricingSchedule,
    /// A dataset or model commitment used in AI accounting.
    DatasetCommitment,
    /// An invoice.
    Invoice,
    /// A settlement instruction.
    Settlement,
    /// A dispute filing or resolution.
    Dispute,
    /// An auditor's attestation.
    Attestation,
}

/// One entry: a commitment plus the metadata needed to locate the object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogEntry {
    #[serde(rename = "k")]
    pub kind: EntryKind,
    /// Digest of the committed object, under that object's own domain.
    #[serde(rename = "d")]
    pub object: Digest,
    /// The party that submitted the entry.
    #[serde(rename = "s")]
    pub submitter: OrgId,
    /// Log-assigned sequencing time. Assigned by the log, not the submitter.
    #[serde(rename = "t")]
    pub sequenced_at: Timestamp,
    /// Optional shard tag, for jurisdictionally partitioned logs.
    #[serde(rename = "sh", default, skip_serializing_if = "Option::is_none")]
    pub shard: Option<String>,
}

impl LogEntry {
    /// Canonical bytes committed as the leaf payload.
    pub fn to_leaf_bytes(&self) -> Result<Vec<u8>, ModelError> {
        Ok(duap_canon::to_canonical_cbor(self)?)
    }
}

/// A signed tree head: the log's assertion about its own state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TreeHead {
    #[serde(rename = "l")]
    pub log_id: String,
    #[serde(rename = "n")]
    pub size: u64,
    #[serde(rename = "r")]
    pub root: Digest,
    #[serde(rename = "t")]
    pub issued_at: Timestamp,
    /// Hash algorithm used throughout the tree.
    #[serde(rename = "a")]
    pub alg: HashAlg,
}

impl TreeHead {
    pub fn digest(&self) -> Result<Digest, ModelError> {
        Ok(Digest::of_object(STH_DOMAIN, self)?)
    }
}

/// Errors from log operations.
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum LogError {
    #[error("tree head is for log {got:?}, expected {want:?}")]
    WrongLog { want: String, got: String },
    #[error("tree head size {got} is behind the known size {want}")]
    Regression { want: u64, got: u64 },
    #[error("consistency proof between sizes {old} and {new} failed to verify")]
    Inconsistent { old: u64, new: u64 },
    #[error("inclusion proof for index {index} in a tree of size {size} failed to verify")]
    NotIncluded { index: u64, size: u64 },
    #[error("tree head signature did not verify: {0}")]
    BadSignature(String),
    #[error("{0}")]
    Model(#[from] ModelError),
}

/// An append-only transparency log.
#[derive(Debug, Clone)]
pub struct TransparencyLog {
    pub log_id: String,
    tree: MerkleLog,
    entries: Vec<LogEntry>,
}

impl TransparencyLog {
    pub fn new(log_id: impl Into<String>, alg: HashAlg) -> TransparencyLog {
        TransparencyLog {
            log_id: log_id.into(),
            tree: MerkleLog::new(alg),
            entries: Vec::new(),
        }
    }

    pub fn size(&self) -> u64 {
        self.tree.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    /// Append an entry, returning its index.
    pub fn append(&mut self, entry: LogEntry) -> Result<u64, LogError> {
        let bytes = entry.to_leaf_bytes()?;
        let (idx, _) = self.tree.append(&bytes);
        self.entries.push(entry);
        Ok(idx)
    }

    pub fn entry(&self, index: u64) -> Option<&LogEntry> {
        self.entries.get(index as usize)
    }

    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    /// Find the first index committing to `object`.
    ///
    /// Linear in the reference implementation; a production log keeps a
    /// digest index.
    pub fn find(&self, object: &Digest) -> Option<u64> {
        self.entries
            .iter()
            .position(|e| e.object == *object)
            .map(|i| i as u64)
    }

    /// Current tree head.
    pub fn head(&self, issued_at: Timestamp) -> TreeHead {
        TreeHead {
            log_id: self.log_id.clone(),
            size: self.tree.len(),
            root: self.tree.root(),
            issued_at,
            alg: self.tree.alg(),
        }
    }

    /// Sign a tree head with the log's key.
    pub fn sign_head(
        &self,
        key: &duap_crypto::SecretKey,
        issued_at: Timestamp,
    ) -> Result<Envelope, LogError> {
        let head = self.head(issued_at);
        let mut env =
            Envelope::seal(STH_DOMAIN, &head).map_err(|e| LogError::BadSignature(e.to_string()))?;
        env.sign(key, issued_at.0, None)
            .map_err(|e| LogError::BadSignature(e.to_string()))?;
        Ok(env)
    }

    pub fn inclusion_proof(&self, index: u64, size: u64) -> Option<InclusionProof> {
        self.tree.inclusion_proof(index, size)
    }

    pub fn consistency_proof(&self, old: u64, new: u64) -> Option<ConsistencyProof> {
        self.tree.consistency_proof(old, new)
    }

    pub fn leaf_hash_at(&self, index: u64) -> Option<Digest> {
        self.tree.leaf(index)
    }

    pub fn root_at(&self, size: u64) -> Option<Digest> {
        self.tree.root_at(size)
    }
}

/// A receipt of inclusion that a client can keep and re-verify offline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InclusionWitness {
    #[serde(rename = "e")]
    pub entry: LogEntry,
    #[serde(rename = "p")]
    pub proof: InclusionProof,
    #[serde(rename = "h")]
    pub head: TreeHead,
}

impl InclusionWitness {
    /// Verify the witness end to end against a trusted log key.
    ///
    /// Checks, in order: the tree head's signature is by a key the registry
    /// accepts in the log-signer role; the entry hashes to the leaf the proof
    /// starts from; and the proof reconstructs the head's root.
    pub fn verify(
        &self,
        head_envelope: &Envelope,
        registry: &KeyRegistry,
        policy: &SuitePolicy,
        ctx: &VerificationContext,
    ) -> Result<Vec<KeyId>, LogError> {
        let signers = head_envelope
            .verify(registry, policy, ctx)
            .map_err(|e| LogError::BadSignature(e.to_string()))?;
        for kid in &signers {
            let rec = registry
                .get(kid)
                .ok_or_else(|| LogError::BadSignature(format!("unknown key {kid}")))?;
            if !rec.has_role(duap_crypto::KeyRole::LogSigner) {
                return Err(LogError::BadSignature(format!(
                    "key {kid} is not authorised as a log signer"
                )));
            }
        }
        let signed_head: TreeHead = head_envelope
            .open()
            .map_err(|e| LogError::BadSignature(e.to_string()))?;
        if signed_head != self.head {
            return Err(LogError::BadSignature(
                "signed tree head does not match the witness".into(),
            ));
        }
        let leaf = crate::merkle::leaf_hash(self.head.alg, &self.entry.to_leaf_bytes()?);
        if !self.proof.verify(leaf, self.head.root) {
            return Err(LogError::NotIncluded {
                index: self.proof.index,
                size: self.proof.size,
            });
        }
        Ok(signers)
    }
}

/// A client's view of a log, maintained across observations.
///
/// Holding the most recent verified head and demanding a consistency proof
/// for every advance is what turns "the log says so" into "the log has not
/// rewritten anything I have seen".
#[derive(Debug, Clone, Default)]
pub struct LogMonitor {
    pub log_id: String,
    pub last: Option<TreeHead>,
}

impl LogMonitor {
    pub fn new(log_id: impl Into<String>) -> LogMonitor {
        LogMonitor {
            log_id: log_id.into(),
            last: None,
        }
    }

    /// Accept a new head, requiring a consistency proof from the last one.
    pub fn advance(
        &mut self,
        head: TreeHead,
        proof: Option<&ConsistencyProof>,
    ) -> Result<(), LogError> {
        if head.log_id != self.log_id {
            return Err(LogError::WrongLog {
                want: self.log_id.clone(),
                got: head.log_id,
            });
        }
        if let Some(prev) = &self.last {
            if head.size < prev.size {
                return Err(LogError::Regression {
                    want: prev.size,
                    got: head.size,
                });
            }
            if head.size == prev.size {
                if head.root != prev.root {
                    return Err(LogError::Inconsistent {
                        old: prev.size,
                        new: head.size,
                    });
                }
                return Ok(());
            }
            let p = proof.ok_or(LogError::Inconsistent {
                old: prev.size,
                new: head.size,
            })?;
            if p.old != prev.size || p.new != head.size || !p.verify(prev.root, head.root) {
                return Err(LogError::Inconsistent {
                    old: prev.size,
                    new: head.size,
                });
            }
        }
        self.last = Some(head);
        Ok(())
    }
}
