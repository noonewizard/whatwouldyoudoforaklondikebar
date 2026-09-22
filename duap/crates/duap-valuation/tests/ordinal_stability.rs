//! The multiplier set must remain an *ordinal* instrument.
//!
//! `research/valuation-sensitivity/` measured what `VALUATION.md` had only
//! asserted: across defensible coefficient policies the *amount* moves by
//! a median factor of 11.3, while the *ranking* holds at Spearman
//! +0.686..+0.987 with unanimous agreement on the ten most expensive keys.
//!
//! That finding is what licenses every comparative claim the project
//! makes — "this usage costs more than that one" — and it is a property of
//! the shipped coefficients, not a theorem. A change to
//! `MultiplierPolicy::default` could quietly destroy it.
//!
//! These tests pin it. They are deliberately looser than the measured
//! values, so ordinary re-tuning does not fail the build and a change that
//! breaks the ordinal claim does.

use duap_meter::aggregate::{ScopeTag, UsageCounter, UsageKey};
use duap_model::prelude::*;
use duap_valuation::multiplier::{MultiplierContext, MultiplierPolicy};
use duap_valuation::{PriceEngine, PricingInputs};

const BASE_NMU: i128 = 1_000_000;
const QUANTITY: u64 = 1_000;
const T0: u64 = 1_700_000_000;

fn r(n: i128, d: u64) -> Ratio {
    Ratio::new(n, d).expect("non-zero denominator")
}

/// Rival policies, matching the experiment's non-null set. Each is a
/// position someone could hold in good faith.
fn rivals() -> Vec<(&'static str, MultiplierPolicy)> {
    let d = MultiplierPolicy::default;
    vec![
        ("shipped", d()),
        (
            "linear-sensitivity",
            MultiplierPolicy {
                sensitivity: [r(1, 1), r(2, 1), r(3, 1), r(4, 1), r(5, 1)],
                ..d()
            },
        ),
        (
            "steep-sensitivity",
            MultiplierPolicy {
                sensitivity: [r(1, 4), r(1, 1), r(4, 1), r(16, 1), r(64, 1)],
                ..d()
            },
        ),
        (
            "risk-led",
            MultiplierPolicy {
                sensitivity: [r(1, 1); 5],
                reid_step: r(5, 4),
                ..d()
            },
        ),
        (
            "retention-led",
            MultiplierPolicy {
                sensitivity: [r(1, 1), r(1, 1), r(3, 2), r(2, 1), r(5, 2)],
                indefinite_retention: r(8, 1),
                long_retention: r(4, 1),
                ..d()
            },
        ),
        (
            "regulator-flavoured",
            MultiplierPolicy {
                special_category: r(4, 1),
                cross_border: r(3, 1),
                ..d()
            },
        ),
        (
            "no-decay",
            MultiplierPolicy {
                freshness_half_life_days: 0,
                ..d()
            },
        ),
    ]
}

struct Profile {
    retention: Option<RetentionPolicy>,
    exclusive: bool,
    age_days: u32,
    cross_border: bool,
}

fn profiles() -> Vec<Profile> {
    let fixed = |d: u32| {
        Some(RetentionPolicy {
            basis: RetentionBasis::FixedPeriod,
            days: Some(d),
            until: None,
        })
    };
    vec![
        Profile {
            retention: fixed(30),
            exclusive: false,
            age_days: 1,
            cross_border: false,
        },
        Profile {
            retention: fixed(1095),
            exclusive: false,
            age_days: 30,
            cross_border: false,
        },
        Profile {
            retention: Some(RetentionPolicy {
                basis: RetentionBasis::Indefinite,
                days: None,
                until: None,
            }),
            exclusive: true,
            age_days: 7,
            cross_border: false,
        },
        Profile {
            retention: fixed(90),
            exclusive: false,
            age_days: 720,
            cross_border: true,
        },
    ]
}

fn charges(policy: &MultiplierPolicy) -> Vec<f64> {
    let engine = PriceEngine::new(Currency::EUR).with_policy(policy.clone());
    let counter = UsageCounter {
        quantity: QUANTITY,
        event_count: QUANTITY,
        first: Timestamp::from_secs(T0),
        last: Timestamp::from_secs(T0 + 3600),
        evidence_root: duap_canon::Digest::of(
            duap_canon::HashAlg::Sha2_256,
            "duap.usage-counter.v1",
            b"synthetic",
        ),
        evidence_size: QUANTITY,
    };
    let rule = PricingRule::per_unit(Unit::Query, Precise::new(Currency::EUR, BASE_NMU));
    let mut out = Vec::new();
    for class in DataClass::ALL {
        for p in &profiles() {
            let key = UsageKey {
                controller: "org:duap/acme-example-corp".parse().expect("valid"),
                processor: None,
                subject: Some(SubjectRef([7u8; 16])),
                scope_tag: ScopeTag::Subject,
                data_class: class,
                operation: Operation::AccessQuery,
                purpose: Purpose::ServiceCore,
                country: "DE".to_owned(),
                unit: Unit::Query,
                window_start: Timestamp::from_secs(T0),
            };
            let inputs = PricingInputs {
                exclusive: p.exclusive,
                retention: p.retention,
                multiplier_ctx: MultiplierContext {
                    age_days: Some(p.age_days),
                    cross_border: p.cross_border,
                },
                ..Default::default()
            };
            out.push(
                engine
                    .price(&key, &counter, &rule, &inputs)
                    .expect("the corpus prices under every rival policy")
                    .amount
                    .nmu as f64,
            );
        }
    }
    out
}

