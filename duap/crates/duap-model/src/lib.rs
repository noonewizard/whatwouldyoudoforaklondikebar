//! # duap-model
//!
//! The canonical object model of the Data Usage Accounting Protocol: data
//! usage events, typed identifiers, pseudonyms, exact money, the taxonomy, and
//! commitments.
//!
//! STATUS: REFERENCE. Tests cover the specified behaviour and the event
//! and taxonomy conformance vectors exist. Not PRODUCTION: no independent
//! review, and VS-5 is open in `security/findings.md`.
//!
//! The central object is [`event::DataUsageEvent`]. Everything else in the
//! protocol either produces one, authorises one, prices one, or proves
//! something about one.
//!
//! ```
//! use duap_model::prelude::*;
//! # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//! let controller: OrgId = "org:duap/acme".parse()?;
//! let root = SubjectRoot::from_secret([7u8; 32]);
//!
//! let ev = EventBuilder::new(
//!     EventId::random()?,
//!     AgentRef::new("duap-sdk-rust", "0.1.0"),
//!     controller.clone(),
//!     SubjectScope::Subject { subject: root.pseudonym_for(&controller) },
//!     Jurisdiction::new("DE")?.with_regimes([Regime::EuGdpr]),
//!     DataClass::LocationCoarse,
//!     Operation::ProcessProfile,
//!     Purpose::MarketingAdvertisingBehavioral,
//!     AuthorizationRef {
//!         grant: GrantId::random()?,
//!         grant_digest: duap_canon::Digest::of(duap_canon::HashAlg::Sha2_256, "x", b"y"),
//!         epoch: 1,
//!     },
//!     1,
//!     Timestamp::from_secs(1_750_000_000),
//! )
//! .provenance(Provenance { output: Some(ContentId::of_bytes("duap.object.v1", b"profile")), ..Default::default() })
//! .build()?;
//!
//! assert_eq!(ev.quantity.unit, Unit::Inference);
//! assert!(ev.validate().is_ok());
//! # Ok(()) }
//! ```

pub mod commitment;
pub mod error;
pub mod event;
pub mod ids;
pub mod jurisdiction;
pub mod money;
pub mod pricing;
pub mod pseudonym;
pub mod taxonomy;
pub mod time;

pub use error::{ModelError, Result, TaxonomyError};

/// Everything a typical integration needs.
pub mod prelude {
    pub use crate::commitment::{Commitment, Opening};
    pub use crate::error::ModelError;
    pub use crate::event::{
        AuthorizationRef, DataUsageEvent, EVENT_DOMAIN, EconomicContext, EventBuilder,
        EventSequence, Provenance, Quantity, RetentionBasis, RetentionPolicy, SubjectScope,
    };
    pub use crate::ids::{
        AgentRef, BatchId, ContentId, DisputeId, EventId, GrantId, InvoiceId, OrgId, SettlementId,
        SubjectRef,
    };
    pub use crate::jurisdiction::Jurisdiction;
    pub use crate::money::{Currency, Money, NANO, Precise, Ratio, Rounding};
    pub use crate::pricing::{PricingRule, Tier};
    pub use crate::pseudonym::SubjectRoot;
    pub use crate::taxonomy::{
        CollectionMethod, DataClass, LawfulBasis, ONTOLOGY_SHA256, ONTOLOGY_VERSION, Operation,
        OperationFamily, Purpose, Regime, SensitivityTier, Unit,
    };
    pub use crate::time::{DAY, HOUR, MINUTE, SECOND, TimeRange, Timestamp};
}
