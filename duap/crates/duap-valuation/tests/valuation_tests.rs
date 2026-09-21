//! Tests for valuation, auctions and distribution.

use duap_meter::{ScopeTag, UsageCounter, UsageKey};
use duap_model::prelude::*;
use duap_valuation::*;
use proptest::prelude::*;
use std::collections::BTreeMap;

const T0: u64 = 1_750_000_000;

fn key(class: DataClass, unit: Unit) -> UsageKey {
    UsageKey {
        controller: "org:duap/acme".parse().unwrap(),
        processor: None,
        subject: Some(SubjectRef([1u8; 16])),
        scope_tag: ScopeTag::Subject,
        data_class: class,
        operation: Operation::AccessQuery,
        purpose: Purpose::ServiceCore,
        country: "DE".into(),
        unit,
        window_start: Timestamp::from_secs(T0),
    }
}

fn counter(n: u64) -> UsageCounter {
    UsageCounter {
        quantity: n,
        event_count: n,
        first: Timestamp::from_secs(T0),
        last: Timestamp::from_secs(T0 + 3600),
        evidence_root: duap_canon::Digest::of(duap_canon::HashAlg::Sha2_256, "x", b"y"),
        evidence_size: n,
    }
}

// ---------------------------------------------------------------------------
// Engine
// ---------------------------------------------------------------------------

#[test]
fn per_unit_pricing_is_exact() {
    let e = PriceEngine::new(Currency::EUR);
    let k = key(DataClass::DerivedAggregate, Unit::Query); // t1, no multipliers above 1
    let rule = PricingRule::PerUnit {
        unit: Unit::Query,
        // 1_250_000 nmu = 0.00125 minor units = 0.0000125 EUR per query.
        unit_price: Precise::new(Currency::EUR, 1_250_000),
    };
    let b = e.price(&k, &counter(1_000_000), &rule, &PricingInputs::default()).unwrap();
    // 1e6 queries x 1.25e6 nmu = 1.25e12 nmu = 1250 minor units = 12.50 EUR.
    assert_eq!(b.amount.nmu, 1_250_000_000_000);
    let (money, residue) = b.amount.round_to_money(Rounding::HalfEven);
    assert_eq!(money, Money::new(Currency::EUR, 1250));
    assert_eq!(residue.nmu, 0);
    assert_eq!(money.to_string(), "12.50 EUR");
}

#[test]
fn multipliers_are_itemised_and_explainable() {
    let e = PriceEngine::new(Currency::EUR);
    let k = key(DataClass::LocationPrecise, Unit::Query); // t4, reid 92, special category
    let rule = PricingRule::PerUnit {
        unit: Unit::Query,
        unit_price: Precise::new(Currency::EUR, 1_000_000),
    };
    let inputs = PricingInputs {
        exclusive: true,
        retention: Some(RetentionPolicy {
            basis: RetentionBasis::Indefinite,
            days: None,
            until: None,
        }),
        ..Default::default()
    };
    let b = e.price(&k, &counter(1), &rule, &inputs).unwrap();
    let names: Vec<&str> = b.multipliers.iter().map(|m| m.name.as_str()).collect();
    assert!(names.contains(&"sensitivity"));
    assert!(names.contains(&"reidentification"));
    assert!(names.contains(&"special_category"));
    assert!(names.contains(&"exclusivity"));
    assert!(names.contains(&"retention"));
    // Recomputing from the itemisation must reproduce the combined factor.
    let mut f = Ratio::ONE;
    for m in &b.multipliers {
        f = f.mul(m.factor).unwrap();
    }
    assert_eq!(f, b.combined_factor);
    assert_eq!(
        b.amount.nmu,
        Precise::new(Currency::EUR, 1_000_000).mul_ratio(b.combined_factor).unwrap().nmu
    );
}

#[test]
fn sensitivity_ordering_is_monotone_in_price() {
    let e = PriceEngine::new(Currency::EUR);
    let rule = PricingRule::PerUnit {
        unit: Unit::Query,
        unit_price: Precise::new(Currency::EUR, 1_000_000),
    };
    let mut last = 0i128;
    for class in [
        DataClass::SensorEnvironmental,   // t0
        DataClass::DeviceConfiguration,   // t1
        DataClass::LocationCoarse,        // t2
        DataClass::ContentUserGenerated,  // t3
        DataClass::HealthClinical,        // t4
    ] {
        let b = e
            .price(&key(class, Unit::Query), &counter(1), &rule, &PricingInputs::default())
            .unwrap();
        assert!(
            b.amount.nmu > last,
            "{} priced at {} is not above the previous tier",
            class.code(),
            b.amount.nmu
        );
        last = b.amount.nmu;
    }
}

