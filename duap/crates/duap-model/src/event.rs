//! The canonical Data Usage Event.
//!
//! STATUS: PRODUCTION.
//!
//! A DUAP event is an accounting record, not a copy of the data. It says:
//! *this controller performed this operation on this class of data about this
//! pseudonymous subject, for this purpose, under this authorization, in this
//! quantity*. It never carries the data itself; where the content matters for
//! a later dispute, a salted [`Commitment`] stands in for it.
//!
//! # Wire field names
//!
//! Fields are encoded under two- to three-character names. At the protocol's
//! target volume the difference between `"data_class"` and `"dc"` is a
//! double-digit percentage of total storage and bandwidth; the mapping is
//! fixed, documented in `PROTOCOL.md`, and generated into every SDK, so
//! nothing depends on remembering it. See
//! `docs/adr/0007-compact-wire-names.md`.
//!
//! # Schema evolution
//!
//! * Within a major version, only *optional* fields may be added, and only
//!   with a new wire name. Receivers reject unknown fields
//!   (`deny_unknown_fields`) rather than ignoring them, because silently
//!   dropping a field that changes the meaning of an event -- a new
//!   `derivative_rights` restriction, say -- is an accounting error that no
//!   test would catch.
//! * Anything else is a new major version and a new domain label
//!   (`duap.event.v2`), which changes every digest, so signatures cannot cross
//!   versions.
//! * Relays that do not understand a version can still forward and verify
//!   signatures, because signatures cover exact bytes, not parsed structures.

use crate::commitment::Commitment;
use crate::error::{ModelError, Result};
use crate::ids::{AgentRef, ContentId, EventId, GrantId, OrgId, SubjectRef};
use crate::jurisdiction::Jurisdiction;
use crate::money::Money;
use crate::taxonomy::{
    CollectionMethod, DataClass, LawfulBasis, Operation, OperationFamily, Purpose,
    SensitivityTier, Unit,
};
use crate::time::Timestamp;
use duap_canon::Value;
use duap_canon::digest::Digest;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Domain label for event digests and signatures.
pub const EVENT_DOMAIN: &str = "duap.event.v1";

/// Current event schema version.
pub const EVENT_SCHEMA_VERSION: u16 = 1;

/// Who, or what group, an event is about.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", deny_unknown_fields)]
pub enum SubjectScope {
    /// A single subject, under the pseudonym they present to this controller.
    #[serde(rename = "s")]
    Subject {
        #[serde(rename = "r")]
        subject: SubjectRef,
    },
    /// A cohort of at least `size` subjects, named by an opaque cohort handle.
    ///
    /// Used for aggregate and differentially private operations where naming
    /// an individual would defeat the point. `size` is the asserted minimum
    /// cohort size; the clearing node treats a small `size` as a
    /// re-identification risk signal, not as a guarantee.
    #[serde(rename = "c")]
    Cohort {
        #[serde(rename = "h")]
        handle: ContentId,
        #[serde(rename = "n")]
        size: u64,
    },
    /// Not about a natural person at all (industrial telemetry, scientific
    /// measurement). Still metered, because rights may attach to it under
    /// non-privacy regimes such as the EU Data Act.
    #[serde(rename = "n")]
    NonPersonal,
}

impl SubjectScope {
    pub fn subject_ref(&self) -> Option<SubjectRef> {
        match self {
            SubjectScope::Subject { subject } => Some(*subject),
            _ => None,
        }
    }
}

/// A per-stream monotonic sequence number.
///
/// Gap detection is the only defence against an organisation that suppresses
/// inconvenient events: the clearing node cannot see what it was not sent, but
/// it can see that index 41 and 43 arrived and 42 did not. See
/// `THREAT_MODEL.md` T-20.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventSequence {
    /// Opaque stream identifier, stable for the life of an agent instance.
    #[serde(rename = "s")]
    pub stream: ContentId,
    /// Zero-based index within the stream.
    #[serde(rename = "i")]
    pub index: u64,
}

