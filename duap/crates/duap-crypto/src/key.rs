//! Keys, key identifiers, and deterministic key derivation.
//!
//! STATUS: PRODUCTION.
//!
//! # Self-certifying key identifiers
//!
//! A [`KeyId`] is a truncated domain-separated hash of `(suite, public key)`.
//! Nothing but the key material determines it, so a registry entry cannot
//! silently bind an identifier to a different key: [`PublicKey::key_id`]
//! recomputes it and [`KeyRecord`](crate::registry::KeyRecord) checks it. This
//! removes a whole class of substitution attacks that afflict systems where
//! `kid` is an arbitrary opaque string chosen by the key's owner.
//!
//! Truncation is to 128 bits. For an identifier whose only security
//! requirement is collision resistance against an adversary who may choose
//! both keys, 128 bits gives a ~2^64 birthday bound. That is the same margin
//! as a UUID-sized identifier and is adequate here because a colliding pair
//! still has to pass signature verification under the *specific* registered
//! public key; the identifier is a lookup handle, not an authenticator.
//!
//! # Seeds
//!
//! A DUAP secret key is a 256-bit seed. Component private keys are derived
//! from it by domain-separated hashing:
//!
//! ```text
//! seed_ed25519 = H("DUAP/1" || 0 || "duap.kdf.ed25519.v1" || 0 || seed)
//! seed_mldsa   = H("DUAP/1" || 0 || "duap.kdf.ml-dsa.v1"  || 0 || seed)
//! ```
//!
//! This is a hash-based derivation from a *uniformly random 256-bit* input,
//! not a password KDF: it must never be fed low-entropy material. The benefit
//! is that backup, escrow, and HSM import handle exactly 32 bytes regardless
//! of suite -- ML-DSA-44 private keys are 2560 bytes, which is awkward to
//! move around, and re-deriving them is cheap.

use crate::error::{CryptoError, Result};
use crate::suite::SuiteId;
use duap_canon::digest::{Digest, HashAlg};
use ed25519_dalek as ed;
use fips204::traits::{KeyGen, SerDes, Signer as _, Verifier as _};
use fips204::{ml_dsa_44, ml_dsa_65};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Domain label for key identifier derivation.
pub const KEY_ID_DOMAIN: &str = "duap.key-id.v1";
/// Length of a key identifier in bytes.
pub const KEY_ID_LEN: usize = 16;

/// A self-certifying key identifier, rendered as `kid1:<32 hex chars>`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyId(pub [u8; KEY_ID_LEN]);

impl KeyId {
    /// Derive the identifier for a suite and encoded public key.
    pub fn derive(suite: SuiteId, public_key: &[u8]) -> KeyId {
        let mut input = Vec::with_capacity(suite.label().len() + 1 + public_key.len());
        input.extend_from_slice(suite.label().as_bytes());
        input.push(0);
        input.extend_from_slice(public_key);
        let d = Digest::of(HashAlg::Sha2_256, KEY_ID_DOMAIN, &input);
        let mut out = [0u8; KEY_ID_LEN];
        out.copy_from_slice(&d.bytes[..KEY_ID_LEN]);
        KeyId(out)
    }

    pub fn to_hex(self) -> String {
        hex::encode(self.0)
    }
}

impl fmt::Display for KeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "kid1:{}", self.to_hex())
    }
}

impl fmt::Debug for KeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl FromStr for KeyId {
    type Err = CryptoError;
    fn from_str(s: &str) -> Result<Self> {
        let hexs = s
            .strip_prefix("kid1:")
            .ok_or_else(|| CryptoError::MalformedKey(format!("missing kid1: prefix in {s:?}")))?;
        let raw = hex::decode(hexs)
            .map_err(|e| CryptoError::MalformedKey(format!("bad key id hex: {e}")))?;
        let arr: [u8; KEY_ID_LEN] = raw
            .try_into()
            .map_err(|_| CryptoError::MalformedKey("key id must be 16 bytes".into()))?;
        Ok(KeyId(arr))
    }
}

impl Serialize for KeyId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for KeyId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

/// A public key: a suite tag plus the suite's encoded key material.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicKey {
    pub suite: SuiteId,
    #[serde(with = "crate::bytes_serde")]
    pub bytes: Vec<u8>,
}

impl PublicKey {
    pub fn new(suite: SuiteId, bytes: Vec<u8>) -> Result<PublicKey> {
        if bytes.len() != suite.public_key_len() {
            return Err(CryptoError::KeyLength {
                suite: suite.label(),
                expected: suite.public_key_len(),
                actual: bytes.len(),
            });
        }
        Ok(PublicKey { suite, bytes })
    }

    pub fn key_id(&self) -> KeyId {
        KeyId::derive(self.suite, &self.bytes)
    }

