//! Disputes.
//!
//! STATUS: PRODUCTION for the case machinery and the evidence bundle;
//! resolution is explicitly *not* automated.
//!
//! # Why disputes cannot be fully automated
//!
//! The protocol can prove that a receipt was signed by a particular key,
//! that an event was included in the log by a particular time, and that an
//! invoice's arithmetic follows from its lines. It cannot prove that the
//! event described something that really happened. A controller can sign a
//! truthful receipt for a fabricated operation, and a subject can deny an
//! operation that did occur. Deciding between them is a question of fact,
//! and questions of fact are settled by people and, in the end, by courts.
//!
//! What DUAP contributes is to make the evidence *portable and verifiable*:
//! a dispute bundle is a self-contained set of signed objects and proofs
//! that a third party can check offline, without trusting the clearing node,
//! the controller, or the subject. That turns "your logs say otherwise" into
//! a checkable claim.
//!
//! # The state machine
//!
//! ```text
//! Filed -> UnderReview -> { Upheld, Rejected, Withdrawn, Settled } -> Closed
//! ```
//!
//! Only `Upheld` and `Settled` produce an accounting adjustment, and the
//! adjustment is always a *reversing entry plus a new entry*, never an edit.

use crate::journal::{EntryRef, JournalEntry, Ledger, LedgerError};
use duap_canon::digest::Digest;
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};

pub const DISPUTE_DOMAIN: &str = "duap.dispute.v1";

/// What is being disputed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "snake_case")]
pub enum Claim {
    /// The subject says the operation never happened.
    OperationDenied { event: Digest },
    /// The subject says the operation was not authorised.
    NotAuthorised { event: Digest, grant: GrantId },
    /// A party says the quantity is wrong.
    QuantityWrong {
        line: u32,
        asserted: u64,
        claimed: u64,
    },
    /// A party says the price is wrong.
    PriceWrong { line: u32, reason: String },
    /// A party says an operation happened and was not reported.
    UnreportedUsage { description: String },
    /// A subject says a deletion obligation was not honoured.
    ObligationBreached { obligation: String, event: Digest },
    /// A party says the same operation was billed twice.
    DoubleCounted { a: Digest, b: Digest },
}

/// A piece of evidence, referenced rather than embedded so that a bundle
/// stays small and the referenced objects stay content-addressed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "snake_case")]
pub enum Evidence {
    /// A signed event.
    Event { digest: Digest },
    /// A signed grant at a specific epoch.
    Grant {
        id: GrantId,
        epoch: u32,
        digest: Digest,
    },
    /// A revocation.
    Revocation { digest: Digest },
    /// A receipt.
    Receipt { digest: Digest },
    /// An inclusion proof in the transparency log.
    LogInclusion {
        entry: Digest,
        log: String,
        size: u64,
    },
    /// An opening of a value commitment.
    CommitmentOpening { commitment: Digest },
    /// An invoice line.
    InvoiceLine { invoice: InvoiceId, line: u32 },
    /// A statement from a party that is not itself protocol evidence. Marked
    /// as such so that a reviewer never mistakes an assertion for a proof.
    Assertion { by: String, statement: String },
}

