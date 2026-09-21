//! Serde helper that encodes `Vec<u8>` as a CBOR byte string rather than an
//! array of integers.
//!
//! STATUS: REFERENCE.
//!
//! Serde has no byte-string type, so `Vec<u8>` round-trips through `ciborium`
//! as `[u8, u8, ...]` by default -- 1-2 bytes per element instead of one, and
//! a different canonical encoding from every other implementation. Protocol
//! fields carrying opaque bytes use `#[serde(with = "bytes_serde")]`.

use serde::de::{Error as DeError, SeqAccess, Visitor};
use serde::{Deserializer, Serializer};
use std::fmt;

pub fn serialize<S: Serializer>(v: &[u8], s: S) -> Result<S::Ok, S::Error> {
    s.serialize_bytes(v)
}

pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
    d.deserialize_bytes(BytesVisitor)
}

struct BytesVisitor;

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = Vec<u8>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a byte string")
    }

    fn visit_bytes<E: DeError>(self, v: &[u8]) -> Result<Vec<u8>, E> {
        Ok(v.to_vec())
    }

    fn visit_byte_buf<E: DeError>(self, v: Vec<u8>) -> Result<Vec<u8>, E> {
        Ok(v)
    }

    fn visit_str<E: DeError>(self, v: &str) -> Result<Vec<u8>, E> {
        // Tolerated so that hand-written JSON fixtures can use hex.
        hex::decode(v).map_err(|e| E::custom(format!("expected hex byte string: {e}")))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Vec<u8>, A::Error> {
        let mut out = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(b) = seq.next_element::<u8>()? {
            out.push(b);
        }
        Ok(out)
    }
}

/// The same helper for `Option<Vec<u8>>`.
pub mod opt {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &Option<Vec<u8>>, s: S) -> Result<S::Ok, S::Error> {
        match v {
            Some(b) => s.serialize_some(&Wrap(b)),
            None => s.serialize_none(),
        }
    }

    struct Wrap<'a>(&'a Vec<u8>);
    impl serde::Serialize for Wrap<'_> {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            super::serialize(self.0, s)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Vec<u8>>, D::Error> {
        #[derive(Deserialize)]
        struct W(#[serde(with = "super")] Vec<u8>);
        Ok(Option::<W>::deserialize(d)?.map(|w| w.0))
    }
}

/// Alias used by `PublicKey`, kept separate so the module path in
/// `#[serde(with = ...)]` reads clearly.
pub use self::{deserialize as de, serialize as ser};