    /// Verify `signature` over `message`.
    ///
    /// `ctx` is the suite-level context string; DUAP always passes
    /// [`crate::SIG_CONTEXT`].
    pub fn verify(&self, message: &[u8], signature: &[u8], ctx: &[u8]) -> Result<()> {
        if signature.len() != self.suite.signature_len() {
            return Err(CryptoError::SignatureLength {
                suite: self.suite.label(),
                expected: self.suite.signature_len(),
                actual: signature.len(),
            });
        }
        match self.suite {
            SuiteId::Ed25519 => verify_ed25519(&self.bytes, message, signature, ctx),
            SuiteId::MlDsa44 => verify_mldsa44(&self.bytes, message, signature, ctx),
            SuiteId::MlDsa65 => verify_mldsa65(&self.bytes, message, signature, ctx),
            SuiteId::Ed25519MlDsa44 => {
                // Both halves must verify. Verify the cheap one first so that
                // a malformed hybrid costs ~50us rather than ~150us.
                let (ed_pk, pq_pk) = self.bytes.split_at(32);
                let (ed_sig, pq_sig) = signature.split_at(64);
                verify_ed25519(ed_pk, message, ed_sig, ctx)?;
                verify_mldsa44(pq_pk, message, pq_sig, ctx)
            }
        }
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey({}, {})", self.suite, self.key_id())
    }
}

fn verify_ed25519(pk: &[u8], msg: &[u8], sig: &[u8], ctx: &[u8]) -> Result<()> {
    let pk: [u8; 32] = pk.try_into().map_err(|_| CryptoError::KeyLength {
        suite: "ed25519",
        expected: 32,
        actual: pk.len(),
    })?;
    let vk = ed::VerifyingKey::from_bytes(&pk)
        .map_err(|_| CryptoError::MalformedKey("ed25519 point is not on the curve".into()))?;
    let sig: [u8; 64] = sig.try_into().map_err(|_| CryptoError::SignatureLength {
        suite: "ed25519",
        expected: 64,
        actual: sig.len(),
    })?;
    // Ed25519 has no context parameter in RFC 8032 pure mode; DUAP binds the
    // context by prefixing the message instead (see `bind_ctx`).
    let msg = bind_ctx(ctx, msg);
    vk.verify_strict(&msg, &ed::Signature::from_bytes(&sig))
        .map_err(|_| CryptoError::BadSignature("ed25519 verification failed"))
}

fn verify_mldsa44(pk: &[u8], msg: &[u8], sig: &[u8], ctx: &[u8]) -> Result<()> {
    let pk: [u8; 1312] = pk.try_into().map_err(|_| CryptoError::KeyLength {
        suite: "ml-dsa-44",
        expected: 1312,
        actual: pk.len(),
    })?;
    let pk = ml_dsa_44::PublicKey::try_from_bytes(pk)
        .map_err(|e| CryptoError::MalformedKey(format!("ml-dsa-44 public key: {e}")))?;
    let sig: [u8; 2420] = sig.try_into().map_err(|_| CryptoError::SignatureLength {
        suite: "ml-dsa-44",
        expected: 2420,
        actual: sig.len(),
    })?;
    if pk.verify(msg, &sig, ctx) {
        Ok(())
    } else {
        Err(CryptoError::BadSignature("ml-dsa-44 verification failed"))
    }
}

fn verify_mldsa65(pk: &[u8], msg: &[u8], sig: &[u8], ctx: &[u8]) -> Result<()> {
    let pk: [u8; 1952] = pk.try_into().map_err(|_| CryptoError::KeyLength {
        suite: "ml-dsa-65",
        expected: 1952,
        actual: pk.len(),
    })?;
    let pk = ml_dsa_65::PublicKey::try_from_bytes(pk)
        .map_err(|e| CryptoError::MalformedKey(format!("ml-dsa-65 public key: {e}")))?;
    let sig: [u8; 3309] = sig.try_into().map_err(|_| CryptoError::SignatureLength {
        suite: "ml-dsa-65",
        expected: 3309,
        actual: sig.len(),
    })?;
    if pk.verify(msg, &sig, ctx) {
        Ok(())
    } else {
        Err(CryptoError::BadSignature("ml-dsa-65 verification failed"))
    }
}

/// Bind a context string to a message for suites with no native context
/// parameter. Length-prefixed, so `(ctx, msg)` pairs map injectively.
fn bind_ctx(ctx: &[u8], msg: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(9 + ctx.len() + msg.len());
    out.extend_from_slice(&(ctx.len() as u64).to_be_bytes());
    out.extend_from_slice(ctx);
    out.extend_from_slice(msg);
    out
}

/// A DUAP secret key: a 256-bit seed plus the suite it is used with.
///
/// The seed is zeroed on drop on a best-effort basis (`std` gives no guarantee
/// the compiler will not have copied it; deployments handling high-value keys
/// should use an HSM or enclave and the [`crate::signer::Signer`] trait).
#[derive(Clone)]
pub struct SecretKey {
    pub suite: SuiteId,
    seed: [u8; 32],
}

