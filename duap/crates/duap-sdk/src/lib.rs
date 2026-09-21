//! # duap-sdk
//!
//! The DUAP Rust SDK.
//!
//! STATUS: PROTOTYPE. Implemented and exercised end to end by
//! `duap-demo`, but it has no tests of its own, and VS-5 (nothing
//! prevents a constant pseudonym salt) is open against the derivation it
//! exposes. That finding blocks any release of this crate.
//!
//! # Design rule: expose primitives, not conveniences
//!
//! The API is deliberately eight operations wide. Every one corresponds to a
//! protocol primitive; none wraps several primitives into a
//! "do-the-right-thing" call. That is not minimalism for its own sake: an
//! SDK that hides which primitive is being exercised makes it impossible for
//! an integrator to reason about what they are signing, and makes a second
//! implementation of the protocol harder, not easier -- because the *SDK*
//! becomes the specification.
//!
//! | operation | who calls it | what it produces |
//! |---|---|---|
//! | [`SubjectAgent::authorize`] | a data subject's agent | a signed grant |
//! | [`SubjectAgent::revoke_authorization`] | a data subject's agent | a signed revocation |
//! | [`ControllerAgent::record_usage`] | an instrumented system | a signed event |
//! | [`verify_receipt`] | anyone | a checked receipt, with its claims |
//! | [`query_provenance`] | anyone with the graph | attribution shares |
//! | [`calculate_charge`] | either party | a reproducible price breakdown |
//! | [`create_invoice`] | a clearing node | an invoice plus its journal entry |
//! | [`generate_receipt`] | a clearing node | a signed, anchorable receipt |
//!
//! ```
//! use duap_sdk::prelude::*;
//! # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//! let controller: OrgId = "org:duap/acme".parse()?;
//! let root = SubjectRoot::from_secret([1u8; 32]);
//! let subject_key = root.key_for(&controller, SuiteId::Ed25519);
//!
//! let mut agent = SubjectAgent::new(root, subject_key);
//! let grant = agent.authorize(
//!     &controller,
//!     GrantId([7u8; 16]),
//!     vec![Term::permit(1, Matcher::any()).with_pricing(PricingRule::Free)],
//!     Timestamp::from_secs(1_750_000_000),
//!     Currency::EUR,
//! )?;
//! assert_eq!(grant.0.epoch, 1);
//! # Ok(()) }
//! ```

pub mod controller;
pub mod subject;

pub use controller::{ControllerAgent, SdkError, UsageRecord};
pub use subject::{GrantPolicy, SubjectAgent};

use duap_canon::digest::HashAlg;
use duap_crypto::{Envelope, KeyRegistry, SuitePolicy, VerificationContext};
use duap_ledger::{Invoice, InvoiceBuilder, JournalEntry, TaxPolicy};
use duap_meter::{UsageCounter, UsageKey};
use duap_model::prelude::*;
use duap_provenance::{DerivationPolicy, ProvenanceGraph};
use duap_receipt::{Coverage, DecisionSummary, Receipt, ReceiptBuilder, VerifiedReceipt};
use duap_valuation::{PriceBreakdown, PriceEngine, PricingInputs};
use std::collections::BTreeMap;

/// Verify a sealed receipt against a key registry.
///
/// Returns the receipt together with the list of claims it does and does not
/// support, so a caller cannot use the result without being handed the
/// limits alongside it.
pub fn verify_receipt(
    envelope: &Envelope,
    registry: &KeyRegistry,
    policy: &SuitePolicy,
    now: Timestamp,
) -> Result<(VerifiedReceipt, Vec<duap_receipt::Claim>), SdkError> {
    let v = Receipt::verify(
        envelope,
        registry,
        policy,
        &VerificationContext::archival(now.0),
    )?;
    let claims = v.receipt.claims();
    Ok((v, claims))
}