fn ranks(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&i, &j| v[i].partial_cmp(&v[j]).expect("no NaN"));
    let mut out = vec![0.0; v.len()];
    let mut i = 0;
    while i < idx.len() {
        let mut j = i;
        while j + 1 < idx.len() && v[idx[j + 1]] == v[idx[i]] {
            j += 1;
        }
        let avg = ((i + j) as f64) / 2.0 + 1.0;
        for &k in &idx[i..=j] {
            out[k] = avg;
        }
        i = j + 1;
    }
    out
}

fn spearman(a: &[f64], b: &[f64]) -> f64 {
    let (ra, rb) = (ranks(a), ranks(b));
    let n = ra.len() as f64;
    let (ma, mb) = (ra.iter().sum::<f64>() / n, rb.iter().sum::<f64>() / n);
    let mut num = 0.0;
    let (mut da, mut db) = (0.0, 0.0);
    for i in 0..ra.len() {
        let (x, y) = (ra[i] - ma, rb[i] - mb);
        num += x * y;
        da += x * x;
        db += y * y;
    }
    num / (da.sqrt() * db.sqrt())
}

fn top_k(v: &[f64], k: usize) -> std::collections::BTreeSet<usize> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&i, &j| v[j].partial_cmp(&v[i]).expect("no NaN"));
    idx.into_iter().take(k).collect()
}

/// Every pair of defensible policies must still agree about rank.
///
/// Measured minimum was +0.686. The floor here is +0.60: loose enough for
/// ordinary re-tuning, tight enough that a change destroying the ordinal
/// claim fails rather than silently invalidating every comparative
/// statement in `VALUATION.md`.
#[test]
fn rival_policies_agree_on_rank() {
    let pols = rivals();
    let charged: Vec<(&str, Vec<f64>)> = pols.iter().map(|(n, p)| (*n, charges(p))).collect();

    let mut worst = (f64::INFINITY, String::new());
    for i in 0..charged.len() {
        for j in (i + 1)..charged.len() {
            let rho = spearman(&charged[i].1, &charged[j].1);
            assert!(
                rho.is_finite(),
                "{} vs {}: rank correlation is undefined, which means one \
                 policy produced a constant ranking",
                charged[i].0,
                charged[j].0
            );
            if rho < worst.0 {
                worst = (rho, format!("{} vs {}", charged[i].0, charged[j].0));
            }
        }
    }
    assert!(
        worst.0 >= 0.60,
        "ordinal agreement has degraded: worst pair {} at rho {:.3}, floor \
         0.60 (measured +0.686 on 2026-09-22). Every comparative claim in \
         VALUATION.md depends on this holding; see \
         research/valuation-sensitivity/RESULTS.md",
        worst.1,
        worst.0
    );
}

/// The policies must still agree which usages are the expensive ones.
///
/// Measured overlap was unanimous (10 of 10) for every pair. The floor is
/// 7 of 10 — this is the claim a rights-holder relies on most directly,
/// so it is pinned tighter than the correlation.
#[test]
fn rival_policies_agree_on_the_most_expensive_usages() {
    let pols = rivals();
    let charged: Vec<(&str, Vec<f64>)> = pols.iter().map(|(n, p)| (*n, charges(p))).collect();
    let k = 10usize;

    for i in 0..charged.len() {
        for j in (i + 1)..charged.len() {
            let a = top_k(&charged[i].1, k);
            let b = top_k(&charged[j].1, k);
            let shared = a.intersection(&b).count();
            assert!(
                shared >= 7,
                "{} and {} now agree on only {shared} of the top {k} most \
                 expensive usages (measured: {k} of {k})",
                charged[i].0,
                charged[j].0
            );
        }
    }
}

/// The modifiers must do something. A policy set that prices everything
/// identically would pass the rank tests trivially and be useless.
#[test]
fn the_shipped_multipliers_concentrate_charge() {
    let c = charges(&MultiplierPolicy::default());
    let mut sorted = c.clone();
    sorted.sort_by(|a, b| b.partial_cmp(a).expect("no NaN"));
    let decile = (sorted.len() as f64 * 0.1).ceil() as usize;
    let share: f64 = sorted[..decile].iter().sum::<f64>() / sorted.iter().sum::<f64>();
    assert!(
        share > 0.2,
        "the top decile takes only {share:.3} of total charge; a uniform \
         policy gives 0.1, so the multipliers are barely doing anything"
    );
}
