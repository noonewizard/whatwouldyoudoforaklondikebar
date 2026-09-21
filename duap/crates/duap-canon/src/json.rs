//! The DUAP JSON view.
//!
//! STATUS: PRODUCTION.
//!
//! JSON is a *transport and debugging view* of a canonical value, never the
//! hashing input. Implementations that speak only JSON must convert to
//! canonical CBOR before computing or verifying any digest or signature; the
//! conversion is total and lossless in both directions, and is covered by
//! round-trip property tests and by the cross-language conformance vectors in
//! `spec/vectors/`.
//!
//! Mapping (normative, see `PROTOCOL.md` section "JSON view"):
//!
//! | canonical value           | JSON                                   |
//! |---------------------------|----------------------------------------|
//! | `Null`                    | `null`                                 |
//! | `Bool(b)`                 | `true` / `false`                       |
//! | `Uint(u)`, u < 2^53       | number                                 |
//! | `Uint(u)`, u >= 2^53      | `{"$u64": "<decimal>"}`                |
//! | `Nint(n)`, value > -2^53  | number                                 |
//! | `Nint(n)`, value <= -2^53 | `{"$n64": "<decimal>"}`                |
//! | `Bytes(b)`                | `{"$b64": "<base64url, unpadded>"}`    |
//! | `Text(s)`                 | string                                 |
//! | `Array(a)`                | array                                  |
//! | `Map(m)`                  | object, keys beginning `$` escaped `$$`|
//!
//! The escape rule makes the three `$`-prefixed markers unambiguous: a real
//! map key `"$b64"` is emitted as `"$$b64"`, so a one-key object whose key is
//! exactly `"$b64"` can only ever be a byte string.

use crate::error::{CanonError, Result};
use crate::value::Value;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD as B64;
use serde_json::{Map as JMap, Value as J};

/// Largest integer that survives a round trip through an IEEE-754 double.
pub const JSON_SAFE_INT: u64 = (1u64 << 53) - 1;

const M_U64: &str = "$u64";
const M_N64: &str = "$n64";
const M_B64: &str = "$b64";

fn escape_key(k: &str) -> String {
    if k.starts_with('$') {
        format!("${k}")
    } else {
        k.to_owned()
    }
}

fn unescape_key(k: &str) -> Result<String> {
    if let Some(rest) = k.strip_prefix("$$") {
        Ok(format!("${rest}"))
    } else if k.starts_with('$') {
        Err(CanonError::JsonView(format!(
            "object key {k:?} uses the reserved `$` prefix but is not a known marker"
        )))
    } else {
        Ok(k.to_owned())
    }
}

/// Render a canonical value as its JSON view.
pub fn to_json(v: &Value) -> J {
    match v {
        Value::Null => J::Null,
        Value::Bool(b) => J::Bool(*b),
        Value::Uint(u) => {
            if *u <= JSON_SAFE_INT {
                J::Number((*u).into())
            } else {
                marker(M_U64, u.to_string())
            }
        }
        Value::Nint(n) => {
            let magnitude = u128::from(*n) + 1;
            if magnitude <= JSON_SAFE_INT as u128 {
                J::Number((-(magnitude as i64)).into())
            } else {
                marker(M_N64, format!("-{magnitude}"))
            }
        }
        Value::Bytes(b) => marker(M_B64, B64.encode(b)),
        Value::Text(s) => J::String(s.clone()),
        Value::Array(a) => J::Array(a.iter().map(to_json).collect()),
        Value::Map(m) => {
            // Emit in canonical (length-first) order so that a pretty-printed
            // JSON view lines up with the CBOR encoding field for field.
            let mut keys: Vec<&String> = m.keys().collect();
            keys.sort_unstable_by(|a, b| (a.len(), a.as_bytes()).cmp(&(b.len(), b.as_bytes())));
            let mut obj = JMap::new();
            for k in keys {
                obj.insert(escape_key(k), to_json(&m[k]));
            }
            J::Object(obj)
        }
    }
}

fn marker(tag: &str, value: String) -> J {
    let mut m = JMap::new();
    m.insert(tag.to_owned(), J::String(value));
    J::Object(m)
}

