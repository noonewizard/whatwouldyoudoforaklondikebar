//! The DUAP canonical data model.
//!
//! STATUS: PRODUCTION (reference implementation).
//!
//! DUAP deliberately uses a *restricted* data model rather than "whatever CBOR
//! or JSON can express". Every construct that has more than one reasonable
//! serialization, or more than one reasonable comparison semantics, is removed:
//!
//! * no floating point (NaN, -0.0, and decimal-shortest-repr ambiguity),
//! * no CBOR tags (no semantic aliasing of the same value),
//! * no indefinite-length items (one encoding per value),
//! * no non-text map keys (one ordering rule, trivially mappable to JSON),
//! * no duplicate map keys (rejected at decode time, not last-one-wins),
//! * integers limited to the CBOR 64-bit range (u64 / -1-u64).
//!
//! Rationale: the digest of a protocol object is the object's identity. Any
//! encoding ambiguity is an attack surface (two parties disagree about what was
//! signed). See `docs/adr/0002-canonical-encoding.md`.

use std::collections::BTreeMap;
use std::fmt;

/// A value in the DUAP canonical data model.
///
/// `Ord`/`Eq` are derived and are *not* the canonical encoding order; use
/// [`crate::codec::encode`] and compare bytes when ordering matters.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    /// CBOR simple value 22 / JSON `null`.
    Null,
    /// CBOR simple values 20-21 / JSON `true`/`false`.
    Bool(bool),
    /// CBOR major type 0: 0 ..= 2^64-1.
    Uint(u64),
    /// CBOR major type 1. The payload `n` encodes the value `-1 - n`.
    Nint(u64),
    /// CBOR major type 2 (definite length only).
    Bytes(Vec<u8>),
    /// CBOR major type 3 (definite length only, valid UTF-8).
    Text(String),
    /// CBOR major type 4 (definite length only).
    Array(Vec<Value>),
    /// CBOR major type 5 (definite length, text keys only, no duplicates).
    ///
    /// Stored in a `BTreeMap` so that iteration is deterministic, but note the
    /// canonical *encoding* order is length-first bytewise (RFC 8949 4.2.1),
    /// not `BTreeMap`'s string order; [`crate::codec::encode`] re-sorts.
    Map(BTreeMap<String, Value>),
}

impl Value {
    /// The integer value, if this is an in-range signed integer.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Uint(u) => i64::try_from(*u).ok(),
            Value::Nint(n) => i64::try_from(*n).ok().and_then(|n| (-1i64).checked_sub(n)),
            _ => None,
        }
    }

    /// Build an integer value from any signed integer.
    pub fn int(v: i64) -> Value {
        if v >= 0 {
            Value::Uint(v as u64)
        } else {
            // -1 - n == v  =>  n = -1 - v ; computed via i128 to avoid i64::MIN overflow.
            Value::Nint((-1i128 - v as i128) as u64)
        }
    }

    pub fn text(s: impl Into<String>) -> Value {
        Value::Text(s.into())
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Value::Text(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Value::Bytes(b) => Some(b.as_slice()),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Value::Map(m) => Some(m),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Value::Array(a) => Some(a.as_slice()),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.as_map().and_then(|m| m.get(key))
    }

    /// Build a map from an iterator of pairs.
    pub fn map<K: Into<String>, I: IntoIterator<Item = (K, Value)>>(pairs: I) -> Value {
        Value::Map(pairs.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }

    /// Name of the value kind, for diagnostics.
    pub fn kind(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Bool(_) => "bool",
            Value::Uint(_) => "uint",
            Value::Nint(_) => "nint",
            Value::Bytes(_) => "bytes",
            Value::Text(_) => "text",
            Value::Array(_) => "array",
            Value::Map(_) => "map",
        }
    }

    /// Total number of nodes, used to bound recursion and to size buffers.
    pub fn node_count(&self) -> usize {
        match self {
            Value::Array(a) => 1 + a.iter().map(Value::node_count).sum::<usize>(),
            Value::Map(m) => 1 + m.values().map(Value::node_count).sum::<usize>(),
            _ => 1,
        }
    }

    /// Maximum nesting depth (a scalar has depth 1).
    pub fn depth(&self) -> usize {
        match self {
            Value::Array(a) => 1 + a.iter().map(Value::depth).max().unwrap_or(0),
            Value::Map(m) => 1 + m.values().map(Value::depth).max().unwrap_or(0),
            _ => 1,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Uint(u) => write!(f, "{u}"),
            Value::Nint(n) => write!(f, "-{}", u128::from(*n) + 1),
            Value::Bytes(b) => write!(f, "h'{}'", hex::encode(b)),
            Value::Text(s) => write!(f, "{s:?}"),
            Value::Array(a) => f.debug_list().entries(a.iter()).finish(),
            Value::Map(m) => f.debug_map().entries(m.iter()).finish(),
        }
    }
}

