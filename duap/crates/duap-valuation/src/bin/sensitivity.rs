//! Does the choice of multiplier policy change who pays?
//!
//! STATUS: EXPERIMENTAL. Results are provisional and the corpus is
//! synthetic.
//!
//! # The question
//!
//! `VALUATION.md` says the default coefficients are "ordinally defensible
//! and nothing stronger", and `MultiplierPolicy`'s own documentation says
//! nothing in it claims to know what data is worth. Both statements were
//! asserted. This measures them.
//!
//! `research/ai-attribution/` did the equivalent job for attribution and
//! got a negative answer: defensible estimators disagree about rank badly
//! enough that no two of them would pay the same people. The pricing
//! question has the same shape and had never been asked.
//!
//! Two sub-questions, which can come apart:
//!
//! * **Cardinal.** How much does the *amount* move between policies? If a
//!   great deal, no artefact may present a multiplier product as a price.
//! * **Ordinal.** Does the *ranking* survive? If two parties using
//!   different defensible policies still agree who owes more than whom,
//!   the multiplier set is usable as a comparative instrument even though
//!   it is useless as an absolute one. That would be a genuinely useful
//!   positive result, and it is the claim `VALUATION.md` actually makes.
//!
//! # Method
//!
//! Every one of the 61 data classes in the ontology, crossed with four
//! usage profiles that vary the things the multipliers key on (retention,
//! exclusivity, freshness, cross-border). 244 usage keys. Each priced by
//! the real `PriceEngine` under eight policies, each defensible and none
//! obviously right.
//!
//! The base price and quantity are held constant across policies on
//! purpose: they are negotiated inputs, and letting them vary would
//! measure the negotiation rather than the coefficients.
//!
//! Run: `cargo run -p duap-valuation --bin sensitivity`

use duap_meter::aggregate::{ScopeTag, UsageCounter, UsageKey};
use duap_model::prelude::*;
use duap_valuation::multiplier::{MultiplierContext, MultiplierPolicy};
use duap_valuation::{PriceEngine, PricingInputs};

const BASE_NMU: i128 = 1_000_000; // 0.001 minor units per unit
const QUANTITY: u64 = 1_000;
const T0: u64 = 1_700_000_000;

fn r(n: i128, d: u64) -> Ratio {
    Ratio::new(n, d).expect("non-zero denominator")
}

/// Eight coefficient policies. Each is a position someone could argue for
/// in good faith; none is a straw man, and the first is what ships.
fn policies() -> Vec<(&'static str, &'static str, MultiplierPolicy)> {
    let d = MultiplierPolicy::default;
    vec![
        ("shipped", "The defaults in MultiplierPolicy::default.", d()),
        (
            "flat",
            "Every multiplier 1. The null hypothesis: modifiers add nothing \
             and only quantity and the negotiated base price matter.",
            MultiplierPolicy {
                sensitivity: [Ratio::ONE; 5],
                reid_step: Ratio::ONE,
                exclusivity: Ratio::ONE,
                indefinite_retention: Ratio::ONE,
                long_retention: Ratio::ONE,
                cross_border: Ratio::ONE,
                freshness_half_life_days: 0,
                freshness_floor: Ratio::ONE,
                special_category: Ratio::ONE,
            },
        ),
        (
            "linear-sensitivity",
            "Sensitivity 1..5 rather than doubling. Same ordinal direction, \
             different curvature -- the disagreement a reasonable person \
             would have with the shipped defaults.",
            MultiplierPolicy {
                sensitivity: [r(1, 1), r(2, 1), r(3, 1), r(4, 1), r(5, 1)],
                ..d()
            },
        ),
        (
            "steep-sensitivity",
            "Sensitivity quadrupling rather than doubling. Someone who \
             thinks severe data should be priced punitively.",
            MultiplierPolicy {
                sensitivity: [r(1, 4), r(1, 1), r(4, 1), r(16, 1), r(64, 1)],
                ..d()
            },
        ),
        (
            "risk-led",
            "Price re-identification risk rather than category. Flat \
             sensitivity, steep reid step.",
            MultiplierPolicy {
                sensitivity: [r(1, 1); 5],
                reid_step: r(5, 4),
                ..d()
            },
        ),
        (
            "retention-led",
            "Price how long they keep it rather than what it is.",
            MultiplierPolicy {
                sensitivity: [r(1, 1), r(1, 1), r(3, 2), r(2, 1), r(5, 2)],
                indefinite_retention: r(8, 1),
                long_retention: r(4, 1),
                ..d()
            },
        ),
        (
            "regulator-flavoured",
            "Heavy on special category and cross-border transfer; the \
             shape a privacy regulator's intuitions would produce.",
            MultiplierPolicy {
                special_category: r(4, 1),
                cross_border: r(3, 1),
                ..d()
            },
        ),
        (
            "no-decay",
            "The shipped policy with freshness decay switched off. Someone \
             who thinks historical data is as valuable as fresh.",
            MultiplierPolicy {
                freshness_half_life_days: 0,
                ..d()
            },
        ),
    ]
}

