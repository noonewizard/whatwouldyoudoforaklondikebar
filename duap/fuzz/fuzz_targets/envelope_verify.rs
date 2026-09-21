//! Signature verification against arbitrary bytes.
//!
//! Verification runs before anything else trusts an event, so it is the
//! first code an attacker reaches. The property here is simple and
//! absolute: **verification never panics and never succeeds on input the
//! attacker chose**. A forged envelope must be rejected, not crash the
//! node and not verify.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(value) = duap_canon::codec::decode(data) else {
        return;
    };
    let Ok(envelope) = duap_canon::from_value::<duap_crypto::Envelope>(&value) else {
        return;
    };

    // A key the fuzzer does not control. If verification ever succeeds
    // against it, the fuzzer has forged a signature.
    let key = duap_crypto::SecretKey::from_seed(duap_crypto::SuiteId::Ed25519, [0x5A; 32]);
    let mut registry = duap_crypto::KeyRegistry::new();
    registry
        .enroll(duap_crypto::KeyRecord::new(
            key.public_key(),
            "org:duap/fuzz",
            vec![duap_crypto::KeyRole::EventSigner],
            0,
            None,
        ))
        .expect("enrolling a fresh key");

    let verified = envelope.verify(
        &registry,
        &duap_crypto::SuitePolicy::draft_default(),
        &duap_crypto::VerificationContext::archival(0),
    );
    assert!(
        verified.is_err(),
        "verification succeeded on fuzzer-supplied bytes: a forgery"
    );
});
