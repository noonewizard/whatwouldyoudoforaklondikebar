//! # duap-clearing
//!
//! The DUAP clearing node: the component that receives events, decides
//! whether they were authorised, counts them, anchors them, prices them,
//! issues receipts, invoices the controller, posts the ledger, and accrues
//! what is owed to data subjects.
//!
//! STATUS: PROTOTYPE. Implemented and exercised end to end by
//! `duap-demo`, but it has no tests of its own and no conformance
//! vectors. Single process: horizontal
//! scaling, multi-region operation and durable replication are
//! UNIMPLEMENTED here and specified in `DEPLOYMENT.md` and `OPERATIONS.md`.
//!
//! The node is deliberately the *only* component that holds all the pieces.
//! Every other crate is usable on its own: an organisation can meter without
//! clearing, verify receipts without metering, or evaluate authorizations
//! without any of it. That is what makes independent implementation
//! possible, which is the test set out in `docs/architecture/minimal-protocol.md`.

pub mod node;
pub mod store;

pub use node::{
    ACK_DOMAIN, Acknowledgement, ClearingConfig, ClearingError, ClearingNode, IngestOutcome,
    NodeStats, PeriodResult, RejectAt,
};
pub use store::{EventStore, MemoryStore, StoreError, StoredEvent};
#[cfg(feature = "sqlite")]
pub use store::SqliteStore;
