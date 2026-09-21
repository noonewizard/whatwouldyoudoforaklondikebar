//! A `DataUsageEvent` decoded from arbitrary bytes.
//!
//! The event is the object a clearing node accepts from a counterparty, so
//! this is the deserialisation an attacker actually reaches. Two
//! properties: the decoder must not panic, and any event it accepts must
//! re-encode identically -- because the event's digest is what every
//! signature commits to, and a digest that depends on how the bytes were
//! spelled is not a digest.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(event) = duap_model::event::DataUsageEvent::from_canonical(data) else {
        return;
    };

    let reencoded = event.to_canonical().expect("an accepted event re-encodes");
    assert_eq!(
        reencoded.as_slice(),
        data,
        "an event decoded from bytes the encoder would not emit"
    );

    // The digest must be stable across a round trip, since it is what
    // signatures commit to.
    let d1 = event.digest().expect("digest");
    let again =
        duap_model::event::DataUsageEvent::from_canonical(&reencoded).expect("re-decode");
    let d2 = again.digest().expect("digest");
    assert_eq!(d1, d2, "event digest changed across a round trip");
});
