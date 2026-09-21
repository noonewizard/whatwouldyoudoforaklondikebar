//! Signature structures and signed envelopes.
//!
//! STATUS: PRODUCTION.
//!
//! # What a DUAP signature covers
//!
//! A signature is never computed over an object's in-memory form. It is
//! computed over the canonical CBOR encoding of a [`SigStructure`], which
//! binds:
//!
//! * the protocol context string (`duap.sig.v1`) -- so a DUAP signature can
//!   never be reused as some other protocol's signature and vice versa;
//! * the suite and key identifier -- so an attacker cannot re-present a
//!   signature as though it came from a different key or a weaker suite
//!   ("algorithm substitution");
//! * the payload *domain* -- so a signature over an authorization can never
//!   verify as a signature over a receipt ("type confusion");
//! * the payload digest;
//! * the creation timestamp and an optional nonce -- inputs to replay
//!   detection, which happens outside this crate (see `duap-meter`).
//!
//! The envelope carries the payload as opaque canonical bytes, not as a typed
//! structure. Verification therefore never depends on the verifier's ability
//! to re-serialise the payload identically -- a common source of signature
//! failures (and of forgeries) in systems that re-encode before verifying.
//!
//! # What a signature does NOT establish
//!
//! `created` is chosen by the signer. A signature alone proves only
//! *authorship*, never *time*. Proof of time requires an independent anchor:
//! in DUAP that is inclusion in the transparency log, whose signed tree heads
//! provide "this existed no later than T". See `duap-provenance` and
//! `THREAT_MODEL.md` section T-12.

use crate::error::{CryptoError, Result};
use crate::key::{KeyId, PublicKey, SecretKey};
use crate::registry::{KeyRegistry, VerificationContext};
use crate::suite::{SuiteId, SuitePolicy};
use duap_canon::digest::{Digest, HashAlg};
use serde::{Deserialize, Serialize};

/// Context string mixed into every DUAP signature.
pub const SIG_CONTEXT: &[u8] = b"DUAP/1 signature";

/// Domain label for the signature input structure.
pub const SIG_INPUT_DOMAIN: &str = "duap.sig-input.v1";

/// The structure that is actually signed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SigStructure {
    /// Fixed protocol context, `duap.sig.v1`.
    pub c: String,
    /// Suite label.
    pub s: SuiteId,
    /// Signing key identifier.
    pub k: KeyId,
    /// Domain label of the payload being signed.
    pub d: String,
    /// Digest of the payload under its domain.
    pub h: Digest,
    /// Signer-asserted creation time, microseconds since the Unix epoch, UTC.
    pub t: u64,
    /// Optional anti-replay nonce.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::bytes_serde::opt"
    )]
    pub n: Option<Vec<u8>>,
}

impl SigStructure {
    pub fn new(
        suite: SuiteId,
        kid: KeyId,
        domain: &str,
        payload_digest: Digest,
        created: u64,
        nonce: Option<Vec<u8>>,
    ) -> SigStructure {
        SigStructure {
            c: "duap.sig.v1".to_owned(),
            s: suite,
            k: kid,
            d: domain.to_owned(),
            h: payload_digest,
            t: created,
            n: nonce,
        }
    }

    /// The exact bytes passed to the signature primitive.
    pub fn to_signing_input(&self) -> Result<Vec<u8>> {
        Ok(duap_canon::to_canonical_cbor(self)?)
    }
}

/// One signature over an envelope payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature {
    pub suite: SuiteId,
    pub kid: KeyId,
    pub created: u64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::bytes_serde::opt"
    )]
    pub nonce: Option<Vec<u8>>,
    #[serde(with = "crate::bytes_serde")]
    pub sig: Vec<u8>,
}

/// A payload plus the signatures over it.
///
/// Multiple signatures are supported so that co-signing is possible without a
/// nested format: an event may be signed by the collecting agent and
/// counter-signed by the controller, and a receipt by the clearing node and by
/// an independent auditor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Envelope {
    /// Domain label naming the payload type, e.g. `duap.event.v1`.
    pub domain: String,
    /// Canonical CBOR encoding of the payload.
    #[serde(with = "crate::bytes_serde")]
    pub payload: Vec<u8>,
    pub signatures: Vec<Signature>,
}

