//! Rust mirrors of the invariants checked in `formal/Authorization.tla`.
//!
//! Each test bears the model invariant's name. They exist so that a change
//! to the implementation which breaks a modelled property fails the ordinary
//! test suite, not only a model check a contributor might never run.
//!
//! These are not a refinement proof. They establish agreement on specific
//! reachable configurations, which is what a test can do.

use duap_auth::prelude::*;
use duap_model::prelude::*;

const T0: u64 = 1_700_000_000;

fn subject() -> SubjectRef {
    SubjectRef([1u8; 16])
}

fn controller() -> OrgId {
    "org:duap/acme".parse().expect("valid org id")
}

fn key_id() -> duap_crypto::KeyId {
    duap_crypto::SecretKey::from_seed(duap_crypto::SuiteId::Ed25519, [2u8; 32]).key_id()
}

/// The model's four-term pool, translated: a narrow permit, a broad permit,
/// a narrow deny and a broad deny over two classes and two purposes.
fn model_terms() -> Vec<Term> {
    vec![
        Term::permit(
            1,
            Matcher::any()
                .classes(ClassSelector::In {
                    values: vec![DataClass::LocationCoarse],
                })
                .purposes(PurposeSelector::Exact {
                    values: vec![Purpose::ServiceCore],
                }),
        )
        .with_pricing(PricingRule::Free),
        Term::permit(
            2,
            Matcher::any()
                .classes(ClassSelector::In {
                    values: vec![DataClass::LocationCoarse, DataClass::ContactEmail],
                })
                .purposes(PurposeSelector::Exact {
                    values: vec![Purpose::ServiceCore, Purpose::MarketingDirect],
                }),
        )
        .with_pricing(PricingRule::Free),
        Term::deny(
            3,
            Matcher::any()
                .classes(ClassSelector::In {
                    values: vec![DataClass::ContactEmail],
                })
                .purposes(PurposeSelector::Exact {
                    values: vec![Purpose::ServiceCore, Purpose::MarketingDirect],
                }),
        ),
        Term::deny(
            4,
            Matcher::any()
                .classes(ClassSelector::In {
                    values: vec![DataClass::LocationCoarse, DataClass::ContactEmail],
                })
                .purposes(PurposeSelector::Exact {
                    values: vec![Purpose::MarketingDirect],
                }),
        ),
    ]
}

fn grant_with(terms: Vec<Term>) -> Grant {
    GrantBuilder::new(
        GrantId([3u8; 16]),
        subject(),
        key_id(),
        controller(),
        Timestamp::from_secs(T0),
        Currency::EUR,
    )
    .terms(terms)
    .build()
    .expect("the model grant is valid")
}

fn event(grant: &Grant, class: DataClass, purpose: Purpose, at_secs: u64) -> DataUsageEvent {
    EventBuilder::new(
        EventId([4u8; 16]),
        AgentRef::new("model-mirror", "0"),
        controller(),
        SubjectScope::Subject { subject: subject() },
        Jurisdiction::new("DE").expect("valid"),
        class,
        Operation::AccessQuery,
        purpose,
        grant.reference().expect("reference"),
        1,
        Timestamp::from_secs(at_secs),
    )
    .build()
    .expect("the model event is valid")
}

/// Every combination the model explores: each subset of the term pool,
/// each class, each purpose.
fn for_each_configuration(mut f: impl FnMut(&Grant, DataUsageEvent, DataClass, Purpose)) {
    let pool = model_terms();
    for mask in 0u8..16 {
        let terms: Vec<Term> = pool
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, t)| t.clone())
            .collect();
        let g = grant_with(terms);
        for class in [DataClass::LocationCoarse, DataClass::ContactEmail] {
            for purpose in [Purpose::ServiceCore, Purpose::MarketingDirect] {
                let ev = event(&g, class, purpose, T0 + 10);
                f(&g, ev, class, purpose);
            }
        }
    }
}

/// `DenyOverrides` in `formal/Authorization.tla` (INV-A1).
#[test]
fn deny_overrides() {
    for_each_configuration(|g, ev, class, purpose| {
        let any_deny = g
            .terms
            .iter()
            .any(|t| t.effect == Effect::Deny && t.matcher.matches(&ev));
        let d = evaluate(g, &[], &ev, &EvalContext::verified());
        if any_deny {
            assert!(
                !d.permitted(),
                "a Deny term applied to {}/{} but the decision was Permit",
                class.code(),
                purpose.code()
            );
        }
    });
}

/// `PermitIsExplained` in `formal/Authorization.tla` (INV-A2).
#[test]
fn permit_is_explained() {
    for_each_configuration(|g, ev, _, _| {
        let d = evaluate(g, &[], &ev, &EvalContext::verified());
        if d.permitted() {
            assert!(
                !d.permitting_terms.is_empty(),
                "a Permit named no terms, and the grant default is Deny"
            );
        }
    });
}

/// `RevocationIsProspective` in `formal/Authorization.tla` (INV-A4).
#[test]
fn revocation_is_prospective() {
    let g = grant_with(model_terms());
    let rev = Revocation::for_grant(
        &g,
        RevocationScope::Purposes {
            purposes: vec![Purpose::ServiceCore],
        },
        Timestamp::from_secs(T0 + 100),
        RetroactiveRequest::None,
    )
    .expect("revocation builds");

    for at in [T0 + 10, T0 + 99, T0 + 100, T0 + 500] {
        let ev = event(&g, DataClass::LocationCoarse, Purpose::ServiceCore, at);
        let d = evaluate(
            &g,
            std::slice::from_ref(&rev),
            &ev,
            &EvalContext::verified(),
        );
        let in_force = at >= T0 + 100;
        if in_force {
            assert!(!d.permitted(), "a revocation in force at {at} did not deny");
        }
    }
}

/// `NoEscapeByAmendment` in `formal/Authorization.tla` (INV-A5).
#[test]
fn no_escape_by_amendment() {
    let g1 = grant_with(model_terms());
    let rev = Revocation::for_grant(
        &g1,
        RevocationScope::All,
        Timestamp::from_secs(T0 + 50),
        RetroactiveRequest::None,
    )
    .expect("revocation builds");

    let mut current = g1;
    for epoch in 2..=3u32 {
        current = current
            .amend(model_terms(), Timestamp::from_secs(T0 + 60 * epoch as u64))
            .expect("amendment builds");
        let ev = event(
            &current,
            DataClass::LocationCoarse,
            Purpose::ServiceCore,
            T0 + 200,
        );
        let d = evaluate(
            &current,
            std::slice::from_ref(&rev),
            &ev,
            &EvalContext::verified(),
        );
        assert!(
            !d.permitted(),
            "epoch {epoch} escaped a revocation issued against epoch 1"
        );
    }
}

/// Non-vacuity: the mirrored configuration space must be able to produce a
/// Permit, or the four tests above pass for the wrong reason.
#[test]
fn a_permit_is_reachable() {
    let mut saw_permit = false;
    for_each_configuration(|g, ev, _, _| {
        if evaluate(g, &[], &ev, &EvalContext::verified()).permitted() {
            saw_permit = true;
        }
    });
    assert!(saw_permit, "no configuration produced a Permit");
}
