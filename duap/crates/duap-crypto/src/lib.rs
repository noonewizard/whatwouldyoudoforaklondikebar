//! # duap-crypto
//!
//! Cryptographic suites, self-certifying key identity, signed envelopes, and
//! the key registry for the Data Usage Accounting Protocol.
//!
//! STATUS: PRODUCTION for Ed25519; PRODUCTION-CANDIDATE for ML-DSA and the
//! hybrid suite. No part of this crate has been independently audited. See
//! `SECURITY.md` for the full statement of what is and is not claimed.
//!
//! ```
//! use duap_crypto::{SecretKey, SuiteId, Envelope, KeyRegistry, KeyRecord, KeyRole,
//!                   SuitePolicy, VerificationContext};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let key = SecretKey::from_seed(SuiteId::Ed25519, [42u8; 32]);
//! let mut reg = KeyRegistry::new();
//! reg.enroll(KeyRecord::new(key.public_key(), "org:acme", vec![KeyRole::EventSigner], 0, None))?;
//!
//! let mut env = Envelope::seal("duap.test.v1", &("hello", 7u64))?;
//! env.sign(&key, 1_000_000, None)?;
//!
//! let ok = env.verify(&reg, &SuitePolicy::draft_default(), &VerificationContext::archival(2_000_000))?;
//! assert_eq!(ok, vec![key.key_id()]);
//! # Ok(()) }
//! ```

pub mod bytes_serde;
pub mod envelope;
pub mod error;
pub mod key;
pub mod registry;
pub mod signer;
pub mod suite;

pub use envelope::{Envelope, SIG_CONTEXT, SigStructure, Signature};
pub use error::{CryptoError, Result};
pub use key::{KeyId, PublicKey, SecretKey};
pub use registry::{
    KeyRecord, KeyRegistry, KeyRole, KeyStatus, Revocation, RevocationReason, VerificationContext,
};
pub use signer::{Signer, SoftwareSigner};
pub use suite::{SuiteId, SuitePolicy};

/// Current microsecond wall-clock time, as used for `created` fields.
pub fn now_micros() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_micros() as u64)
        .unwrap_or(0)
}