#[test]
fn tiered_pricing_is_marginal_not_cliff_edged() {
    let e = PriceEngine::new(Currency::EUR);
    let k = key(DataClass::DerivedAggregate, Unit::Record);
    let rule = PricingRule::Tiered {
        unit: Unit::Record,
        tiers: vec![
            Tier { up_to: Some(1_000), unit_price: Precise::new(Currency::EUR, 1_000_000) },
            Tier { up_to: Some(10_000), unit_price: Precise::new(Currency::EUR, 500_000) },
            Tier { up_to: None, unit_price: Precise::new(Currency::EUR, 100_000) },
        ],
    };
    // 1000 at 1e6 + 9000 at 5e5 + 5000 at 1e5 = 1e9 + 4.5e9 + 5e8 = 6e9
    let b = e.price(&k, &counter(15_000), &rule, &PricingInputs::default()).unwrap();
    assert_eq!(b.amount.nmu, 6_000_000_000);

    // Crossing a boundary by one unit must raise the total by exactly the
    // next tier's rate, not reprice the whole volume.
    let a = e.price(&k, &counter(1_000), &rule, &PricingInputs::default()).unwrap();
    let c = e.price(&k, &counter(1_001), &rule, &PricingInputs::default()).unwrap();
    assert_eq!(c.amount.nmu - a.amount.nmu, 500_000);
}

#[test]
fn revenue_share_needs_declared_revenue() {
    let e = PriceEngine::new(Currency::EUR);
    let k = key(DataClass::DerivedAggregate, Unit::Query);
    let rule = PricingRule::RevenueShare {
        share: Ratio::from_bps(500), // 5%
        floor: None,
    };
    assert!(matches!(
        e.price(&k, &counter(1), &rule, &PricingInputs::default()),
        Err(PricingError::RevenueNotDeclared)
    ));

    let inputs = PricingInputs {
        declared_revenue: Some(Money::new(Currency::EUR, 100_00)), // 100.00 EUR
        ..Default::default()
    };
    let b = e.price(&k, &counter(1), &rule, &inputs).unwrap();
    // 5% of 10000 minor units = 500 minor = 5e11 nmu, times the t1 multiplier 1.
    assert_eq!(b.amount.nmu, 500 * NANO);
}

#[test]
fn unit_mismatch_is_refused() {
    let e = PriceEngine::new(Currency::EUR);
    let k = key(DataClass::DerivedAggregate, Unit::Query);
    let rule = PricingRule::PerUnit {
        unit: Unit::Token,
        unit_price: Precise::new(Currency::EUR, 1),
    };
    assert!(matches!(
        e.price(&k, &counter(1), &rule, &PricingInputs::default()),
        Err(PricingError::UnitMismatch { .. })
    ));
}

#[test]
fn minimum_price_obligation_raises_the_result() {
    let e = PriceEngine::new(Currency::EUR);
    let k = key(DataClass::DerivedAggregate, Unit::Query);
    let rule = PricingRule::PerUnit {
        unit: Unit::Query,
        unit_price: Precise::new(Currency::EUR, 1),
    };
    let inputs = PricingInputs {
        minimum_unit_price: Some(Precise::new(Currency::EUR, 1_000)),
        ..Default::default()
    };
    let b = e.price(&k, &counter(10), &rule, &inputs).unwrap();
    assert!(b.floored);
    assert_eq!(b.amount.nmu, 10_000);
}

#[test]
fn freshness_decays_and_is_floored() {
    let p = MultiplierPolicy::default();
    assert_eq!(p.freshness_factor(0).unwrap(), Ratio::ONE);
    assert_eq!(p.freshness_factor(180).unwrap(), Ratio::new(1, 2).unwrap());
    assert_eq!(p.freshness_factor(360).unwrap(), Ratio::new(1, 4).unwrap());
    // Monotone non-increasing.
    let mut last = Ratio::ONE;
    for d in (0..2000).step_by(17) {
        let f = p.freshness_factor(d).unwrap();
        assert!(
            f.num * last.den as i128 <= last.num * f.den as i128,
            "freshness rose at day {d}"
        );
        last = f;
    }
    // Never below the floor.
    assert_eq!(p.freshness_factor(100_000).unwrap(), p.freshness_floor);
}