/// A metered quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Quantity {
    #[serde(rename = "u")]
    pub unit: Unit,
    #[serde(rename = "n")]
    pub amount: u64,
}

impl Quantity {
    pub fn new(unit: Unit, amount: u64) -> Quantity {
        Quantity { unit, amount }
    }

    /// The quantity in the unit the operation is naturally metered in.
    pub fn for_operation(op: Operation, amount: u64) -> Quantity {
        Quantity {
            unit: op.meter(),
            amount,
        }
    }
}

/// Reference to the authorization the operation relies on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationRef {
    /// The grant being relied on.
    #[serde(rename = "g")]
    pub grant: GrantId,
    /// Digest of the exact grant document, so that a later amendment cannot
    /// be passed off as the version that was relied on.
    #[serde(rename = "d")]
    pub grant_digest: Digest,
    /// Monotonic revision of the grant, incremented on every amendment.
    #[serde(rename = "e")]
    pub epoch: u32,
}

/// Where the data came from and what this operation produced.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    /// Content identifiers of the inputs consumed.
    #[serde(rename = "i", default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<ContentId>,
    /// Content identifier of the object produced, if any.
    #[serde(rename = "o", default, skip_serializing_if = "Option::is_none")]
    pub output: Option<ContentId>,
    /// Identifier of the transformation applied, e.g. a pipeline step name or
    /// the digest of the code that ran.
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<String>,
}

/// How long the controller says it will keep the data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetentionPolicy {
    #[serde(rename = "b")]
    pub basis: RetentionBasis,
    /// Retention period in days, where the basis is a fixed period.
    #[serde(rename = "d", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<u32>,
    /// Hard deletion deadline, where one is known.
    #[serde(rename = "u", default, skip_serializing_if = "Option::is_none")]
    pub until: Option<Timestamp>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionBasis {
    /// Deleted immediately after the operation.
    Transient,
    /// A fixed period from collection.
    FixedPeriod,
    /// Kept while the subject holds an account.
    AccountLifetime,
    /// Kept for a period mandated by law.
    LegalHold,
    /// No defined limit. Priced as an open-ended liability.
    Indefinite,
}

/// Commercial context, present when the operation has a commercial character.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EconomicContext {
    /// Identifier of the pricing schedule the parties agreed to apply.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// The other party to a transfer, sale or licence.
    #[serde(rename = "cp", default, skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<OrgId>,
    /// Revenue the controller attributes to this use, where it is willing to
    /// declare it. Enables revenue-share pricing; absent for most events.
    #[serde(rename = "rv", default, skip_serializing_if = "Option::is_none")]
    pub revenue: Option<Money>,
    /// Whether the counterparty obtained exclusive rights.
    #[serde(rename = "ex", default, skip_serializing_if = "is_false")]
    pub exclusive: bool,
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// The canonical DUAP data usage event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataUsageEvent {
    /// Schema version.
    #[serde(rename = "v")]
    pub schema: u16,
    #[serde(rename = "id")]
    pub id: EventId,
    /// When the operation happened.
    #[serde(rename = "ts")]
    pub occurred_at: Timestamp,
    /// When the agent recorded it. Never earlier than `occurred_at`.
    #[serde(rename = "rt")]
    pub recorded_at: Timestamp,
    #[serde(rename = "sq", default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<EventSequence>,
    #[serde(rename = "ag")]
    pub agent: AgentRef,
    /// The party accountable for the processing.
    #[serde(rename = "ct")]
    pub controller: OrgId,
    /// The party that actually performed it, where different.
    #[serde(rename = "pr", default, skip_serializing_if = "Option::is_none")]
    pub processor: Option<OrgId>,
    #[serde(rename = "sb")]
    pub subject: SubjectScope,
    #[serde(rename = "ju")]
    pub jurisdiction: Jurisdiction,
    #[serde(rename = "dc")]
    pub data_class: DataClass,
    /// Effective sensitivity. May exceed the class default (context can make
    /// ordinary data sensitive) but never fall below it.
    #[serde(rename = "sn")]
    pub sensitivity: SensitivityTier,
    #[serde(rename = "cm", default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<CollectionMethod>,
    #[serde(rename = "op")]
    pub operation: Operation,
    #[serde(rename = "pp")]
    pub purpose: Purpose,
    #[serde(rename = "lb", default, skip_serializing_if = "Option::is_none")]
    pub lawful_basis: Option<LawfulBasis>,
    #[serde(rename = "az")]
    pub authorization: AuthorizationRef,
    #[serde(rename = "qy")]
    pub quantity: Quantity,
    #[serde(rename = "rp", default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<RetentionPolicy>,
    #[serde(rename = "pv", default, skip_serializing_if = "provenance_is_empty")]
    pub provenance: Provenance,
    #[serde(rename = "ec", default, skip_serializing_if = "Option::is_none")]
    pub economics: Option<EconomicContext>,
    /// Salted commitment to the underlying value, for dispute resolution.
    #[serde(rename = "cx", default, skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Namespaced extension fields. Keys must contain a `.` and must not use
    /// the `duap.` namespace unless registered in the protocol registry.
    #[serde(rename = "xt", default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

