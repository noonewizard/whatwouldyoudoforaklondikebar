//! Encode-then-decode over structured values, from the other direction.
//!
//! `canon_decode` starts from bytes and checks the encoder agrees. This
//! starts from a value the encoder produced and checks the decoder does
//! not change it -- which catches the case where both sides are
//! self-consistently wrong about a value the byte-first target cannot
//! reach, because arbitrary bytes rarely decode at all.
#![no_main]

use duap_canon::Value;
use libfuzzer_sys::fuzz_target;

/// Build a `Value` from the fuzzer's bytes. Deliberately crude: the point
/// is reachable structural variety, not a faithful generator.
fn build(data: &[u8], depth: usize) -> (Value, &[u8]) {
    if depth == 0 || data.is_empty() {
        return (Value::Null, data);
    }
    let (tag, rest) = data.split_first().expect("non-empty");
    match tag % 8 {
        0 => (Value::Null, rest),
        1 => (Value::Bool(tag & 8 != 0), rest),
        2 => {
            let (n, rest) = take8(rest);
            (Value::Uint(n), rest)
        }
        3 => {
            let (n, rest) = take8(rest);
            (Value::Nint(n % (i64::MAX as u64)), rest)
        }
        4 => {
            let (n, rest) = take1(rest);
            let k = (n as usize).min(rest.len());
            (Value::Bytes(rest[..k].to_vec()), &rest[k..])
        }
        5 => {
            let (n, rest) = take1(rest);
            let k = (n as usize).min(rest.len());
            let s = String::from_utf8_lossy(&rest[..k]).into_owned();
            (Value::Text(s), &rest[k..])
        }
        6 => {
            let (n, mut rest) = take1(rest);
            let mut items = Vec::new();
            for _ in 0..(n % 5) {
                let (v, r) = build(rest, depth - 1);
                items.push(v);
                rest = r;
            }
            (Value::Array(items), rest)
        }
        _ => {
            let (n, mut rest) = take1(rest);
            let mut map = std::collections::BTreeMap::new();
            for i in 0..(n % 5) {
                let (v, r) = build(rest, depth - 1);
                map.insert(format!("k{i}"), v);
                rest = r;
            }
            (Value::Map(map), rest)
        }
    }
}

fn take1(d: &[u8]) -> (u8, &[u8]) {
    match d.split_first() {
        Some((n, r)) => (*n, r),
        None => (0, d),
    }
}

fn take8(d: &[u8]) -> (u64, &[u8]) {
    let mut b = [0u8; 8];
    let k = d.len().min(8);
    b[..k].copy_from_slice(&d[..k]);
    (u64::from_le_bytes(b), &d[k..])
}

fuzz_target!(|data: &[u8]| {
    let (value, _) = build(data, 4);
    let bytes = duap_canon::codec::encode(&value);
    let decoded = duap_canon::codec::decode(&bytes).expect("encoder output must decode");
    assert_eq!(decoded, value, "round trip changed the value");

    let again = duap_canon::codec::encode(&decoded);
    assert_eq!(again, bytes, "encoding is not deterministic for this value");
});
