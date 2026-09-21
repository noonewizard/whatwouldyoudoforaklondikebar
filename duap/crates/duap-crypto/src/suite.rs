//! Cryptographic suite registry.
//!
//! STATUS: PRODUCTION for `ed25519`; PRODUCTION-CANDIDATE for the ML-DSA and
//! hybrid suites (the underlying `fips204` implementation has not, to our
//! knowledge, been independently audited -- see `SECURITY.md`).
//!
//! DUAP never hard-codes a primitive. Every key, signature and envelope names
//! its suite, verifiers dispatch on that name, and policy decides which suites
//! are acceptable in which role. Adding a suite is a registry entry plus an
//! implementation arm; it is not a wire-format change.

use crate::error::{CryptoError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A signature suite in the DUAP registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum SuiteId {
    /// Ed25519 (RFC 8032), verified with the strict (cofactorless,
    /// non-malleable) equation. Mandatory to implement.
    Ed25519,
    /// ML-DSA-44 (FIPS 204). Post-quantum, category 2.
    MlDsa44,
    /// ML-DSA-65 (FIPS 204). Post-quantum, category 3.
    MlDsa65,
    /// Concatenated hybrid: an Ed25519 signature *and* an ML-DSA-44 signature
    /// over the same input, both of which must verify.
    ///
    /// Security argument: an existential forgery of the hybrid requires
    /// forging both component signatures, so the hybrid is unforgeable if
    /// *either* component is unforgeable. This protects against (a) a future
    /// quantum adversary breaking Ed25519 and (b) a cryptanalytic or
    /// implementation break of ML-DSA, at the cost of 2452 signature bytes.
    Ed25519MlDsa44,
}

impl SuiteId {
    pub const fn label(self) -> &'static str {
        match self {
            SuiteId::Ed25519 => "ed25519",
            SuiteId::MlDsa44 => "ml-dsa-44",
            SuiteId::MlDsa65 => "ml-dsa-65",
            SuiteId::Ed25519MlDsa44 => "ed25519+ml-dsa-44",
        }
    }

    /// Encoded public-key length in bytes.
    pub const fn public_key_len(self) -> usize {
        match self {
            SuiteId::Ed25519 => 32,
            SuiteId::MlDsa44 => 1312,
            SuiteId::MlDsa65 => 1952,
            SuiteId::Ed25519MlDsa44 => 32 + 1312,
        }
    }

    /// Encoded signature length in bytes.
    pub const fn signature_len(self) -> usize {
        match self {
            SuiteId::Ed25519 => 64,
            SuiteId::MlDsa44 => 2420,
            SuiteId::MlDsa65 => 3309,
            SuiteId::Ed25519MlDsa44 => 64 + 2420,
        }
    }

    /// Whether the suite is believed to resist a cryptanalytically relevant
    /// quantum computer. This reflects the *design intent* of the underlying
    /// primitive, not a proof.
    pub const fn quantum_resistant(self) -> bool {
        match self {
            SuiteId::Ed25519 => false,
            SuiteId::MlDsa44 | SuiteId::MlDsa65 | SuiteId::Ed25519MlDsa44 => true,
        }
    }

    /// All registered suites, in registry order.
    pub const ALL: [SuiteId; 4] = [
        SuiteId::Ed25519,
        SuiteId::MlDsa44,
        SuiteId::MlDsa65,
        SuiteId::Ed25519MlDsa44,
    ];
}

impl FromStr for SuiteId {
    type Err = CryptoError;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "ed25519" => Ok(SuiteId::Ed25519),
            "ml-dsa-44" => Ok(SuiteId::MlDsa44),
            "ml-dsa-65" => Ok(SuiteId::MlDsa65),
            "ed25519+ml-dsa-44" => Ok(SuiteId::Ed25519MlDsa44),
            other => Err(CryptoError::UnknownSuite(other.to_owned())),
        }
    }
}

impl TryFrom<String> for SuiteId {
    type Error = CryptoError;
    fn try_from(s: String) -> Result<Self> {
        s.parse()
    }
}

impl From<SuiteId> for String {
    fn from(s: SuiteId) -> String {
        s.label().to_owned()
    }
}

impl fmt::Display for SuiteId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

/// Policy describing which suites a verifier will accept in a given role.
///
/// STATUS: PRODUCTION. Used by the clearing node to run a migration: during a
/// transition the node *accepts* classical signatures but *requires* hybrid
/// ones from newly enrolled organisations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuitePolicy {
    /// Suites accepted when verifying.
    pub accept: Vec<SuiteId>,
    /// Suites acceptable when a party enrols a new key.
    pub enroll: Vec<SuiteId>,
}

impl SuitePolicy {
    /// The policy for the current draft: accept everything registered, enrol
    /// classical or hybrid.
    pub fn draft_default() -> Self {
        SuitePolicy {
            accept: SuiteId::ALL.to_vec(),
            enroll: vec![SuiteId::Ed25519, SuiteId::Ed25519MlDsa44],
        }
    }

    /// A post-quantum-only policy, for deployments that have completed
    /// migration.
    pub fn pq_only() -> Self {
        SuitePolicy {
            accept: vec![SuiteId::MlDsa44, SuiteId::MlDsa65, SuiteId::Ed25519MlDsa44],
            enroll: vec![SuiteId::Ed25519MlDsa44, SuiteId::MlDsa65],
        }
    }

    pub fn accepts(&self, s: SuiteId) -> bool {
        self.accept.contains(&s)
    }

    pub fn allows_enrollment(&self, s: SuiteId) -> bool {
        self.enroll.contains(&s)
    }
}
