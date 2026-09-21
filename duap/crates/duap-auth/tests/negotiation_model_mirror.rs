//! Rust mirrors of the invariants checked in `formal/Negotiation.tla`.
//!
//! Each test bears the model invariant's name, so that a change to
//! `duap_auth::negotiation` which breaks a modelled property fails the
//! ordinary test suite rather than only a model check nobody runs locally.
//!
//! These are not a refinement proof. The model quantifies over every
//! reachable configuration within its bounds; these tests exercise
//! particular ones. Agreement on particular cases is what a test can
//! establish, and the distinction is stated in `formal/README.md`.

use duap_auth::negotiation::*;
use duap_auth::prelude::*;
use duap_model::prelude::*;

const T0: u64 = 1_700_000_000;

fn nid() -> NegotiationId {
    duap_model::ids::BatchId([9u8; 16])
}

fn subject() -> SubjectRef {
    SubjectRef([1u8; 16])
}

fn key_id() -> duap_crypto::KeyId {
    duap_crypto::SecretKey::from_seed(duap_crypto::SuiteId::Ed25519, [2u8; 32]).key_id()
}

fn request_at(secs: u64, expires: u64) -> NegotiationMessage {
    NegotiationMessage::Request(AccessRequest {
        id: nid(),
        requester: "org:duap/buyer".parse().expect("valid org id"),
        subject: Some(subject()),
        wants: Matcher::any()
            .classes(ClassSelector::Namespace { namespaces: vec!["location".into()] }),
        disclosure: "synthetic: deliver area weather alerts".into(),
        duration_hours: 24,
        proposed_pricing: Some(PricingRule::PerUnit {
            unit: Unit::Record,
            unit_price: Precise::new(Currency::EUR, 100_000),
        }),
        currency: Currency::EUR,
        sent_at: Timestamp::from_secs(secs),
        expires_at: Timestamp::from_secs(expires),
    })
}

fn offer_answering(req: &NegotiationMessage, secs: u64, expires: u64) -> NegotiationMessage {
    NegotiationMessage::Offer(AccessOffer {
        id: nid(),
        request_digest: req.digest().expect("request digests"),
        terms: vec![Term::permit(1, Matcher::any()).with_pricing(PricingRule::Free)],
        duration_hours: 24,
        currency: Currency::EUR,
        sent_at: Timestamp::from_secs(secs),
        expires_at: Timestamp::from_secs(expires),
        counter: false,
    })
}

fn accept_of(offer: &NegotiationMessage, secs: u64) -> NegotiationMessage {
    NegotiationMessage::Accept(Acceptance {
        id: nid(),
        offer_digest: offer.digest().expect("offer digests"),
        sent_at: Timestamp::from_secs(secs),
    })
}

fn grant_binding(offer: &NegotiationMessage) -> Grant {
    let mut g = GrantBuilder::new(
        GrantId([3u8; 16]),
        subject(),
        key_id(),
        "org:duap/acme".parse().expect("valid org id"),
        Timestamp::from_secs(T0),
        Currency::EUR,
    )
    .term(Term::permit(1, Matcher::any()).with_pricing(PricingRule::Free))
    .build()
    .expect("the mirror grant is valid");
    g.extensions.insert(
        "negotiation.offer".into(),
        duap_canon::Value::text(offer.digest().expect("offer digests").to_string()),
    );
    g
}

/// INV-N1. No grant without a request: every state past Idle requires a
/// request to have been applied first.
#[test]
fn no_grant_without_request() {
    let mut n = Negotiation::new(nid());

    // An offer with no request cannot be applied.
    let req = request_at(T0, T0 + 3600);
    let offer = offer_answering(&req, T0 + 10, T0 + 1800);
    assert!(
        matches!(n.apply(&offer), Err(NegotiationError::OutOfOrder { .. })),
        "an offer answering nothing must be refused"
    );
    assert_eq!(n.state, NegotiationState::Idle);

    // Nor can an acceptance.
    assert!(matches!(
        n.apply(&accept_of(&offer, T0 + 20)),
        Err(NegotiationError::OutOfOrder { .. })
    ));
    assert_eq!(n.state, NegotiationState::Idle);
}