#[test]
fn free_is_distinct_from_unpriced() {
    let e = PriceEngine::new(Currency::EUR);
    let k = key(DataClass::HealthClinical, Unit::Query);
    let b = e
        .price(&k, &counter(1000), &PricingRule::Free, &PricingInputs::default())
        .unwrap();
    assert_eq!(b.amount.nmu, 0);
    assert_eq!(b.rule, "free");
    assert_eq!(b.quantity, 1000, "a free use is still metered");
}

// ---------------------------------------------------------------------------
// Schedules
// ---------------------------------------------------------------------------

fn schedule() -> PricingSchedule {
    PricingSchedule {
        schema: 1,
        id: "sched:acme-2026".into(),
        publisher: "org:duap/acme".parse().unwrap(),
        currency: Currency::EUR,
        effective_from: Timestamp::from_secs(T0),
        effective_until: None,
        entries: vec![
            ScheduleEntry {
                data_class: None,
                namespace: Some("location".into()),
                operation: None,
                purpose: None,
                rule: PricingRule::PerUnit {
                    unit: Unit::Query,
                    unit_price: Precise::new(Currency::EUR, 100),
                },
            },
            ScheduleEntry {
                data_class: Some(DataClass::LocationPrecise),
                namespace: None,
                operation: None,
                purpose: None,
                rule: PricingRule::PerUnit {
                    unit: Unit::Query,
                    unit_price: Precise::new(Currency::EUR, 900),
                },
            },
        ],
        default_rule: Some(PricingRule::Free),
        extensions: Default::default(),
    }
}

#[test]
fn schedule_resolves_most_specific_first() {
    let s = schedule();
    s.validate().unwrap();
    let r = s.resolve(DataClass::LocationPrecise, Operation::AccessQuery, Purpose::ServiceCore);
    assert!(matches!(
        r,
        Some(PricingRule::PerUnit { unit_price, .. }) if unit_price.nmu == 900
    ));
    let r = s.resolve(DataClass::LocationCoarse, Operation::AccessQuery, Purpose::ServiceCore);
    assert!(matches!(
        r,
        Some(PricingRule::PerUnit { unit_price, .. }) if unit_price.nmu == 100
    ));
    let r = s.resolve(DataClass::ContactEmail, Operation::AccessQuery, Purpose::ServiceCore);
    assert!(matches!(r, Some(PricingRule::Free)));
}

#[test]
fn a_schedule_cannot_be_edited_behind_a_pinned_digest() {
    let mut reg = ScheduleRegistry::new();
    let s = schedule();
    let d = reg.publish(s.clone()).unwrap();
    assert!(reg.get(&d).is_some());

    let mut edited = s;
    if let PricingRule::PerUnit { unit_price, .. } = &mut edited.entries[0].rule {
        *unit_price = Precise::new(Currency::EUR, 1);
    }
    let d2 = reg.publish(edited).unwrap();
    assert_ne!(d, d2, "editing a schedule must change its digest");
    // The old digest still resolves to the old content.
    let old = reg.get(&d).unwrap();
    assert!(matches!(
        &old.entries[0].rule,
        PricingRule::PerUnit { unit_price, .. } if unit_price.nmu == 100
    ));
}

// ---------------------------------------------------------------------------
// Auctions
// ---------------------------------------------------------------------------

fn lot() -> Lot {
    Lot {
        id: "lot:1".into(),
        data_class: DataClass::BehaviorWebBrowsing,
        operation: Operation::CommercialAdvertise,
        purpose: Purpose::MarketingAdvertisingBehavioral,
        unit: Unit::Impression,
        quantity: 1_000_000,
        reserve_unit_price: Some(Precise::new(Currency::EUR, 1_000)),
        commit_closes: Timestamp::from_secs(T0 + 100),
        reveal_closes: Timestamp::from_secs(T0 + 200),
    }
}

fn bid(a: &mut SealedBidAuction, who: &str, price: i128, salt: u8) {
    let bidder: OrgId = who.parse().unwrap();
    let p = Precise::new(Currency::EUR, price);
    let s = [salt; 32];
    a.commit(BidCommitment {
        bidder: bidder.clone(),
        commitment: bid_commitment(&bidder, p, &s),
        submitted_at: Timestamp::from_secs(T0 + 10),
    })
    .unwrap();
    a.reveal(BidReveal {
        bidder,
        unit_price: p,
        salt: s,
        revealed_at: Timestamp::from_secs(T0 + 150),
    })
    .unwrap();
}

