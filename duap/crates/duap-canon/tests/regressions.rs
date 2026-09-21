//! Regression tests for defects found by fuzzing.
//!
//! Each test names the finding it pins and reproduces the input in source
//! rather than as a committed blob, because a blob nobody can read is not
//! evidence. `fuzz/` holds the targets; `security/findings.md` holds the
//! findings; `benchmarks/results/2026-09-21-fuzzing.md` holds the campaign
//! these came from.
//!
//! Where the fuzzer found one input, the test generalises to the class:
//! the point is to pin the defect, not the byte string that happened to
//! surface it.

/// FUZZ-01. A 64-bit length header is entirely attacker-controlled. `0x7b`
/// (text string, 8-byte length) followed by `u64::MAX` made the decoder
/// compute `pos + n`, which overflows: a panic with debug assertions, a
/// wrapped comparison without them. Nine bytes either way.
///
/// The decoder must reject every such header as a short read, for every
/// major type that carries a length.
#[test]
fn a_64_bit_length_header_cannot_overflow_the_bounds_check() {
    // The exact input libFuzzer produced, plus the same shape for each
    // length-carrying major type: byte string (0x5b), text string (0x7b),
    // array (0x9b) and map (0xbb).
    for head in [0x5bu8, 0x7b, 0x9b, 0xbb] {
        let mut input = vec![head];
        input.extend_from_slice(&u64::MAX.to_be_bytes());
        input.push(0xa0); // trailing byte, as in the original finding

        let err = duap_canon::codec::decode(&input)
            .expect_err("a length of u64::MAX must be rejected, not decoded");
        // The specific error matters less than that it is an error and not
        // a panic, but it should describe a short read or an oversized
        // collection rather than something unrelated.
        let text = err.to_string();
        assert!(
            text.contains("end of input") || text.contains("too large") || text.contains("EOF"),
            "major type header {head:#04x} gave an unexpected error: {text}"
        );
    }
}

/// The same class, one below the boundary: a length that fits in `usize`
/// but exceeds the input. This is the ordinary short-read case and must
/// stay an error rather than an allocation.
#[test]
fn a_length_beyond_the_input_is_a_short_read_not_an_allocation() {
    let mut input = vec![0x5b]; // byte string, 8-byte length
    input.extend_from_slice(&(1u64 << 40).to_be_bytes()); // 1 TiB
    assert!(
        duap_canon::codec::decode(&input).is_err(),
        "a 1 TiB length in a 9-byte input must be rejected"
    );
}
