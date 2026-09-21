//! Accounts and the double-entry invariant.
//!
//! STATUS: PRODUCTION.
//!
//! # Why double entry
//!
//! A single-entry counter of "what each organisation owes" cannot answer
//! where the money came from, cannot be reconciled against a bank statement,
//! and cannot detect its own corruption. Double entry can: every economic
//! fact is recorded twice, with opposite signs, so the whole ledger sums to
//! zero and any deviation is a bug that surfaces immediately rather than a
//! discrepancy discovered at audit.
//!
//! DUAP uses the signed-amount convention: a posting's amount is positive for
//! a debit and negative for a credit, and an entry is valid only if its
//! postings sum to zero **per currency**. Cross-currency movements are
//! modelled as two entries through an FX account, never as one entry with
//! mixed currencies, because netting mixed currencies inside one entry hides
//! the rate that was used.
//!
//! # The invariant
//!
//! ```text
//! for every journal entry e, for every currency c:
//!     sum { p.amount : p in e.postings, p.currency == c } == 0
//! ```
//!
//! and therefore, over the whole ledger,
//!
//! ```text
//! sum of all debits == sum of all credits
//! ```
//!
//! This is checked on every post ([`crate::journal::Ledger::post`]), asserted
//! by [`crate::journal::Ledger::trial_balance`], and model-checked in
//! `formal/Accounting.tla`.

use duap_model::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The five classical account kinds, plus two DUAP-specific ones whose
/// purpose is to make rounding and suspense *visible* rather than absorbed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountKind {
    /// Resources controlled by the entity. Normal balance: debit.
    Asset,
    /// Obligations. Normal balance: credit. Subject balances live here:
    /// money owed to a data subject is a liability from the moment it
    /// accrues, not from the moment it is paid.
    Liability,
    /// Owners' residual interest. Normal balance: credit.
    Equity,
    /// Income. Normal balance: credit.
    Revenue,
    /// Costs. Normal balance: debit.
    Expense,
    /// Sub-minor-unit residue from rounding. Normal balance: either.
    ///
    /// Every rounding operation posts its residue here, so the ledger can
    /// prove that rounding neither created nor destroyed value. A production
    /// deployment sweeps this to revenue or expense on a schedule, and the
    /// sweep is itself a journal entry.
    Rounding,
    /// Amounts whose treatment is not yet decided: a suspended double count,
    /// a disputed line, an unidentified receipt. Normal balance: either.
    ///
    /// A non-zero suspense balance at period end is an operational
    /// exception, and `OPERATIONS.md` makes clearing it a close-out task.
    Suspense,
}

impl AccountKind {
    /// Whether the kind normally carries a debit (positive) balance.
    pub const fn normal_debit(self) -> bool {
        matches!(self, AccountKind::Asset | AccountKind::Expense)
    }

    pub const fn code(self) -> &'static str {
        match self {
            AccountKind::Asset => "asset",
            AccountKind::Liability => "liability",
            AccountKind::Equity => "equity",
            AccountKind::Revenue => "revenue",
            AccountKind::Expense => "expense",
            AccountKind::Rounding => "rounding",
            AccountKind::Suspense => "suspense",
        }
    }
}

/// An account identifier, `acct:<owner-or-system>/<path>`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountId(pub String);

impl AccountId {
    pub fn new(s: impl Into<String>) -> AccountId {
        AccountId(s.into())
    }

    /// Receivable from a controller: money the clearing node expects.
    pub fn receivable(org: &OrgId) -> AccountId {
        AccountId(format!("acct:{}/receivable", org.as_str()))
    }

    /// Revenue recognised by the clearing node on its own fee.
    pub fn clearing_fee_revenue() -> AccountId {
        AccountId("acct:system/clearing-fee-revenue".into())
    }

    /// Amount owed to a data subject.
    pub fn subject_payable(subject: &SubjectRef) -> AccountId {
        AccountId(format!("acct:subject/{subject}/payable"))
    }

    /// Amount owed to a controller acting as a data supplier.
    pub fn org_payable(org: &OrgId) -> AccountId {
        AccountId(format!("acct:{}/payable", org.as_str()))
    }

    /// Cash held pending settlement.
    pub fn settlement_cash(currency: Currency) -> AccountId {
        AccountId(format!("acct:system/cash/{currency}"))
    }

    pub fn rounding(currency: Currency) -> AccountId {
        AccountId(format!("acct:system/rounding/{currency}"))
    }

    pub fn suspense(currency: Currency) -> AccountId {
        AccountId(format!("acct:system/suspense/{currency}"))
    }

    pub fn tax_payable(jurisdiction: &str, currency: Currency) -> AccountId {
        AccountId(format!("acct:system/tax-payable/{jurisdiction}/{currency}"))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Account {
    pub id: AccountId,
    pub kind: AccountKind,
    pub currency: Currency,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<OrgId>,
    pub name: String,
    /// Whether postings may still be made.
    #[serde(default)]
    pub closed: bool,
}

impl Account {
    pub fn new(
        id: AccountId,
        kind: AccountKind,
        currency: Currency,
        name: impl Into<String>,
    ) -> Account {
        Account {
            id,
            kind,
            currency,
            owner: None,
            name: name.into(),
            closed: false,
        }
    }

    pub fn owned_by(mut self, o: OrgId) -> Account {
        self.owner = Some(o);
        self
    }
}
