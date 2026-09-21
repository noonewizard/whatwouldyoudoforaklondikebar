//! Subject pseudonyms.
//!
//! STATUS: PRODUCTION (derivation); the unlinkable-payout extension is
//! EXPERIMENTAL and lives in `research/` -- see the limitations below.
//!
//! # The problem
//!
//! An accounting system needs a stable handle for "the person this event is
//! about", because otherwise usage cannot be attributed and compensation
//! cannot be routed. A globally stable handle is a tracking identifier: it
//! would let every participating organisation join its records with every
//! other's, which is precisely the harm the protocol exists to price.
//!
//! # The construction
//!
//! A subject holds a 256-bit root secret. For each controller, the subject's
//! agent derives
//!
//! ```text
//! SubjectRef(controller) = truncate_128( H( "duap.pseudonym.v1" , root || controller_id ) )
//! ```
//!
//! and a per-controller signing key from the same root. The controller sees
//! only the derived value.
//!
//! # What this does and does not give you
//!
//! * **Holds:** two controllers who do not collude and who see only
//!   `SubjectRef` values cannot tell whether two references belong to the same
//!   person. The values are outputs of a hash on inputs that differ in the
//!   controller identifier; distinguishing them requires the root secret.
//! * **Holds:** the subject can recompute all their pseudonyms offline from
//!   the root secret, which is what makes a single dashboard possible without
//!   a central linkage table.
//! * **Does NOT hold:** unlinkability against *content*. If two controllers
//!   both hold a precise location trace, they can link the underlying person
//!   regardless of the identifier. The pseudonym removes the trivial join key;
//!   it does not make the data anonymous. `location.trajectory` carries a
//!   re-identification prior of 96 for exactly this reason.
//! * **Does NOT hold:** unlinkability against the clearing node at payout
//!   time. To pay one person for usage recorded under many pseudonyms, some
//!   party must learn that those pseudonyms belong together. In the reference
//!   design the subject's settlement agent learns it, and the clearing node
//!   learns a settlement-account identifier that groups them. A construction
//!   that avoids this (blind signatures over per-pseudonym payout tokens, or a
//!   cryptographic accumulator) is prototyped in
//!   `research/unlinkable-payout/` and is NOT part of the production path.
//!   `PRIVACY.md` states the residual exposure precisely.
//! * **Does NOT hold:** protection against a controller who already knows the
//!   subject's legal identity. A logged-in service knows who you are; the
//!   pseudonym only stops it sharing a join key with others.

use crate::error::Result;
use crate::ids::{OrgId, SubjectRef};
use duap_canon::digest::{Digest, HashAlg};
use duap_crypto::{SecretKey, SuiteId};

/// Domain label for pseudonym derivation.
pub const PSEUDONYM_DOMAIN: &str = "duap.pseudonym.v1";
/// Domain label for per-controller subject key derivation.
pub const SUBJECT_KEY_DOMAIN: &str = "duap.subject-key.v1";

/// A data subject's root secret. Never leaves the subject's own device or
/// agent.
pub struct SubjectRoot {
    secret: [u8; 32],
}

impl SubjectRoot {
    pub fn from_secret(secret: [u8; 32]) -> SubjectRoot {
        SubjectRoot { secret }
    }

    pub fn generate() -> Result<SubjectRoot> {
        let mut s = [0u8; 32];
        getrandom::fill(&mut s)
            .map_err(|e| crate::error::ModelError::Entropy(e.to_string()))?;
        Ok(SubjectRoot { secret: s })
    }

    fn mix(&self, domain: &str, org: &OrgId) -> Digest {
        let mut input = Vec::with_capacity(32 + 1 + org.as_str().len());
        input.extend_from_slice(&self.secret);
        input.push(0);
        input.extend_from_slice(org.as_str().as_bytes());
        Digest::of(HashAlg::Sha2_256, domain, &input)
    }

    /// The pseudonym this subject presents to `org`.
    pub fn pseudonym_for(&self, org: &OrgId) -> SubjectRef {
        SubjectRef::from_digest(&self.mix(PSEUDONYM_DOMAIN, org))
    }

    /// The signing key this subject uses when dealing with `org`.
    ///
    /// Distinct from the pseudonym so that publishing a public key does not
    /// reveal the pseudonym and vice versa.
    pub fn key_for(&self, org: &OrgId, suite: SuiteId) -> SecretKey {
        SecretKey::from_seed(suite, self.mix(SUBJECT_KEY_DOMAIN, org).bytes)
    }

    /// Best-effort scrub.
    pub fn zeroize(&mut self) {
        for b in self.secret.iter_mut() {
            unsafe { std::ptr::write_volatile(b, 0) };
        }
    }
}

impl Drop for SubjectRoot {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl std::fmt::Debug for SubjectRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SubjectRoot(<redacted>)")
    }
}
