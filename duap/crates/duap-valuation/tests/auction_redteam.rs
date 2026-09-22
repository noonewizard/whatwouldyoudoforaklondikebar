//! Adversarial tests for the sealed-bid auction.
//!
//! The auction is the only mechanism in DUAP that can be *gamed* rather
//! than merely miscalculated, and its module documentation makes several
//! claims about what commit-reveal buys. Those claims had never been
//! attacked. These tests attack them.
//!
//! Where an attack succeeds, the test asserts the successful attack and
//! names the finding, so the behaviour is pinned and visible rather than
//! discovered later by someone with money at stake.

use duap_model::prelude::*;
use duap_valuation::auction::*;

const T0: u64 = 1_700_000_000;

fn org(name: &str) -> OrgId {
    format!("org:duap/{name}").parse().expect("valid org id")
}

fn eur(nmu: i128) -> Precise {
    Precise::new(Currency::EUR, nmu)
}

fn lot(reserve: Option<i128>) -> Lot {
    Lot {
        id: "lot-1".into(),
        data_class: DataClass::LocationCoarse,
        operation: Operation::AccessQuery,
        purpose: Purpose::ServiceCore,
        unit: Unit::Query,
        quantity: 1_000,
        reserve_unit_price: reserve.map(eur),
        commit_closes: Timestamp::from_secs(T0 + 100),
        reveal_closes: Timestamp::from_secs(T0 + 200),
    }
}

fn commit_at(a: &mut SealedBidAuction, who: &str, price: i128, salt: u8, at: u64) {
    let b = org(who);
    a.commit(BidCommitment {
        bidder: b.clone(),
        commitment: bid_commitment(&b, eur(price), &[salt; 32]),
        submitted_at: Timestamp::from_secs(at),
    })
    .expect("commitment accepted");
}

fn reveal_at(
    a: &mut SealedBidAuction,
    who: &str,
    price: i128,
    salt: u8,
    at: u64,
) -> Result<(), AuctionError> {
    a.reveal(BidReveal {
        bidder: org(who),
        unit_price: eur(price),
        salt: [salt; 32],
        revealed_at: Timestamp::from_secs(at),
    })
}

// ---------------------------------------------------------------------
// Properties the mechanism must have
// ---------------------------------------------------------------------

/// Vickrey: the winner pays the second price, not their own bid.
#[test]
fn the_winner_pays_the_second_price() {
    let mut a = SealedBidAuction::new(lot(None), Currency::EUR);
    commit_at(&mut a, "alice", 100, 1, T0);
    commit_at(&mut a, "bob", 90, 2, T0);
    commit_at(&mut a, "carol", 80, 3, T0);
    reveal_at(&mut a, "alice", 100, 1, T0 + 150).expect("reveal");
    reveal_at(&mut a, "bob", 90, 2, T0 + 150).expect("reveal");
    reveal_at(&mut a, "carol", 80, 3, T0 + 150).expect("reveal");

    let r = a.settle();
    assert_eq!(
        r.winner.as_ref().map(|o| o.to_string()),
        Some(org("alice").to_string())
    );
    assert_eq!(
        r.clearing_unit_price,
        Some(eur(90)),
        "must pay the second bid"
    );
    assert_eq!(
        r.winning_bid,
        Some(eur(100)),
        "the bid is recorded for audit"
    );
}

/// A reveal that does not open its commitment is rejected. Without this
/// the commitment buys nothing.
#[test]
fn a_bid_cannot_be_changed_after_commitment() {
    let mut a = SealedBidAuction::new(lot(None), Currency::EUR);
    commit_at(&mut a, "alice", 100, 1, T0);
    assert!(matches!(
        reveal_at(&mut a, "alice", 500, 1, T0 + 150),
        Err(AuctionError::RevealMismatch(_))
    ));
    assert!(matches!(
        reveal_at(&mut a, "alice", 100, 9, T0 + 150),
        Err(AuctionError::RevealMismatch(_))
    ));
    reveal_at(&mut a, "alice", 100, 1, T0 + 150).expect("the committed bid opens");
}

/// Nobody can bid without having committed, and nobody can commit after
/// the commit phase closes -- the "last look" the module documentation
/// says commit-reveal exists to prevent.
#[test]
fn no_late_entry() {
    let mut a = SealedBidAuction::new(lot(None), Currency::EUR);
    commit_at(&mut a, "alice", 100, 1, T0);

    let late = org("latecomer");
    assert!(matches!(
        a.commit(BidCommitment {
            bidder: late.clone(),
            commitment: bid_commitment(&late, eur(101), &[7; 32]),
            submitted_at: Timestamp::from_secs(T0 + 101),
        }),
        Err(AuctionError::CommitClosed(_))
    ));
    assert!(matches!(
        reveal_at(&mut a, "latecomer", 101, 7, T0 + 150),
        Err(AuctionError::NoCommitment(_))
    ));
}