#[test]
fn second_price_auction_charges_the_runner_up() {
    let mut a = SealedBidAuction::new(lot(), Currency::EUR);
    bid(&mut a, "org:duap/b1", 5_000, 1);
    bid(&mut a, "org:duap/b2", 9_000, 2);
    bid(&mut a, "org:duap/b3", 7_000, 3);
    let r = a.settle();
    assert_eq!(r.winner.unwrap().as_str(), "org:duap/b2");
    assert_eq!(r.clearing_unit_price.unwrap().nmu, 7_000);
    assert_eq!(r.winning_bid.unwrap().nmu, 9_000);
}

#[test]
fn a_lone_bidder_pays_the_reserve() {
    let mut a = SealedBidAuction::new(lot(), Currency::EUR);
    bid(&mut a, "org:duap/b1", 5_000, 1);
    let r = a.settle();
    assert_eq!(r.clearing_unit_price.unwrap().nmu, 1_000);
}

#[test]
fn bids_below_the_reserve_do_not_win() {
    let mut a = SealedBidAuction::new(lot(), Currency::EUR);
    bid(&mut a, "org:duap/b1", 500, 1);
    let r = a.settle();
    assert!(r.winner.is_none());
    assert!(r.clearing_unit_price.is_none());
}

#[test]
fn a_reveal_must_open_its_commitment() {
    let mut a = SealedBidAuction::new(lot(), Currency::EUR);
    let bidder: OrgId = "org:duap/b1".parse().unwrap();
    let committed = Precise::new(Currency::EUR, 5_000);
    a.commit(BidCommitment {
        bidder: bidder.clone(),
        commitment: bid_commitment(&bidder, committed, &[1u8; 32]),
        submitted_at: Timestamp::from_secs(T0 + 10),
    })
    .unwrap();
    // Trying to reveal a different price must fail: this is what stops a
    // bidder changing its mind after seeing the others.
    let r = a.reveal(BidReveal {
        bidder: bidder.clone(),
        unit_price: Precise::new(Currency::EUR, 9_999),
        salt: [1u8; 32],
        revealed_at: Timestamp::from_secs(T0 + 150),
    });
    assert!(matches!(r, Err(auction::AuctionError::RevealMismatch(_))));
}

#[test]
fn unrevealed_commitments_are_counted_not_guessed() {
    let mut a = SealedBidAuction::new(lot(), Currency::EUR);
    bid(&mut a, "org:duap/b1", 5_000, 1);
    let ghost: OrgId = "org:duap/ghost".parse().unwrap();
    a.commit(BidCommitment {
        bidder: ghost.clone(),
        commitment: bid_commitment(&ghost, Precise::new(Currency::EUR, 99_999), &[9u8; 32]),
        submitted_at: Timestamp::from_secs(T0 + 10),
    })
    .unwrap();
    let r = a.settle();
    assert_eq!(r.unrevealed_commitments, 1);
    assert_eq!(r.revealed_bids, 1);
    assert_eq!(
        r.winner.unwrap().as_str(),
        "org:duap/b1",
        "an unrevealed commitment must not influence the outcome"
    );
}

#[test]
fn late_commits_and_reveals_are_refused() {
    let mut a = SealedBidAuction::new(lot(), Currency::EUR);
    let bidder: OrgId = "org:duap/b1".parse().unwrap();
    let p = Precise::new(Currency::EUR, 5_000);
    assert!(matches!(
        a.commit(BidCommitment {
            bidder: bidder.clone(),
            commitment: bid_commitment(&bidder, p, &[1u8; 32]),
            submitted_at: Timestamp::from_secs(T0 + 1_000),
        }),
        Err(auction::AuctionError::CommitClosed(_))
    ));
    a.commit(BidCommitment {
        bidder: bidder.clone(),
        commitment: bid_commitment(&bidder, p, &[1u8; 32]),
        submitted_at: Timestamp::from_secs(T0 + 10),
    })
    .unwrap();
    assert!(matches!(
        a.reveal(BidReveal {
            bidder,
            unit_price: p,
            salt: [1u8; 32],
            revealed_at: Timestamp::from_secs(T0 + 10_000),
        }),
        Err(auction::AuctionError::RevealClosed(_))
    ));
}

#[test]
fn ties_break_deterministically() {
    let mut a = SealedBidAuction::new(lot(), Currency::EUR);
    bid(&mut a, "org:duap/zz", 5_000, 1);
    bid(&mut a, "org:duap/aa", 5_000, 2);
    let r1 = a.settle();
    let r2 = a.settle();
    assert_eq!(r1.winner, r2.winner);
    assert_eq!(r1.winner.unwrap().as_str(), "org:duap/aa");
}

