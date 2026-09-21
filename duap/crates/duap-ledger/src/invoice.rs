//! Invoices and their journal effects.
//!
//! STATUS: PRODUCTION for the invoice model and posting rules. Tax is a
//! **hook**, not an implementation: [`TaxPolicy`] is a trait a deployment
//! implements against its own advice. Shipping a built-in tax engine would
//! be pretending to know the answer to a question that differs by
//! jurisdiction, by the nature of the supply, and by the status of both
//! parties. `COMPLIANCE.md` says what DUAP records that a tax determination
//! needs.

use crate::account::{AccountId, AccountKind};
use crate::journal::{EntryRef, JournalEntry, Posting};
use duap_canon::digest::Digest;
use duap_meter::UsageKey;
use duap_model::prelude::*;
use duap_valuation::PriceBreakdown;
use serde::{Deserialize, Serialize};

pub const INVOICE_DOMAIN: &str = "duap.invoice.v1";

/// One charged line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InvoiceLine {
    #[serde(rename = "n")]
    pub line_no: u32,
    #[serde(rename = "d")]
    pub description: String,
    /// The aggregation key that produced the line, so the charge can be
    /// traced to its events.
    #[serde(rename = "k")]
    pub key: UsageKey,
    /// Merkle root over the events behind the line.
    #[serde(rename = "ev")]
    pub evidence_root: Digest,
    #[serde(rename = "b")]
    pub breakdown: PriceBreakdown,
    /// Rounded charge.
    #[serde(rename = "a")]
    pub amount: Money,
    /// Sub-minor-unit residue left by rounding this line.
    #[serde(rename = "rs")]
    pub residue: Precise,
    /// Share of the line payable onward to the data subject, if any.
    #[serde(rename = "sp", default, skip_serializing_if = "Option::is_none")]
    pub subject_share: Option<Money>,
}

/// A tax line, computed by the deployment's [`TaxPolicy`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaxLine {
    #[serde(rename = "c")]
    pub code: String,
    #[serde(rename = "j")]
    pub jurisdiction: String,
    #[serde(rename = "r")]
    pub rate: Ratio,
    #[serde(rename = "b")]
    pub base: Money,
    #[serde(rename = "a")]
    pub amount: Money,
}

/// A deployment-supplied tax determination.
///
/// DUAP ships [`NoTax`] and nothing else. An implementation receives the
/// invoice's lines and parties and returns the tax lines; it is expected to
/// be backed by a tax engine, not by code written by a protocol author.
pub trait TaxPolicy {
    fn tax_lines(&self, invoice: &Invoice) -> Result<Vec<TaxLine>, ModelError>;
}

/// The default: charge no tax and say so, rather than silently charging zero.
pub struct NoTax;

impl TaxPolicy for NoTax {
    fn tax_lines(&self, _invoice: &Invoice) -> Result<Vec<TaxLine>, ModelError> {
        Ok(Vec::new())
    }
}

/// A flat-rate policy, for tests and for deployments where a single rate has
/// been determined externally. NOT a tax engine.
pub struct FlatRateTax {
    pub code: String,
    pub jurisdiction: String,
    pub rate: Ratio,
    pub rounding: Rounding,
}

