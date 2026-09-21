//! # duap-auth
//!
//! The DUAP Usage Authorization Language (UAL): grants, matchers,
//! obligations, deterministic evaluation, revocation, and machine-to-machine
//! negotiation.
//!
//! STATUS: PRODUCTION (reference implementation).
//!
//! ```
//! use duap_auth::prelude::*;
//! use duap_model::prelude::*;
//! # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//! let controller: OrgId = "org:duap/acme".parse()?;
//! let subject = SubjectRef([1u8; 16]);
//! let key = duap_crypto::SecretKey::from_seed(duap_crypto::SuiteId::Ed25519, [2u8; 32]);
//!
//! let grant = GrantBuilder::new(
//!         GrantId([3u8; 16]), subject, key.key_id(), controller,
//!         Timestamp::from_secs(1_700_000_000), Currency::EUR)
//!     .term(Term::permit(1, Matcher::any()
//!             .classes(ClassSelector::Namespace { namespaces: vec!["location".into()] })
//!             .purposes(PurposeSelector::Under { roots: vec![Purpose::ServiceCore] }))
//!         .with_pricing(PricingRule::Free))
//!     .term(Term::deny(2, Matcher::any()
//!             .purposes(PurposeSelector::Commercial { value: true })))
//!     .build()?;
//!
//! // A commercial use of location data is denied by term 2, whatever term 1 says.
//! assert_eq!(grant.terms.len(), 2);
//! # Ok(()) }
//! ```

pub mod evaluate;
pub mod grant;
pub mod matcher;
pub mod negotiation;
pub mod obligation;

pub use evaluate::{AuthorizationStore, Decision, DecisionReason, EvalContext, evaluate};
pub use grant::{Effect, GRANT_DOMAIN, Grant, GrantBuilder, RevocationPolicy, Term};
pub use matcher::{ClassSelector, Matcher, PurposeSelector, Selector};
pub use obligation::{DerivationContext, Obligation, ObligationStatus};
pub use revocation_reexport::*;

mod revocation_reexport {
    pub use crate::revocation::{
        REVOCATION_DOMAIN, RetroactiveRequest, Revocation, RevocationScope,
    };
}

pub mod revocation;

/// Common imports for callers.
pub mod prelude {
    pub use crate::evaluate::{AuthorizationStore, Decision, DecisionReason, EvalContext, evaluate};
    pub use crate::grant::{Effect, Grant, GrantBuilder, RevocationPolicy, Term};
    pub use crate::matcher::{ClassSelector, Matcher, PurposeSelector, Selector};
    pub use crate::obligation::{DerivationContext, Obligation};
    pub use crate::revocation::{RetroactiveRequest, Revocation, RevocationScope};
}
