//! # duap-receipt
//!
//! The Data Usage Receipt (DUR): the object the whole protocol exists to
//! produce.
//!
//! STATUS: REFERENCE. Tests cover the specified behaviour and L3 receipt
//! vectors exist. Not PRODUCTION: no independent review.
//!
//! A receipt is a signed statement by a clearing node that, over a stated
//! period, a stated controller performed a stated quantity of a stated
//! operation on a stated class of data about a stated (pseudonymous)
//! subject, under a stated authorization, at a stated charge -- and that the
//! events supporting the statement hash to a stated Merkle root which is
//! anchored in a transparency log.
//!
//! # What a receipt proves
//!
//! Exactly four things, and they are worth stating separately because they
//! are routinely run together:
//!
//! 1. **Authorship.** Some holder of the issuer's private key produced this
//!    statement. (Signature verification.)
//! 2. **Integrity.** The statement has not been altered since. (Same.)
//! 3. **Binding.** The statement is about *this* grant document at *this*
//!    epoch, and about a set of events whose digests produce *this* root --
//!    not some other set the issuer might later prefer. (Digest binding.)
//! 4. **Existence by a time.** The statement was in the transparency log by
//!    the time of the tree head that covers it, and the log's history has
//!    not been rewritten since. (Inclusion plus consistency proofs.)
//!
//! # What a receipt does NOT prove
//!
//! 1. **That the operation happened.** Nothing in cryptography can establish
//!    that a signed assertion about the world is true. A controller can
//!    correctly sign a receipt for an operation it never performed. Detecting
//!    that requires the subject's own records, audit sampling, or discovery.
//! 2. **That all operations were reported.** A receipt is evidence of what
//!    was declared, never of what was omitted. Sequence gaps
//!    (`duap-meter`) narrow this; they do not close it.
//! 3. **That the charge is fair.** The receipt shows the price rule that was
//!    applied and reproduces the arithmetic. Whether that price is right is a
//!    question of agreement and market, not of protocol.
//! 4. **That the data was handled as promised afterwards.** Deferred
//!    obligations (deletion, notification) are recorded, monitored, and
//!    unenforceable by the receipt itself.
//! 5. **That the subject is who they say they are.** The receipt names a
//!    pseudonym. Binding a pseudonym to a legal person is an identity-layer
//!    question that DUAP deliberately keeps separate.
//!
//! [`Receipt::claims`] returns this list programmatically, so a user
//! interface can display it next to the receipt instead of implying more.

use duap_canon::digest::{Digest, HashAlg};
use duap_crypto::{Envelope, KeyId, KeyRegistry, KeyRole, SuitePolicy, VerificationContext};
use duap_model::prelude::*;
use duap_provenance::{InclusionProof, TreeHead};
use serde::{Deserialize, Serialize};

/// Domain label for receipt digests and signatures.
pub const RECEIPT_DOMAIN: &str = "duap.receipt.v1";
/// Current receipt schema version.
pub const RECEIPT_SCHEMA_VERSION: u16 = 1;

/// Summary of the authorization decision that permitted the usage.
///
/// Carries term identifiers and obligation labels rather than the terms
/// themselves: the grant is content-addressed and can be fetched, and
/// embedding it would put the subject's whole policy into every receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionSummary {
    /// Terms of the grant that permitted the usage, ascending.
    #[serde(rename = "t")]
    pub permitting_terms: Vec<u32>,
    /// Labels of the obligations the controller carries as a result.
    #[serde(rename = "o", default, skip_serializing_if = "Vec::is_empty")]
    pub obligations: Vec<String>,
    /// Labels of obligations that are promises rather than checks.
    #[serde(rename = "d", default, skip_serializing_if = "Vec::is_empty")]
    pub deferred: Vec<String>,
}

/// The transparency-log position of a receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogAnchor {
    #[serde(rename = "l")]
    pub log_id: String,
    #[serde(rename = "i")]
    pub index: u64,
    #[serde(rename = "p")]
    pub proof: InclusionProof,
    #[serde(rename = "h")]
    pub head: TreeHead,
}

/// What the receipt covers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Coverage {
    #[serde(rename = "dc")]
    pub data_class: DataClass,
    #[serde(rename = "op")]
    pub operation: Operation,
    #[serde(rename = "pp")]
    pub purpose: Purpose,
    #[serde(rename = "ju")]
    pub country: String,
    #[serde(rename = "qy")]
    pub quantity: Quantity,
    /// Number of events behind the quantity.
    #[serde(rename = "n")]
    pub event_count: u64,
    /// Merkle root over the covered events' digests, in the order the
    /// clearing node accepted them.
    #[serde(rename = "r")]
    pub events_root: Digest,
}

