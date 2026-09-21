//! Adversarial and functional tests for the DUAP crypto layer.
//!
//! Each test names the threat it covers; see `THREAT_MODEL.md`.

use duap_canon::digest::{Digest, HashAlg};
use duap_crypto::*;
use proptest::prelude::*;

fn reg_with(pk: PublicKey, role: KeyRole) -> KeyRegistry {
    let mut r = KeyRegistry::new();
    r.enroll(KeyRecord::new(pk, "org:acme", vec![role], 0, None))
        .unwrap();
    r
}

fn ctx(now: u64) -> VerificationContext {
    VerificationContext::archival(now)
}

#[test]
fn sign_and_verify_all_suites() {
    for suite in SuiteId::ALL {
        let key = SecretKey::from_seed(suite, [5u8; 32]);
        let pk = key.public_key();
        assert_eq!(pk.bytes.len(), suite.public_key_len(), "{suite}");
        let reg = reg_with(pk.clone(), KeyRole::EventSigner);
        let mut env = Envelope::seal("duap.event.v1", &("payload", 1u64)).unwrap();
        let sig = env.sign(&key, 1_000, None).unwrap().clone();
        assert_eq!(sig.sig.len(), suite.signature_len(), "{suite}");
        let ok = env
            .verify(&reg, &SuitePolicy::draft_default(), &ctx(2_000))
            .unwrap();
        assert_eq!(ok, vec![key.key_id()], "{suite}");
    }
}

#[test]
fn deterministic_signatures_are_reproducible() {
    for suite in SuiteId::ALL {
        let k1 = SecretKey::from_seed(suite, [9u8; 32]);
        let k2 = SecretKey::from_seed(suite, [9u8; 32]);
        let input = b"the same input";
        assert_eq!(
            k1.sign(input, SIG_CONTEXT).unwrap(),
            k2.sign(input, SIG_CONTEXT).unwrap(),
            "{suite} signing must be deterministic for reproducible vectors"
        );
    }
}

/// T-03 (forged receipt): flipping any payload byte must break verification.
#[test]
fn payload_tampering_detected() {
    let key = SecretKey::from_seed(SuiteId::Ed25519, [1u8; 32]);
    let reg = reg_with(key.public_key(), KeyRole::EventSigner);
    let mut env = Envelope::seal("duap.event.v1", &("payload", 1u64)).unwrap();
    env.sign(&key, 1_000, None).unwrap();
    for i in 0..env.payload.len() {
        let mut bad = env.clone();
        bad.payload[i] ^= 0x01;
        assert!(
            bad.verify(&reg, &SuitePolicy::draft_default(), &ctx(2_000))
                .is_err(),
            "byte {i} flip went undetected"
        );
    }
}

/// T-04 (type confusion): a signature over one domain must not verify under
/// another, even with identical payload bytes.
#[test]
fn domain_substitution_detected() {
    let key = SecretKey::from_seed(SuiteId::Ed25519, [2u8; 32]);
    let reg = reg_with(key.public_key(), KeyRole::EventSigner);
    let mut env = Envelope::seal("duap.authorization.v1", &("payload", 1u64)).unwrap();
    env.sign(&key, 1_000, None).unwrap();

    let mut swapped = env.clone();
    swapped.domain = "duap.receipt.v1".to_owned();
    assert!(
        swapped
            .verify(&reg, &SuitePolicy::draft_default(), &ctx(2_000))
            .is_err(),
        "a signature must not survive a change of payload domain"
    );
}

