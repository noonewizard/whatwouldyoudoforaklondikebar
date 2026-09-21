//! Tests for the authorization layer.
//!
//! The invariants asserted here mirror `formal/Authorization.tla`:
//!
//!   INV-A1  A Deny term always wins over any set of Permit terms.
//!   INV-A2  No decision is Permit unless some Permit term matched and was
//!           not revoked, or the grant's default is Permit.
//!   INV-A3  Evaluation is a pure function of its inputs.
//!   INV-A4  A revocation effective at or before the event time suppresses
//!           the terms in its scope.
//!   INV-A5  Amending a grant cannot resurrect a revoked permission.
//!   INV-A6  A violated checkable obligation yields Deny, never Permit.

use duap_auth::prelude::*;
use duap_model::prelude::*;
use proptest::prelude::*;

const T0: u64 = 1_700_000_000;

fn subject() -> SubjectRef {
    SubjectRef([1u8; 16])
}

fn controller() -> OrgId {
    "org:duap/acme".parse().unwrap()
}

fn subject_key() -> duap_crypto::KeyId {
    duap_crypto::SecretKey::from_seed(duap_crypto::SuiteId::Ed25519, [2u8; 32]).key_id()
}

fn builder() -> GrantBuilder {
    GrantBuilder::new(
        GrantId([3u8; 16]),
        subject(),
        subject_key(),
        controller(),
        Timestamp::from_secs(T0),
        Currency::EUR,
    )
}

fn event_for(grant: &Grant, class: DataClass, op: Operation, purpose: Purpose) -> DataUsageEvent {
    let mut b = EventBuilder::new(
        EventId([4u8; 16]),
        AgentRef::new("test", "0"),
        controller(),
        SubjectScope::Subject { subject: subject() },
        Jurisdiction::new("DE").unwrap(),
        class,
        op,
        purpose,
        grant.reference().unwrap(),
        1,
        Timestamp::from_secs(T0 + 100),
    );
    if op.derives() {
        b = b.provenance(Provenance {
            output: Some(ContentId::of_bytes("duap.object.v1", b"o")),
            ..Default::default()
        });
    }
    if op.family() == OperationFamily::Transfer && op != Operation::TransferInternal {
        b = b.economics(EconomicContext {
            counterparty: Some("org:duap/partner".parse().unwrap()),
            ..Default::default()
        });
    }
    b.build().unwrap()
}

fn ctx() -> EvalContext<'static> {
    EvalContext::verified()
}

/// A facts provider for tests, answering exactly what the test sets.
#[derive(Debug, Default)]
struct TestFacts {
    input_depth: Option<Fact<u8>>,
    epsilon_micro: Option<Fact<u64>>,
}

impl EvalFacts for TestFacts {
    fn input_depth(&self, _ev: &DataUsageEvent) -> Fact<u8> {
        self.input_depth.unwrap_or(Fact::Unavailable)
    }
    fn epsilon_micro(&self, _ev: &DataUsageEvent) -> Fact<u64> {
        self.epsilon_micro.unwrap_or(Fact::Unavailable)
    }
}

// ---------------------------------------------------------------------------
// INV-A1 / INV-A2: combining algorithm
// ---------------------------------------------------------------------------