impl TaxPolicy for FlatRateTax {
    fn tax_lines(&self, invoice: &Invoice) -> Result<Vec<TaxLine>, ModelError> {
        let base = invoice.subtotal;
        let amount = Precise::from_money(base)?
            .mul_ratio(self.rate)?
            .round_to_money(self.rounding)
            .0;
        Ok(vec![TaxLine {
            code: self.code.clone(),
            jurisdiction: self.jurisdiction.clone(),
            rate: self.rate,
            base,
            amount,
        }])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Issued,
    PartiallyPaid,
    Paid,
    Disputed,
    Credited,
    WrittenOff,
}

/// An invoice from the clearing node to a controller.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Invoice {
    #[serde(rename = "v")]
    pub schema: u16,
    #[serde(rename = "id")]
    pub id: InvoiceId,
    #[serde(rename = "is")]
    pub issuer: OrgId,
    #[serde(rename = "py")]
    pub payer: OrgId,
    #[serde(rename = "pd")]
    pub period: TimeRange,
    #[serde(rename = "cu")]
    pub currency: Currency,
    #[serde(rename = "ln")]
    pub lines: Vec<InvoiceLine>,
    #[serde(rename = "sb")]
    pub subtotal: Money,
    #[serde(rename = "tx", default, skip_serializing_if = "Vec::is_empty")]
    pub taxes: Vec<TaxLine>,
    #[serde(rename = "tt")]
    pub total: Money,
    /// Residue swept to the rounding account.
    #[serde(rename = "rs")]
    pub residue: Precise,
    #[serde(rename = "st")]
    pub status: InvoiceStatus,
    #[serde(rename = "at")]
    pub issued_at: Timestamp,
    #[serde(rename = "du")]
    pub due_at: Timestamp,
}

impl Invoice {
    pub fn digest(&self) -> Result<Digest, ModelError> {
        Ok(Digest::of_object(INVOICE_DOMAIN, self)?)
    }

    /// Recompute the subtotal from the lines. An invoice whose stated
    /// subtotal disagrees with its lines is rejected rather than trusted.
    pub fn check_arithmetic(&self) -> Result<(), ModelError> {
        let mut sum = Money::zero(self.currency);
        for l in &self.lines {
            sum = sum.add(&l.amount)?;
        }
        if sum != self.subtotal {
            return Err(ModelError::Invalid {
                field: "invoice.subtotal",
                reason: format!("lines sum to {sum} but the subtotal states {}", self.subtotal),
            });
        }
        let mut total = self.subtotal;
        for t in &self.taxes {
            total = total.add(&t.amount)?;
        }
        if total != self.total {
            return Err(ModelError::Invalid {
                field: "invoice.total",
                reason: format!("subtotal plus tax is {total} but the total states {}", self.total),
            });
        }
        Ok(())
    }

    /// The journal entry that recognises this invoice.
    ///
    /// Debit the payer's receivable, credit revenue for the clearing fee,
    /// credit the subjects' payable for their shares, credit tax payable,
    /// and post the rounding residue so that nothing is lost.
    pub fn to_journal_entry(&self, at: Timestamp) -> Result<JournalEntry, ModelError> {
        let mut postings = Vec::new();
        postings.push(
            Posting::debit(AccountId::receivable(&self.payer), self.total)
                .with_memo(format!("invoice {}", self.id)),
        );

        let mut subject_total = Money::zero(self.currency);
        for l in &self.lines {
            if let Some(s) = l.subject_share {
                if !s.is_zero() {
                    let account = match l.key.subject {
                        Some(sub) => AccountId::subject_payable(&sub),
                        None => AccountId::org_payable(&l.key.controller),
                    };
                    postings.push(
                        Posting::credit(account, s)
                            .with_memo(format!("line {} subject share", l.line_no)),
                    );
                    subject_total = subject_total.add(&s)?;
                }
            }
        }

        let mut tax_total = Money::zero(self.currency);
        for t in &self.taxes {
            if !t.amount.is_zero() {
                postings.push(
                    Posting::credit(
                        AccountId::tax_payable(&t.jurisdiction, self.currency),
                        t.amount,
                    )
                    .with_memo(t.code.clone()),
                );
                tax_total = tax_total.add(&t.amount)?;
            }
        }

        let fee = self.total.sub(&subject_total)?.sub(&tax_total)?;
        if !fee.is_zero() {
            postings.push(
                Posting::credit(AccountId::clearing_fee_revenue(), fee)
                    .with_memo("clearing margin and unattributed remainder"),
            );
        }

        Ok(JournalEntry::new(
            format!("je:invoice:{}", self.id),
            at,
            format!("Invoice {} to {}", self.id, self.payer),
            postings,
        )
        .with_ref(EntryRef::Invoice { id: self.id }))
    }

