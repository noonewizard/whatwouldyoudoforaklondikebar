//! Merkle proof verification against arbitrary proofs.
//!
//! A verifier holds a root and a proof it was handed by a party it does
//! not trust. Two properties: verification must not panic on a malformed
//! proof, however deep or wide its path; and a proof must not verify
//! against a root the fuzzer did not legitimately produce.
//!
//! The second property is what stops a log operator convincing a verifier
//! that an entry it never logged is in the tree.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(proof) = serde_json::from_slice::<duap_provenance::InclusionProof>(data) else {
        return;
    };

    // A leaf and a root the fuzzer does not control.
    let leaf = duap_canon::Digest::of(duap_canon::HashAlg::Sha2_256, "fuzz", b"leaf");
    let root = duap_canon::Digest::of(duap_canon::HashAlg::Sha2_256, "fuzz", b"root");

    // Must terminate and must not panic, whatever the path length claims.
    let ok = proof.verify(leaf, root);
    assert!(
        !ok,
        "a fuzzer-supplied proof verified against an unrelated root"
    );
});