// ---------------------------------------------------------------------------
// Distribution
// ---------------------------------------------------------------------------

#[test]
fn distribution_conserves_the_pool() {
    let mut shares = BTreeMap::new();
    shares.insert("a", Ratio::new(1, 3).unwrap());
    shares.insert("b", Ratio::new(1, 3).unwrap());
    shares.insert("c", Ratio::new(1, 3).unwrap());
    let pool = Money::new(Currency::EUR, 100);
    let (alloc, residual) = distribute(pool, &shares).unwrap();
    let sum: i128 = alloc.iter().map(|a| a.amount.minor).sum();
    assert_eq!(sum + residual.minor, 100);
    // 100/3 = 33.33: two get 33, one gets 34.
    let mut amounts: Vec<i128> = alloc.iter().map(|a| a.amount.minor).collect();
    amounts.sort();
    assert_eq!(amounts, vec![33, 33, 34]);
    assert_eq!(residual.minor, 0);
}

#[test]
fn unallocated_share_stays_with_the_payer() {
    let mut shares = BTreeMap::new();
    shares.insert("a", Ratio::new(1, 4).unwrap());
    shares.insert("b", Ratio::new(1, 4).unwrap());
    let (alloc, residual) = distribute(Money::new(Currency::EUR, 100), &shares).unwrap();
    assert_eq!(alloc.iter().map(|a| a.amount.minor).sum::<i128>(), 50);
    assert_eq!(residual.minor, 50, "the unattributed half is not given away");
}

#[test]
fn every_recipient_is_within_one_unit_of_their_exact_share() {
    let mut shares = BTreeMap::new();
    for i in 0..7u32 {
        shares.insert(i, Ratio::new(1, 7).unwrap());
    }
    let (alloc, _) = distribute(Money::new(Currency::EUR, 1000), &shares).unwrap();
    for a in &alloc {
        let exact = 1000f64 / 7.0;
        assert!((a.amount.minor as f64 - exact).abs() < 1.0);
    }
}

#[test]
fn negative_shares_are_refused() {
    let mut shares = BTreeMap::new();
    shares.insert("a", Ratio::new(-1, 2).unwrap());
    assert!(distribute(Money::new(Currency::EUR, 100), &shares).is_err());
}

// ---------------------------------------------------------------------------
// Micropayments
// ---------------------------------------------------------------------------

#[test]
fn tiny_balances_accumulate_rather_than_vanish() {
    // Threshold 0.05 EUR: five minor units, a realistic floor once rail
    // costs are taken into account.
    let mut acc: PayoutAccumulator<SubjectRef> =
        PayoutAccumulator::new(Currency::EUR, Money::new(Currency::EUR, 5));
    let s = SubjectRef([1u8; 16]);
    // 0.0004 EUR a month == 0.04 minor units == 4e7 nano-minor-units.
    const PER_MONTH: i128 = 40_000_000;
    for m in 0..30 {
        acc.accrue(s, Precise::new(Currency::EUR, PER_MONTH), Timestamp::from_secs(T0 + m * 86_400))
            .unwrap();
    }
    assert!(
        acc.release(Timestamp::from_secs(T0)).unwrap().is_empty(),
        "1.2 minor units is below the 5 minor unit threshold"
    );
    let b = acc.balance(&s).unwrap();
    assert_eq!(b.accrued.nmu, 30 * PER_MONTH);
    assert!(b.accrued.nmu > 0, "the balance is owed even though it is unpaid");

    // Keep accruing until the threshold is crossed: 125 months in total.
    for m in 0..100 {
        acc.accrue(s, Precise::new(Currency::EUR, PER_MONTH), Timestamp::from_secs(T0 + m)).unwrap();
    }
    let payouts = acc.release(Timestamp::from_secs(T0 + 10_000)).unwrap();
    assert_eq!(payouts.len(), 1);
    assert_eq!(payouts[0].amount.minor, 5, "130 x 0.04 == 5.2 minor units, paid as 5");
    assert_eq!(
        acc.balance(&s).unwrap().accrued.nmu,
        200_000_000,
        "the 0.2 minor unit remainder stays on the books"
    );
}

