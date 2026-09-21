//! The journal: entries, postings, and the ledger that enforces balance.
//!
//! STATUS: PRODUCTION.

use crate::account::{Account, AccountId, AccountKind};
use duap_canon::digest::Digest;
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const ENTRY_DOMAIN: &str = "duap.journal-entry.v1";

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum LedgerError {
    #[error("journal entry {entry} does not balance in {currency}: residual {residual}")]
    Unbalanced {
        entry: String,
        currency: String,
        residual: i128,
    },
    #[error("account {0} does not exist")]
    NoSuchAccount(String),
    #[error("account {0} is closed")]
    AccountClosed(String),
    #[error("posting to {account} is in {got}, but the account is denominated in {want}")]
    CurrencyMismatch {
        account: String,
        want: String,
        got: String,
    },
    #[error("journal entry {0} has already been posted")]
    Duplicate(String),
    #[error("journal entry {0} has no postings")]
    Empty(String),
    #[error("entry {entry} would break the ledger invariant: {detail}")]
    InvariantViolated { entry: String, detail: String },
    #[error("{0}")]
    Model(#[from] ModelError),
}

/// One leg of an entry. Positive is a debit, negative a credit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Posting {
    #[serde(rename = "a")]
    pub account: AccountId,
    #[serde(rename = "m")]
    pub amount: Money,
    #[serde(rename = "n", default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

impl Posting {
    pub fn debit(account: AccountId, amount: Money) -> Posting {
        Posting {
            account,
            amount,
            memo: None,
        }
    }

    pub fn credit(account: AccountId, amount: Money) -> Posting {
        Posting {
            account,
            amount: amount.neg(),
            memo: None,
        }
    }

    pub fn with_memo(mut self, m: impl Into<String>) -> Posting {
        self.memo = Some(m.into());
        self
    }

    pub fn is_debit(&self) -> bool {
        self.amount.minor > 0
    }
}

/// What an entry is about, so that every posting traces to protocol evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "snake_case")]
pub enum EntryRef {
    Invoice { id: InvoiceId },
    Receipt { digest: Digest },
    Dispute { id: DisputeId },
    Settlement { id: SettlementId },
    Adjustment { reason: String },
    PeriodClose { period: String },
}

/// An immutable journal entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JournalEntry {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "t")]
    pub at: Timestamp,
    #[serde(rename = "d")]
    pub description: String,
    #[serde(rename = "p")]
    pub postings: Vec<Posting>,
    #[serde(rename = "r", default, skip_serializing_if = "Vec::is_empty")]
    pub refs: Vec<EntryRef>,
    /// Digest of the entry this one reverses, if any. Entries are never
    /// edited or deleted; a mistake is corrected by a reversing entry, so the
    /// history of a correction is itself auditable.
    #[serde(rename = "rv", default, skip_serializing_if = "Option::is_none")]
    pub reverses: Option<String>,
}

impl JournalEntry {
    pub fn new(
        id: impl Into<String>,
        at: Timestamp,
        description: impl Into<String>,
        postings: Vec<Posting>,
    ) -> JournalEntry {
        JournalEntry {
            id: id.into(),
            at,
            description: description.into(),
            postings,
            refs: Vec::new(),
            reverses: None,
        }
    }

    pub fn with_ref(mut self, r: EntryRef) -> Self {
        self.refs.push(r);
        self
    }

    pub fn digest(&self) -> Result<Digest, ModelError> {
        Ok(Digest::of_object(ENTRY_DOMAIN, self)?)
    }

    /// Residual per currency; all zero means the entry balances.
    pub fn residuals(&self) -> BTreeMap<String, i128> {
        let mut m: BTreeMap<String, i128> = BTreeMap::new();
        for p in &self.postings {
            *m.entry(p.amount.currency.to_string()).or_insert(0) += p.amount.minor;
        }
        m.retain(|_, v| *v != 0);
        m
    }

    pub fn balances(&self) -> bool {
        self.residuals().is_empty()
    }

    /// Build the entry that reverses this one.
    pub fn reversal(&self, id: impl Into<String>, at: Timestamp, why: &str) -> JournalEntry {
        JournalEntry {
            id: id.into(),
            at,
            description: format!("Reversal of {}: {why}", self.id),
            postings: self
                .postings
                .iter()
                .map(|p| Posting {
                    account: p.account.clone(),
                    amount: p.amount.neg(),
                    memo: p.memo.clone(),
                })
                .collect(),
            refs: self.refs.clone(),
            reverses: Some(self.id.clone()),
        }
    }
}

