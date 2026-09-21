//! Tests for the DUAP object model.

use duap_canon::Value;
use duap_model::prelude::*;
use duap_model::taxonomy;
use proptest::prelude::*;

fn auth_ref() -> AuthorizationRef {
    AuthorizationRef {
        grant: GrantId([1u8; 16]),
        grant_digest: duap_canon::Digest::of(
            duap_canon::HashAlg::Sha2_256,
            "duap.authorization.v1",
            b"grant",
        ),
        epoch: 1,
    }
}

fn base_builder(class: DataClass, op: Operation, purpose: Purpose) -> EventBuilder {
    let controller: OrgId = "org:duap/acme".parse().unwrap();
    EventBuilder::new(
        EventId([2u8; 16]),
        AgentRef::new("duap-sdk-rust", "0.1.0"),
        controller,
        SubjectScope::Subject {
            subject: SubjectRef([3u8; 16]),
        },
        Jurisdiction::new("DE")
            .unwrap()
            .with_regimes([Regime::EuGdpr]),
        class,
        op,
        purpose,
        auth_ref(),
        1,
        Timestamp::from_secs(1_750_000_000),
    )
}

// ---------------------------------------------------------------------------
// Taxonomy
// ---------------------------------------------------------------------------

#[test]
fn purpose_lattice_is_acyclic_and_rooted() {
    for p in Purpose::ALL {
        let mut seen = vec![p];
        let mut cur = p.parent();
        while let Some(x) = cur {
            assert!(!seen.contains(&x), "cycle at {}", x.code());
            seen.push(x);
            cur = x.parent();
        }
        assert!(p.depth() < 8);
    }
}

#[test]
fn purpose_covers_is_reflexive_and_transitive() {
    for a in Purpose::ALL {
        assert!(a.covers(a));
    }
    assert!(Purpose::Marketing.covers(Purpose::MarketingAdvertisingBehavioral));
    assert!(Purpose::MarketingAdvertising.covers(Purpose::MarketingAdvertisingContextual));
    assert!(
        !Purpose::MarketingAdvertisingContextual.covers(Purpose::MarketingAdvertisingBehavioral)
    );
    assert!(!Purpose::Service.covers(Purpose::MarketingDirect));
}

#[test]
fn commercial_purposes_are_inherited_consistently() {
    // A child of a commercial purpose must itself be commercial: otherwise a
    // grant limited to non-commercial use could be widened by descending.
    for p in Purpose::ALL {
        if let Some(parent) = p.parent() {
            if parent.commercial() {
                assert!(
                    p.commercial(),
                    "{} is non-commercial under commercial parent {}",
                    p.code(),
                    parent.code()
                );
            }
        }
    }
}

#[test]
fn every_code_round_trips() {
    for d in DataClass::ALL {
        assert_eq!(DataClass::parse(d.code()).unwrap(), d);
    }
    for o in Operation::ALL {
        assert_eq!(Operation::parse(o.code()).unwrap(), o);
    }
    for p in Purpose::ALL {
        assert_eq!(Purpose::parse(p.code()).unwrap(), p);
    }
    for u in Unit::ALL {
        assert_eq!(Unit::parse(u.code()).unwrap(), u);
    }
    assert!(DataClass::parse("nope.nope").is_err());
}

#[test]
fn special_category_classes_are_high_tier() {
    for d in DataClass::ALL {
        if d.is_special_category() {
            assert!(
                d.sensitivity().rank() >= 3,
                "{} is a special category at tier {}",
                d.code(),
                d.sensitivity().code()
            );
        }
    }
}

#[test]
fn attribution_share_unit_is_not_additive() {
    assert!(!Unit::Share.additive());
    for u in Unit::ALL {
        if u != Unit::Share {
            assert!(u.additive(), "{} should be additive", u.code());
        }
    }
}

#[test]
fn ontology_hash_is_recorded() {
    assert_eq!(taxonomy::ONTOLOGY_SHA256.len(), 64);
    assert!(taxonomy::ONTOLOGY_VERSION.starts_with('1'));
}

// ---------------------------------------------------------------------------
// Identifiers
// ---------------------------------------------------------------------------

