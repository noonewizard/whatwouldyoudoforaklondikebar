//! # duap-canon
//!
//! Canonical data model, deterministic CBOR codec, JSON view, and content
//! digests for the Data Usage Accounting Protocol (DUAP).
//!
//! STATUS: PRODUCTION (reference implementation).
//!
//! Everything in DUAP that is hashed, signed, logged, or compared for equality
//! passes through this crate. The invariants it provides are:
//!
//! 1. **One encoding per value.** [`codec::encode`] is injective and
//!    [`codec::decode`] rejects every byte string the encoder would not emit,
//!    so `decode(b) = v` implies `encode(v) = b`.
//! 2. **No ambiguous scalars.** Floating point, CBOR tags, indefinite-length
//!    items, non-text map keys, duplicate keys and `undefined` are outside the
//!    model and are rejected, not coerced.
//! 3. **Domain-separated identity.** [`digest::Digest::of`] binds each digest
//!    to a protocol version and an object-type label, so a signature over one
//!    object type can never be replayed as another.
//! 4. **A lossless JSON view.** [`json`] maps canonical values to JSON and
//!    back without loss, for systems that cannot speak CBOR — but the JSON
//!    text is never the hashing input.
//!
//! ```
//! use duap_canon::{Value, codec, digest::{Digest, HashAlg}};
//! let v = Value::map([
//!     ("purpose", Value::text("fraud_detection")),
//!     ("count", Value::Uint(42)),
//! ]);
//! let bytes = codec::encode(&v);
//! assert_eq!(codec::decode(&bytes).unwrap(), v);
//! let d = Digest::of(HashAlg::Sha2_256, "duap.test.v1", &bytes);
//! assert_eq!(d.to_string().len(), 8 + 1 + 64);
//! ```

pub mod codec;
pub mod digest;
pub mod error;
pub mod interop;
pub mod json;
pub mod value;

pub use codec::{decode, encode, is_canonical};
pub use digest::{Digest, HashAlg};
pub use error::{CanonError, Result};
pub use interop::{from_canonical_cbor, from_value, to_canonical_cbor, to_value};
pub use value::Value;

/// Protocol identifier embedded in digests and signature inputs.
pub const PROTOCOL_ID: &str = "DUAP/1";

/// Wire format version of the canonical encoding.
pub const CANON_VERSION: u32 = 1;