/// INV-N2. Acceptance binds an offer that was actually made: an acceptance
/// naming any digest other than the live offer's is refused. This is the
/// digest chain the model abstracts as `acceptedOffer <= offerCount`.
#[test]
fn acceptance_binds_an_offer() {
    let mut n = Negotiation::new(nid());
    let req = request_at(T0, T0 + 3600);
    n.apply(&req).expect("request applies");

    let offer = offer_answering(&req, T0 + 10, T0 + 1800);
    n.apply(&offer).expect("offer applies");

    // An acceptance naming a digest nobody offered.
    let forged = NegotiationMessage::Accept(Acceptance {
        id: nid(),
        offer_digest: duap_canon::Digest::of(
            duap_canon::HashAlg::Sha2_256,
            "duap.negotiation.v1",
            b"an offer that was never made",
        ),
        sent_at: Timestamp::from_secs(T0 + 20),
    });
    assert!(
        matches!(n.apply(&forged), Err(NegotiationError::OfferMismatch { .. })),
        "accepting terms that were never offered must be refused"
    );
    assert_eq!(n.state, NegotiationState::Offered);

    // And an offer that answers a request nobody sent.
    let mut m = Negotiation::new(nid());
    m.apply(&request_at(T0, T0 + 3600)).expect("request applies");
    let wrong = NegotiationMessage::Offer(AccessOffer {
        id: nid(),
        request_digest: duap_canon::Digest::of(
            duap_canon::HashAlg::Sha2_256,
            "duap.negotiation.v1",
            b"a request that was never sent",
        ),
        terms: vec![Term::permit(1, Matcher::any()).with_pricing(PricingRule::Free)],
        duration_hours: 24,
        currency: Currency::EUR,
        sent_at: Timestamp::from_secs(T0 + 10),
        expires_at: Timestamp::from_secs(T0 + 1800),
        counter: false,
    });
    assert!(matches!(m.apply(&wrong), Err(NegotiationError::RequestMismatch { .. })));
}

/// INV-N3. A grant follows an acceptance: `settle` refuses in any state
/// other than Accepted, and refuses a grant that does not carry the
/// accepted offer's digest.
#[test]
fn grant_follows_acceptance() {
    let mut n = Negotiation::new(nid());
    let req = request_at(T0, T0 + 3600);
    n.apply(&req).expect("request applies");
    let offer = offer_answering(&req, T0 + 10, T0 + 1800);
    n.apply(&offer).expect("offer applies");

    // Settling before acceptance is refused.
    assert!(matches!(
        n.settle(grant_binding(&offer)),
        Err(NegotiationError::OutOfOrder { .. })
    ));
    assert_eq!(n.state, NegotiationState::Offered);

    n.apply(&accept_of(&offer, T0 + 20)).expect("acceptance applies");

    // A grant that does not name the accepted offer is refused.
    let unbound = GrantBuilder::new(
        GrantId([4u8; 16]),
        subject(),
        key_id(),
        "org:duap/acme".parse().expect("valid org id"),
        Timestamp::from_secs(T0),
        Currency::EUR,
    )
    .term(Term::permit(1, Matcher::any()).with_pricing(PricingRule::Free))
    .build()
    .expect("valid grant");
    assert!(matches!(n.settle(unbound), Err(NegotiationError::OfferMismatch { .. })));
    assert_eq!(n.state, NegotiationState::Accepted);

    n.settle(grant_binding(&offer)).expect("a bound grant settles");
    assert_eq!(n.state, NegotiationState::Granted);
}

/// INV-N4. Terminal states are absorbing: nothing applies after Granted or
/// Closed.
#[test]
fn terminal_is_absorbing() {
    // Granted.
    let mut n = Negotiation::new(nid());
    let req = request_at(T0, T0 + 3600);
    n.apply(&req).expect("request applies");
    let offer = offer_answering(&req, T0 + 10, T0 + 1800);
    n.apply(&offer).expect("offer applies");
    n.apply(&accept_of(&offer, T0 + 20)).expect("acceptance applies");
    n.settle(grant_binding(&offer)).expect("settles");
    assert!(n.is_terminal());

    let later = request_at(T0 + 30, T0 + 3600);
    assert!(
        n.apply(&later).is_err(),
        "no message may be applied after a grant is issued"
    );
    assert_eq!(n.state, NegotiationState::Granted);

    // Closed.
    let mut m = Negotiation::new(nid());
    m.apply(&request_at(T0, T0 + 3600)).expect("request applies");
    m.apply(&NegotiationMessage::Withdraw(Rejection {
        id: nid(),
        code: RejectCode::Unspecified,
        sent_at: Timestamp::from_secs(T0 + 5),
        message: None,
    }))
    .expect("withdrawal applies");
    assert_eq!(m.state, NegotiationState::Closed);
    assert!(m.is_terminal());

    let offer2 = offer_answering(&request_at(T0, T0 + 3600), T0 + 10, T0 + 1800);
    assert!(m.apply(&offer2).is_err(), "a closed negotiation accepts nothing");
    assert_eq!(m.state, NegotiationState::Closed);
}