#[test]
fn org_id_syntax() {
    assert!(OrgId::new("org:duap/acme").is_ok());
    assert!(OrgId::new("org:did/did:web:acme.example").is_ok());
    assert!(OrgId::new("acme").is_err());
    assert!(OrgId::new("org:acme").is_err());
    assert!(OrgId::new("org:/acme").is_err());
    assert!(OrgId::new("org:DUAP/acme").is_err());
    assert!(OrgId::new("org:duap/with space").is_err());
    assert_eq!(
        OrgId::new("org:lei/5493001KJTIIGC8Y1R12")
            .unwrap()
            .authority(),
        "lei"
    );
}

#[test]
fn ids_round_trip_through_text() {
    let e = EventId([9u8; 16]);
    assert_eq!(e.to_string().parse::<EventId>().unwrap(), e);
    assert!(
        "gr1:00".parse::<EventId>().is_err(),
        "prefix must match the type"
    );
}

#[test]
fn content_ids_are_stable_and_type_separated() {
    let a = ContentId::of_bytes("duap.object.v1", b"same bytes");
    let b = ContentId::of_bytes("duap.object.v1", b"same bytes");
    let c = ContentId::of_bytes("duap.dataset.v1", b"same bytes");
    assert_eq!(a, b);
    assert_ne!(a, c);
}

// ---------------------------------------------------------------------------
// Pseudonyms
// ---------------------------------------------------------------------------

#[test]
fn pseudonyms_differ_per_controller_and_are_stable() {
    let root = SubjectRoot::from_secret([5u8; 32]);
    let a: OrgId = "org:duap/acme".parse().unwrap();
    let b: OrgId = "org:duap/globex".parse().unwrap();
    assert_eq!(root.pseudonym_for(&a), root.pseudonym_for(&a));
    assert_ne!(root.pseudonym_for(&a), root.pseudonym_for(&b));

    let other = SubjectRoot::from_secret([6u8; 32]);
    assert_ne!(root.pseudonym_for(&a), other.pseudonym_for(&a));
}

#[test]
fn subject_keys_are_separate_from_pseudonyms() {
    let root = SubjectRoot::from_secret([5u8; 32]);
    let a: OrgId = "org:duap/acme".parse().unwrap();
    let key = root.key_for(&a, duap_crypto::SuiteId::Ed25519);
    // The key id must not equal the pseudonym: leaking one must not leak the
    // other.
    assert_ne!(&key.key_id().0[..], &root.pseudonym_for(&a).0[..]);
}

// ---------------------------------------------------------------------------
// Commitments
// ---------------------------------------------------------------------------

#[test]
fn commitments_bind_and_hide() {
    let (c, o) = Commitment::commit(b"52.5200,13.4050").unwrap();
    assert!(c.verify(b"52.5200,13.4050", &o));
    assert!(!c.verify(b"52.5200,13.4051", &o));
    // A different salt on the same value gives a different commitment, which
    // is what stops an observer from recognising a repeated low-entropy value.
    let (c2, _) = Commitment::commit(b"52.5200,13.4050").unwrap();
    assert_ne!(c, c2);
}

#[test]
fn commitment_concatenation_is_injective() {
    // Without length prefixing, salt||value could be re-split. Check that the
    // obvious re-split attempt gives a different commitment.
    let c1 = Commitment::with_salt(b"BC", [0xAAu8; 32]);
    let c2 = Commitment::with_salt(b"C", [0xAAu8; 32]);
    assert_ne!(c1, c2);
}

// ---------------------------------------------------------------------------
// Money
// ---------------------------------------------------------------------------

#[test]
fn money_formats_by_currency_exponent() {
    assert_eq!(Money::new(Currency::USD, 12345).to_string(), "123.45 USD");
    assert_eq!(Money::new(Currency::JPY, 12345).to_string(), "12345 JPY");
    assert_eq!(Money::new(Currency::BHD, 12345).to_string(), "12.345 BHD");
    assert_eq!(Money::new(Currency::USD, -5).to_string(), "-0.05 USD");
}

#[test]
fn money_refuses_cross_currency_arithmetic() {
    let a = Money::new(Currency::USD, 100);
    let b = Money::new(Currency::EUR, 100);
    assert!(a.add(&b).is_err());
}