/// Attribution shares for a derived object.
///
/// The shares are exact rationals. The second element is the share that
/// terminated (severed edges, negligible contributions, policy cut-offs) and
/// therefore belongs to nobody; ignoring it is how attribution silently
/// stops summing to one.
pub fn query_provenance(
    graph: &ProvenanceGraph,
    object: &ContentId,
    policy: &DerivationPolicy,
) -> Result<(BTreeMap<SubjectRef, Ratio>, Ratio), SdkError> {
    Ok(graph.attribution(object, policy)?)
}

/// Price a metered quantity.
pub fn calculate_charge(
    engine: &PriceEngine,
    key: &UsageKey,
    counter: &UsageCounter,
    rule: &PricingRule,
    inputs: &PricingInputs,
) -> Result<PriceBreakdown, SdkError> {
    Ok(engine.price(key, counter, rule, inputs)?)
}

/// Assemble a single-line invoice and its journal entry.
///
/// Multi-line invoicing goes through [`duap_ledger::InvoiceBuilder`]
/// directly; this helper exists for the common case and to make the
/// accounting consequence visible at the call site.
#[allow(clippy::too_many_arguments)]
pub fn create_invoice<T: TaxPolicy>(
    id: InvoiceId,
    issuer: OrgId,
    payer: OrgId,
    period: TimeRange,
    currency: Currency,
    lines: Vec<(
        UsageKey,
        duap_canon::Digest,
        PriceBreakdown,
        String,
        Option<Ratio>,
    )>,
    tax: &T,
    issued_at: Timestamp,
) -> Result<(Invoice, JournalEntry), SdkError> {
    let mut b = InvoiceBuilder::new(id, issuer, payer, period, currency);
    for (key, root, breakdown, description, share) in lines {
        b.line(key, root, breakdown, description, share)?;
    }
    let inv = b.build(tax, issued_at, issued_at.saturating_add(30 * DAY))?;
    inv.check_arithmetic()?;
    let entry = inv.to_journal_entry(issued_at)?;
    Ok((inv, entry))
}

/// Build and sign a receipt over a set of events.
///
/// The caller supplies the event digests in acceptance order; the SDK
/// computes the coverage root so that two implementations cannot disagree
/// about it.
#[allow(clippy::too_many_arguments)]
pub fn generate_receipt(
    issuer: OrgId,
    controller: OrgId,
    subject: SubjectScope,
    period: TimeRange,
    data_class: DataClass,
    operation: Operation,
    purpose: Purpose,
    country: &str,
    quantity: Quantity,
    event_digests: &[duap_canon::Digest],
    authorization: AuthorizationRef,
    decision: DecisionSummary,
    charge: Money,
    price_digest: duap_canon::Digest,
    signer: &duap_crypto::SecretKey,
    at: Timestamp,
) -> Result<(Receipt, Envelope), SdkError> {
    let coverage = Coverage {
        data_class,
        operation,
        purpose,
        country: country.to_owned(),
        quantity,
        event_count: event_digests.len() as u64,
        events_root: duap_receipt::events_root(HashAlg::Sha2_256, event_digests),
    };
    let receipt = ReceiptBuilder::new(
        issuer,
        controller,
        subject,
        period,
        coverage,
        authorization,
        decision,
        charge,
        price_digest,
        at,
    )
    .build()?;
    let mut env = receipt.seal()?;
    env.sign(signer, at.0, None)?;
    Ok((receipt, env))
}

/// Everything an integration normally needs.
pub mod prelude {
    pub use crate::{
        ControllerAgent, GrantPolicy, SdkError, SubjectAgent, UsageRecord, calculate_charge,
        create_invoice, generate_receipt, query_provenance, verify_receipt,
    };
    pub use duap_auth::prelude::*;
    pub use duap_crypto::{
        Envelope, KeyRecord, KeyRegistry, KeyRole, PublicKey, SecretKey, SuiteId, SuitePolicy,
    };
    pub use duap_model::prelude::*;
    pub use duap_provenance::{DerivationPolicy, ProvenanceGraph};
    pub use duap_receipt::{Claim, Receipt, VerifiedReceipt};
}