/// INV-N5. The outcome is unique: a negotiation holds one state, so it
/// cannot both grant and close. Demonstrated by showing that the branch
/// taken first wins and the other is refused.
#[test]
fn outcome_is_unique() {
    let mut n = Negotiation::new(nid());
    let req = request_at(T0, T0 + 3600);
    n.apply(&req).expect("request applies");
    let offer = offer_answering(&req, T0 + 10, T0 + 1800);
    n.apply(&offer).expect("offer applies");
    n.apply(&accept_of(&offer, T0 + 20)).expect("acceptance applies");
    n.settle(grant_binding(&offer)).expect("settles");

    // A rejection arriving after the grant cannot un-grant it.
    let late_reject = NegotiationMessage::Reject(Rejection {
        id: nid(),
        code: RejectCode::PurposeRefused,
        sent_at: Timestamp::from_secs(T0 + 25),
        message: None,
    });
    assert!(n.apply(&late_reject).is_err());
    assert_eq!(n.state, NegotiationState::Granted);
}

/// INV-N6. No replay: a message already applied is refused if it arrives
/// again, which is what stops a captured acceptance from re-opening a
/// settled exchange.
#[test]
fn no_replay() {
    let mut n = Negotiation::new(nid());
    let req = request_at(T0, T0 + 3600);
    n.apply(&req).expect("request applies");
    assert!(
        matches!(n.apply(&req), Err(NegotiationError::Replay)),
        "the same request applied twice must be refused as a replay"
    );
    assert_eq!(n.state, NegotiationState::Requested);

    let offer = offer_answering(&req, T0 + 10, T0 + 1800);
    n.apply(&offer).expect("offer applies");
    assert!(matches!(n.apply(&offer), Err(NegotiationError::Replay)));
    assert_eq!(n.state, NegotiationState::Offered);
}

/// A counter-offer supersedes the previous one, and accepting the
/// superseded offer is refused. The model's `liveOffer` is the state this
/// exercises; it is the property most likely to be got wrong by an
/// implementation that keeps a set of offers rather than the latest.
#[test]
fn a_counter_offer_supersedes_the_previous_one() {
    let mut n = Negotiation::new(nid());
    let req = request_at(T0, T0 + 3600);
    n.apply(&req).expect("request applies");

    let first = offer_answering(&req, T0 + 10, T0 + 1800);
    n.apply(&first).expect("first offer applies");

    let second = NegotiationMessage::Offer(AccessOffer {
        id: nid(),
        request_digest: req.digest().expect("request digests"),
        terms: vec![Term::permit(1, Matcher::any()).with_pricing(PricingRule::Free)],
        duration_hours: 12,
        currency: Currency::EUR,
        sent_at: Timestamp::from_secs(T0 + 20),
        expires_at: Timestamp::from_secs(T0 + 1800),
        counter: true,
    });
    n.apply(&second).expect("counter-offer applies");

    assert!(
        matches!(
            n.apply(&accept_of(&first, T0 + 30)),
            Err(NegotiationError::OfferMismatch { .. })
        ),
        "the superseded offer must no longer be acceptable"
    );
    n.apply(&accept_of(&second, T0 + 31)).expect("the live offer is acceptable");
    assert_eq!(n.state, NegotiationState::Accepted);
}

/// Expiry is enforced, and rejection is exempt: a party may always refuse,
/// including after the deadline. This is the `InTime` guard in the model
/// and its `Reject` exemption.
#[test]
fn expiry_binds_every_message_except_rejection() {
    let mut n = Negotiation::new(nid());
    n.apply(&request_at(T0, T0 + 100)).expect("request applies");

    let req = request_at(T0, T0 + 100);
    let late_offer = offer_answering(&req, T0 + 500, T0 + 1800);
    assert!(
        matches!(n.apply(&late_offer), Err(NegotiationError::Expired { .. })),
        "an offer after the request's expiry must be refused"
    );
    assert_eq!(n.state, NegotiationState::Requested);

    let late_reject = NegotiationMessage::Reject(Rejection {
        id: nid(),
        code: RejectCode::Expired,
        sent_at: Timestamp::from_secs(T0 + 500),
        message: None,
    });
    n.apply(&late_reject).expect("a late rejection is always permitted");
    assert_eq!(n.state, NegotiationState::Closed);
}