/// Below the reserve, nothing sells; at or above it, the reserve floors
/// the clearing price.
#[test]
fn the_reserve_is_a_floor_and_a_gate() {
    let mut a = SealedBidAuction::new(lot(Some(100)), Currency::EUR);
    commit_at(&mut a, "alice", 50, 1, T0);
    reveal_at(&mut a, "alice", 50, 1, T0 + 150).expect("reveal");
    let r = a.settle();
    assert!(r.winner.is_none(), "a bid below the reserve must not win");

    let mut b = SealedBidAuction::new(lot(Some(100)), Currency::EUR);
    commit_at(&mut b, "alice", 200, 1, T0);
    commit_at(&mut b, "bob", 120, 2, T0);
    reveal_at(&mut b, "alice", 200, 1, T0 + 150).expect("reveal");
    reveal_at(&mut b, "bob", 120, 2, T0 + 150).expect("reveal");
    assert_eq!(b.settle().clearing_unit_price, Some(eur(120)));

    // Sole bidder above the reserve pays the reserve, not their own bid.
    let mut c = SealedBidAuction::new(lot(Some(100)), Currency::EUR);
    commit_at(&mut c, "alice", 900, 1, T0);
    reveal_at(&mut c, "alice", 900, 1, T0 + 150).expect("reveal");
    assert_eq!(c.settle().clearing_unit_price, Some(eur(100)));
}

/// Ties resolve deterministically, so the auctioneer cannot choose.
#[test]
fn ties_are_broken_deterministically() {
    let settle_once = || {
        let mut a = SealedBidAuction::new(lot(None), Currency::EUR);
        commit_at(&mut a, "zeta", 100, 1, T0);
        commit_at(&mut a, "alpha", 100, 2, T0);
        reveal_at(&mut a, "zeta", 100, 1, T0 + 150).expect("reveal");
        reveal_at(&mut a, "alpha", 100, 2, T0 + 150).expect("reveal");
        a.settle()
    };
    let first = settle_once();
    assert_eq!(first, settle_once(), "settlement must be deterministic");
    assert_eq!(
        first.winner.map(|o| o.to_string()),
        Some(org("alpha").to_string()),
        "the lexicographically first bidder wins a tie"
    );
}

// ---------------------------------------------------------------------
// Attacks
// ---------------------------------------------------------------------

/// AUC-01. Revealing during the commit phase breaks the seal.
///
/// `reveal` bounds `revealed_at` above by `reveal_closes` and does not
/// bound it below by `commit_closes`. A bidder may therefore open its bid
/// while others can still commit, and anyone watching can commit to one
/// unit more.
///
/// That converts a sealed-bid auction into an open ascending one for
/// every bidder who has not yet committed -- the precise property the
/// module documentation says commit-reveal provides.
#[test]
fn auc_01_a_reveal_during_the_commit_phase_breaks_the_seal() {
    let mut a = SealedBidAuction::new(lot(None), Currency::EUR);

    // Alice commits and immediately tries to reveal, still inside the
    // commit window.
    commit_at(&mut a, "alice", 100, 1, T0);
    assert!(
        matches!(
            reveal_at(&mut a, "alice", 100, 1, T0 + 1),
            Err(AuctionError::RevealTooEarly(_))
        ),
        "AUC-01: a reveal inside the commit window must be rejected; \
         otherwise a later bidder can read Alice's bid and commit to one \
         more, which is the sealed-bid property the mechanism claims to \
         provide"
    );
    // Exactly at the boundary is still too early: the commit phase is
    // inclusive of its closing instant.
    assert!(matches!(
        reveal_at(&mut a, "alice", 100, 1, T0 + 100),
        Err(AuctionError::RevealTooEarly(_))
    ));
    // One tick after, it opens normally.
    reveal_at(&mut a, "alice", 100, 1, T0 + 101).expect("reveal after the commit phase");
}

