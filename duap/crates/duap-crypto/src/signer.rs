//! The signing abstraction.
//!
//! STATUS: PRODUCTION for the software signer; the HSM/KMS/enclave signer is
//! UNIMPLEMENTED in this repository (see `docs/adr/0004-key-custody.md`) --
//! the trait exists so that a deployment can supply one without touching the
//! protocol code, and `duap-clearing` is written against the trait, not
//! against [`SecretKey`].

use crate::error::Result;
use crate::key::{KeyId, PublicKey, SecretKey};
use crate::suite::SuiteId;

/// Anything that can produce DUAP signatures for a key it controls.
pub trait Signer: Send + Sync {
    fn suite(&self) -> SuiteId;
    fn key_id(&self) -> KeyId;
    fn public_key(&self) -> PublicKey;
    /// Sign the already-assembled signing input under the DUAP context.
    fn sign_input(&self, input: &[u8]) -> Result<Vec<u8>>;
}

/// Software signer holding the seed in process memory.
pub struct SoftwareSigner {
    key: SecretKey,
}

impl SoftwareSigner {
    pub fn new(key: SecretKey) -> Self {
        SoftwareSigner { key }
    }

    pub fn generate(suite: SuiteId) -> Result<Self> {
        Ok(SoftwareSigner {
            key: SecretKey::generate(suite)?,
        })
    }

    pub fn secret(&self) -> &SecretKey {
        &self.key
    }
}

impl Signer for SoftwareSigner {
    fn suite(&self) -> SuiteId {
        self.key.suite
    }
    fn key_id(&self) -> KeyId {
        self.key.key_id()
    }
    fn public_key(&self) -> PublicKey {
        self.key.public_key()
    }
    fn sign_input(&self, input: &[u8]) -> Result<Vec<u8>> {
        self.key.sign(input, crate::envelope::SIG_CONTEXT)
    }
}
