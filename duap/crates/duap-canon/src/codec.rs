//! Deterministic CBOR codec for the DUAP canonical data model.
//!
//! STATUS: PRODUCTION (reference implementation).
//!
//! The encoder emits the RFC 8949 section 4.2.1 "core deterministic encoding"
//! subset, further restricted by the DUAP data model (see [`crate::value`]).
//! The decoder is *strict*: it rejects any input that the encoder would not
//! have produced. Together this gives the property the protocol depends on:
//!
//! ```text
//! decode(b) = Ok(v)  =>  encode(v) == b            (bit-for-bit)
//! encode(v) = b      =>  decode(b) == Ok(v)
//! ```
//!
//! Both directions are exercised by property tests in `tests/prop_codec.rs`.
//!
//! Strictness matters because a signature covers bytes, while authorization
//! decisions are made on the decoded structure. If two byte strings could
//! decode to the same structure, a receipt could be re-encoded and replayed
//! with a different digest; if one byte string could decode two ways, two
//! verifiers could disagree about what was authorized.

use crate::error::{CanonError, Result};
use crate::value::Value;
use std::collections::BTreeMap;

/// Maximum nesting depth accepted by the strict decoder.
///
/// Bounded to keep decoding non-recursive in stack terms for adversarial
/// input; DUAP protocol objects are at most 8 deep.
pub const MAX_DEPTH: usize = 64;

/// Maximum number of items accepted in a single array or map.
pub const MAX_COLLECTION_LEN: usize = 1 << 20;

/// Maximum total encoded size accepted by [`decode`], in bytes (16 MiB).
pub const MAX_INPUT_LEN: usize = 16 * 1024 * 1024;

const MT_UINT: u8 = 0;
const MT_NINT: u8 = 1;
const MT_BYTES: u8 = 2;
const MT_TEXT: u8 = 3;
const MT_ARRAY: u8 = 4;
const MT_MAP: u8 = 5;
const MT_SIMPLE: u8 = 7;

// ---------------------------------------------------------------------------
// Encoding
// ---------------------------------------------------------------------------

/// Encode a value as deterministic CBOR.
pub fn encode(v: &Value) -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
    encode_into(v, &mut out);
    out
}

/// Encode a value, appending to `out`.
pub fn encode_into(v: &Value, out: &mut Vec<u8>) {
    match v {
        Value::Null => out.push(0xf6),
        Value::Bool(false) => out.push(0xf4),
        Value::Bool(true) => out.push(0xf5),
        Value::Uint(u) => head(out, MT_UINT, *u),
        Value::Nint(n) => head(out, MT_NINT, *n),
        Value::Bytes(b) => {
            head(out, MT_BYTES, b.len() as u64);
            out.extend_from_slice(b);
        }
        Value::Text(s) => {
            head(out, MT_TEXT, s.len() as u64);
            out.extend_from_slice(s.as_bytes());
        }
        Value::Array(a) => {
            head(out, MT_ARRAY, a.len() as u64);
            for item in a {
                encode_into(item, out);
            }
        }
        Value::Map(m) => {
            head(out, MT_MAP, m.len() as u64);
            // RFC 8949 4.2.1: keys are sorted by their *encoded* bytes,
            // lexicographically. For text keys this is length-first, then
            // bytewise on UTF-8 -- which is NOT BTreeMap's ordering, so the
            // keys are re-sorted here rather than taken in map order.
            let mut keys: Vec<&String> = m.keys().collect();
            keys.sort_unstable_by(|a, b| {
                (a.len(), a.as_bytes()).cmp(&(b.len(), b.as_bytes()))
            });
            for k in keys {
                head(out, MT_TEXT, k.len() as u64);
                out.extend_from_slice(k.as_bytes());
                encode_into(&m[k], out);
            }
        }
    }
}

/// Write a CBOR head (major type + argument) in shortest form.
fn head(out: &mut Vec<u8>, mt: u8, arg: u64) {
    let mt = mt << 5;
    match arg {
        0..=23 => out.push(mt | arg as u8),
        24..=0xff => {
            out.push(mt | 24);
            out.push(arg as u8);
        }
        0x100..=0xffff => {
            out.push(mt | 25);
            out.extend_from_slice(&(arg as u16).to_be_bytes());
        }
        0x1_0000..=0xffff_ffff => {
            out.push(mt | 26);
            out.extend_from_slice(&(arg as u32).to_be_bytes());
        }
        _ => {
            out.push(mt | 27);
            out.extend_from_slice(&arg.to_be_bytes());
        }
    }
}