#[test]
fn rounding_conserves_value() {
    for nmu in [
        -2_500_000_000i128,
        -1,
        0,
        1,
        499_999_999,
        500_000_000,
        1_500_000_000,
        999_999_999_999,
    ] {
        let p = Precise::new(Currency::USD, nmu);
        for mode in [
            Rounding::HalfUp,
            Rounding::HalfEven,
            Rounding::Floor,
            Rounding::Ceil,
            Rounding::TowardZero,
        ] {
            let (m, residue) = p.round_to_money(mode);
            assert_eq!(
                m.minor * NANO + residue.nmu,
                nmu,
                "value not conserved for {nmu} under {mode:?}"
            );
            assert!(residue.nmu.abs() < NANO);
        }
    }
}

#[test]
fn half_even_is_unbiased_where_half_up_is_not() {
    // 0.5, 1.5, 2.5, 3.5 minor units.
    let halves: Vec<i128> = (0..4).map(|k| k * NANO + NANO / 2).collect();
    let he: i128 = halves
        .iter()
        .map(|n| {
            Precise::new(Currency::USD, *n)
                .round_to_money(Rounding::HalfEven)
                .0
                .minor
        })
        .sum();
    let hu: i128 = halves
        .iter()
        .map(|n| {
            Precise::new(Currency::USD, *n)
                .round_to_money(Rounding::HalfUp)
                .0
                .minor
        })
        .sum();
    let exact_doubled: i128 = halves.iter().map(|n| n * 2 / NANO).sum::<i128>();
    assert_eq!(he * 2, exact_doubled, "half-even should not drift");
    assert!(hu * 2 > exact_doubled, "half-up drifts upward");
}

#[test]
fn toward_zero_never_overcharges() {
    for nmu in [1, 999_999_999, 1_000_000_001] {
        let (m, _) = Precise::new(Currency::USD, nmu).round_to_money(Rounding::TowardZero);
        assert!(m.minor * NANO <= nmu);
    }
}

#[test]
fn ratios_normalise() {
    assert_eq!(Ratio::new(2, 4).unwrap(), Ratio::new(1, 2).unwrap());
    assert_eq!(Ratio::new(-6, 3).unwrap(), Ratio::new(-2, 1).unwrap());
    assert_eq!(Ratio::from_bps(2500), Ratio::new(1, 4).unwrap());
    assert!(Ratio::new(1, 0).is_err());
    assert_eq!("3/4".parse::<Ratio>().unwrap(), Ratio::new(3, 4).unwrap());
    assert_eq!("5".parse::<Ratio>().unwrap(), Ratio::new(5, 1).unwrap());
}

#[test]
fn ratio_canonical_encoding_is_unique_for_equal_values() {
    let a = Ratio::new(2, 4).unwrap();
    let b = Ratio::new(50, 100).unwrap();
    assert_eq!(
        duap_canon::to_canonical_cbor(&a).unwrap(),
        duap_canon::to_canonical_cbor(&b).unwrap()
    );
}

// ---------------------------------------------------------------------------
// Time
// ---------------------------------------------------------------------------

#[test]
fn timestamps_round_trip_rfc3339() {
    let t = Timestamp(1_750_000_000_123_456);
    let s = t.to_rfc3339();
    assert_eq!(Timestamp::parse_rfc3339(&s).unwrap(), t);
    assert_eq!(s.parse::<Timestamp>().unwrap(), t);
    assert_eq!("@123".parse::<Timestamp>().unwrap(), Timestamp(123));
}

