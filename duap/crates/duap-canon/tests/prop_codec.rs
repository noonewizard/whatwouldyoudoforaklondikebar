//! Property tests for the canonical codec.
//!
//! These encode the invariants the rest of the protocol relies on. A failure
//! here is a protocol-level break, not a cosmetic bug.

use duap_canon::json;
use duap_canon::value::Value;
use duap_canon::{codec, digest::Digest, digest::HashAlg};
use proptest::prelude::*;
use std::collections::BTreeMap;

/// Generate arbitrary canonical values, including deep and wide ones.
fn arb_value() -> impl Strategy<Value = Value> {
    let leaf = prop_oneof![
        Just(Value::Null),
        any::<bool>().prop_map(Value::Bool),
        any::<u64>().prop_map(Value::Uint),
        any::<u64>().prop_map(Value::Nint),
        proptest::collection::vec(any::<u8>(), 0..40).prop_map(Value::Bytes),
        ".{0,40}".prop_map(Value::Text),
    ];
    leaf.prop_recursive(5, 96, 8, |inner| {
        prop_oneof![
            proptest::collection::vec(inner.clone(), 0..6).prop_map(Value::Array),
            proptest::collection::btree_map("[\\$a-z0-9_.]{0,12}", inner, 0..6)
                .prop_map(|m| Value::Map(m.into_iter().collect::<BTreeMap<_, _>>())),
        ]
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    /// encode . decode = id on valid canonical bytes.
    #[test]
    fn roundtrip_value(v in arb_value()) {
        let b = codec::encode(&v);
        let back = codec::decode(&b).expect("canonical bytes must decode");
        prop_assert_eq!(&back, &v);
    }

    /// decode . encode = id on bytes: the encoding is the unique canonical one.
    #[test]
    fn roundtrip_bytes(v in arb_value()) {
        let b = codec::encode(&v);
        let b2 = codec::encode(&codec::decode(&b).unwrap());
        prop_assert_eq!(b, b2);
    }

    /// Distinct values never share an encoding (injectivity, tested pairwise).
    #[test]
    fn injective(a in arb_value(), b in arb_value()) {
        let ea = codec::encode(&a);
        let eb = codec::encode(&b);
        prop_assert_eq!(ea == eb, a == b);
    }

    /// The JSON view is lossless.
    #[test]
    fn json_view_roundtrip(v in arb_value()) {
        let s = json::to_json_string(&v);
        let back = json::from_json_str(&s).expect("JSON view must parse");
        prop_assert_eq!(back, v);
    }

    /// Truncating canonical bytes never yields a valid decode.
    #[test]
    fn truncation_rejected(v in arb_value(), cut in 0usize..64) {
        let b = codec::encode(&v);
        prop_assume!(!b.is_empty());
        let cut = cut % b.len();
        prop_assume!(cut > 0);
        let truncated = &b[..b.len() - cut];
        prop_assert!(codec::decode(truncated).is_err());
    }

    /// Appending bytes never yields a valid decode (no trailing data allowed).
    #[test]
    fn trailing_rejected(v in arb_value(), extra in proptest::collection::vec(any::<u8>(), 1..8)) {
        let mut b = codec::encode(&v);
        b.extend_from_slice(&extra);
        prop_assert!(codec::decode(&b).is_err());
    }

    /// Digests are domain separated: the same payload under two domains differs.
    #[test]
    fn domain_separation(v in arb_value()) {
        let b = codec::encode(&v);
        let d1 = Digest::of(HashAlg::Sha2_256, "duap.event.v1", &b);
        let d2 = Digest::of(HashAlg::Sha2_256, "duap.receipt.v1", &b);
        prop_assert_ne!(d1, d2);
    }

    /// Decoding never panics, whatever the input.
    #[test]
    fn decode_never_panics(raw in proptest::collection::vec(any::<u8>(), 0..512)) {
        let _ = codec::decode(&raw);
    }

    /// Arbitrary bytes that do decode must decode to their own canonical form.
    #[test]
    fn accepted_input_is_canonical(raw in proptest::collection::vec(any::<u8>(), 0..512)) {
        if let Ok(v) = codec::decode(&raw) {
            prop_assert_eq!(codec::encode(&v), raw);
        }
    }
}

// ---------------------------------------------------------------------------
// Explicit non-canonical inputs (regression corpus)
// ---------------------------------------------------------------------------

#[test]
fn rejects_non_shortest_integer() {
    // 0x18 0x05 == uint(5) encoded in two bytes; canonical form is 0x05.
    assert!(codec::decode(&[0x18, 0x05]).is_err());
    assert!(codec::decode(&[0x05]).is_ok());
}

#[test]
fn rejects_indefinite_length() {
    // 0x9f ... 0xff == indefinite-length array.
    assert!(codec::decode(&[0x9f, 0x01, 0xff]).is_err());
}

#[test]
fn rejects_float() {
    // 0xfa 0x47 0xc3 0x50 0x00 == float32(100000.0)
    assert!(codec::decode(&[0xfa, 0x47, 0xc3, 0x50, 0x00]).is_err());
    // half and double too
    assert!(codec::decode(&[0xf9, 0x3c, 0x00]).is_err());
    assert!(codec::decode(&[0xfb, 0, 0, 0, 0, 0, 0, 0, 0]).is_err());
}

#[test]
fn rejects_tags() {
    // 0xc0 0x61 0x61 == tag(0) "a"  (standard date/time string)
    assert!(codec::decode(&[0xc0, 0x61, 0x61]).is_err());
}

#[test]
fn rejects_integer_map_key() {
    // {1: 2}
    assert!(codec::decode(&[0xa1, 0x01, 0x02]).is_err());
}

#[test]
fn rejects_duplicate_keys() {
    // {"a": 1, "a": 2}
    assert!(codec::decode(&[0xa2, 0x61, 0x61, 0x01, 0x61, 0x61, 0x02]).is_err());
}

#[test]
fn rejects_unsorted_keys() {
    // {"b": 1, "a": 2}  -- canonical order is "a" then "b"
    assert!(codec::decode(&[0xa2, 0x61, 0x62, 0x01, 0x61, 0x61, 0x02]).is_err());
    // {"aa": 1, "b": 2} -- length-first ordering puts "b" before "aa"
    assert!(codec::decode(&[0xa2, 0x62, 0x61, 0x61, 0x01, 0x61, 0x62, 0x02]).is_err());
    assert!(codec::decode(&[0xa2, 0x61, 0x62, 0x02, 0x62, 0x61, 0x61, 0x01]).is_ok());
}

#[test]
fn rejects_undefined_and_unassigned_simple() {
    assert!(codec::decode(&[0xf7]).is_err()); // undefined
    assert!(codec::decode(&[0xf0]).is_err()); // simple(16)
}

#[test]
fn length_first_key_order_matches_rfc8949() {
    let v = Value::map([
        ("zz", Value::Uint(1)),
        ("a", Value::Uint(2)),
        ("bbb", Value::Uint(3)),
        ("b", Value::Uint(4)),
    ]);
    let enc = codec::encode(&v);
    // Expect keys in order: "a", "b", "zz", "bbb"
    let expect = vec![
        0xa4, // map(4)
        0x61, b'a', 0x02, //
        0x61, b'b', 0x04, //
        0x62, b'z', b'z', 0x01, //
        0x63, b'b', b'b', b'b', 0x03,
    ];
    assert_eq!(enc, expect);
}

#[test]
fn json_view_escapes_dollar_keys() {
    let v = Value::map([("$b64", Value::text("not bytes"))]);
    let s = json::to_json_string(&v);
    assert!(s.contains("$$b64"), "got {s}");
    assert_eq!(json::from_json_str(&s).unwrap(), v);
}

#[test]
fn json_view_large_integers() {
    let v = Value::Uint(u64::MAX);
    let s = json::to_json_string(&v);
    assert_eq!(s, r#"{"$u64":"18446744073709551615"}"#);
    assert_eq!(json::from_json_str(&s).unwrap(), v);

    let v = Value::Nint(u64::MAX); // -(2^64)
    let s = json::to_json_string(&v);
    assert_eq!(s, r#"{"$n64":"-18446744073709551616"}"#);
    assert_eq!(json::from_json_str(&s).unwrap(), v);
}

#[test]
fn json_view_rejects_floats_and_bad_markers() {
    assert!(json::from_json_str("1.5").is_err());
    assert!(json::from_json_str(r#"{"$b64": 3}"#).is_err());
    assert!(json::from_json_str(r#"{"$u64": "1"}"#).is_err()); // in safe range
    assert!(json::from_json_str(r#"{"$nope": 1}"#).is_err()); // unknown marker
}

#[test]
fn depth_limit_enforced() {
    // Build 128 nested arrays; decoding must fail rather than blow the stack.
    let mut bytes = vec![0x81u8; 128];
    bytes.push(0x00);
    assert!(matches!(
        codec::decode(&bytes),
        Err(duap_canon::CanonError::DepthLimit { .. })
    ));
}

#[test]
fn hostile_length_header_does_not_allocate() {
    // array(2^32-1) with no payload: must fail immediately on the bound check.
    let bytes = [0x9a, 0xff, 0xff, 0xff, 0xff];
    assert!(codec::decode(&bytes).is_err());
}

#[test]
fn serde_bridge_roundtrip() {
    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct Ev {
        org: String,
        n: u64,
        tags: Vec<String>,
    }
    let e = Ev {
        org: "org:acme".into(),
        n: 7,
        tags: vec!["a".into(), "b".into()],
    };
    let b = duap_canon::to_canonical_cbor(&e).unwrap();
    assert!(codec::is_canonical(&b));
    let back: Ev = duap_canon::from_canonical_cbor(&b).unwrap();
    assert_eq!(back, e);
}

#[test]
fn serde_bridge_rejects_floats() {
    #[derive(serde::Serialize)]
    struct HasFloat {
        x: f64,
    }
    assert!(duap_canon::to_canonical_cbor(&HasFloat { x: 1.0 }).is_err());
}