/// T-05 (algorithm substitution): claiming a different suite in the signature
/// must fail, and the key id check must catch a swapped key.
#[test]
fn suite_and_key_substitution_detected() {
    let key = SecretKey::from_seed(SuiteId::Ed25519, [3u8; 32]);
    let other = SecretKey::from_seed(SuiteId::Ed25519, [4u8; 32]);
    let reg = {
        let mut r = KeyRegistry::new();
        r.enroll(KeyRecord::new(
            key.public_key(),
            "org:a",
            vec![KeyRole::EventSigner],
            0,
            None,
        ))
        .unwrap();
        r.enroll(KeyRecord::new(
            other.public_key(),
            "org:b",
            vec![KeyRole::EventSigner],
            0,
            None,
        ))
        .unwrap();
        r
    };
    let mut env = Envelope::seal("duap.event.v1", &"x").unwrap();
    env.sign(&key, 1_000, None).unwrap();

    // Re-attribute the signature to the other key id.
    let mut swapped = env.clone();
    swapped.signatures[0].kid = other.key_id();
    assert!(
        swapped
            .verify(&reg, &SuitePolicy::draft_default(), &ctx(2_000))
            .is_err()
    );

    // Claim a different suite.
    let mut relabeled = env.clone();
    relabeled.signatures[0].suite = SuiteId::MlDsa44;
    assert!(
        relabeled
            .verify(&reg, &SuitePolicy::draft_default(), &ctx(2_000))
            .is_err()
    );
}

/// The key id is a hash of the key: it cannot be chosen by the enroller.
#[test]
fn key_ids_are_self_certifying() {
    let a = SecretKey::from_seed(SuiteId::Ed25519, [10u8; 32]);
    let b = SecretKey::from_seed(SuiteId::Ed25519, [11u8; 32]);
    let mut bad = KeyRecord::new(a.public_key(), "org:a", vec![KeyRole::EventSigner], 0, None);
    bad.kid = b.key_id();
    let mut reg = KeyRegistry::new();
    assert!(matches!(reg.enroll(bad), Err(CryptoError::KeyIdMismatch)));
}

/// Hybrid signatures must fail if either half is broken.
#[test]
fn hybrid_requires_both_halves() {
    let key = SecretKey::from_seed(SuiteId::Ed25519MlDsa44, [12u8; 32]);
    let pk = key.public_key();
    let mut env = Envelope::seal("duap.event.v1", &"x").unwrap();
    env.sign(&key, 1_000, None).unwrap();
    let good = env.signatures[0].clone();
    assert!(env.verify_with_key(&good, &pk).is_ok());

    // Corrupt the classical half.
    let mut s = good.clone();
    s.sig[3] ^= 0xff;
    assert!(env.verify_with_key(&s, &pk).is_err());

    // Corrupt the post-quantum half.
    let mut s = good.clone();
    let n = s.sig.len();
    s.sig[n - 5] ^= 0xff;
    assert!(env.verify_with_key(&s, &pk).is_err());

    // Truncating to just the classical signature must fail on length.
    let mut s = good.clone();
    s.sig.truncate(64);
    assert!(env.verify_with_key(&s, &pk).is_err());
}

#[test]
fn revocation_for_compromise_invalidates_later_signatures() {
    let key = SecretKey::from_seed(SuiteId::Ed25519, [20u8; 32]);
    let mut reg = reg_with(key.public_key(), KeyRole::EventSigner);

    let mut early = Envelope::seal("duap.event.v1", &"early").unwrap();
    early.sign(&key, 1_000, None).unwrap();
    let mut late = Envelope::seal("duap.event.v1", &"late").unwrap();
    late.sign(&key, 9_000, None).unwrap();

    reg.revoke(
        &key.key_id(),
        Revocation {
            reason: RevocationReason::Compromise,
            declared_at: 10_000,
            effective_from: 5_000,
            note: Some("laptop stolen".into()),
        },
    )
    .unwrap();

    let p = SuitePolicy::draft_default();
    assert!(
        early.verify(&reg, &p, &ctx(20_000)).is_ok(),
        "signatures before the compromise window stay valid"
    );
    assert!(
        late.verify(&reg, &p, &ctx(20_000)).is_err(),
        "signatures after the compromise window must be rejected"
    );
}

#[test]
fn unknown_compromise_time_invalidates_everything() {
    let key = SecretKey::from_seed(SuiteId::Ed25519, [21u8; 32]);
    let mut reg = KeyRegistry::new();
    reg.enroll(KeyRecord::new(
        key.public_key(),
        "org:a",
        vec![KeyRole::EventSigner],
        1_000,
        None,
    ))
    .unwrap();
    let mut env = Envelope::seal("duap.event.v1", &"x").unwrap();
    env.sign(&key, 2_000, None).unwrap();
    reg.revoke(
        &key.key_id(),
        Revocation {
            reason: RevocationReason::Compromise,
            declared_at: 9_000,
            effective_from: 1_000, // == not_before: full revocation
            note: None,
        },
    )
    .unwrap();
    assert!(
        env.verify(&reg, &SuitePolicy::draft_default(), &ctx(20_000))
            .is_err()
    );
}