impl SecretKey {
    /// Construct from an explicit seed. The seed MUST be uniformly random.
    pub fn from_seed(suite: SuiteId, seed: [u8; 32]) -> SecretKey {
        SecretKey { suite, seed }
    }

    /// Generate a key from the operating system CSPRNG.
    pub fn generate(suite: SuiteId) -> Result<SecretKey> {
        let mut seed = [0u8; 32];
        getrandom::fill(&mut seed).map_err(|e| CryptoError::Entropy(e.to_string()))?;
        Ok(SecretKey { suite, seed })
    }

    /// The raw seed. Handle as secret material.
    pub fn seed(&self) -> &[u8; 32] {
        &self.seed
    }

    fn derive(&self, label: &str) -> [u8; 32] {
        Digest::of(HashAlg::Sha2_256, label, &self.seed).bytes
    }

    fn ed25519(&self) -> ed::SigningKey {
        ed::SigningKey::from_bytes(&self.derive("duap.kdf.ed25519.v1"))
    }

    /// The matching public key.
    pub fn public_key(&self) -> PublicKey {
        let bytes = match self.suite {
            SuiteId::Ed25519 => self.ed25519().verifying_key().to_bytes().to_vec(),
            SuiteId::MlDsa44 => {
                let (pk, _) = ml_dsa_44::KG::keygen_from_seed(&self.derive("duap.kdf.ml-dsa.v1"));
                pk.into_bytes().to_vec()
            }
            SuiteId::MlDsa65 => {
                let (pk, _) = ml_dsa_65::KG::keygen_from_seed(&self.derive("duap.kdf.ml-dsa.v1"));
                pk.into_bytes().to_vec()
            }
            SuiteId::Ed25519MlDsa44 => {
                let mut v = self.ed25519().verifying_key().to_bytes().to_vec();
                let (pk, _) = ml_dsa_44::KG::keygen_from_seed(&self.derive("duap.kdf.ml-dsa.v1"));
                v.extend_from_slice(&pk.into_bytes());
                v
            }
        };
        PublicKey {
            suite: self.suite,
            bytes,
        }
    }

    pub fn key_id(&self) -> KeyId {
        self.public_key().key_id()
    }

    /// Sign `message` under `ctx`, deterministically.
    ///
    /// Determinism is chosen so that test vectors and conformance suites are
    /// reproducible across implementations. The known trade-off is reduced
    /// resistance to fault-injection attacks on the signing device; a
    /// deployment that cares about that should implement
    /// [`crate::signer::Signer`] over an HSM performing hedged signing.
    pub fn sign(&self, message: &[u8], ctx: &[u8]) -> Result<Vec<u8>> {
        use ed25519_dalek::Signer as _;
        Ok(match self.suite {
            SuiteId::Ed25519 => {
                let m = bind_ctx(ctx, message);
                self.ed25519().sign(&m).to_bytes().to_vec()
            }
            SuiteId::MlDsa44 => {
                let (_, sk) = ml_dsa_44::KG::keygen_from_seed(&self.derive("duap.kdf.ml-dsa.v1"));
                sk.try_sign_with_seed(&self.message_seed(message), message, ctx)
                    .map_err(|_| CryptoError::SigningFailed("ml-dsa-44 signing failed"))?
                    .to_vec()
            }
            SuiteId::MlDsa65 => {
                let (_, sk) = ml_dsa_65::KG::keygen_from_seed(&self.derive("duap.kdf.ml-dsa.v1"));
                sk.try_sign_with_seed(&self.message_seed(message), message, ctx)
                    .map_err(|_| CryptoError::SigningFailed("ml-dsa-65 signing failed"))?
                    .to_vec()
            }
            SuiteId::Ed25519MlDsa44 => {
                let m = bind_ctx(ctx, message);
                let mut out = self.ed25519().sign(&m).to_bytes().to_vec();
                let (_, sk) = ml_dsa_44::KG::keygen_from_seed(&self.derive("duap.kdf.ml-dsa.v1"));
                let pq = sk
                    .try_sign_with_seed(&self.message_seed(message), message, ctx)
                    .map_err(|_| CryptoError::SigningFailed("ml-dsa-44 signing failed"))?;
                out.extend_from_slice(&pq);
                out
            }
        })
    }

    /// Per-message randomness for deterministic ML-DSA signing.
    fn message_seed(&self, message: &[u8]) -> [u8; 32] {
        let mut input = Vec::with_capacity(64 + message.len());
        input.extend_from_slice(&self.seed);
        input.extend_from_slice(message);
        Digest::of(HashAlg::Sha2_256, "duap.kdf.ml-dsa-msg.v1", &input).bytes
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        // Best-effort scrub. `write_volatile` prevents this being optimised
        // out; it does not prevent earlier copies made by the optimiser.
        for b in self.seed.iter_mut() {
            unsafe { std::ptr::write_volatile(b, 0) };
        }
    }
}

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SecretKey({}, {}, <redacted>)",
            self.suite,
            self.key_id()
        )
    }
}