// ---------------------------------------------------------------------------
// Decoding
// ---------------------------------------------------------------------------

/// Decode deterministic CBOR into a canonical value.
///
/// Returns [`CanonError::NonCanonical`] for input that is valid CBOR but not
/// the canonical encoding of its value, and [`CanonError::Unsupported`] for
/// CBOR constructs outside the DUAP data model.
pub fn decode(bytes: &[u8]) -> Result<Value> {
    if bytes.len() > MAX_INPUT_LEN {
        return Err(CanonError::TooLarge {
            len: bytes.len(),
            max: MAX_INPUT_LEN,
        });
    }
    let mut d = Decoder { b: bytes, pos: 0 };
    let v = d.value(0)?;
    if d.pos != bytes.len() {
        return Err(CanonError::TrailingBytes {
            consumed: d.pos,
            total: bytes.len(),
        });
    }
    Ok(v)
}

/// True if `bytes` is exactly the canonical encoding of some canonical value.
pub fn is_canonical(bytes: &[u8]) -> bool {
    decode(bytes).is_ok()
}

struct Decoder<'a> {
    b: &'a [u8],
    pos: usize,
}

impl<'a> Decoder<'a> {
    fn need(&self, n: usize) -> Result<()> {
        if self.pos + n > self.b.len() {
            Err(CanonError::UnexpectedEof {
                at: self.pos,
                want: n,
            })
        } else {
            Ok(())
        }
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        self.need(n)?;
        let s = &self.b[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    /// Read a head, returning (major type, argument), enforcing shortest-form.
    fn head(&mut self) -> Result<(u8, u64)> {
        self.need(1)?;
        let ib = self.b[self.pos];
        self.pos += 1;
        let mt = ib >> 5;
        let ai = ib & 0x1f;
        let arg = match ai {
            0..=23 => ai as u64,
            24 => {
                let v = self.take(1)?[0] as u64;
                if v < 24 {
                    return Err(CanonError::NonCanonical {
                        at: self.pos - 2,
                        reason: "argument not in shortest form (1-byte)",
                    });
                }
                v
            }
            25 => {
                let s = self.take(2)?;
                let v = u16::from_be_bytes([s[0], s[1]]) as u64;
                if v <= 0xff {
                    return Err(CanonError::NonCanonical {
                        at: self.pos - 3,
                        reason: "argument not in shortest form (2-byte)",
                    });
                }
                v
            }
            26 => {
                let s = self.take(4)?;
                let v = u32::from_be_bytes([s[0], s[1], s[2], s[3]]) as u64;
                if v <= 0xffff {
                    return Err(CanonError::NonCanonical {
                        at: self.pos - 5,
                        reason: "argument not in shortest form (4-byte)",
                    });
                }
                v
            }
            27 => {
                let s = self.take(8)?;
                let v = u64::from_be_bytes([
                    s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7],
                ]);
                if v <= 0xffff_ffff {
                    return Err(CanonError::NonCanonical {
                        at: self.pos - 9,
                        reason: "argument not in shortest form (8-byte)",
                    });
                }
                v
            }
            28..=30 => {
                return Err(CanonError::Malformed {
                    at: self.pos - 1,
                    reason: "reserved additional information 28-30",
                });
            }
            _ => {
                // ai == 31: indefinite length / break
                return Err(CanonError::Unsupported {
                    at: self.pos - 1,
                    reason: "indefinite-length item or break stop code",
                });
            }
        };
        Ok((mt, arg))
    }

    fn value(&mut self, depth: usize) -> Result<Value> {
        if depth >= MAX_DEPTH {
            return Err(CanonError::DepthLimit { max: MAX_DEPTH });
        }
        let start = self.pos;
        let (mt, arg) = self.head()?;
        match mt {
            MT_UINT => Ok(Value::Uint(arg)),
            MT_NINT => Ok(Value::Nint(arg)),
            MT_BYTES => {
                let n = self.len_of(arg)?;
                Ok(Value::Bytes(self.take(n)?.to_vec()))
            }
            MT_TEXT => {
                let n = self.len_of(arg)?;
                let raw = self.take(n)?;
                let s = std::str::from_utf8(raw).map_err(|_| CanonError::Malformed {
                    at: start,
                    reason: "text string is not valid UTF-8",
                })?;
                Ok(Value::Text(s.to_owned()))
            }
            MT_ARRAY => {
                let n = self.collection_len(arg)?;
                let mut items = Vec::with_capacity(n.min(1024));
                for _ in 0..n {
                    items.push(self.value(depth + 1)?);
                }
                Ok(Value::Array(items))
            }
            MT_MAP => {
                let n = self.collection_len(arg)?;
                let mut m = BTreeMap::new();
                let mut prev: Option<Vec<u8>> = None;
                for _ in 0..n {
                    let kstart = self.pos;
                    let (kmt, karg) = self.head()?;
                    if kmt != MT_TEXT {
                        return Err(CanonError::Unsupported {
                            at: kstart,
                            reason: "map key is not a text string",
                        });
                    }
                    let klen = self.len_of(karg)?;
                    let kraw = self.take(klen)?;
                    let key = std::str::from_utf8(kraw)
                        .map_err(|_| CanonError::Malformed {
                            at: kstart,
                            reason: "map key is not valid UTF-8",
                        })?
                        .to_owned();
                    // Canonical key order: length-first, then bytewise.
                    let ord_key = {
                        let mut v = Vec::with_capacity(8 + key.len());
                        v.extend_from_slice(&(key.len() as u64).to_be_bytes());
                        v.extend_from_slice(key.as_bytes());
                        v
                    };
                    if let Some(p) = &prev {
                        if *p == ord_key {
                            return Err(CanonError::DuplicateKey { key });
                        }
                        if *p > ord_key {
                            return Err(CanonError::NonCanonical {
                                at: kstart,
                                reason: "map keys are not in canonical order",
                            });
                        }
                    }
                    prev = Some(ord_key);
                    let val = self.value(depth + 1)?;
                    if m.insert(key.clone(), val).is_some() {
                        return Err(CanonError::DuplicateKey { key });
                    }
                }
                Ok(Value::Map(m))
            }
            6 => Err(CanonError::Unsupported {
                at: start,
                reason: "CBOR tags are not part of the DUAP data model",
            }),
            MT_SIMPLE => match arg {
                20 => Ok(Value::Bool(false)),
                21 => Ok(Value::Bool(true)),
                22 => Ok(Value::Null),
                23 => Err(CanonError::Unsupported {
                    at: start,
                    reason: "`undefined` is not part of the DUAP data model",
                }),
                25 | 26 | 27 => Err(CanonError::Unsupported {
                    at: start,
                    reason: "floating point is not part of the DUAP data model",
                }),
                _ => Err(CanonError::Unsupported {
                    at: start,
                    reason: "unassigned CBOR simple value",
                }),
            },
            _ => unreachable!("major type is 3 bits"),
        }
    }

    fn len_of(&self, arg: u64) -> Result<usize> {
        let n = usize::try_from(arg).map_err(|_| CanonError::TooLarge {
            len: usize::MAX,
            max: MAX_INPUT_LEN,
        })?;
        if self.pos + n > self.b.len() {
            return Err(CanonError::UnexpectedEof { at: self.pos, want: n });
        }
        Ok(n)
    }

    fn collection_len(&self, arg: u64) -> Result<usize> {
        let n = usize::try_from(arg).unwrap_or(usize::MAX);
        if n > MAX_COLLECTION_LEN {
            return Err(CanonError::TooLarge {
                len: n,
                max: MAX_COLLECTION_LEN,
            });
        }
        // Each remaining item costs at least one byte, so a length larger than
        // the remaining input is malformed. This bounds allocation from a
        // hostile length header without allocating first.
        if n > self.b.len() - self.pos {
            return Err(CanonError::UnexpectedEof {
                at: self.pos,
                want: n,
            });
        }
        Ok(n)
    }
}

/// Re-encode arbitrary (possibly non-canonical) CBOR into canonical form.
///
/// STATUS: REFERENCE. Used at trust boundaries where a peer may have produced
/// merely-valid CBOR. Signature verification never uses this: a signature is
/// over exact bytes, and [`decode`] rejects anything non-canonical.
pub fn canonicalize_lenient(bytes: &[u8]) -> Result<Vec<u8>> {
    let v: ciborium::Value =
        ciborium::from_reader(bytes).map_err(|e| CanonError::Foreign(e.to_string()))?;
    let v = crate::interop::from_ciborium(&v)?;
    Ok(encode(&v))
}
