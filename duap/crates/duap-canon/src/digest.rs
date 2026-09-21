//! Content digests and domain-separated object identifiers.
//!
//! STATUS: PRODUCTION.
//!
//! # Domain separation
//!
//! Every digest in DUAP is computed over
//!
//! ```text
//! H( "DUAP/1" || 0x00 || <domain> || 0x00 || <canonical CBOR bytes> )
//! ```
//!
//! `<domain>` is an ASCII label naming the object type (`duap.event.v1`,
//! `duap.receipt.v1`, ...). Domain labels contain no NUL byte, so the
//! concatenation is injective: two different (domain, payload) pairs cannot
//! produce the same hash input. Without this, an attacker could present an
//! object of one type where another was expected and reuse a signature over
//! the digest ("type confusion"); with it, a signature over a
//! `duap.authorization.v1` digest can never verify as a `duap.receipt.v1`.
//!
//! # Algorithm agility
//!
//! A digest is self-describing (`sha2-256:<hex>`), and comparisons require
//! the algorithms to match. Adding an algorithm is a registry addition, not a
//! format change. See `docs/adr/0003-cryptographic-agility.md`.

use crate::error::{CanonError, Result};
use sha2::Digest as _;
use std::fmt;
use std::str::FromStr;

/// Prefix binding every digest to this protocol and major version.
pub const DIGEST_PREFIX: &[u8] = b"DUAP/1";

/// Hash algorithms in the DUAP registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HashAlg {
    /// SHA-256 (FIPS 180-4). The mandatory-to-implement algorithm.
    #[serde(rename = "sha2-256")]
    Sha2_256,
    /// BLAKE3, 256-bit output. Optional; used where throughput dominates.
    #[serde(rename = "blake3-256")]
    Blake3_256,
}

impl HashAlg {
    pub const fn label(self) -> &'static str {
        match self {
            HashAlg::Sha2_256 => "sha2-256",
            HashAlg::Blake3_256 => "blake3-256",
        }
    }

    pub const fn output_len(self) -> usize {
        32
    }

    pub fn parse(s: &str) -> Result<Self> {
        match s {
            "sha2-256" => Ok(HashAlg::Sha2_256),
            "blake3-256" => Ok(HashAlg::Blake3_256),
            other => Err(CanonError::UnknownHashAlg(other.to_owned())),
        }
    }

    /// Hash a pre-assembled input.
    pub fn hash(self, input: &[u8]) -> [u8; 32] {
        match self {
            HashAlg::Sha2_256 => {
                let mut h = sha2::Sha256::new();
                h.update(input);
                h.finalize().into()
            }
            HashAlg::Blake3_256 => *blake3::hash(input).as_bytes(),
        }
    }
}

/// A self-describing content digest.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digest {
    pub alg: HashAlg,
    pub bytes: [u8; 32],
}

impl Digest {
    /// Digest of `payload` under `domain` using `alg`.
    pub fn of(alg: HashAlg, domain: &str, payload: &[u8]) -> Digest {
        debug_assert!(
            !domain.as_bytes().contains(&0),
            "domain labels must not contain NUL"
        );
        let mut input = Vec::with_capacity(DIGEST_PREFIX.len() + domain.len() + 2 + payload.len());
        input.extend_from_slice(DIGEST_PREFIX);
        input.push(0);
        input.extend_from_slice(domain.as_bytes());
        input.push(0);
        input.extend_from_slice(payload);
        Digest {
            alg,
            bytes: alg.hash(&input),
        }
    }

    /// Digest of a canonical value under `domain`, using SHA-256.
    pub fn of_value(domain: &str, v: &crate::value::Value) -> Digest {
        Digest::of(HashAlg::Sha2_256, domain, &crate::codec::encode(v))
    }

    /// Digest of any serialisable object under `domain`, using SHA-256.
    pub fn of_object<T: serde::Serialize + ?Sized>(domain: &str, t: &T) -> Result<Digest> {
        Ok(Digest::of(
            HashAlg::Sha2_256,
            domain,
            &crate::interop::to_canonical_cbor(t)?,
        ))
    }

    pub fn to_hex(self) -> String {
        hex::encode(self.bytes)
    }

    /// Short form for logs and UIs: first 8 hex characters. Never use this
    /// for equality: 32 bits of a digest is not collision resistant.
    pub fn short(self) -> String {
        self.to_hex()[..8].to_owned()
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.alg.label(), self.to_hex())
    }
}

impl fmt::Debug for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl FromStr for Digest {
    type Err = CanonError;
    fn from_str(s: &str) -> Result<Self> {
        let (alg, hexs) = s
            .split_once(':')
            .ok_or_else(|| CanonError::BadDigest(format!("missing ':' in {s:?}")))?;
        let alg = HashAlg::parse(alg)?;
        let raw =
            hex::decode(hexs).map_err(|e| CanonError::BadDigest(format!("bad hex in {s:?}: {e}")))?;
        let bytes: [u8; 32] = raw
            .try_into()
            .map_err(|_| CanonError::BadDigest(format!("expected 32 bytes in {s:?}")))?;
        Ok(Digest { alg, bytes })
    }
}

impl serde::Serialize for Digest {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Digest {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Digest::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Combine two digests into one (used by Merkle constructions outside this
/// crate). The inputs are length-prefixed to keep the combination injective.
pub fn combine(alg: HashAlg, domain: &str, left: &Digest, right: &Digest) -> Digest {
    let mut buf = Vec::with_capacity(72);
    buf.extend_from_slice(left.alg.label().as_bytes());
    buf.push(0);
    buf.extend_from_slice(&left.bytes);
    buf.extend_from_slice(right.alg.label().as_bytes());
    buf.push(0);
    buf.extend_from_slice(&right.bytes);
    Digest::of(alg, domain, &buf)
}