#[test]
fn payout_accumulator_conserves_value() {
    let mut acc: PayoutAccumulator<u32> =
        PayoutAccumulator::new(Currency::EUR, Money::new(Currency::EUR, 10));
    let mut total = 0i128;
    for i in 0..100u32 {
        let amount = 1_234_567_891i128 * (i as i128 + 1);
        total += amount;
        acc.accrue(i, Precise::new(Currency::EUR, amount), Timestamp::from_secs(T0)).unwrap();
    }
    let payouts = acc.release(Timestamp::from_secs(T0 + 1)).unwrap();
    let paid: i128 = payouts.iter().map(|p| p.amount.minor).sum::<i128>() * NANO;
    let outstanding = acc.total_outstanding().unwrap().nmu;
    assert_eq!(
        paid + outstanding,
        total,
        "paying out must not create or destroy value"
    );
}

#[test]
fn dust_is_identifiable() {
    let mut acc: PayoutAccumulator<u32> =
        PayoutAccumulator::new(Currency::EUR, Money::new(Currency::EUR, 100));
    acc.accrue(1, Precise::new(Currency::EUR, 5), Timestamp::from_secs(T0)).unwrap();
    acc.accrue(2, Precise::new(Currency::EUR, 500 * NANO), Timestamp::from_secs(T0)).unwrap();
    let dust = acc.dust(Timestamp::from_secs(T0 + 1)).unwrap();
    assert_eq!(dust.len(), 1);
    assert_eq!(dust[0].0, 1);
}

// ---------------------------------------------------------------------------
// Properties
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Distribution always conserves the pool exactly.
    #[test]
    fn distribution_always_conserves(pool in -1_000_000i128..1_000_000, n in 1usize..20, d in 1u64..50) {
        let mut shares = BTreeMap::new();
        for i in 0..n {
            shares.insert(i as u32, Ratio::new(1, (n as u64) * d).unwrap());
        }
        let (alloc, residual) = distribute(Money::new(Currency::USD, pool), &shares).unwrap();
        let sum: i128 = alloc.iter().map(|a| a.amount.minor).sum();
        prop_assert_eq!(sum + residual.minor, pool);
    }

    /// Pricing is deterministic and monotone in quantity.
    #[test]
    fn price_is_monotone_in_quantity(a in 1u64..10_000, b in 1u64..10_000) {
        let e = PriceEngine::new(Currency::EUR);
        let k = key(DataClass::LocationCoarse, Unit::Query);
        let rule = PricingRule::PerUnit { unit: Unit::Query, unit_price: Precise::new(Currency::EUR, 7) };
        let pa = e.price(&k, &counter(a), &rule, &PricingInputs::default()).unwrap();
        let pb = e.price(&k, &counter(b), &rule, &PricingInputs::default()).unwrap();
        if a <= b { prop_assert!(pa.amount.nmu <= pb.amount.nmu); }
        let again = e.price(&k, &counter(a), &rule, &PricingInputs::default()).unwrap();
        prop_assert_eq!(pa.amount.nmu, again.amount.nmu);
    }

    /// The accumulator never loses value however the accruals interleave.
    #[test]
    fn accumulator_conserves(amounts in proptest::collection::vec(0i128..1_000_000_000_000, 1..40)) {
        let mut acc: PayoutAccumulator<u32> = PayoutAccumulator::new(Currency::EUR, Money::new(Currency::EUR, 1));
        let mut total = 0i128;
        for (i, a) in amounts.iter().enumerate() {
            total += a;
            acc.accrue((i % 5) as u32, Precise::new(Currency::EUR, *a), Timestamp::from_secs(T0)).unwrap();
        }
        let mut paid = 0i128;
        for _ in 0..3 {
            for p in acc.release(Timestamp::from_secs(T0 + 1)).unwrap() {
                paid += p.amount.minor * NANO;
            }
        }
        prop_assert_eq!(paid + acc.total_outstanding().unwrap().nmu, total);
    }

    /// A second-price auction never charges more than the winning bid.
    #[test]
    fn clearing_price_never_exceeds_the_winning_bid(prices in proptest::collection::vec(1_000i128..100_000, 1..8)) {
        let mut a = SealedBidAuction::new(lot(), Currency::EUR);
        for (i, p) in prices.iter().enumerate() {
            bid(&mut a, &format!("org:duap/b{i}"), *p, i as u8);
        }
        let r = a.settle();
        if let (Some(c), Some(w)) = (r.clearing_unit_price, r.winning_bid) {
            prop_assert!(c.nmu <= w.nmu);
            prop_assert!(c.nmu >= 1_000);
        }
    }
}