#[test]
fn rotation_preserves_earlier_signatures() {
    let old = SecretKey::from_seed(SuiteId::Ed25519, [30u8; 32]);
    let new = SecretKey::from_seed(SuiteId::Ed25519, [31u8; 32]);
    let mut reg = KeyRegistry::new();
    for k in [&old, &new] {
        reg.enroll(KeyRecord::new(
            k.public_key(),
            "org:a",
            vec![KeyRole::EventSigner],
            0,
            None,
        ))
        .unwrap();
    }
    let mut env = Envelope::seal("duap.event.v1", &"x").unwrap();
    env.sign(&old, 1_000, None).unwrap();
    reg.rotate(&old.key_id(), &new.key_id(), 5_000).unwrap();
    assert!(
        env.verify(&reg, &SuitePolicy::draft_default(), &ctx(9_000))
            .is_ok()
    );
    assert_eq!(
        reg.get(&new.key_id()).unwrap().supersedes,
        Some(old.key_id())
    );

    let mut after = Envelope::seal("duap.event.v1", &"y").unwrap();
    after.sign(&old, 6_000, None).unwrap();
    assert!(
        after
            .verify(&reg, &SuitePolicy::draft_default(), &ctx(9_000))
            .is_err()
    );
}

#[test]
fn policy_can_refuse_classical_suites() {
    let key = SecretKey::from_seed(SuiteId::Ed25519, [40u8; 32]);
    let reg = reg_with(key.public_key(), KeyRole::EventSigner);
    let mut env = Envelope::seal("duap.event.v1", &"x").unwrap();
    env.sign(&key, 1_000, None).unwrap();
    assert!(matches!(
        env.verify(&reg, &SuitePolicy::pq_only(), &ctx(2_000)),
        Err(CryptoError::SuiteRejected { .. })
    ));
}

#[test]
fn future_dated_signatures_rejected_on_live_ingest() {
    let key = SecretKey::from_seed(SuiteId::Ed25519, [41u8; 32]);
    let reg = reg_with(key.public_key(), KeyRole::EventSigner);
    let now = 1_000_000_000_000u64;
    let mut env = Envelope::seal("duap.event.v1", &"x").unwrap();
    env.sign(&key, now + 3_600_000_000, None).unwrap(); // one hour ahead
    assert!(matches!(
        env.verify(
            &reg,
            &SuitePolicy::draft_default(),
            &VerificationContext::live(now)
        ),
        Err(CryptoError::ClockSkew { .. })
    ));
}

#[test]
fn multiple_signatures_are_independent() {
    let a = SecretKey::from_seed(SuiteId::Ed25519, [50u8; 32]);
    let b = SecretKey::from_seed(SuiteId::Ed25519MlDsa44, [51u8; 32]);
    let mut reg = KeyRegistry::new();
    reg.enroll(KeyRecord::new(
        a.public_key(),
        "org:a",
        vec![KeyRole::EventSigner],
        0,
        None,
    ))
    .unwrap();
    reg.enroll(KeyRecord::new(
        b.public_key(),
        "org:b",
        vec![KeyRole::ReceiptSigner],
        0,
        None,
    ))
    .unwrap();
    let mut env = Envelope::seal("duap.event.v1", &"x").unwrap();
    env.sign(&a, 1_000, None).unwrap();
    env.sign(&b, 1_100, None).unwrap();
    let ok = env
        .verify(&reg, &SuitePolicy::draft_default(), &ctx(2_000))
        .unwrap();
    assert_eq!(ok.len(), 2);

    // Break one: the other still verifies, and the caller sees which.
    let mut broken = env.clone();
    broken.signatures[0].sig[0] ^= 0xff;
    let ok = broken
        .verify(&reg, &SuitePolicy::draft_default(), &ctx(2_000))
        .unwrap();
    assert_eq!(ok, vec![b.key_id()]);
}