#[test]
fn deny_by_default() {
    let g = builder().build().unwrap();
    let ev = event_for(
        &g,
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    let d = evaluate(&g, &[], &ev, &ctx());
    assert!(!d.permitted());
    assert_eq!(d.reason, DecisionReason::DefaultEffect);
}

#[test]
fn permit_when_a_term_matches() {
    let g = builder()
        .term(Term::permit(1, Matcher::any()).with_pricing(PricingRule::Free))
        .build()
        .unwrap();
    let ev = event_for(
        &g,
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    let d = evaluate(&g, &[], &ev, &ctx());
    assert!(d.permitted());
    assert_eq!(d.permitting_terms, vec![1]);
    assert_eq!(d.pricing, Some(PricingRule::Free));
    assert_eq!(d.currency, Some(Currency::EUR));
}

/// INV-A1: a Deny wins regardless of ordering.
#[test]
fn deny_overrides_permit_in_both_orders() {
    for (a, b) in [(1u32, 2u32), (2, 1)] {
        let g = builder()
            .term(Term::permit(a, Matcher::any()))
            .term(Term::deny(
                b,
                Matcher::any().purposes(PurposeSelector::Commercial { value: true }),
            ))
            .build()
            .unwrap();
        let ev = event_for(
            &g,
            DataClass::BehaviorWebBrowsing,
            Operation::CommercialAdvertise,
            Purpose::MarketingAdvertisingBehavioral,
        );
        let d = evaluate(&g, &[], &ev, &ctx());
        assert!(
            !d.permitted(),
            "order ({a},{b}) let a commercial use through"
        );
        assert_eq!(d.reason, DecisionReason::DeniedByTerm { term: b });
    }
}

#[test]
fn purpose_lattice_narrows_and_widens_correctly() {
    let g = builder()
        .term(Term::permit(
            1,
            Matcher::any().purposes(PurposeSelector::Under {
                roots: vec![Purpose::Marketing],
            }),
        ))
        .build()
        .unwrap();
    // A descendant of marketing is covered.
    let ev = event_for(
        &g,
        DataClass::BehaviorWebBrowsing,
        Operation::CommercialAdvertise,
        Purpose::MarketingAdvertisingBehavioral,
    );
    assert!(evaluate(&g, &[], &ev, &ctx()).permitted());
    // A sibling root is not.
    let ev = event_for(
        &g,
        DataClass::BehaviorWebBrowsing,
        Operation::CommercialUnderwrite,
        Purpose::RiskInsurance,
    );
    assert!(!evaluate(&g, &[], &ev, &ctx()).permitted());
}

#[test]
fn namespace_selector_matches_whole_family() {
    let g = builder()
        .term(Term::permit(
            1,
            Matcher::any().classes(ClassSelector::Namespace {
                namespaces: vec!["location".into()],
            }),
        ))
        .build()
        .unwrap();
    for c in [
        DataClass::LocationCoarse,
        DataClass::LocationPrecise,
        DataClass::LocationTrajectory,
    ] {
        let ev = event_for(&g, c, Operation::AccessQuery, Purpose::ServiceCore);
        assert!(evaluate(&g, &[], &ev, &ctx()).permitted(), "{}", c.code());
    }
    let ev = event_for(
        &g,
        DataClass::ContactEmail,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    assert!(!evaluate(&g, &[], &ev, &ctx()).permitted());
}

#[test]
fn max_tier_selector_bounds_sensitivity() {
    let g = builder()
        .term(Term::permit(
            1,
            Matcher::any().classes(ClassSelector::MaxTier {
                tier: SensitivityTier::T2,
            }),
        ))
        .build()
        .unwrap();
    let ok = event_for(
        &g,
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    assert!(evaluate(&g, &[], &ok, &ctx()).permitted());
    let no = event_for(
        &g,
        DataClass::BiometricTemplate,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    assert!(!evaluate(&g, &[], &no, &ctx()).permitted());
}

// ---------------------------------------------------------------------------
// Grant binding
// ---------------------------------------------------------------------------

#[test]
fn event_must_cite_the_exact_grant_document() {
    let g = builder()
        .term(Term::permit(1, Matcher::any()))
        .build()
        .unwrap();
    let mut ev = event_for(
        &g,
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    assert!(evaluate(&g, &[], &ev, &ctx()).permitted());

    // Tamper with the cited digest: the controller cannot claim to have
    // relied on a document other than the one that hashes to this value.
    ev.authorization.grant_digest =
        duap_canon::Digest::of(duap_canon::HashAlg::Sha2_256, "x", b"other");
    let d = evaluate(&g, &[], &ev, &ctx());
    assert!(!d.permitted());
    assert!(matches!(d.reason, DecisionReason::GrantMismatch { .. }));
}

#[test]
fn unverified_grants_never_permit() {
    let g = builder()
        .term(Term::permit(1, Matcher::any()))
        .build()
        .unwrap();
    let ev = event_for(
        &g,
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    let unverified = EvalContext::default();
    assert!(!evaluate(&g, &[], &ev, &unverified).permitted());
}

#[test]
fn party_mismatch_denies() {
    let g = builder()
        .term(Term::permit(1, Matcher::any()))
        .build()
        .unwrap();
    let mut ev = event_for(
        &g,
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    ev.controller = "org:duap/other".parse().unwrap();
    let d = evaluate(&g, &[], &ev, &ctx());
    assert!(matches!(d.reason, DecisionReason::PartyMismatch { .. }));

    let mut ev = event_for(
        &g,
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    ev.subject = SubjectScope::Subject {
        subject: SubjectRef([9u8; 16]),
    };
    let d = evaluate(&g, &[], &ev, &ctx());
    assert!(matches!(d.reason, DecisionReason::PartyMismatch { .. }));
}

#[test]
fn outside_the_window_denies() {
    let g = builder()
        .term(Term::permit(1, Matcher::any()))
        .not_before(Timestamp::from_secs(T0 + 1_000))
        .build()
        .unwrap();
    let ev = event_for(
        &g,
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    assert_eq!(
        evaluate(&g, &[], &ev, &ctx()).reason,
        DecisionReason::OutsideGrantWindow
    );
}

// ---------------------------------------------------------------------------
// INV-A6: obligations
// ---------------------------------------------------------------------------

#[test]
fn violated_obligation_denies() {
    let g = builder()
        .term(Term::permit(1, Matcher::any()).with_obligations(vec![Obligation::NoAiTraining]))
        .build()
        .unwrap();
    let ev = event_for(
        &g,
        DataClass::ContentUserGenerated,
        Operation::AiFinetune,
        Purpose::ImprovementAiTraining,
    );
    let d = evaluate(&g, &[], &ev, &ctx());
    assert!(!d.permitted());
    match d.reason {
        DecisionReason::ObligationViolated { obligation, .. } => {
            assert_eq!(obligation, "no_ai_training")
        }
        other => panic!("unexpected reason {other:?}"),
    }
}

#[test]
fn obligations_union_across_matching_permits() {
    // A broad permit must not erase a narrow permit's safeguard.
    let g = builder()
        .term(Term::permit(1, Matcher::any()))
        .term(
            Term::permit(
                2,
                Matcher::any().classes(ClassSelector::In {
                    values: vec![DataClass::LocationPrecise],
                }),
            )
            .with_obligations(vec![Obligation::NoOnwardTransfer]),
        )
        .build()
        .unwrap();
    let ev = event_for(
        &g,
        DataClass::LocationPrecise,
        Operation::TransferSale,
        Purpose::CommerceSale,
    );
    let d = evaluate(&g, &[], &ev, &ctx());
    assert!(
        !d.permitted(),
        "the broad permit must not erase the narrow permit's no-transfer obligation"
    );
}

#[test]
fn transfer_allowlist_enforced() {
    let allowed: OrgId = "org:duap/partner".parse().unwrap();
    let g = builder()
        .term(
            Term::permit(1, Matcher::any()).with_obligations(vec![Obligation::TransferOnlyTo {
                orgs: vec![allowed.clone()],
            }]),
        )
        .build()
        .unwrap();
    let ev = event_for(
        &g,
        DataClass::TransactionPurchase,
        Operation::TransferSale,
        Purpose::CommerceSale,
    );
    assert!(evaluate(&g, &[], &ev, &ctx()).permitted());

    let mut bad = ev.clone();
    bad.economics = Some(EconomicContext {
        counterparty: Some("org:duap/stranger".parse().unwrap()),
        ..Default::default()
    });
    assert!(!evaluate(&g, &[], &bad, &ctx()).permitted());
}

#[test]
fn retention_obligation_checks_declared_policy() {
    let g = builder()
        .term(
            Term::permit(1, Matcher::any())
                .with_obligations(vec![Obligation::MaxRetentionDays { days: 30 }]),
        )
        .build()
        .unwrap();

    let mut ev = event_for(
        &g,
        DataClass::LocationCoarse,
        Operation::StorePersist,
        Purpose::ServiceCore,
    );
    // No retention declared at all: denied, because the obligation is not
    // satisfiable from the record.
    assert!(!evaluate(&g, &[], &ev, &ctx()).permitted());

    ev.retention = Some(RetentionPolicy {
        basis: RetentionBasis::FixedPeriod,
        days: Some(14),
        until: None,
    });
    assert!(evaluate(&g, &[], &ev, &ctx()).permitted());

    ev.retention = Some(RetentionPolicy {
        basis: RetentionBasis::FixedPeriod,
        days: Some(400),
        until: None,
    });
    assert!(!evaluate(&g, &[], &ev, &ctx()).permitted());

    ev.retention = Some(RetentionPolicy {
        basis: RetentionBasis::Indefinite,
        days: None,
        until: None,
    });
    assert!(!evaluate(&g, &[], &ev, &ctx()).permitted());
}

#[test]
fn deferred_obligations_are_reported_not_enforced() {
    let g = builder()
        .term(Term::permit(1, Matcher::any()).with_obligations(vec![
            Obligation::DeleteBy {
                at: Timestamp::from_secs(T0 + 86_400),
            },
            Obligation::NotifyOnUse,
        ]))
        .build()
        .unwrap();
    let ev = event_for(
        &g,
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    let d = evaluate(&g, &[], &ev, &ctx());
    assert!(d.permitted());
    assert_eq!(
        d.deferred.len(),
        2,
        "both promises must be surfaced: {:?}",
        d.deferred
    );
    assert!(d.deferred.iter().any(|s| s.starts_with("delete_by")));
}

#[test]
fn cohort_and_epsilon_obligations() {
    let g = builder()
        .term(Term::permit(1, Matcher::any()).with_obligations(vec![
            Obligation::MinCohort { k: 100 },
            Obligation::MaxEpsilonMicro {
                epsilon_micro: 1_000_000,
            },
        ]))
        .build()
        .unwrap();

    let mk = |size: u64, eps: Option<u64>| {
        let ev = EventBuilder::new(
            EventId([4u8; 16]),
            AgentRef::new("t", "0"),
            controller(),
            SubjectScope::Cohort {
                handle: ContentId::of_bytes("duap.cohort.v1", b"c"),
                size,
            },
            Jurisdiction::new("DE").unwrap(),
            DataClass::DerivedDpAggregate,
            Operation::ProcessDpRelease,
            Purpose::ImprovementAnalytics,
            g.reference().unwrap(),
            1,
            Timestamp::from_secs(T0 + 10),
        )
        .provenance(Provenance {
            output: Some(ContentId::of_bytes("duap.object.v1", b"agg")),
            ..Default::default()
        })
        .build()
        .unwrap();
        let facts = TestFacts {
            input_depth: None,
            epsilon_micro: Some(match eps {
                Some(e) => Fact::Known(e),
                // The provider can answer: the release reported no
                // epsilon. Whether that is acceptable is the obligation's
                // call, not the provider's.
                None => Fact::NotApplicable,
            }),
        };
        let c = EvalContext::verified().with_facts(&facts);
        evaluate(&g, &[], &ev, &c)
    };

    assert!(mk(1000, Some(500_000)).permitted());
    assert!(!mk(10, Some(500_000)).permitted(), "cohort too small");
    assert!(!mk(1000, Some(5_000_000)).permitted(), "epsilon too large");
    assert!(
        !mk(1000, None).permitted(),
        "a DP release must report epsilon"
    );
}

// ---------------------------------------------------------------------------
// INV-A4 / INV-A5: revocation
// ---------------------------------------------------------------------------

#[test]
fn revocation_suppresses_from_its_effective_time() {
    let g = builder()
        .term(Term::permit(1, Matcher::any()))
        .revocation(RevocationPolicy::Immediate)
        .build()
        .unwrap();
    let rev = Revocation::for_grant(
        &g,
        RevocationScope::All,
        Timestamp::from_secs(T0 + 50),
        RetroactiveRequest::None,
    )
    .unwrap();

    let before = {
        let mut e = event_for(
            &g,
            DataClass::LocationCoarse,
            Operation::AccessQuery,
            Purpose::ServiceCore,
        );
        e.occurred_at = Timestamp::from_secs(T0 + 10);
        e.recorded_at = e.occurred_at;
        e
    };
    let after = event_for(
        &g,
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );

    assert!(evaluate(&g, std::slice::from_ref(&rev), &before, &ctx()).permitted());
    let d = evaluate(&g, &[rev], &after, &ctx());
    assert!(!d.permitted());
    assert_eq!(d.reason, DecisionReason::RevokedTerm { term: 1 });
}

#[test]
fn notice_period_delays_effect() {
    let g = builder()
        .term(Term::permit(1, Matcher::any()))
        .revocation(RevocationPolicy::AfterNotice { hours: 24 })
        .build()
        .unwrap();
    let rev = Revocation::for_grant(
        &g,
        RevocationScope::All,
        Timestamp::from_secs(T0),
        RetroactiveRequest::DeleteSource,
    )
    .unwrap();
    assert_eq!(
        rev.effective_from,
        Timestamp::from_secs(T0).saturating_add(24 * HOUR)
    );

    let during = event_for(
        &g,
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    assert!(evaluate(&g, std::slice::from_ref(&rev), &during, &ctx()).permitted());

    let mut later = during.clone();
    later.occurred_at = Timestamp::from_secs(T0 + 48 * 3600);
    later.recorded_at = later.occurred_at;
    assert!(!evaluate(&g, &[rev], &later, &ctx()).permitted());
}

#[test]
fn scoped_revocation_only_hits_its_scope() {
    let g = builder()
        .term(Term::permit(
            1,
            Matcher::any().purposes(PurposeSelector::Under {
                roots: vec![Purpose::Marketing],
            }),
        ))
        .term(Term::permit(
            2,
            Matcher::any().purposes(PurposeSelector::Under {
                roots: vec![Purpose::Service],
            }),
        ))
        .build()
        .unwrap();
    let rev = Revocation {
        schema: 1,
        grant: g.id,
        epoch: g.epoch,
        grant_digest: g.digest().unwrap(),
        scope: RevocationScope::Purposes {
            purposes: vec![Purpose::Marketing],
        },
        declared_at: Timestamp::from_secs(T0),
        effective_from: Timestamp::from_secs(T0),
        retroactive: RetroactiveRequest::None,
        reason: None,
    };
    let marketing = event_for(
        &g,
        DataClass::BehaviorWebBrowsing,
        Operation::CommercialAdvertise,
        Purpose::MarketingAdvertisingBehavioral,
    );
    let service = event_for(
        &g,
        DataClass::BehaviorWebBrowsing,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    assert!(!evaluate(&g, std::slice::from_ref(&rev), &marketing, &ctx()).permitted());
    assert!(evaluate(&g, &[rev], &service, &ctx()).permitted());
}

/// INV-A5: amending a grant must not resurrect a revoked permission.
#[test]
fn amendment_cannot_escape_a_revocation() {
    let g1 = builder()
        .term(Term::permit(1, Matcher::any()))
        .build()
        .unwrap();
    let rev = Revocation::for_grant(
        &g1,
        RevocationScope::All,
        Timestamp::from_secs(T0 + 10),
        RetroactiveRequest::None,
    )
    .unwrap();
    let g2 = g1
        .amend(
            vec![Term::permit(1, Matcher::any())],
            Timestamp::from_secs(T0 + 20),
        )
        .unwrap();

    let ev = event_for(
        &g2,
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    let d = evaluate(&g2, &[rev], &ev, &ctx());
    assert!(
        !d.permitted(),
        "a revocation at epoch 1 must still bind at epoch 2"
    );
}

#[test]
fn grants_chain_by_digest() {
    let g1 = builder()
        .term(Term::permit(1, Matcher::any()))
        .build()
        .unwrap();
    let g2 = g1
        .amend(
            vec![Term::permit(1, Matcher::any())],
            Timestamp::from_secs(T0 + 20),
        )
        .unwrap();
    assert_eq!(g2.epoch, 2);
    assert_eq!(g2.previous, Some(g1.digest().unwrap()));

    let mut store = AuthorizationStore::new();
    store.insert_grant(g1.clone()).unwrap();
    store.insert_grant(g2.clone()).unwrap();
    assert_eq!(store.latest_epoch(&g1.id).unwrap().epoch, 2);

    // A forged epoch 2 that does not chain is refused.
    let mut forged = g2.clone();
    forged.previous = Some(duap_canon::Digest::of(
        duap_canon::HashAlg::Sha2_256,
        "x",
        b"y",
    ));
    let mut store2 = AuthorizationStore::new();
    store2.insert_grant(g1).unwrap();
    assert!(store2.insert_grant(forged).is_err());
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

#[test]
fn grant_validation_rules() {
    // Duplicate term ids.
    assert!(
        builder()
            .term(Term::permit(1, Matcher::any()))
            .term(Term::permit(1, Matcher::any()))
            .build()
            .is_err()
    );
    // Deny with obligations is meaningless.
    assert!(
        builder()
            .term(Term::deny(1, Matcher::any()).with_obligations(vec![Obligation::NoAiTraining]))
            .build()
            .is_err()
    );
    // Expiry before start.
    assert!(
        builder()
            .not_before(Timestamp::from_secs(T0 + 100))
            .expires_at(Timestamp::from_secs(T0))
            .build()
            .is_err()
    );
    // Absurd notice period.
    assert!(
        builder()
            .revocation(RevocationPolicy::AfterNotice { hours: 10_000 })
            .build()
            .is_err()
    );
}

#[test]
fn grants_round_trip_canonically() {
    let g = builder()
        .term(
            Term::permit(
                1,
                Matcher::any().classes(ClassSelector::MaxTier {
                    tier: SensitivityTier::T2,
                }),
            )
            .with_obligations(vec![Obligation::MaxRetentionDays { days: 30 }])
            .with_pricing(PricingRule::per_unit(
                Unit::Record,
                Precise::new(Currency::EUR, 250_000),
            ))
            .with_label("Location for service delivery"),
        )
        .build()
        .unwrap();
    let bytes = g.to_canonical().unwrap();
    assert!(duap_canon::is_canonical(&bytes));
    assert_eq!(Grant::from_canonical(&bytes).unwrap(), g);
}

// ---------------------------------------------------------------------------
// Pricing selection
// ---------------------------------------------------------------------------

#[test]
fn most_specific_priced_term_wins_deterministically() {
    let cheap = PricingRule::per_unit(Unit::Record, Precise::new(Currency::EUR, 1));
    let dear = PricingRule::per_unit(Unit::Record, Precise::new(Currency::EUR, 1_000_000));
    let g = builder()
        .term(Term::permit(1, Matcher::any()).with_pricing(cheap.clone()))
        .term(
            Term::permit(
                2,
                Matcher::any()
                    .classes(ClassSelector::In {
                        values: vec![DataClass::LocationPrecise],
                    })
                    .purposes(PurposeSelector::Commercial { value: true }),
            )
            .with_pricing(dear.clone()),
        )
        .build()
        .unwrap();
    let ev = event_for(
        &g,
        DataClass::LocationPrecise,
        Operation::CommercialAdvertise,
        Purpose::MarketingAdvertisingBehavioral,
    );
    let d = evaluate(&g, &[], &ev, &ctx());
    assert_eq!(d.pricing, Some(dear));

    let ev2 = event_for(
        &g,
        DataClass::ContactEmail,
        Operation::AccessRead,
        Purpose::ServiceCore,
    );
    assert_eq!(evaluate(&g, &[], &ev2, &ctx()).pricing, Some(cheap));
}

// ---------------------------------------------------------------------------
// Negotiation state machine
// ---------------------------------------------------------------------------

#[test]
fn negotiation_happy_path() {
    use duap_auth::negotiation::*;
    let id = BatchId([7u8; 16]);
    let mut n = Negotiation::new(id);
    let req = AccessRequest {
        id,
        requester: "org:duap/buyer".parse().unwrap(),
        subject: Some(subject()),
        wants: Matcher::any().classes(ClassSelector::Namespace {
            namespaces: vec!["location".into()],
        }),
        disclosure: "Deliver weather alerts for your area.".into(),
        duration_hours: 24 * 30,
        proposed_pricing: Some(PricingRule::per_unit(
            Unit::Record,
            Precise::new(Currency::EUR, 100_000),
        )),
        currency: Currency::EUR,
        sent_at: Timestamp::from_secs(T0),
        expires_at: Timestamp::from_secs(T0 + 3600),
    };
    let m = NegotiationMessage::Request(req.clone());
    assert_eq!(n.apply(&m).unwrap(), NegotiationState::Requested);

    let offer = AccessOffer {
        id,
        request_digest: m.digest().unwrap(),
        terms: vec![Term::permit(1, req.wants.clone())],
        duration_hours: 24 * 7,
        currency: Currency::EUR,
        sent_at: Timestamp::from_secs(T0 + 10),
        expires_at: Timestamp::from_secs(T0 + 1800),
        counter: true,
    };
    let mo = NegotiationMessage::Offer(offer);
    assert_eq!(n.apply(&mo).unwrap(), NegotiationState::Offered);

    let acc = NegotiationMessage::Accept(Acceptance {
        id,
        offer_digest: mo.digest().unwrap(),
        sent_at: Timestamp::from_secs(T0 + 20),
    });
    assert_eq!(n.apply(&acc).unwrap(), NegotiationState::Accepted);

    let mut g = builder()
        .term(Term::permit(1, Matcher::any()))
        .build()
        .unwrap();
    g.extensions.insert(
        "negotiation.offer".into(),
        duap_canon::Value::text(mo.digest().unwrap().to_string()),
    );
    n.settle(g).unwrap();
    assert_eq!(n.state, NegotiationState::Granted);
    assert!(n.is_terminal());
}

#[test]
fn negotiation_rejects_out_of_order_replay_and_mismatch() {
    use duap_auth::negotiation::*;
    let id = BatchId([7u8; 16]);
    let mut n = Negotiation::new(id);

    // Accept before anything was offered.
    let acc = NegotiationMessage::Accept(Acceptance {
        id,
        offer_digest: duap_canon::Digest::of(duap_canon::HashAlg::Sha2_256, "x", b"y"),
        sent_at: Timestamp::from_secs(T0),
    });
    assert!(matches!(
        n.apply(&acc),
        Err(NegotiationError::OutOfOrder { .. })
    ));

    let req = NegotiationMessage::Request(AccessRequest {
        id,
        requester: "org:duap/buyer".parse().unwrap(),
        subject: None,
        wants: Matcher::any(),
        disclosure: "d".into(),
        duration_hours: 1,
        proposed_pricing: None,
        currency: Currency::EUR,
        sent_at: Timestamp::from_secs(T0),
        expires_at: Timestamp::from_secs(T0 + 100),
    });
    n.apply(&req).unwrap();
    assert!(matches!(n.apply(&req), Err(NegotiationError::Replay)));

    // An offer answering the wrong request digest.
    let bad_offer = NegotiationMessage::Offer(AccessOffer {
        id,
        request_digest: duap_canon::Digest::of(duap_canon::HashAlg::Sha2_256, "x", b"other"),
        terms: vec![],
        duration_hours: 1,
        currency: Currency::EUR,
        sent_at: Timestamp::from_secs(T0 + 1),
        expires_at: Timestamp::from_secs(T0 + 50),
        counter: false,
    });
    assert!(matches!(
        n.apply(&bad_offer),
        Err(NegotiationError::RequestMismatch { .. })
    ));

    // An offer after expiry.
    let late = NegotiationMessage::Offer(AccessOffer {
        id,
        request_digest: req.digest().unwrap(),
        terms: vec![],
        duration_hours: 1,
        currency: Currency::EUR,
        sent_at: Timestamp::from_secs(T0 + 1_000),
        expires_at: Timestamp::from_secs(T0 + 2_000),
        counter: false,
    });
    assert!(matches!(
        n.apply(&late),
        Err(NegotiationError::Expired { .. })
    ));
}

// ---------------------------------------------------------------------------
// Property tests
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// INV-A3: evaluation is deterministic.
    #[test]
    fn evaluation_is_deterministic(class_i in 0usize..DataClass::ALL.len(), purpose_i in 0usize..Purpose::ALL.len()) {
        let g = builder()
            .term(Term::permit(1, Matcher::any().classes(ClassSelector::MaxTier { tier: SensitivityTier::T2 })))
            .term(Term::deny(2, Matcher::any().purposes(PurposeSelector::Commercial { value: true })))
            .build().unwrap();
        let ev = event_for(&g, DataClass::ALL[class_i], Operation::AccessQuery, Purpose::ALL[purpose_i]);
        let a = evaluate(&g, &[], &ev, &ctx());
        let b = evaluate(&g, &[], &ev, &ctx());
        prop_assert_eq!(a, b);
    }

    /// INV-A1 as a property: adding a matching Deny never yields Permit.
    #[test]
    fn adding_a_matching_deny_never_permits(n in 0usize..8) {
        let mut b = builder();
        for i in 0..n {
            b = b.term(Term::permit(i as u32, Matcher::any()));
        }
        b = b.term(Term::deny(1000, Matcher::any()));
        let g = b.build().unwrap();
        let ev = event_for(&g, DataClass::LocationCoarse, Operation::AccessQuery, Purpose::ServiceCore);
        prop_assert!(!evaluate(&g, &[], &ev, &ctx()).permitted());
    }

    /// INV-A2 as a property: a Permit always names at least one term, or the
    /// grant default is Permit.
    #[test]
    fn permits_are_always_explained(class_i in 0usize..DataClass::ALL.len()) {
        let g = builder()
            .term(Term::permit(1, Matcher::any().classes(ClassSelector::Namespace { namespaces: vec!["location".into()] })))
            .build().unwrap();
        let ev = event_for(&g, DataClass::ALL[class_i], Operation::AccessQuery, Purpose::ServiceCore);
        let d = evaluate(&g, &[], &ev, &ctx());
        if d.permitted() {
            prop_assert!(!d.permitting_terms.is_empty());
            let explained = matches!(d.reason, DecisionReason::PermittedByTerms { .. });
            prop_assert!(explained);
        }
    }

    /// Narrowing a matcher never widens the permitted set.
    #[test]
    fn matchers_are_monotone(class_i in 0usize..DataClass::ALL.len()) {
        let class = DataClass::ALL[class_i];
        let broad = builder().term(Term::permit(1, Matcher::any())).build().unwrap();
        let narrow = builder()
            .term(Term::permit(1, Matcher::any().classes(ClassSelector::In { values: vec![DataClass::LocationCoarse] })))
            .build().unwrap();
        let eb = event_for(&broad, class, Operation::AccessQuery, Purpose::ServiceCore);
        let en = event_for(&narrow, class, Operation::AccessQuery, Purpose::ServiceCore);
        if evaluate(&narrow, &[], &en, &ctx()).permitted() {
            prop_assert!(evaluate(&broad, &[], &eb, &ctx()).permitted());
        }
    }
}

// ---------------------------------------------------------------------------
// ADR-0017: the facts interface
// ---------------------------------------------------------------------------

/// A caller that supplies no facts provider must be *told* so, not left to
/// read a denial that looks like a policy outcome.
///
/// This is the defect VS-2 actually was: `MaxDerivationDepth` denied every
/// deriving operation and the denial was indistinguishable from the
/// subject having refused. It survived a full green test suite because
/// every test of the evaluator passed.
#[test]
fn a_missing_fact_denies_and_says_it_was_missing() {
    let g = builder()
        .term(
            Term::permit(1, Matcher::any())
                .with_obligations(vec![Obligation::MaxDerivationDepth { depth: 2 }]),
        )
        .build()
        .unwrap();
    let ev = event_for(
        &g,
        DataClass::ContentUserGenerated,
        Operation::AiFinetune,
        Purpose::ImprovementAiTraining,
    );

    // No facts provider at all: the default.
    let d = evaluate(&g, &[], &ev, &EvalContext::verified());
    assert!(
        !d.permitted(),
        "a decision made without the facts must deny"
    );
    match &d.reason {
        DecisionReason::FactUnavailable {
            obligation, fact, ..
        } => {
            assert_eq!(fact, "input_depth");
            assert_eq!(obligation, "max_derivation_depth");
        }
        other => panic!(
            "expected FactUnavailable so the caller knows it is a \
             misconfiguration, got {other:?}"
        ),
    }
}

/// The same obligation, with a provider that can answer, is decided on the
/// answer rather than on the caller's diligence.
#[test]
fn a_supplied_fact_is_decided_on_its_value() {
    let g = builder()
        .term(
            Term::permit(1, Matcher::any())
                .with_obligations(vec![Obligation::MaxDerivationDepth { depth: 2 }]),
        )
        .build()
        .unwrap();
    let ev = event_for(
        &g,
        DataClass::ContentUserGenerated,
        Operation::AiFinetune,
        Purpose::ImprovementAiTraining,
    );

    let shallow = TestFacts {
        input_depth: Some(Fact::Known(0)),
        ..Default::default()
    };
    assert!(
        evaluate(&g, &[], &ev, &EvalContext::verified().with_facts(&shallow)).permitted(),
        "depth 0 + 1 is within a limit of 2"
    );

    let deep = TestFacts {
        input_depth: Some(Fact::Known(5)),
        ..Default::default()
    };
    let d = evaluate(&g, &[], &ev, &EvalContext::verified().with_facts(&deep));
    assert!(!d.permitted());
    assert!(
        matches!(d.reason, DecisionReason::ObligationViolated { .. }),
        "an answered fact that breaches the limit is a violation, not a \
         missing fact: {:?}",
        d.reason
    );
}

/// `NotApplicable` is an answer, not an absence. An operation with no
/// recorded inputs still produces a derivative at depth 1, so a limit of 0
/// refuses it and a limit of 2 does not.
#[test]
fn not_applicable_is_an_answer_and_is_decided() {
    let ev_grant = |depth: u8| {
        builder()
            .term(
                Term::permit(1, Matcher::any())
                    .with_obligations(vec![Obligation::MaxDerivationDepth { depth }]),
            )
            .build()
            .unwrap()
    };
    let derive_ev = |g: &Grant| {
        event_for(
            g,
            DataClass::ContentUserGenerated,
            Operation::AiFinetune,
            Purpose::ImprovementAiTraining,
        )
    };
    let facts = TestFacts {
        input_depth: Some(Fact::NotApplicable),
        ..Default::default()
    };

    let g2 = ev_grant(2);
    let ev2 = derive_ev(&g2);
    assert!(
        evaluate(&g2, &[], &ev2, &EvalContext::verified().with_facts(&facts)).permitted(),
        "no inputs means the result is at depth 1, within a limit of 2"
    );

    let g0 = ev_grant(0);
    let ev0 = derive_ev(&g0);
    let d = evaluate(&g0, &[], &ev0, &EvalContext::verified().with_facts(&facts));
    assert!(!d.permitted(), "a limit of 0 forbids deriving at all");
    assert!(
        matches!(d.reason, DecisionReason::ObligationViolated { .. }),
        "NotApplicable is an answer, so breaching the limit is a violation, \
         not a missing fact: {:?}",
        d.reason
    );
}

/// An obligation that does not need a fact is unaffected by the absence of
/// a provider. Failing closed must not mean failing closed on everything.
#[test]
fn an_obligation_needing_no_fact_is_unaffected_by_a_missing_provider() {
    let g = builder()
        .term(Term::permit(1, Matcher::any()).with_obligations(vec![Obligation::NoAiTraining]))
        .build()
        .unwrap();
    let ev = event_for(
        &g,
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
    );
    assert!(
        evaluate(&g, &[], &ev, &EvalContext::verified()).permitted(),
        "NoAiTraining is decided from the event's operation, so no facts \
         provider is needed and its absence must not deny"
    );
}