/// Parse a JSON view back into a canonical value.
pub fn from_json(j: &J) -> Result<Value> {
    Ok(match j {
        J::Null => Value::Null,
        J::Bool(b) => Value::Bool(*b),
        J::Number(n) => {
            if let Some(u) = n.as_u64() {
                if u > JSON_SAFE_INT {
                    return Err(CanonError::JsonView(format!(
                        "integer {u} exceeds the JSON-safe range; use {{\"{M_U64}\": \"{u}\"}}"
                    )));
                }
                Value::Uint(u)
            } else if let Some(i) = n.as_i64() {
                if i < -(JSON_SAFE_INT as i64) {
                    return Err(CanonError::JsonView(format!(
                        "integer {i} exceeds the JSON-safe range; use {{\"{M_N64}\": \"{i}\"}}"
                    )));
                }
                Value::int(i)
            } else {
                return Err(CanonError::JsonView(format!(
                    "non-integer number {n} is not part of the DUAP data model"
                )));
            }
        }
        J::String(s) => Value::Text(s.clone()),
        J::Array(a) => Value::Array(a.iter().map(from_json).collect::<Result<_>>()?),
        J::Object(o) => {
            if o.len() == 1 {
                let (k, v) = o.iter().next().expect("len checked");
                match (k.as_str(), v) {
                    (M_B64, J::String(s)) => {
                        let bytes = B64
                            .decode(s.as_bytes())
                            .map_err(|e| CanonError::JsonView(format!("bad {M_B64}: {e}")))?;
                        // Reject non-canonical base64 (e.g. padded or with
                        // non-zero trailing bits) by re-encoding.
                        if B64.encode(&bytes) != *s {
                            return Err(CanonError::JsonView(
                                "base64url payload is not in canonical unpadded form".into(),
                            ));
                        }
                        return Ok(Value::Bytes(bytes));
                    }
                    (M_U64, J::String(s)) => {
                        let u: u64 = s.parse().map_err(|_| {
                            CanonError::JsonView(format!("bad {M_U64} literal {s:?}"))
                        })?;
                        if u <= JSON_SAFE_INT {
                            return Err(CanonError::JsonView(format!(
                                "{M_U64} must not be used for values in the JSON-safe range ({u})"
                            )));
                        }
                        return Ok(Value::Uint(u));
                    }
                    (M_N64, J::String(s)) => {
                        let i: i128 = s.parse().map_err(|_| {
                            CanonError::JsonView(format!("bad {M_N64} literal {s:?}"))
                        })?;
                        if i >= -(JSON_SAFE_INT as i128) {
                            return Err(CanonError::JsonView(format!(
                                "{M_N64} must not be used for values in the JSON-safe range ({i})"
                            )));
                        }
                        let n = u64::try_from(-1 - i).map_err(|_| {
                            CanonError::JsonView(format!("{M_N64} literal {s:?} out of range"))
                        })?;
                        return Ok(Value::Nint(n));
                    }
                    (M_B64 | M_U64 | M_N64, other) => {
                        return Err(CanonError::JsonView(format!(
                            "marker {k} must carry a string, found {other}"
                        )));
                    }
                    _ => {}
                }
            }
            let mut m = std::collections::BTreeMap::new();
            for (k, v) in o {
                let key = unescape_key(k)?;
                if m.insert(key.clone(), from_json(v)?).is_some() {
                    return Err(CanonError::DuplicateKey { key });
                }
            }
            Value::Map(m)
        }
    })
}

/// Serialise the JSON view as a compact string.
pub fn to_json_string(v: &Value) -> String {
    serde_json::to_string(&to_json(v)).expect("JSON view is always serialisable")
}

/// Serialise the JSON view as an indented string.
pub fn to_json_pretty(v: &Value) -> String {
    serde_json::to_string_pretty(&to_json(v)).expect("JSON view is always serialisable")
}

/// Parse a JSON view from a string.
pub fn from_json_str(s: &str) -> Result<Value> {
    let j: J = serde_json::from_str(s).map_err(|e| CanonError::JsonView(e.to_string()))?;
    from_json(&j)
}