/// A Data Usage Receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    #[serde(rename = "v")]
    pub schema: u16,
    /// Protocol identifier, always `DUAP/1` for this version.
    #[serde(rename = "pr")]
    pub protocol: String,
    /// Clearing node that issued the receipt.
    #[serde(rename = "is")]
    pub issuer: OrgId,
    /// The accountable controller.
    #[serde(rename = "ct")]
    pub controller: OrgId,
    /// The acting processor, if different.
    #[serde(rename = "px", default, skip_serializing_if = "Option::is_none")]
    pub processor: Option<OrgId>,
    /// Who, or what group, the usage was about.
    #[serde(rename = "sb")]
    pub subject: SubjectScope,
    /// Accounting period the receipt covers.
    #[serde(rename = "pd")]
    pub period: TimeRange,
    #[serde(rename = "cv")]
    pub coverage: Coverage,
    /// The authorization relied on.
    #[serde(rename = "az")]
    pub authorization: AuthorizationRef,
    #[serde(rename = "ds")]
    pub decision: DecisionSummary,
    /// Commitment to the derived object produced, where the operation
    /// produced one.
    #[serde(rename = "pv", default, skip_serializing_if = "Option::is_none")]
    pub provenance_output: Option<ContentId>,
    /// The charge, after rounding.
    #[serde(rename = "ch")]
    pub charge: Money,
    /// Share of the charge payable to the subject.
    #[serde(rename = "ss", default, skip_serializing_if = "Option::is_none")]
    pub subject_share: Option<Money>,
    /// Digest of the price breakdown, so the arithmetic can be re-checked
    /// against a document the issuer must retain without putting the whole
    /// derivation in every receipt.
    #[serde(rename = "pb")]
    pub price_digest: Digest,
    /// Where the receipt sits in the transparency log. Absent only for a
    /// receipt that has been issued but not yet anchored; an unanchored
    /// receipt proves authorship but not time.
    #[serde(rename = "an", default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<LogAnchor>,
    #[serde(rename = "at")]
    pub issued_at: Timestamp,
}

impl Receipt {
    /// The receipt with its log anchor detached.
    ///
    /// The anchor is a *proof about* the receipt, not part of it. It cannot
    /// be inside the digest the log commits to, because the log entry is
    /// created before the proof exists. Detaching it also lets a holder
    /// upgrade a receipt with a fresher proof without changing its identity.
    pub fn core(&self) -> Receipt {
        let mut c = self.clone();
        c.anchor = None;
        c
    }

    /// The receipt's digest, over the anchor-free core.
    pub fn digest(&self) -> Result<Digest, ModelError> {
        Ok(Digest::of_object(RECEIPT_DOMAIN, &self.core())?)
    }

    /// Digest over the whole object including any attached anchor. Used only
    /// where the exact transmitted bytes matter.
    pub fn digest_with_anchor(&self) -> Result<Digest, ModelError> {
        Ok(Digest::of_object(RECEIPT_DOMAIN, self)?)
    }

    /// Stable identifier: the leading 128 bits of the receipt digest. Two
    /// parties holding the same receipt name it identically.
    pub fn id(&self) -> Result<String, ModelError> {
        Ok(format!("rcpt1:{}", &self.digest()?.to_hex()[..32]))
    }

    pub fn to_canonical(&self) -> Result<Vec<u8>, ModelError> {
        Ok(duap_canon::to_canonical_cbor(self)?)
    }

    pub fn from_canonical(b: &[u8]) -> Result<Receipt, ModelError> {
        Ok(duap_canon::from_canonical_cbor(b)?)
    }

    /// Seal into a signable envelope.
    pub fn seal(&self) -> Result<Envelope, ReceiptError> {
        Ok(Envelope::seal(RECEIPT_DOMAIN, self)?)
    }