/// AUC-02. Withholding a reveal is free, and profitable for a ring.
///
/// A losing bidder who declines to reveal lowers the winner's price,
/// because the second price is computed over *revealed* bids only. Reveals
/// are published as they arrive, so a bidder can condition the decision on
/// what others have already opened.
///
/// The module documentation acknowledges that bidder collusion lowers the
/// clearing price. It does not say that the mechanism makes the collusion
/// *costless*, which is the part that matters: there is no deposit to
/// forfeit and no penalty of any kind.
///
/// This test asserts the attack works. It is pinned deliberately: the
/// behaviour is a property of the design, and if it ever changes, the
/// accepted risk in `security/findings.md` is stale.
#[test]
fn auc_02_withholding_a_reveal_lowers_the_price_for_free() {
    // Honest: everyone reveals. Alice wins at Bob's 90.
    let mut honest = SealedBidAuction::new(lot(None), Currency::EUR);
    commit_at(&mut honest, "alice", 100, 1, T0);
    commit_at(&mut honest, "bob", 90, 2, T0);
    commit_at(&mut honest, "carol", 50, 3, T0);
    reveal_at(&mut honest, "alice", 100, 1, T0 + 150).expect("reveal");
    reveal_at(&mut honest, "bob", 90, 2, T0 + 150).expect("reveal");
    reveal_at(&mut honest, "carol", 50, 3, T0 + 150).expect("reveal");
    let honest = honest.settle();
    assert_eq!(honest.clearing_unit_price, Some(eur(90)));

    // Colluding: Bob commits as before but never opens. Alice still wins
    // and now pays Carol's 50 instead of Bob's 90.
    let mut ring = SealedBidAuction::new(lot(None), Currency::EUR);
    commit_at(&mut ring, "alice", 100, 1, T0);
    commit_at(&mut ring, "bob", 90, 2, T0);
    commit_at(&mut ring, "carol", 50, 3, T0);
    reveal_at(&mut ring, "alice", 100, 1, T0 + 150).expect("reveal");
    reveal_at(&mut ring, "carol", 50, 3, T0 + 150).expect("reveal");
    let ring = ring.settle();

    assert_eq!(
        ring.clearing_unit_price,
        Some(eur(50)),
        "AUC-02: withholding one reveal cut the clearing price"
    );
    assert_eq!(
        ring.winner.map(|o| o.to_string()),
        Some(org("alice").to_string()),
        "the winner is unchanged; only the price moved"
    );

    // The only defence the mechanism offers is that the withholding is
    // *visible*. That is deterrence, not prevention, and the count is the
    // whole of it.
    assert_eq!(
        ring.unrevealed_commitments, 1,
        "the unrevealed commitment must at least be counted, since it is \
         the only signal an auditor gets"
    );
    assert_eq!(honest.unrevealed_commitments, 0);
}

/// AUC-03. A withheld reveal is indistinguishable from a crash.
///
/// The count in AUC-02 is the only signal, and it cannot separate a ring
/// from a bidder whose process died. Any enforcement built on it must be
/// economic (a forfeited deposit, which penalises both equally and is the
/// standard answer) rather than punitive against suspected collusion.
#[test]
fn auc_03_withholding_is_indistinguishable_from_failure() {
    let mut a = SealedBidAuction::new(lot(None), Currency::EUR);
    commit_at(&mut a, "alice", 100, 1, T0);
    commit_at(&mut a, "bob", 90, 2, T0);
    reveal_at(&mut a, "alice", 100, 1, T0 + 150).expect("reveal");
    let r = a.settle();

    // Nothing in the result distinguishes why Bob did not reveal.
    assert_eq!(r.unrevealed_commitments, 1);
    let as_json = serde_json::to_string(&r).expect("result serialises");
    assert!(
        !as_json.contains("reason") && !as_json.contains("suspect"),
        "the result must not editorialise about why a commitment went \
         unrevealed; it cannot know"
    );
}

/// AUC-04. A lot whose phases do not overlap correctly is refused
/// outright, rather than accepting bids that can never be opened.
#[test]
fn auc_04_a_malformed_window_is_refused() {
    let mut bad = lot(None);
    bad.reveal_closes = bad.commit_closes; // no reveal window at all
    let mut a = SealedBidAuction::new(bad, Currency::EUR);

    let alice = org("alice");
    // A lot with no reveal window must not accept commitments it can
    // never let anyone open.
    assert!(matches!(
        a.commit(BidCommitment {
            bidder: alice.clone(),
            commitment: bid_commitment(&alice, eur(100), &[1; 32]),
            submitted_at: Timestamp::from_secs(T0),
        }),
        Err(AuctionError::InvalidWindow(_))
    ));
    assert!(matches!(
        reveal_at(&mut a, "alice", 100, 1, T0 + 150),
        Err(AuctionError::InvalidWindow(_))
    ));
}
