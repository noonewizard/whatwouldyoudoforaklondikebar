//! Typed identifiers.
//!
//! STATUS: PRODUCTION.
//!
//! Every identifier is typed and prefixed. Untyped opaque strings invite the
//! class of bug where an event identifier is accepted where an authorization
//! identifier was meant; the prefix also makes logs and disputes readable
//! without a schema to hand.
//!
//! Identifiers that name *content* (datasets, derived objects, receipts) are
//! content digests, not random values, so that two parties who hold the same
//! object agree on its name without coordinating.

use crate::error::{ModelError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Declare a 16-byte, prefixed, opaque identifier type.
macro_rules! opaque_id {
    ($name:ident, $prefix:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub [u8; 16]);

        impl $name {
            /// The textual prefix used in the rendered form.
            pub const PREFIX: &'static str = $prefix;

            /// Draw a fresh identifier from the operating system CSPRNG.
            pub fn random() -> Result<Self> {
                let mut b = [0u8; 16];
                getrandom::fill(&mut b).map_err(|e| ModelError::Entropy(e.to_string()))?;
                Ok($name(b))
            }

            /// Derive an identifier deterministically from a digest, taking
            /// the leading 128 bits.
            pub fn from_digest(d: &duap_canon::Digest) -> Self {
                let mut b = [0u8; 16];
                b.copy_from_slice(&d.bytes[..16]);
                $name(b)
            }

            pub fn to_hex(&self) -> String {
                hex::encode(self.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}:{}", $prefix, self.to_hex())
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self)
            }
        }

        impl FromStr for $name {
            type Err = ModelError;
            fn from_str(s: &str) -> Result<Self> {
                let rest = s.strip_prefix(concat!($prefix, ":")).ok_or_else(|| {
                    ModelError::BadId(format!("expected {}: prefix in {:?}", $prefix, s))
                })?;
                let raw = hex::decode(rest)
                    .map_err(|e| ModelError::BadId(format!("bad hex in {s:?}: {e}")))?;
                let arr: [u8; 16] = raw
                    .try_into()
                    .map_err(|_| ModelError::BadId(format!("{s:?} is not 16 bytes")))?;
                Ok($name(arr))
            }
        }

        impl Serialize for $name {
            fn serialize<S: serde::Serializer>(
                &self,
                s: S,
            ) -> std::result::Result<S::Ok, S::Error> {
                s.serialize_str(&self.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(
                d: D,
            ) -> std::result::Result<Self, D::Error> {
                let s = String::deserialize(d)?;
                s.parse().map_err(serde::de::Error::custom)
            }
        }
    };
}

opaque_id!(EventId, "evt1", "Identifier of a data usage event.");

impl EventId {
    /// Derive an event identifier from an agent's stream and sequence index.
    ///
    /// Deterministic derivation is preferred to a random identifier for
    /// three reasons: an embedded agent need not reach for entropy on the
    /// hot path; the identifier becomes recomputable by anyone holding the
    /// stream and index, so a claim that two events are the same is
    /// checkable; and a collision can only mean a reused sequence slot,
    /// which the meter already treats as a conflict. The event's canonical
    /// identity remains its digest -- this is a human-facing handle.
    pub fn for_sequence(stream: &ContentId, index: u64) -> EventId {
        let mut input = Vec::with_capacity(80);
        input.extend_from_slice(stream.to_string().as_bytes());
        input.push(0);
        input.extend_from_slice(&index.to_be_bytes());
        EventId::from_digest(&duap_canon::Digest::of(
            duap_canon::HashAlg::Sha2_256,
            "duap.event-id.v1",
            &input,
        ))
    }
}
opaque_id!(
    SubjectRef,
    "sub1",
    "Pseudonymous reference to a data subject, scoped to one controller.\n\nSee [`crate::pseudonym`] for how these are derived and what unlinkability\nproperty the derivation does and does not provide."
);
opaque_id!(GrantId, "gr1", "Identifier of an authorization grant.");
opaque_id!(BatchId, "bat1", "Identifier of an ingestion batch.");
opaque_id!(InvoiceId, "inv1", "Identifier of an invoice.");
opaque_id!(DisputeId, "dsp1", "Identifier of a dispute case.");
opaque_id!(
    SettlementId,
    "stl1",
    "Identifier of a settlement instruction."
);

/// Identifier of an organisation.
///
/// The syntax is `org:<authority>/<local>`, where `<authority>` names the
/// registry that vouches for the identifier (`duap` for the protocol's own
/// registry, `did` for a W3C DID, `lei` for a Legal Entity Identifier).
/// Binding an `OrgId` to a real-world legal person is the registry's job, not
/// the protocol's; DUAP only requires that the binding be *stable* and
/// *auditable*. See `docs/adr/0006-organisation-identity.md`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct OrgId(String);

impl OrgId {
    pub fn new(s: impl Into<String>) -> Result<OrgId> {
        let s = s.into();
        let body = s
            .strip_prefix("org:")
            .ok_or_else(|| ModelError::BadId(format!("expected org: prefix in {s:?}")))?;
        let (authority, local) = body.split_once('/').ok_or_else(|| {
            ModelError::BadId(format!("expected org:<authority>/<local> in {s:?}"))
        })?;
        if authority.is_empty() || local.is_empty() {
            return Err(ModelError::BadId(format!(
                "empty authority or local part in {s:?}"
            )));
        }
        if !authority
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        {
            return Err(ModelError::BadId(format!(
                "authority must be lowercase alphanumeric or '-' in {s:?}"
            )));
        }
        if local.len() > 128 || local.bytes().any(|b| b.is_ascii_control() || b == b' ') {
            return Err(ModelError::BadId(format!("invalid local part in {s:?}")));
        }
        Ok(OrgId(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn authority(&self) -> &str {
        let body = &self.0["org:".len()..];
        body.split_once('/').map(|(a, _)| a).unwrap_or(body)
    }
}

impl fmt::Display for OrgId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Debug for OrgId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for OrgId {
    type Err = ModelError;
    fn from_str(s: &str) -> Result<Self> {
        OrgId::new(s)
    }
}

impl TryFrom<String> for OrgId {
    type Error = ModelError;
    fn try_from(s: String) -> Result<Self> {
        OrgId::new(s)
    }
}

impl From<OrgId> for String {
    fn from(o: OrgId) -> String {
        o.0
    }
}

/// A content address: the digest of a canonical object.
///
/// Used to name datasets, derived objects, model artefacts and receipts. Two
/// parties holding the same bytes compute the same `ContentId` with no
/// coordination, which is what makes the provenance graph joinable across
/// organisational boundaries.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ContentId(pub duap_canon::Digest);

impl ContentId {
    pub fn of_bytes(domain: &str, bytes: &[u8]) -> ContentId {
        ContentId(duap_canon::Digest::of(
            duap_canon::HashAlg::Sha2_256,
            domain,
            bytes,
        ))
    }

    pub fn of_object<T: Serialize + ?Sized>(domain: &str, t: &T) -> Result<ContentId> {
        Ok(ContentId(duap_canon::Digest::of_object(domain, t)?))
    }

    pub fn digest(&self) -> duap_canon::Digest {
        self.0
    }
}

impl fmt::Display for ContentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for ContentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cid({})", self.0)
    }
}

impl FromStr for ContentId {
    type Err = ModelError;
    fn from_str(s: &str) -> Result<Self> {
        Ok(ContentId(s.parse::<duap_canon::Digest>()?))
    }
}

/// Reference to the piece of software that produced an event.
///
/// Recorded so that a systematic measurement error can be traced to a build,
/// and so that a compromised agent version can be quarantined by the clearing
/// node without disabling the whole organisation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AgentRef {
    /// Implementation name, e.g. `duap-sdk-rust`.
    pub name: String,
    /// Implementation version, e.g. `0.1.0`.
    pub version: String,
    /// Optional build provenance reference (an in-toto/SLSA attestation
    /// digest). `None` means the agent build is unattested.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<duap_canon::Digest>,
}

impl AgentRef {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> AgentRef {
        AgentRef {
            name: name.into(),
            version: version.into(),
            build: None,
        }
    }
}