impl Envelope {
    /// Wrap already-canonical payload bytes.
    pub fn from_payload_bytes(domain: impl Into<String>, payload: Vec<u8>) -> Result<Envelope> {
        duap_canon::codec::decode(&payload)?;
        Ok(Envelope {
            domain: domain.into(),
            payload,
            signatures: Vec::new(),
        })
    }

    /// Serialise `value` canonically and wrap it.
    pub fn seal<T: Serialize + ?Sized>(domain: impl Into<String>, value: &T) -> Result<Envelope> {
        let payload = duap_canon::to_canonical_cbor(value)?;
        Ok(Envelope {
            domain: domain.into(),
            payload,
            signatures: Vec::new(),
        })
    }

    /// Digest of the payload under the envelope's domain.
    pub fn payload_digest(&self) -> Digest {
        Digest::of(HashAlg::Sha2_256, &self.domain, &self.payload)
    }

    /// Decode the payload into a typed value, enforcing canonicity.
    pub fn open<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        Ok(duap_canon::from_canonical_cbor(&self.payload)?)
    }

    /// Append a signature by `key`.
    pub fn sign(
        &mut self,
        key: &SecretKey,
        created: u64,
        nonce: Option<Vec<u8>>,
    ) -> Result<&Signature> {
        let kid = key.key_id();
        let st = SigStructure::new(
            key.suite,
            kid,
            &self.domain,
            self.payload_digest(),
            created,
            nonce.clone(),
        );
        let input = st.to_signing_input()?;
        let sig = key.sign(&input, SIG_CONTEXT)?;
        self.signatures.push(Signature {
            suite: key.suite,
            kid,
            created,
            nonce,
            sig,
        });
        Ok(self.signatures.last().expect("just pushed"))
    }

    /// Verify a single signature against an explicit public key.
    ///
    /// This is the primitive check; it applies no policy and consults no
    /// registry, so callers must separately establish that the key is
    /// authorised for the role in question.
    pub fn verify_with_key(&self, sig: &Signature, pk: &PublicKey) -> Result<()> {
        if pk.suite != sig.suite {
            return Err(CryptoError::BadSignature("suite does not match the key"));
        }
        if pk.key_id() != sig.kid {
            return Err(CryptoError::KeyIdMismatch);
        }
        let st = SigStructure::new(
            sig.suite,
            sig.kid,
            &self.domain,
            self.payload_digest(),
            sig.created,
            sig.nonce.clone(),
        );
        pk.verify(&st.to_signing_input()?, &sig.sig, SIG_CONTEXT)
    }

    /// Verify every signature against a registry, applying suite policy and
    /// key validity at the verification time.
    ///
    /// Returns the identifiers of the keys whose signatures verified. An empty
    /// result is an error: an envelope that satisfies no policy-acceptable
    /// signature is not authenticated.
    pub fn verify(
        &self,
        registry: &KeyRegistry,
        policy: &SuitePolicy,
        ctx: &VerificationContext,
    ) -> Result<Vec<KeyId>> {
        let mut ok = Vec::new();
        let mut last_err = None;
        for sig in &self.signatures {
            if !policy.accepts(sig.suite) {
                last_err = Some(CryptoError::SuiteRejected {
                    suite: sig.suite.label(),
                });
                continue;
            }
            match registry.check_usable(&sig.kid, sig.created, ctx) {
                Ok(record) => match self.verify_with_key(sig, &record.public_key) {
                    Ok(()) => ok.push(sig.kid),
                    Err(e) => last_err = Some(e),
                },
                Err(e) => last_err = Some(e),
            }
        }
        if ok.is_empty() {
            Err(last_err.unwrap_or(CryptoError::NoAcceptableSignature))
        } else {
            Ok(ok)
        }
    }

    /// Total encoded size of the envelope in bytes.
    pub fn encoded_len(&self) -> Result<usize> {
        Ok(duap_canon::to_canonical_cbor(self)?.len())
    }
}
