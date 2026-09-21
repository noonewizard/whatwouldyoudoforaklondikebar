//! Netting and settlement instructions.
//!
//! STATUS: PRODUCTION for netting and the instruction model. Payment rails
//! are INTERFACES ONLY: DUAP does not move money and does not embed a
//! payment provider. A deployment implements [`SettlementRail`] against
//! whatever it actually uses.
//!
//! # Netting
//!
//! Without netting, a period with n participants can require O(n^2)
//! transfers. Multilateral netting reduces that to at most n-1: compute each
//! participant's net position, then match debtors to creditors greedily.
//!
//! The greedy matching is not guaranteed to minimise the transfer *count*
//! (that problem is NP-hard in general), but it is guaranteed to produce at
//! most n-1 transfers and to conserve value exactly, both of which are
//! asserted by tests. Minimising count further is not worth an
//! approximation algorithm whose output a participant would then have to
//! audit.
//!
//! # What netting changes legally
//!
//! Netting replaces many obligations with fewer. Whether that replacement is
//! effective against an insolvent participant depends on the netting
//! agreement and the applicable insolvency law, and is exactly the kind of
//! question DUAP records rather than answers: the gross positions remain in
//! the ledger, so a liquidator can reconstruct them.

use crate::account::AccountId;
use crate::journal::{EntryRef, JournalEntry, Posting};
use duap_canon::digest::Digest;
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SETTLEMENT_DOMAIN: &str = "duap.settlement.v1";

/// A gross obligation before netting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Obligation {
    pub from: String,
    pub to: String,
    pub amount: Money,
}

/// A transfer produced by netting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetTransfer {
    pub from: String,
    pub to: String,
    pub amount: Money,
}

/// Net a set of gross obligations.
///
/// Returns the transfers and the per-participant net positions (positive
/// means owed money).
pub fn net(
    obligations: &[Obligation],
    currency: Currency,
) -> Result<(Vec<NetTransfer>, BTreeMap<String, i128>), ModelError> {
    let mut pos: BTreeMap<String, i128> = BTreeMap::new();
    for o in obligations {
        if o.amount.currency != currency {
            return Err(ModelError::CurrencyMismatch {
                a: currency.to_string(),
                b: o.amount.currency.to_string(),
            });
        }
        *pos.entry(o.from.clone()).or_insert(0) -= o.amount.minor;
        *pos.entry(o.to.clone()).or_insert(0) += o.amount.minor;
    }

    let mut debtors: Vec<(String, i128)> = pos
        .iter()
        .filter(|(_, v)| **v < 0)
        .map(|(k, v)| (k.clone(), -*v))
        .collect();
    let mut creditors: Vec<(String, i128)> = pos
        .iter()
        .filter(|(_, v)| **v > 0)
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    // Deterministic order: largest first, ties by name.
    debtors.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    creditors.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let mut transfers = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);
    while i < debtors.len() && j < creditors.len() {
        let amount = debtors[i].1.min(creditors[j].1);
        if amount > 0 {
            transfers.push(NetTransfer {
                from: debtors[i].0.clone(),
                to: creditors[j].0.clone(),
                amount: Money::new(currency, amount),
            });
            debtors[i].1 -= amount;
            creditors[j].1 -= amount;
        }
        if debtors[i].1 == 0 {
            i += 1;
        }
        if creditors[j].1 == 0 {
            j += 1;
        }
    }
    Ok((transfers, pos))
}

/// A payment rail. DUAP records the rail; it does not implement one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "snake_case")]
pub enum Rail {
    /// Movement between accounts held by the same clearing node. The only
    /// rail the reference implementation can actually execute.
    Internal,
    /// SEPA credit transfer. INTERFACE ONLY.
    SepaCreditTransfer { iban_ref: String },
    /// US ACH. INTERFACE ONLY.
    Ach { account_ref: String },
    /// Wire. INTERFACE ONLY.
    Wire { reference: String },
    /// Card payout. INTERFACE ONLY.
    Card { token_ref: String },
    /// Distributed-ledger settlement. UNIMPLEMENTED and, on the analysis in
    /// `docs/adr/0005-transparency-log-not-blockchain.md`, not required by
    /// anything in the protocol. Recorded here so a deployment that chooses
    /// it can do so without a format change.
    DistributedLedger { network: String, address_ref: String },
}

