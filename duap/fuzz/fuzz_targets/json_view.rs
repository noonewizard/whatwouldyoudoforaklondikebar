//! The JSON view, in both directions.
//!
//! The JSON view exists so implementations without a CBOR library can read
//! DUAP objects, which makes it a second parser on the trust path and
//! therefore a second place canonical form can break. VS-4 was found here:
//! the view required arbitrary-precision arithmetic for one value no DUAP
//! object uses, which every implementation would have had to carry.
//!
//! The property: a value that survives the JSON view must be the same
//! value, and must re-encode to the same canonical bytes.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Direction 1: arbitrary bytes as canonical CBOR, then through JSON.
    if let Ok(value) = duap_canon::codec::decode(data) {
        let json = duap_canon::json::to_json(&value);
        let text = serde_json::to_string(&json).expect("the view serialises");
        let parsed: serde_json::Value = serde_json::from_str(&text).expect("and parses");
        let back = duap_canon::json::from_json(&parsed).expect("a view of a valid value converts back");
        assert_eq!(back, value, "the JSON view did not preserve the value");

        let bytes = duap_canon::codec::encode(&back);
        assert_eq!(
            bytes.as_slice(),
            data,
            "a round trip through the JSON view changed the canonical bytes"
        );
    }

    // Direction 2: arbitrary text as a JSON view. Must not panic.
    if let Ok(text) = std::str::from_utf8(data) {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(text) {
            if let Ok(value) = duap_canon::json::from_json(&parsed) {
                // Anything the view accepts must be encodable.
                let _ = duap_canon::codec::encode(&value);
            }
        }
    }
});
