//! Bridges between the canonical model and `serde` / `ciborium` / `serde_json`.
//!
//! STATUS: PRODUCTION.
//!
//! DUAP protocol types derive `Serialize`/`Deserialize` and reach canonical
//! bytes through this module. The path is deliberately "serialize with a
//! general codec, then normalise", rather than a bespoke `serde::Serializer`:
//! the normalisation pass is the same code that runs on bytes received from
//! the network, so there is exactly one definition of canonical form in the
//! implementation.

use crate::codec;
use crate::error::{CanonError, Result};
use crate::value::Value;
use serde::{Serialize, de::DeserializeOwned};
use std::collections::BTreeMap;

/// Convert a `ciborium::Value` into the canonical model, rejecting anything
/// outside it.
pub fn from_ciborium(v: &ciborium::Value) -> Result<Value> {
    use ciborium::Value as C;
    Ok(match v {
        C::Null => Value::Null,
        C::Bool(b) => Value::Bool(*b),
        C::Integer(i) => {
            let n: i128 = (*i).into();
            if n >= 0 {
                Value::Uint(u64::try_from(n).map_err(|_| {
                    CanonError::OutOfModel("integer exceeds 64-bit CBOR range".into())
                })?)
            } else {
                {
                    let payload = u64::try_from(-1 - n).map_err(|_| {
                        CanonError::OutOfModel("integer exceeds 64-bit CBOR range".into())
                    })?;
                    if payload > Value::MAX_NINT_PAYLOAD {
                        return Err(CanonError::OutOfModel(
                            "negative integers below i64::MIN are outside the DUAP data model"
                                .into(),
                        ));
                    }
                    Value::Nint(payload)
                }
            }
        }
        C::Bytes(b) => Value::Bytes(b.clone()),
        C::Text(s) => Value::Text(s.clone()),
        C::Array(a) => Value::Array(a.iter().map(from_ciborium).collect::<Result<_>>()?),
        C::Map(m) => {
            let mut out = BTreeMap::new();
            for (k, val) in m {
                let key = match k {
                    C::Text(s) => s.clone(),
                    other => {
                        return Err(CanonError::OutOfModel(format!(
                            "map key of kind {other:?} is not a text string"
                        )));
                    }
                };
                if out.insert(key.clone(), from_ciborium(val)?).is_some() {
                    return Err(CanonError::DuplicateKey { key });
                }
            }
            Value::Map(out)
        }
        C::Float(_) => {
            return Err(CanonError::OutOfModel(
                "floating point is not part of the DUAP data model".into(),
            ));
        }
        C::Tag(t, _) => {
            return Err(CanonError::OutOfModel(format!(
                "CBOR tag {t} is not part of the DUAP data model"
            )));
        }
        other => {
            return Err(CanonError::OutOfModel(format!(
                "CBOR value {other:?} is not part of the DUAP data model"
            )));
        }
    })
}

/// Serialise any `Serialize` type to canonical bytes.
pub fn to_canonical_cbor<T: Serialize + ?Sized>(t: &T) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    ciborium::into_writer(t, &mut buf).map_err(|e| CanonError::Foreign(e.to_string()))?;
    codec::canonicalize_lenient(&buf)
}

/// Serialise any `Serialize` type to a canonical [`Value`].
pub fn to_value<T: Serialize + ?Sized>(t: &T) -> Result<Value> {
    let mut buf = Vec::new();
    ciborium::into_writer(t, &mut buf).map_err(|e| CanonError::Foreign(e.to_string()))?;
    let v: ciborium::Value =
        ciborium::from_reader(&buf[..]).map_err(|e| CanonError::Foreign(e.to_string()))?;
    from_ciborium(&v)
}

/// Deserialise a type from canonical bytes.
///
/// The bytes are first checked for canonicity; merely-valid CBOR is rejected.
pub fn from_canonical_cbor<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    codec::decode(bytes)?;
    ciborium::from_reader(bytes).map_err(|e| CanonError::Foreign(e.to_string()))
}

/// Deserialise a type from a canonical [`Value`].
pub fn from_value<T: DeserializeOwned>(v: &Value) -> Result<T> {
    let bytes = codec::encode(v);
    ciborium::from_reader(&bytes[..]).map_err(|e| CanonError::Foreign(e.to_string()))
}