/// An append-only double-entry ledger.
#[derive(Debug, Clone, Default)]
pub struct Ledger {
    accounts: BTreeMap<String, Account>,
    entries: Vec<JournalEntry>,
    index: BTreeMap<String, usize>,
    balances: BTreeMap<String, i128>,
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger::default()
    }

    pub fn open_account(&mut self, a: Account) -> &Account {
        self.accounts.entry(a.id.0.clone()).or_insert(a)
    }

    /// Open an account if it does not exist, inferring name from the id.
    pub fn ensure_account(
        &mut self,
        id: AccountId,
        kind: AccountKind,
        currency: Currency,
    ) -> &Account {
        let name = id.0.clone();
        self.accounts
            .entry(id.0.clone())
            .or_insert_with(|| Account::new(id, kind, currency, name))
    }

    pub fn account(&self, id: &AccountId) -> Option<&Account> {
        self.accounts.get(&id.0)
    }

    pub fn accounts(&self) -> impl Iterator<Item = &Account> {
        self.accounts.values()
    }

    pub fn entries(&self) -> &[JournalEntry] {
        &self.entries
    }

    pub fn entry(&self, id: &str) -> Option<&JournalEntry> {
        self.index.get(id).and_then(|i| self.entries.get(*i))
    }

    /// Balance of an account in minor units, signed (debit positive).
    pub fn balance(&self, id: &AccountId) -> i128 {
        self.balances.get(&id.0).copied().unwrap_or(0)
    }

    /// Balance as `Money`, or `None` if the account is unknown.
    pub fn balance_money(&self, id: &AccountId) -> Option<Money> {
        self.accounts
            .get(&id.0)
            .map(|a| Money::new(a.currency, self.balance(id)))
    }

    /// Post an entry, enforcing every invariant.
    pub fn post(&mut self, e: JournalEntry) -> Result<Digest, LedgerError> {
        if e.postings.is_empty() {
            return Err(LedgerError::Empty(e.id));
        }
        if self.index.contains_key(&e.id) {
            return Err(LedgerError::Duplicate(e.id));
        }
        let residuals = e.residuals();
        if let Some((currency, residual)) = residuals.into_iter().next() {
            return Err(LedgerError::Unbalanced {
                entry: e.id,
                currency,
                residual,
            });
        }
        for p in &e.postings {
            let a = self
                .accounts
                .get(&p.account.0)
                .ok_or_else(|| LedgerError::NoSuchAccount(p.account.0.clone()))?;
            if a.closed {
                return Err(LedgerError::AccountClosed(p.account.0.clone()));
            }
            if a.currency != p.amount.currency {
                return Err(LedgerError::CurrencyMismatch {
                    account: p.account.0.clone(),
                    want: a.currency.to_string(),
                    got: p.amount.currency.to_string(),
                });
            }
        }
        let digest = e.digest()?;
        for p in &e.postings {
            *self.balances.entry(p.account.0.clone()).or_insert(0) += p.amount.minor;
        }
        self.index.insert(e.id.clone(), self.entries.len());
        self.entries.push(e);
        Ok(digest)
    }

    /// Sum of all balances per currency. Every value must be zero.
    pub fn trial_balance(&self) -> BTreeMap<String, i128> {
        let mut m: BTreeMap<String, i128> = BTreeMap::new();
        for (id, bal) in &self.balances {
            if let Some(a) = self.accounts.get(id) {
                *m.entry(a.currency.to_string()).or_insert(0) += bal;
            }
        }
        m
    }

    /// Whether the ledger is in balance.
    pub fn is_balanced(&self) -> bool {
        self.trial_balance().values().all(|v| *v == 0)
    }

    /// Total debits and total credits per currency, for a conventional trial
    /// balance report.
    pub fn debits_and_credits(&self) -> BTreeMap<String, (i128, i128)> {
        let mut m: BTreeMap<String, (i128, i128)> = BTreeMap::new();
        for e in &self.entries {
            for p in &e.postings {
                let slot = m.entry(p.amount.currency.to_string()).or_insert((0, 0));
                if p.amount.minor > 0 {
                    slot.0 += p.amount.minor;
                } else {
                    slot.1 += -p.amount.minor;
                }
            }
        }
        m
    }

    /// Statement for one account: every posting that touched it.
    pub fn statement(&self, id: &AccountId) -> Vec<(&JournalEntry, &Posting)> {
        self.entries
            .iter()
            .flat_map(|e| e.postings.iter().map(move |p| (e, p)))
            .filter(|(_, p)| p.account == *id)
            .collect()
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}