    /// Structural validation independent of any signature.
    pub fn validate(&self) -> Result<(), ReceiptError> {
        let bad = |r: String| Err(ReceiptError::Malformed(r));
        if self.schema != RECEIPT_SCHEMA_VERSION {
            return bad(format!("unsupported receipt schema {}", self.schema));
        }
        if self.protocol != duap_canon::PROTOCOL_ID {
            return bad(format!(
                "unexpected protocol identifier {:?}",
                self.protocol
            ));
        }
        if self.coverage.event_count == 0 {
            return bad("a receipt must cover at least one event".into());
        }
        if self.coverage.quantity.amount == 0 {
            return bad("a receipt must cover a positive quantity".into());
        }
        if self.coverage.quantity.unit != self.coverage.operation.meter() {
            return bad(format!(
                "operation {} is metered in {}, not {}",
                self.coverage.operation.code(),
                self.coverage.operation.meter().code(),
                self.coverage.quantity.unit.code()
            ));
        }
        if self.period.end <= self.period.start {
            return bad("the period must be non-empty".into());
        }
        if self.issued_at < self.period.start {
            return bad("a receipt cannot be issued before the period it covers".into());
        }
        if let Some(s) = self.subject_share {
            if s.currency != self.charge.currency {
                return bad("the subject share is in a different currency from the charge".into());
            }
            if s.minor.abs() > self.charge.minor.abs() {
                return bad("the subject share exceeds the charge".into());
            }
        }
        if self.decision.permitting_terms.is_empty() {
            return bad(
                "a receipt must name the terms that permitted the usage; a usage permitted only \
                 by a grant's default effect must say so with term list [0]"
                    .into(),
            );
        }
        if self.coverage.operation.derives() && self.provenance_output.is_none() {
            return bad(format!(
                "operation {} produces a derived object, so the receipt must commit to it",
                self.coverage.operation.code()
            ));
        }
        Ok(())
    }

    /// Check that `event_digests` are exactly the events this receipt covers.
    ///
    /// The clearing node retains the event set; a subject or auditor who
    /// obtains it can confirm that the receipt's root is the root of that set
    /// and of no other.
    pub fn verify_coverage(&self, event_digests: &[Digest]) -> Result<(), ReceiptError> {
        if event_digests.len() as u64 != self.coverage.event_count {
            return Err(ReceiptError::CoverageMismatch {
                expected: self.coverage.event_count,
                got: event_digests.len() as u64,
            });
        }
        let leaves: Vec<Digest> = event_digests
            .iter()
            .map(|d| duap_provenance::merkle::leaf_hash(self.coverage.events_root.alg, &d.bytes))
            .collect();
        let root = duap_provenance::merkle::root_of(self.coverage.events_root.alg, &leaves);
        if root != self.coverage.events_root {
            return Err(ReceiptError::RootMismatch {
                expected: self.coverage.events_root.to_string(),
                got: root.to_string(),
            });
        }
        Ok(())
    }

    /// Verify a sealed receipt end to end.
    ///
    /// Establishes, in order: the envelope is signed by a key the registry
    /// accepts in the receipt-signer role; the sealed payload is this
    /// receipt; the receipt is structurally valid; and, if an anchor is
    /// present, the receipt is included in the log at the stated head.
    ///
    /// It does *not* establish that the underlying operations occurred. See
    /// [`Receipt::claims`].
    pub fn verify(
        envelope: &Envelope,
        registry: &KeyRegistry,
        policy: &SuitePolicy,
        ctx: &VerificationContext,
    ) -> Result<VerifiedReceipt, ReceiptError> {
        let signers = envelope.verify(registry, policy, ctx)?;
        for kid in &signers {
            let rec = registry
                .get(kid)
                .ok_or_else(|| ReceiptError::UnauthorisedSigner(kid.to_string()))?;
            if !rec.has_role(KeyRole::ReceiptSigner) {
                return Err(ReceiptError::UnauthorisedSigner(format!(
                    "{kid} is not a receipt signer"
                )));
            }
        }
        let receipt: Receipt = envelope.open()?;
        receipt.validate()?;
        if receipt.issuer.as_str().is_empty() {
            return Err(ReceiptError::Malformed("empty issuer".into()));
        }

        let mut anchored = false;
        if let Some(a) = &receipt.anchor {
            let leaf_payload = duap_provenance::LogEntry {
                kind: duap_provenance::EntryKind::Receipt,
                object: receipt.digest()?,
                submitter: receipt.issuer.clone(),
                sequenced_at: a.head.issued_at,
                shard: None,
            };
            // The anchor's proof must be over the log entry that commits to
            // this receipt, at the index the anchor names.
            let leaf = duap_provenance::merkle::leaf_hash(
                a.head.alg,
                &leaf_payload
                    .to_leaf_bytes()
                    .map_err(|e| ReceiptError::Malformed(e.to_string()))?,
            );
            if a.proof.index != a.index || !a.proof.verify(leaf, a.head.root) {
                return Err(ReceiptError::AnchorInvalid);
            }
            anchored = true;
        }

        Ok(VerifiedReceipt {
            receipt,
            signers,
            anchored,
        })
    }