fn provenance_is_empty(p: &Provenance) -> bool {
    p.inputs.is_empty() && p.output.is_none() && p.transform.is_none()
}

impl DataUsageEvent {
    /// Digest of this event under the event domain.
    pub fn digest(&self) -> Result<Digest> {
        Ok(Digest::of_object(EVENT_DOMAIN, self)?)
    }

    /// Canonical encoding.
    pub fn to_canonical(&self) -> Result<Vec<u8>> {
        Ok(duap_canon::to_canonical_cbor(self)?)
    }

    pub fn from_canonical(bytes: &[u8]) -> Result<DataUsageEvent> {
        Ok(duap_canon::from_canonical_cbor(bytes)?)
    }

    /// Structural validation.
    ///
    /// These are the invariants the clearing node enforces at ingest. They are
    /// deliberately mechanical: every rule here is checkable from the event
    /// alone, without external state, so an agent can enforce them before
    /// transmitting and a verifier can re-check them years later.
    pub fn validate(&self) -> Result<()> {
        let bad = |r: String| Err(ModelError::InvalidEvent(r));

        if self.schema != EVENT_SCHEMA_VERSION {
            return bad(format!(
                "schema version {} is not {EVENT_SCHEMA_VERSION}",
                self.schema
            ));
        }
        if self.recorded_at < self.occurred_at {
            return bad(format!(
                "recorded_at {} precedes occurred_at {}",
                self.recorded_at, self.occurred_at
            ));
        }
        if self.sensitivity.rank() < self.data_class.sensitivity().rank() {
            return bad(format!(
                "sensitivity {} is below the class default {} for {}",
                self.sensitivity.code(),
                self.data_class.sensitivity().code(),
                self.data_class.code()
            ));
        }
        if self.quantity.unit != self.operation.meter() {
            return bad(format!(
                "operation {} is metered in {}, not {}",
                self.operation.code(),
                self.operation.meter().code(),
                self.quantity.unit.code()
            ));
        }
        if self.quantity.amount == 0 {
            return bad("quantity must be positive".into());
        }
        if self.operation.derives() && self.provenance.output.is_none() {
            return bad(format!(
                "operation {} produces a derived object but no provenance output was recorded",
                self.operation.code()
            ));
        }
        if self.operation.family() == OperationFamily::Transfer
            && self.operation != Operation::TransferInternal
            && self
                .economics
                .as_ref()
                .and_then(|e| e.counterparty.as_ref())
                .is_none()
        {
            return bad(format!(
                "external transfer {} must name a counterparty",
                self.operation.code()
            ));
        }
        if let SubjectScope::Cohort { size, .. } = &self.subject {
            if *size < 2 {
                return bad("cohort size must be at least 2".into());
            }
        }
        if matches!(self.subject, SubjectScope::NonPersonal)
            && self.data_class.sensitivity().rank() > 1
        {
            return bad(format!(
                "class {} cannot be recorded as non-personal (tier {})",
                self.data_class.code(),
                self.data_class.sensitivity().code()
            ));
        }
        for k in self.extensions.keys() {
            if !k.contains('.') {
                return bad(format!("extension key {k:?} must be namespaced"));
            }
            if k.starts_with("duap.") {
                return bad(format!(
                    "extension key {k:?} uses the reserved duap. namespace"
                ));
            }
        }
        if let Some(rp) = &self.retention {
            if rp.basis == RetentionBasis::FixedPeriod && rp.days.is_none() {
                return bad("fixed-period retention must state a number of days".into());
            }
        }
        Ok(())
    }
}

