//! # duap-ledger
//!
//! Double-entry accounting, invoicing, disputes, netting and settlement
//! instructions.
//!
//! STATUS: PRODUCTION for the ledger, invoicing, disputes and netting.
//! Payment rails and tax are deliberately interfaces: DUAP records what was
//! charged and what is owed, and hands execution to systems that exist for
//! that purpose.
//!
//! The invariant the whole crate exists to maintain:
//!
//! ```text
//! sum of all debits == sum of all credits, per currency, always
//! ```
//!
//! Rounding cannot break it (residues post to a rounding account),
//! disputes cannot break it (adjustments are new balanced entries),
//! and settlement cannot break it (a payment moves a liability to cash).

pub mod account;
pub mod dispute;
pub mod invoice;
pub mod journal;
pub mod settlement;

pub use account::{Account, AccountId, AccountKind};
pub use dispute::{Claim, Dispute, DisputeError, DisputeState, Evidence};
pub use invoice::{
    FlatRateTax, Invoice, InvoiceBuilder, InvoiceLine, InvoiceStatus, NoTax, TaxLine, TaxPolicy,
};
pub use journal::{EntryRef, JournalEntry, Ledger, LedgerError, Posting};
pub use settlement::{
    InternalRail, NetTransfer, Obligation, Rail, SettlementError, SettlementInstruction,
    SettlementRail, SettlementState, net,
};