#[test]
fn day_truncation_is_idempotent() {
    let t = Timestamp(1_750_000_000_123_456);
    let d = t.truncate_day();
    assert_eq!(d.truncate_day(), d);
    assert!(d <= t && t.since(d) < DAY);
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

#[test]
fn builder_fills_derivable_fields() {
    let ev = base_builder(
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    )
    .build()
    .unwrap();
    assert_eq!(ev.quantity.unit, Unit::Query);
    assert_eq!(ev.sensitivity, DataClass::LocationCoarse.sensitivity());
    assert_eq!(ev.recorded_at, ev.occurred_at);
}

#[test]
fn validation_rejects_unit_mismatch() {
    let mut ev = base_builder(
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    )
    .build()
    .unwrap();
    ev.quantity.unit = Unit::Token;
    assert!(ev.validate().is_err());
}

#[test]
fn validation_rejects_sensitivity_downgrade() {
    let mut ev = base_builder(
        DataClass::HealthClinical,
        Operation::StorePersist,
        Purpose::ServiceCore,
    )
    .build()
    .unwrap();
    ev.sensitivity = SensitivityTier::T1;
    assert!(
        ev.validate().is_err(),
        "a controller must not be able to relabel clinical data as low sensitivity"
    );
    ev.sensitivity = SensitivityTier::T4;
    assert!(ev.validate().is_ok());
}

#[test]
fn validation_requires_provenance_output_for_deriving_operations() {
    let b = base_builder(
        DataClass::BehaviorWebBrowsing,
        Operation::ProcessProfile,
        Purpose::MarketingAdvertisingBehavioral,
    );
    assert!(b.build().is_err());

    let ev = base_builder(
        DataClass::BehaviorWebBrowsing,
        Operation::ProcessProfile,
        Purpose::MarketingAdvertisingBehavioral,
    )
    .provenance(Provenance {
        output: Some(ContentId::of_bytes("duap.object.v1", b"profile")),
        ..Default::default()
    })
    .build();
    assert!(ev.is_ok());
}

#[test]
fn validation_requires_counterparty_for_external_transfer() {
    let b = base_builder(
        DataClass::TransactionPurchase,
        Operation::TransferSale,
        Purpose::CommerceSale,
    );
    assert!(b.build().is_err());

    let ev = base_builder(
        DataClass::TransactionPurchase,
        Operation::TransferSale,
        Purpose::CommerceSale,
    )
    .economics(EconomicContext {
        counterparty: Some("org:duap/broker".parse().unwrap()),
        ..Default::default()
    })
    .build();
    assert!(ev.is_ok());

    // Internal transfers need no counterparty.
    assert!(
        base_builder(
            DataClass::TransactionPurchase,
            Operation::TransferInternal,
            Purpose::OperationsReliability,
        )
        .build()
        .is_ok()
    );
}

#[test]
fn validation_rejects_non_personal_label_on_sensitive_class() {
    let controller: OrgId = "org:duap/acme".parse().unwrap();
    let ev = EventBuilder::new(
        EventId([2u8; 16]),
        AgentRef::new("a", "1"),
        controller,
        SubjectScope::NonPersonal,
        Jurisdiction::new("US").unwrap(),
        DataClass::BiometricTemplate,
        Operation::StorePersist,
        Purpose::ServiceCore,
        auth_ref(),
        1,
        Timestamp::from_secs(1),
    )
    .build();
    assert!(ev.is_err());
}

#[test]
fn validation_rejects_reserved_extension_namespace() {
    let ev = base_builder(
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    )
    .extension("duap.secret", Value::Uint(1))
    .build();
    assert!(ev.is_err());

    let ev = base_builder(
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    )
    .extension("nospace", Value::Uint(1))
    .build();
    assert!(ev.is_err());

    let ev = base_builder(
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    )
    .extension("acme.experiment_id", Value::text("A/B-17"))
    .build();
    assert!(ev.is_ok());
}

#[test]
fn events_round_trip_canonically_and_digest_is_stable() {
    let ev = base_builder(
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    )
    .build()
    .unwrap();
    let bytes = ev.to_canonical().unwrap();
    assert!(duap_canon::is_canonical(&bytes));
    let back = DataUsageEvent::from_canonical(&bytes).unwrap();
    assert_eq!(back, ev);
    assert_eq!(back.digest().unwrap(), ev.digest().unwrap());

    // Any field change changes the digest.
    let mut other = ev.clone();
    other.purpose = Purpose::MarketingDirect;
    assert_ne!(other.digest().unwrap(), ev.digest().unwrap());
}

#[test]
fn unknown_wire_fields_are_rejected() {
    let ev = base_builder(
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    )
    .build()
    .unwrap();
    let mut v = duap_canon::to_value(&ev).unwrap();
    if let Value::Map(m) = &mut v {
        m.insert("zz".into(), Value::Uint(1));
    }
    let bytes = duap_canon::encode(&v);
    assert!(
        DataUsageEvent::from_canonical(&bytes).is_err(),
        "an unknown field must not be silently ignored"
    );
}

#[test]
fn compact_wire_names_are_used() {
    let ev = base_builder(
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    )
    .build()
    .unwrap();
    let v = duap_canon::to_value(&ev).unwrap();
    let m = v.as_map().unwrap();
    for k in [
        "v", "id", "ts", "rt", "ag", "ct", "sb", "ju", "dc", "sn", "op", "pp", "az", "qy",
    ] {
        assert!(m.contains_key(k), "missing wire field {k}");
    }
    assert!(!m.contains_key("data_class"));
    // Absent optional fields must not be emitted: they would cost bytes and
    // change the digest for no reason.
    assert!(!m.contains_key("pr"));
    assert!(!m.contains_key("xt"));
}

#[test]
fn event_encoded_size_is_bounded() {
    let ev = base_builder(
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    )
    .build()
    .unwrap();
    let n = ev.to_canonical().unwrap().len();
    assert!(
        n < 400,
        "a minimal event should stay well under 400 bytes, got {n}"
    );
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn valid_events_always_round_trip(
        class_i in 0usize..DataClass::ALL.len(),
        op_i in 0usize..Operation::ALL.len(),
        purpose_i in 0usize..Purpose::ALL.len(),
        amount in 1u64..1_000_000,
    ) {
        let class = DataClass::ALL[class_i];
        let op = Operation::ALL[op_i];
        let purpose = Purpose::ALL[purpose_i];
        let mut b = EventBuilder::new(
            EventId([2u8; 16]),
            AgentRef::new("a", "1"),
            "org:duap/acme".parse().unwrap(),
            SubjectScope::Subject { subject: SubjectRef([3u8; 16]) },
            Jurisdiction::new("DE").unwrap(),
            class, op, purpose, auth_ref(), amount,
            Timestamp::from_secs(1_750_000_000),
        );
        if op.derives() {
            b = b.provenance(Provenance {
                output: Some(ContentId::of_bytes("duap.object.v1", b"o")),
                ..Default::default()
            });
        }
        b = b.economics(EconomicContext {
            counterparty: Some("org:duap/other".parse().unwrap()),
            ..Default::default()
        });
        let ev = b.build_unchecked();
        if ev.validate().is_ok() {
            let bytes = ev.to_canonical().unwrap();
            let back = DataUsageEvent::from_canonical(&bytes).unwrap();
            prop_assert_eq!(back, ev);
        }
    }

    #[test]
    fn money_addition_is_associative(a in -1_000_000i128..1_000_000, b in -1_000_000i128..1_000_000, c in -1_000_000i128..1_000_000) {
        let (x, y, z) = (Money::new(Currency::USD, a), Money::new(Currency::USD, b), Money::new(Currency::USD, c));
        prop_assert_eq!(x.add(&y).unwrap().add(&z).unwrap(), x.add(&y.add(&z).unwrap()).unwrap());
    }

    #[test]
    fn rounding_always_conserves(nmu in -1_000_000_000_000i128..1_000_000_000_000, mode in 0usize..5) {
        let mode = [Rounding::HalfUp, Rounding::HalfEven, Rounding::Floor, Rounding::Ceil, Rounding::TowardZero][mode];
        let p = Precise::new(Currency::EUR, nmu);
        let (m, r) = p.round_to_money(mode);
        prop_assert_eq!(m.minor * NANO + r.nmu, nmu);
    }

    #[test]
    fn pseudonyms_are_collision_free_over_controllers(n in 0usize..64) {
        let root = SubjectRoot::from_secret([11u8; 32]);
        let orgs: Vec<OrgId> = (0..=n).map(|i| format!("org:duap/o{i}").parse().unwrap()).collect();
        let mut seen = std::collections::BTreeSet::new();
        for o in &orgs {
            prop_assert!(seen.insert(root.pseudonym_for(o)));
        }
    }
}
