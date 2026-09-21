//! The decoder is the only place DUAP parses adversary-supplied bytes.
//!
//! This target asserts the property the whole protocol rests on: **the
//! strict decoder accepts exactly what the encoder emits**. Anything it
//! accepts must re-encode to the identical bytes. A byte string that
//! decodes successfully but re-encodes differently is a canonicalisation
//! break, and a canonicalisation break is a signature break -- two
//! implementations would compute different digests for what they both
//! consider the same value.
//!
//! Panicking on malformed input is also a finding: the decoder must return
//! an error, never unwind, because it runs on a network path.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(value) = duap_canon::codec::decode(data) else {
        // Rejecting arbitrary bytes is the expected outcome.
        return;
    };

    let reencoded = duap_canon::codec::encode(&value);

    assert_eq!(
        reencoded.as_slice(),
        data,
        "the decoder accepted bytes the encoder would not emit: \
         canonical form is not injective for this input"
    );

    // Re-decoding must reach the same value, or decode is not a function.
    let again = duap_canon::codec::decode(&reencoded).expect("re-encoded bytes decode");
    assert_eq!(again, value, "decode(encode(decode(x))) != decode(x)");
});