// ---------------------------------------------------------------------------
// serde
// ---------------------------------------------------------------------------
//
// `Value` is serialisable so that protocol objects can carry open extension
// maps without giving up the canonical model. The impls map each variant to
// the matching serde primitive, so a `Value` embedded in a struct encodes
// exactly as the equivalent inline field would.

impl serde::Serialize for Value {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Value::Null => s.serialize_none(),
            Value::Bool(b) => s.serialize_bool(*b),
            Value::Uint(u) => s.serialize_u64(*u),
            Value::Nint(n) => s.serialize_i128(-1i128 - *n as i128),
            Value::Bytes(b) => s.serialize_bytes(b),
            Value::Text(t) => s.serialize_str(t),
            Value::Array(a) => s.collect_seq(a.iter()),
            Value::Map(m) => s.collect_map(m.iter()),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(ValueVisitor)
    }
}

struct ValueVisitor;

impl<'de> serde::de::Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a value in the DUAP canonical data model")
    }

    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Value, E> {
        Ok(Value::Bool(v))
    }

    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Value, E> {
        Ok(Value::Uint(v))
    }

    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Value, E> {
        Ok(Value::int(v))
    }

    fn visit_u128<E: serde::de::Error>(self, v: u128) -> Result<Value, E> {
        u64::try_from(v)
            .map(Value::Uint)
            .map_err(|_| E::custom("integer exceeds the 64-bit CBOR range"))
    }

    fn visit_i128<E: serde::de::Error>(self, v: i128) -> Result<Value, E> {
        if v >= 0 {
            u64::try_from(v)
                .map(Value::Uint)
                .map_err(|_| E::custom("integer exceeds the 64-bit CBOR range"))
        } else {
            u64::try_from(-1 - v)
                .map(Value::Nint)
                .map_err(|_| E::custom("integer exceeds the 64-bit CBOR range"))
        }
    }

    fn visit_f64<E: serde::de::Error>(self, _v: f64) -> Result<Value, E> {
        Err(E::custom(
            "floating point is not part of the DUAP data model",
        ))
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Value, E> {
        Ok(Value::Text(v.to_owned()))
    }

    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Value, E> {
        Ok(Value::Text(v))
    }

    fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Value, E> {
        Ok(Value::Bytes(v.to_vec()))
    }

    fn visit_byte_buf<E: serde::de::Error>(self, v: Vec<u8>) -> Result<Value, E> {
        Ok(Value::Bytes(v))
    }

    fn visit_none<E: serde::de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_unit<E: serde::de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_some<D: serde::Deserializer<'de>>(self, d: D) -> Result<Value, D::Error> {
        d.deserialize_any(ValueVisitor)
    }

    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Value, A::Error> {
        let mut out = Vec::with_capacity(seq.size_hint().unwrap_or(0).min(1024));
        while let Some(v) = seq.next_element::<Value>()? {
            out.push(v);
        }
        Ok(Value::Array(out))
    }

    fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
        let mut out = BTreeMap::new();
        while let Some((k, v)) = map.next_entry::<String, Value>()? {
            if out.insert(k.clone(), v).is_some() {
                return Err(serde::de::Error::custom(format!("duplicate map key {k:?}")));
            }
        }
        Ok(Value::Map(out))
    }
}