/// Builder for events, filling defaults that are derivable.
///
/// The builder exists so that the SDK's `track()` call cannot produce an event
/// that fails `validate()` for a reason the SDK could have worked out itself
/// (the metering unit, the sensitivity floor, the record time).
pub struct EventBuilder {
    ev: DataUsageEvent,
}

impl EventBuilder {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: EventId,
        agent: AgentRef,
        controller: OrgId,
        subject: SubjectScope,
        jurisdiction: Jurisdiction,
        data_class: DataClass,
        operation: Operation,
        purpose: Purpose,
        authorization: AuthorizationRef,
        amount: u64,
        occurred_at: Timestamp,
    ) -> EventBuilder {
        EventBuilder {
            ev: DataUsageEvent {
                schema: EVENT_SCHEMA_VERSION,
                id,
                occurred_at,
                recorded_at: occurred_at,
                sequence: None,
                agent,
                controller,
                processor: None,
                subject,
                jurisdiction,
                data_class,
                sensitivity: data_class.sensitivity(),
                collection: None,
                operation,
                purpose,
                lawful_basis: None,
                authorization,
                quantity: Quantity::for_operation(operation, amount),
                retention: None,
                provenance: Provenance::default(),
                economics: None,
                commitment: None,
                extensions: BTreeMap::new(),
            },
        }
    }

    pub fn recorded_at(mut self, t: Timestamp) -> Self {
        self.ev.recorded_at = t;
        self
    }
    pub fn sequence(mut self, s: EventSequence) -> Self {
        self.ev.sequence = Some(s);
        self
    }
    pub fn processor(mut self, p: OrgId) -> Self {
        self.ev.processor = Some(p);
        self
    }
    pub fn sensitivity(mut self, s: SensitivityTier) -> Self {
        self.ev.sensitivity = s;
        self
    }
    pub fn collection(mut self, c: CollectionMethod) -> Self {
        self.ev.collection = Some(c);
        self
    }
    pub fn lawful_basis(mut self, b: LawfulBasis) -> Self {
        self.ev.lawful_basis = Some(b);
        self
    }
    pub fn retention(mut self, r: RetentionPolicy) -> Self {
        self.ev.retention = Some(r);
        self
    }
    pub fn provenance(mut self, p: Provenance) -> Self {
        self.ev.provenance = p;
        self
    }
    pub fn economics(mut self, e: EconomicContext) -> Self {
        self.ev.economics = Some(e);
        self
    }
    pub fn commitment(mut self, c: Commitment) -> Self {
        self.ev.commitment = Some(c);
        self
    }
    pub fn extension(mut self, k: impl Into<String>, v: Value) -> Self {
        self.ev.extensions.insert(k.into(), v);
        self
    }

    /// Finish, validating the result.
    pub fn build(self) -> Result<DataUsageEvent> {
        self.ev.validate()?;
        Ok(self.ev)
    }

    /// Finish without validating. For tests that need invalid events.
    pub fn build_unchecked(self) -> DataUsageEvent {
        self.ev
    }
}