#[test]
fn nonce_is_bound_to_the_signature() {
    let key = SecretKey::from_seed(SuiteId::Ed25519, [60u8; 32]);
    let pk = key.public_key();
    let mut env = Envelope::seal("duap.event.v1", &"x").unwrap();
    env.sign(&key, 1_000, Some(vec![1, 2, 3])).unwrap();
    let mut s = env.signatures[0].clone();
    s.nonce = Some(vec![9, 9, 9]);
    assert!(env.verify_with_key(&s, &pk).is_err());
    s.nonce = None;
    assert!(env.verify_with_key(&s, &pk).is_err());
}

#[test]
fn envelope_rejects_non_canonical_payload() {
    // 0x18 0x05 is a non-canonical encoding of uint(5).
    assert!(Envelope::from_payload_bytes("duap.event.v1", vec![0x18, 0x05]).is_err());
}

#[test]
fn registry_state_digest_changes_with_content() {
    let a = SecretKey::from_seed(SuiteId::Ed25519, [70u8; 32]);
    let mut reg = KeyRegistry::new();
    let d0 = reg.state_digest().unwrap();
    reg.enroll(KeyRecord::new(
        a.public_key(),
        "org:a",
        vec![KeyRole::EventSigner],
        0,
        None,
    ))
    .unwrap();
    let d1 = reg.state_digest().unwrap();
    assert_ne!(d0, d1);
}

#[test]
fn signing_input_is_domain_separated_from_raw_payload() {
    // A signature must not be reusable as a signature over the payload bytes
    // themselves under a different protocol that signs raw canonical CBOR.
    let key = SecretKey::from_seed(SuiteId::Ed25519, [80u8; 32]);
    let mut env = Envelope::seal("duap.event.v1", &"x").unwrap();
    env.sign(&key, 1_000, None).unwrap();
    let pk = key.public_key();
    assert!(
        pk.verify(&env.payload, &env.signatures[0].sig, SIG_CONTEXT)
            .is_err()
    );
    let raw_digest = Digest::of(HashAlg::Sha2_256, "duap.event.v1", &env.payload);
    assert!(
        pk.verify(&raw_digest.bytes, &env.signatures[0].sig, SIG_CONTEXT)
            .is_err()
    );
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    /// Any perturbation of the signature bytes must be rejected.
    #[test]
    fn signature_bitflips_rejected(idx in 0usize..64, bit in 0u8..8) {
        let key = SecretKey::from_seed(SuiteId::Ed25519, [90u8; 32]);
        let pk = key.public_key();
        let mut env = Envelope::seal("duap.event.v1", &"x").unwrap();
        env.sign(&key, 1_000, None).unwrap();
        let mut s = env.signatures[0].clone();
        s.sig[idx] ^= 1 << bit;
        prop_assert!(env.verify_with_key(&s, &pk).is_err());
    }

    /// Distinct seeds yield distinct key identifiers.
    #[test]
    fn distinct_seeds_distinct_kids(a in any::<[u8; 32]>(), b in any::<[u8; 32]>()) {
        prop_assume!(a != b);
        let ka = SecretKey::from_seed(SuiteId::Ed25519, a);
        let kb = SecretKey::from_seed(SuiteId::Ed25519, b);
        prop_assert_ne!(ka.key_id(), kb.key_id());
    }

    /// Envelope round-trips through canonical CBOR.
    #[test]
    fn envelope_roundtrip(msg in ".{0,64}") {
        let key = SecretKey::from_seed(SuiteId::Ed25519, [91u8; 32]);
        let mut env = Envelope::seal("duap.event.v1", &msg).unwrap();
        env.sign(&key, 1_000, None).unwrap();
        let bytes = duap_canon::to_canonical_cbor(&env).unwrap();
        let back: Envelope = duap_canon::from_canonical_cbor(&bytes).unwrap();
        prop_assert_eq!(&back, &env);
        prop_assert_eq!(back.open::<String>().unwrap(), msg);
    }
}
