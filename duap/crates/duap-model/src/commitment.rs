//! Hash commitments to raw values.
//!
//! STATUS: PRODUCTION.
//!
//! DUAP never transports the data it accounts for. When a dispute requires
//! proving *what* a record contained, the controller opens a commitment it
//! published at collection time.
//!
//! The scheme is `C = H(domain, salt || value)` with a fresh 256-bit salt.
//!
//! * **Binding:** finding `(salt', value') != (salt, value)` with the same
//!   commitment is a collision on SHA-256. The length prefix on `salt` makes
//!   the concatenation injective, so no `(salt, value)` pair can be re-split.
//! * **Hiding:** the salt is uniform and 256 bits, so the commitment is
//!   indistinguishable from random to anyone without it, *including* for
//!   low-entropy values like a boolean or a postcode. Committing without a
//!   salt -- which several real consent-logging systems do -- lets anyone
//!   recover a small-domain value by brute force.
//!
//! The salt is held by the committer. Losing it makes the commitment
//! unopenable, which is a durability requirement on the controller, not a
//! protocol weakness: an unopenable commitment simply means the controller
//! cannot win that part of a dispute.

use duap_canon::digest::{Digest, HashAlg};
use serde::{Deserialize, Serialize};

/// Domain label for value commitments.
pub const COMMITMENT_DOMAIN: &str = "duap.commitment.v1";

/// A published commitment to a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Commitment(pub Digest);

/// The secret needed to open a commitment.
#[derive(Clone)]
pub struct Opening {
    pub salt: [u8; 32],
}

impl std::fmt::Debug for Opening {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Opening(<redacted>)")
    }
}

impl Commitment {
    /// Commit to `value` with an explicit salt.
    pub fn with_salt(value: &[u8], salt: [u8; 32]) -> Commitment {
        let mut input = Vec::with_capacity(40 + value.len());
        input.extend_from_slice(&(salt.len() as u64).to_be_bytes());
        input.extend_from_slice(&salt);
        input.extend_from_slice(value);
        Commitment(Digest::of(HashAlg::Sha2_256, COMMITMENT_DOMAIN, &input))
    }

    /// Commit to `value` with a fresh random salt.
    pub fn commit(value: &[u8]) -> crate::error::Result<(Commitment, Opening)> {
        let mut salt = [0u8; 32];
        getrandom::fill(&mut salt).map_err(|e| crate::error::ModelError::Entropy(e.to_string()))?;
        Ok((Commitment::with_salt(value, salt), Opening { salt }))
    }

    /// Check an opening.
    pub fn verify(&self, value: &[u8], opening: &Opening) -> bool {
        // Digest comparison is not secret-dependent here (the commitment is
        // public), so a non-constant-time compare is fine.
        Commitment::with_salt(value, opening.salt) == *self
    }
}