impl Rail {
    /// Whether the reference implementation can execute this rail.
    pub fn executable_here(&self) -> bool {
        matches!(self, Rail::Internal)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementState {
    Created,
    Submitted,
    Confirmed,
    Failed,
    Returned,
    Cancelled,
}

impl SettlementState {
    fn can_move_to(self, next: SettlementState) -> bool {
        use SettlementState::*;
        matches!(
            (self, next),
            (Created, Submitted)
                | (Created, Cancelled)
                | (Submitted, Confirmed)
                | (Submitted, Failed)
                | (Confirmed, Returned)
                | (Failed, Submitted)
        )
    }
}

/// A signed instruction to move money.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettlementInstruction {
    #[serde(rename = "v")]
    pub schema: u16,
    #[serde(rename = "id")]
    pub id: SettlementId,
    #[serde(rename = "fr")]
    pub from: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "am")]
    pub amount: Money,
    #[serde(rename = "rl")]
    pub rail: Rail,
    #[serde(rename = "st")]
    pub state: SettlementState,
    #[serde(rename = "at")]
    pub created_at: Timestamp,
    /// External reference returned by the rail, once submitted.
    #[serde(rename = "xr", default, skip_serializing_if = "Option::is_none")]
    pub external_ref: Option<String>,
    /// Invoices this instruction settles.
    #[serde(rename = "iv", default, skip_serializing_if = "Vec::is_empty")]
    pub invoices: Vec<InvoiceId>,
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum SettlementError {
    #[error("cannot move settlement from {from:?} to {to:?}")]
    BadTransition {
        from: SettlementState,
        to: SettlementState,
    },
    #[error("rail {0} is not executable by the reference implementation")]
    RailNotExecutable(String),
    #[error("{0}")]
    Model(#[from] ModelError),
}

impl SettlementInstruction {
    pub fn new(
        id: SettlementId,
        from: impl Into<String>,
        to: impl Into<String>,
        amount: Money,
        rail: Rail,
        at: Timestamp,
    ) -> SettlementInstruction {
        SettlementInstruction {
            schema: 1,
            id,
            from: from.into(),
            to: to.into(),
            amount,
            rail,
            state: SettlementState::Created,
            created_at: at,
            external_ref: None,
            invoices: Vec::new(),
        }
    }

    pub fn digest(&self) -> Result<Digest, ModelError> {
        Ok(Digest::of_object(SETTLEMENT_DOMAIN, self)?)
    }

    pub fn transition(
        &mut self,
        to: SettlementState,
        external_ref: Option<String>,
    ) -> Result<(), SettlementError> {
        if !self.state.can_move_to(to) {
            return Err(SettlementError::BadTransition {
                from: self.state,
                to,
            });
        }
        self.state = to;
        if external_ref.is_some() {
            self.external_ref = external_ref;
        }
        Ok(())
    }

    /// The journal entry that records a confirmed settlement.
    ///
    /// Debit the payable (reducing what is owed), credit cash.
    pub fn to_journal_entry(
        &self,
        payable: AccountId,
        cash: AccountId,
        at: Timestamp,
    ) -> JournalEntry {
        JournalEntry::new(
            format!("je:settlement:{}", self.id),
            at,
            format!("Settlement {} to {}", self.id, self.to),
            vec![
                Posting::debit(payable, self.amount).with_memo("discharge of liability"),
                Posting::credit(cash, self.amount).with_memo("cash out"),
            ],
        )
        .with_ref(EntryRef::Settlement { id: self.id })
    }
}

/// The interface a deployment implements to actually move money.
pub trait SettlementRail {
    /// Submit an instruction. Returns the rail's own reference.
    fn submit(&self, instruction: &SettlementInstruction) -> Result<String, String>;
    /// Poll for the outcome.
    fn status(&self, external_ref: &str) -> Result<SettlementState, String>;
}

/// The only rail the reference implementation ships: internal book transfer.
pub struct InternalRail;

impl SettlementRail for InternalRail {
    fn submit(&self, instruction: &SettlementInstruction) -> Result<String, String> {
        if !instruction.rail.executable_here() {
            return Err(format!(
                "rail {:?} is an interface only; supply an implementation",
                instruction.rail
            ));
        }
        Ok(format!("internal:{}", instruction.id))
    }

    fn status(&self, _external_ref: &str) -> Result<SettlementState, String> {
        Ok(SettlementState::Confirmed)
    }
}