#[derive(Clone, Copy)]
struct Profile {
    name: &'static str,
    retention: Option<(RetentionBasis, Option<u32>)>,
    exclusive: bool,
    age_days: u32,
    cross_border: bool,
}

fn profiles() -> Vec<Profile> {
    vec![
        Profile {
            name: "routine",
            retention: Some((RetentionBasis::FixedPeriod, Some(30))),
            exclusive: false,
            age_days: 1,
            cross_border: false,
        },
        Profile {
            name: "long-retention",
            retention: Some((RetentionBasis::FixedPeriod, Some(1095))),
            exclusive: false,
            age_days: 30,
            cross_border: false,
        },
        Profile {
            name: "exclusive-indefinite",
            retention: Some((RetentionBasis::Indefinite, None)),
            exclusive: true,
            age_days: 7,
            cross_border: false,
        },
        Profile {
            name: "stale-cross-border",
            retention: Some((RetentionBasis::FixedPeriod, Some(90))),
            exclusive: false,
            age_days: 720,
            cross_border: true,
        },
    ]
}

fn key_for(class: DataClass) -> UsageKey {
    UsageKey {
        controller: "org:duap/acme-example-corp".parse().expect("valid org id"),
        processor: None,
        subject: Some(SubjectRef([7u8; 16])),
        scope_tag: ScopeTag::Subject,
        data_class: class,
        operation: Operation::AccessQuery,
        purpose: Purpose::ServiceCore,
        country: "DE".to_owned(),
        unit: Unit::Query,
        window_start: Timestamp::from_secs(T0),
    }
}

fn counter() -> UsageCounter {
    UsageCounter {
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
    }
}

/// Spearman rank correlation. Ties get average ranks.
fn spearman(a: &[f64], b: &[f64]) -> f64 {
    let ra = ranks(a);
    let rb = ranks(b);
    let n = ra.len() as f64;
    let ma = ra.iter().sum::<f64>() / n;
    let mb = rb.iter().sum::<f64>() / n;
    let mut num = 0.0;
    let mut da = 0.0;
    let mut db = 0.0;
    for i in 0..ra.len() {
        let x = ra[i] - ma;
        let y = rb[i] - mb;
        num += x * y;
        da += x * x;
        db += y * y;
    }
    if da == 0.0 || db == 0.0 {
        return f64::NAN;
    }
    num / (da.sqrt() * db.sqrt())
}

fn ranks(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&i, &j| v[i].partial_cmp(&v[j]).expect("no NaN in charges"));
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

/// Fraction of pairs that both policies order strictly, and order
/// *oppositely*.
///
/// Ties are excluded rather than counted as disagreement. That matters:
/// the `flat` policy charges every key the same, so every pair is a tie
/// under it. Counting a tie against a strict order as an inversion would
/// report flat as disagreeing with everything about everything, which is
/// the opposite of what a constant ranking means.
fn strict_inversion_rate(a: &[f64], b: &[f64]) -> f64 {
    use std::cmp::Ordering::{Greater, Less};
    let n = a.len();
    let mut inverted = 0usize;
    let mut comparable = 0usize;
    for i in 0..n {
        for j in (i + 1)..n {
            let oa = a[i].partial_cmp(&a[j]).expect("no NaN");
            let ob = b[i].partial_cmp(&b[j]).expect("no NaN");
            let both_strict = matches!(oa, Less | Greater) && matches!(ob, Less | Greater);
            if both_strict {
                comparable += 1;
                if oa != ob {
                    inverted += 1;
                }
            }
        }
    }
    if comparable == 0 {
        f64::NAN
    } else {
        inverted as f64 / comparable as f64
    }
}

/// Indices of the `k` most expensive keys.
fn top_k(v: &[f64], k: usize) -> std::collections::BTreeSet<usize> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&i, &j| v[j].partial_cmp(&v[i]).expect("no NaN"));
    idx.into_iter().take(k).collect()
}

/// True when every entry is identical -- a ranking with no information,
/// against which rank correlation is undefined rather than zero.
fn is_constant(v: &[f64]) -> bool {
    v.windows(2).all(|w| w[0] == w[1])
}