impl Evidence {
    /// Whether this evidence is cryptographically checkable, as opposed to
    /// someone's word.
    pub fn is_verifiable(&self) -> bool {
        !matches!(self, Evidence::Assertion { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisputeState {
    Filed,
    UnderReview,
    Upheld,
    Rejected,
    Withdrawn,
    Settled,
    Closed,
}

impl DisputeState {
    pub fn is_terminal(self) -> bool {
        matches!(self, DisputeState::Closed)
    }

    fn can_move_to(self, next: DisputeState) -> bool {
        use DisputeState::*;
        matches!(
            (self, next),
            (Filed, UnderReview)
                | (Filed, Withdrawn)
                | (UnderReview, Upheld)
                | (UnderReview, Rejected)
                | (UnderReview, Settled)
                | (UnderReview, Withdrawn)
                | (Upheld, Closed)
                | (Rejected, Closed)
                | (Settled, Closed)
                | (Withdrawn, Closed)
        )
    }
}

/// A dispute case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dispute {
    #[serde(rename = "v")]
    pub schema: u16,
    #[serde(rename = "id")]
    pub id: DisputeId,
    #[serde(rename = "cl")]
    pub claimant: String,
    #[serde(rename = "rs")]
    pub respondent: OrgId,
    #[serde(rename = "iv", default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<InvoiceId>,
    #[serde(rename = "cm")]
    pub claim: Claim,
    #[serde(rename = "ev")]
    pub evidence: Vec<Evidence>,
    #[serde(rename = "ce", default, skip_serializing_if = "Vec::is_empty")]
    pub counter_evidence: Vec<Evidence>,
    #[serde(rename = "st")]
    pub state: DisputeState,
    #[serde(rename = "at")]
    pub filed_at: Timestamp,
    #[serde(rename = "up", default, skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<Timestamp>,
    /// Adjustment agreed or ordered, if any.
    #[serde(rename = "aj", default, skip_serializing_if = "Option::is_none")]
    pub adjustment: Option<Money>,
    #[serde(rename = "nt", default, skip_serializing_if = "Option::is_none")]
    pub resolution_note: Option<String>,
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum DisputeError {
    #[error("cannot move a dispute from {from:?} to {to:?}")]
    BadTransition {
        from: DisputeState,
        to: DisputeState,
    },
    #[error("a dispute must carry at least one item of verifiable evidence")]
    NoVerifiableEvidence,
    #[error("an adjustment is required when a dispute is upheld or settled")]
    AdjustmentRequired,
    #[error("{0}")]
    Ledger(#[from] LedgerError),
    #[error("{0}")]
    Model(#[from] ModelError),
}

impl Dispute {
    pub fn file(
        id: DisputeId,
        claimant: impl Into<String>,
        respondent: OrgId,
        claim: Claim,
        evidence: Vec<Evidence>,
        at: Timestamp,
    ) -> Result<Dispute, DisputeError> {
        if !evidence.iter().any(Evidence::is_verifiable) {
            return Err(DisputeError::NoVerifiableEvidence);
        }
        Ok(Dispute {
            schema: 1,
            id,
            claimant: claimant.into(),
            respondent,
            invoice: None,
            claim,
            evidence,
            counter_evidence: Vec::new(),
            state: DisputeState::Filed,
            filed_at: at,
            resolved_at: None,
            adjustment: None,
            resolution_note: None,
        })
    }

    pub fn digest(&self) -> Result<Digest, ModelError> {
        Ok(Digest::of_object(DISPUTE_DOMAIN, self)?)
    }

    pub fn transition(
        &mut self,
        to: DisputeState,
        at: Timestamp,
        note: Option<String>,
    ) -> Result<(), DisputeError> {
        if !self.state.can_move_to(to) {
            return Err(DisputeError::BadTransition {
                from: self.state,
                to,
            });
        }
        if matches!(to, DisputeState::Upheld | DisputeState::Settled) && self.adjustment.is_none() {
            return Err(DisputeError::AdjustmentRequired);
        }
        self.state = to;
        if matches!(
            to,
            DisputeState::Upheld
                | DisputeState::Rejected
                | DisputeState::Settled
                | DisputeState::Withdrawn
        ) {
            self.resolved_at = Some(at);
        }
        if note.is_some() {
            self.resolution_note = note;
        }
        Ok(())
    }

    pub fn propose_adjustment(&mut self, m: Money) {
        self.adjustment = Some(m);
    }

    /// Count of evidence items a third party can check without trusting
    /// anyone. A dispute resting entirely on assertions is not refused, but
    /// the count makes its weakness visible in a case list.
    pub fn verifiable_evidence_count(&self) -> usize {
        self.evidence
            .iter()
            .chain(self.counter_evidence.iter())
            .filter(|e| e.is_verifiable())
            .count()
    }

    /// The adjusting journal entry. Always a new entry; the original stands.
    pub fn to_adjustment_entry(
        &self,
        ledger: &Ledger,
        at: Timestamp,
    ) -> Result<Option<JournalEntry>, DisputeError> {
        let Some(adj) = self.adjustment else {
            return Ok(None);
        };
        if adj.is_zero() {
            return Ok(None);
        }
        let Some(inv) = self.invoice else {
            return Ok(None);
        };
        let original = ledger.entry(&format!("je:invoice:{inv}"));
        let Some(original) = original else {
            return Ok(None);
        };
        // Scale every posting of the original entry by adj/total, so the
        // adjustment lands in the same accounts in the same proportions.
        let total: i128 = original
            .postings
            .iter()
            .filter(|p| p.amount.minor > 0)
            .map(|p| p.amount.minor)
            .sum();
        if total == 0 {
            return Ok(None);
        }
        let mut postings = Vec::new();
        let mut assigned = 0i128;
        let n = original.postings.len();
        for (i, p) in original.postings.iter().enumerate() {
            let scaled = if i + 1 == n {
                // Last posting absorbs the rounding so the entry balances.
                -assigned
            } else {
                let v = -(p.amount.minor * adj.minor) / total;
                assigned += v;
                v
            };
            if scaled != 0 {
                postings.push(crate::journal::Posting {
                    account: p.account.clone(),
                    amount: Money::new(p.amount.currency, scaled),
                    memo: Some(format!("dispute {} adjustment", self.id)),
                });
            }
        }
        if postings.is_empty() {
            return Ok(None);
        }
        Ok(Some(
            JournalEntry::new(
                format!("je:dispute:{}", self.id),
                at,
                format!("Adjustment for dispute {}", self.id),
                postings,
            )
            .with_ref(EntryRef::Dispute { id: self.id }),
        ))
    }
}