    /// The claims this receipt supports, and those it does not.
    ///
    /// Returned as data so that a dashboard shows the same list the
    /// specification states, rather than a marketing paraphrase of it.
    pub fn claims(&self) -> Vec<Claim> {
        let anchored = self.anchor.is_some();
        vec![
            Claim {
                statement: "The issuer's key signed this exact statement".into(),
                established: true,
                basis: "digital signature over the canonical encoding".into(),
            },
            Claim {
                statement: "The statement has not been altered since signing".into(),
                established: true,
                basis: "digest binding inside the signature input".into(),
            },
            Claim {
                statement: "The statement is bound to one specific grant document and epoch".into(),
                established: true,
                basis: "the authorization reference carries the grant's digest".into(),
            },
            Claim {
                statement: "The statement covers one specific, fixed set of events".into(),
                established: true,
                basis: "Merkle root over the covered event digests".into(),
            },
            Claim {
                statement: "The statement existed no later than the anchored tree head".into(),
                established: anchored,
                basis: if anchored {
                    "inclusion proof against a signed transparency-log head".into()
                } else {
                    "NOT ESTABLISHED: this receipt carries no log anchor".into()
                },
            },
            Claim {
                statement: "The described operations actually took place".into(),
                established: false,
                basis: "NOT ESTABLISHED: no signature can attest to a fact about the world".into(),
            },
            Claim {
                statement: "No other operations on this data went unreported".into(),
                established: false,
                basis: "NOT ESTABLISHED: absence of evidence; sequence gaps narrow but do not \
                        close this"
                    .into(),
            },
            Claim {
                statement: "The charge is a fair price".into(),
                established: false,
                basis: "NOT ESTABLISHED: the receipt reproduces the agreed arithmetic, not the \
                        fairness of the agreement"
                    .into(),
            },
            Claim {
                statement: "Deferred obligations will be honoured".into(),
                established: false,
                basis: "NOT ESTABLISHED: recorded and monitored; breach becomes provable after \
                        the fact"
                    .into(),
            },
            Claim {
                statement: "The named pseudonym corresponds to a particular legal person".into(),
                established: false,
                basis: "NOT ESTABLISHED: binding a pseudonym to an identity is outside the \
                        protocol"
                    .into(),
            },
        ]
    }
}

/// One statement a receipt does or does not support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claim {
    pub statement: String,
    pub established: bool,
    pub basis: String,
}

/// The result of verifying a sealed receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedReceipt {
    pub receipt: Receipt,
    pub signers: Vec<KeyId>,
    /// Whether an inclusion proof was present and checked.
    pub anchored: bool,
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum ReceiptError {
    #[error("receipt is malformed: {0}")]
    Malformed(String),
    #[error("receipt covers {expected} events but {got} were supplied")]
    CoverageMismatch { expected: u64, got: u64 },
    #[error("supplied events hash to {got}, not the receipt's {expected}")]
    RootMismatch { expected: String, got: String },
    #[error("the transparency-log anchor does not verify")]
    AnchorInvalid,
    #[error("signer is not authorised to issue receipts: {0}")]
    UnauthorisedSigner(String),
    #[error("{0}")]
    Crypto(#[from] duap_crypto::CryptoError),
    #[error("{0}")]
    Model(#[from] ModelError),
}

/// Builder that makes an invalid receipt hard to construct.
pub struct ReceiptBuilder {
    r: Receipt,
}

impl ReceiptBuilder {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        issuer: OrgId,
        controller: OrgId,
        subject: SubjectScope,
        period: TimeRange,
        coverage: Coverage,
        authorization: AuthorizationRef,
        decision: DecisionSummary,
        charge: Money,
        price_digest: Digest,
        issued_at: Timestamp,
    ) -> ReceiptBuilder {
        ReceiptBuilder {
            r: Receipt {
                schema: RECEIPT_SCHEMA_VERSION,
                protocol: duap_canon::PROTOCOL_ID.to_owned(),
                issuer,
                controller,
                processor: None,
                subject,
                period,
                coverage,
                authorization,
                decision,
                provenance_output: None,
                charge,
                subject_share: None,
                price_digest,
                anchor: None,
                issued_at,
            },
        }
    }

    pub fn processor(mut self, p: OrgId) -> Self {
        self.r.processor = Some(p);
        self
    }
    pub fn provenance_output(mut self, c: ContentId) -> Self {
        self.r.provenance_output = Some(c);
        self
    }
    pub fn subject_share(mut self, m: Money) -> Self {
        self.r.subject_share = Some(m);
        self
    }
    pub fn anchor(mut self, a: LogAnchor) -> Self {
        self.r.anchor = Some(a);
        self
    }

    pub fn build(self) -> Result<Receipt, ReceiptError> {
        self.r.validate()?;
        Ok(self.r)
    }
}

/// Compute the events root for a set of event digests, in acceptance order.
pub fn events_root(alg: HashAlg, event_digests: &[Digest]) -> Digest {
    let leaves: Vec<Digest> = event_digests
        .iter()
        .map(|d| duap_provenance::merkle::leaf_hash(alg, &d.bytes))
        .collect();
    duap_provenance::merkle::root_of(alg, &leaves)
}