    /// Accounts this invoice needs, with their kinds, so a caller can open
    /// them before posting.
    pub fn required_accounts(&self) -> Vec<(AccountId, AccountKind)> {
        let mut v = vec![
            (AccountId::receivable(&self.payer), AccountKind::Asset),
            (AccountId::clearing_fee_revenue(), AccountKind::Revenue),
        ];
        for l in &self.lines {
            if l.subject_share.is_some_and(|s| !s.is_zero()) {
                let a = match l.key.subject {
                    Some(sub) => AccountId::subject_payable(&sub),
                    None => AccountId::org_payable(&l.key.controller),
                };
                v.push((a, AccountKind::Liability));
            }
        }
        for t in &self.taxes {
            v.push((
                AccountId::tax_payable(&t.jurisdiction, self.currency),
                AccountKind::Liability,
            ));
        }
        v
    }
}

/// Assembles invoices from priced usage.
pub struct InvoiceBuilder {
    id: InvoiceId,
    issuer: OrgId,
    payer: OrgId,
    period: TimeRange,
    currency: Currency,
    lines: Vec<InvoiceLine>,
    residue: Precise,
    rounding: Rounding,
}

impl InvoiceBuilder {
    pub fn new(
        id: InvoiceId,
        issuer: OrgId,
        payer: OrgId,
        period: TimeRange,
        currency: Currency,
    ) -> InvoiceBuilder {
        InvoiceBuilder {
            id,
            issuer,
            payer,
            period,
            currency,
            lines: Vec::new(),
            residue: Precise::zero(currency),
            rounding: Rounding::HalfEven,
        }
    }

    pub fn rounding(mut self, r: Rounding) -> Self {
        self.rounding = r;
        self
    }

    /// Add a priced usage counter as a line.
    ///
    /// Rounding happens here, once per line, and the residue is accumulated
    /// so that the invoice as a whole can prove conservation.
    pub fn line(
        &mut self,
        key: UsageKey,
        evidence_root: Digest,
        breakdown: PriceBreakdown,
        description: impl Into<String>,
        subject_share_of_line: Option<Ratio>,
    ) -> Result<&InvoiceLine, ModelError> {
        let (amount, residue) = breakdown.amount.round_to_money(self.rounding);
        self.residue = self.residue.add(&residue)?;
        let subject_share = match subject_share_of_line {
            Some(r) => {
                let (m, extra) = Precise::from_money(amount)?
                    .mul_ratio(r)?
                    .round_to_money(Rounding::TowardZero);
                self.residue = self.residue.add(&extra)?;
                Some(m)
            }
            None => None,
        };
        self.lines.push(InvoiceLine {
            line_no: self.lines.len() as u32 + 1,
            description: description.into(),
            key,
            evidence_root,
            breakdown,
            amount,
            residue,
            subject_share,
        });
        Ok(self.lines.last().expect("just pushed"))
    }

    pub fn build<T: TaxPolicy>(
        self,
        tax: &T,
        issued_at: Timestamp,
        due_at: Timestamp,
    ) -> Result<Invoice, ModelError> {
        let mut subtotal = Money::zero(self.currency);
        for l in &self.lines {
            subtotal = subtotal.add(&l.amount)?;
        }
        let mut inv = Invoice {
            schema: 1,
            id: self.id,
            issuer: self.issuer,
            payer: self.payer,
            period: self.period,
            currency: self.currency,
            lines: self.lines,
            subtotal,
            taxes: Vec::new(),
            total: subtotal,
            residue: self.residue,
            status: InvoiceStatus::Draft,
            issued_at,
            due_at,
        };
        inv.taxes = tax.tax_lines(&inv)?;
        let mut total = subtotal;
        for t in &inv.taxes {
            total = total.add(&t.amount)?;
        }
        inv.total = total;
        inv.check_arithmetic()?;
        Ok(inv)
    }
}