/// Share of total charge taken by the most expensive decile of keys.
fn top_decile_share(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| b.partial_cmp(a).expect("no NaN"));
    let k = (s.len() as f64 * 0.1).ceil() as usize;
    let total: f64 = s.iter().sum();
    if total == 0.0 {
        return f64::NAN;
    }
    s[..k].iter().sum::<f64>() / total
}

fn main() {
    let pols = policies();
    let profs = profiles();
    let counter = counter();

    // charges[policy][key]
    let mut charges: Vec<Vec<f64>> = Vec::new();
    let mut labels: Vec<String> = Vec::new();
    let mut first = true;

    for (_, _, policy) in &pols {
        let engine = PriceEngine::new(Currency::EUR).with_policy(policy.clone());
        let mut row = Vec::new();
        for class in DataClass::ALL {
            for p in &profs {
                let key = key_for(class);
                let inputs = PricingInputs {
                    exclusive: p.exclusive,
                    retention: p.retention.map(|(basis, days)| RetentionPolicy {
                        basis,
                        days,
                        until: None,
                    }),
                    multiplier_ctx: MultiplierContext {
                        age_days: Some(p.age_days),
                        cross_border: p.cross_border,
                    },
                    ..Default::default()
                };
                let rule =
                    PricingRule::per_unit(Unit::Query, Precise::new(Currency::EUR, BASE_NMU));
                let b = engine
                    .price(&key, &counter, &rule, &inputs)
                    .expect("the synthetic corpus prices under every policy");
                row.push(b.amount.nmu as f64);
                if first {
                    labels.push(format!("{}/{}", class.code(), p.name));
                }
            }
        }
        first = false;
        charges.push(row);
    }

    let n_keys = charges[0].len();

    // --- Cardinal spread: same key, different policy ----------------------
    //
    // Reported twice. Including `flat` measures "how much do the modifiers
    // move the price at all", which is inflated by construction because
    // flat is the floor. Excluding it measures the disagreement among
    // genuine rivals, which is the number that matters.
    let spread = |include_flat: bool| -> (f64, f64, f64, String) {
        let rows: Vec<&Vec<f64>> = pols
            .iter()
            .zip(&charges)
            .filter(|((n, _, _), _)| include_flat || *n != "flat")
            .map(|(_, c)| c)
            .collect();
        let mut ratios: Vec<f64> = Vec::with_capacity(n_keys);
        for k in 0..n_keys {
            let vals: Vec<f64> = rows.iter().map(|c| c[k]).collect();
            let lo = vals.iter().cloned().fold(f64::INFINITY, f64::min);
            let hi = vals.iter().cloned().fold(0.0, f64::max);
            ratios.push(if lo > 0.0 { hi / lo } else { f64::INFINITY });
        }
        let mut sorted = ratios.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).expect("no NaN"));
        let worst_key = labels[ratios
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).expect("no NaN"))
            .expect("non-empty")
            .0]
            .clone();
        (
            sorted[sorted.len() / 2],
            sorted[(sorted.len() as f64 * 0.9) as usize],
            sorted[sorted.len() - 1],
            worst_key,
        )
    };
    let (median_all, _, worst_all, _) = spread(true);
    let (median, p90, worst, worst_key) = spread(false);

    // --- Ordinal agreement ------------------------------------------------
    let mut pairs = Vec::new();
    for i in 0..pols.len() {
        for j in (i + 1)..pols.len() {
            pairs.push((
                pols[i].0,
                pols[j].0,
                spearman(&charges[i], &charges[j]),
                strict_inversion_rate(&charges[i], &charges[j]),
            ));
        }
    }
    // `flat` charges every key identically, so rank correlation against it
    // is undefined rather than low. It is kept in the corpus as the null
    // hypothesis -- it answers "do the modifiers do anything at all" -- and
    // excluded from every rank statistic.
    let defined: Vec<&(&str, &str, f64, f64)> = pairs.iter().filter(|p| p.2.is_finite()).collect();
    let rho_min = defined.iter().map(|p| p.2).fold(f64::INFINITY, f64::min);
    let rho_mean = defined.iter().map(|p| p.2).sum::<f64>() / defined.len() as f64;
    let worst_pair = defined
        .iter()
        .min_by(|a, b| a.2.partial_cmp(&b.2).expect("finite"))
        .expect("at least one defined pair");
    let worst_pair_label = format!("{} vs {}", worst_pair.0, worst_pair.1);
    let inv_max = defined
        .iter()
        .map(|p| p.3)
        .filter(|x| x.is_finite())
        .fold(0.0, f64::max);

    // Top-10 overlap: do the policies agree which keys are the expensive
    // ones? This is the pricing analogue of the attribution experiment's
    // top-3 overlap, and it is the question a rights-holder asks first.
    let k = 10usize;
    let mut overlap_min = 1.0f64;
    for i in 0..pols.len() {
        for j in (i + 1)..pols.len() {
            if is_constant(&charges[i]) || is_constant(&charges[j]) {
                continue;
            }
            let a = top_k(&charges[i], k);
            let b = top_k(&charges[j], k);
            let shared = a.intersection(&b).count() as f64 / k as f64;
            overlap_min = overlap_min.min(shared);
        }
    }

    // --- Portfolio share shift -------------------------------------------
    // If two parties price the same portfolio under different policies, how
    // much does each key's SHARE of the total move? This is the question
    // that matters for paying people.
    let mut max_share_shift = 0.0f64;
    let mut max_share_key = String::new();
    let totals: Vec<f64> = charges.iter().map(|c| c.iter().sum()).collect();
    for k in 0..n_keys {
        let shares: Vec<f64> = charges
            .iter()
            .zip(&totals)
            .map(|(c, t)| if *t > 0.0 { c[k] / t } else { 0.0 })
            .collect();
        let lo = shares.iter().cloned().fold(f64::INFINITY, f64::min);
        let hi = shares.iter().cloned().fold(0.0, f64::max);
        if hi - lo > max_share_shift {
            max_share_shift = hi - lo;
            max_share_key = labels[k].clone();
        }
    }

    // --- Output -----------------------------------------------------------
    let mut out = serde_json::Map::new();
    out.insert("keys".into(), (n_keys as u64).into());
    out.insert("classes".into(), (DataClass::ALL.len() as u64).into());
    out.insert("profiles".into(), (profs.len() as u64).into());
    out.insert("policies".into(), (pols.len() as u64).into());
    out.insert(
        "policy_descriptions".into(),
        serde_json::Value::Object(
            pols.iter()
                .map(|(n, d, _)| ((*n).to_owned(), serde_json::Value::String((*d).to_owned())))
                .collect(),
        ),
    );
    out.insert("cardinal_ratio_median".into(), round3(median).into());
    out.insert("cardinal_ratio_p90".into(), round3(p90).into());
    out.insert("cardinal_ratio_max".into(), round3(worst).into());
    out.insert("cardinal_ratio_max_key".into(), worst_key.into());
    out.insert(
        "cardinal_ratio_median_incl_flat".into(),
        round3(median_all).into(),
    );
    out.insert(
        "cardinal_ratio_max_incl_flat".into(),
        round3(worst_all).into(),
    );
    out.insert("spearman_min".into(), round3(rho_min).into());
    out.insert("spearman_mean".into(), round3(rho_mean).into());
    out.insert("spearman_min_pair".into(), worst_pair_label.into());
    out.insert("strict_inversion_rate_max".into(), round3(inv_max).into());
    out.insert("top10_overlap_min".into(), round3(overlap_min).into());
    out.insert(
        "share_shift_max_pp".into(),
        round3(max_share_shift * 100.0).into(),
    );
    out.insert(
        "share_shift_max_as_multiple_of_mean_share".into(),
        round3(max_share_shift * n_keys as f64).into(),
    );
    out.insert("share_shift_max_key".into(), max_share_key.into());
    out.insert(
        "note_flat".into(),
        serde_json::Value::String(
            "The `flat` policy charges every key identically, so rank \
             statistics against it are undefined and are excluded. It is \
             retained as the null hypothesis: it answers whether the \
             modifiers change anything at all."
                .to_owned(),
        ),
    );
    out.insert(
        "top_decile_share".into(),
        serde_json::Value::Object(
            pols.iter()
                .zip(&charges)
                .map(|((n, _, _), c)| {
                    (
                        (*n).to_owned(),
                        serde_json::Value::from(round3(top_decile_share(c))),
                    )
                })
                .collect(),
        ),
    );
    out.insert(
        "pairs".into(),
        serde_json::Value::Array(
            pairs
                .iter()
                .map(|(a, b, rho, inv)| {
                    serde_json::json!({
                        "a": a, "b": b,
                        "spearman": if rho.is_finite() { serde_json::Value::from(round3(*rho)) } else { serde_json::Value::Null },
                        "strict_inversion_rate": if inv.is_finite() { serde_json::Value::from(round3(*inv)) } else { serde_json::Value::Null },
                    })
                })
                .collect(),
        ),
    );

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::Value::Object(out)).expect("results serialise")
    );
}

fn round3(x: f64) -> f64 {
    (x * 1000.0).round() / 1000.0
}
