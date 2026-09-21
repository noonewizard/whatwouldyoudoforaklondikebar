//! Royalty-pool distribution and the micropayment problem.
//!
//! STATUS: PRODUCTION.
//!
//! # Apportionment
//!
//! Splitting an integer amount by fractional shares cannot generally be done
//! exactly. [`distribute`] uses the largest-remainder (Hamilton) method:
//! floor every share, then hand the remaining units to the largest
//! remainders, breaking ties deterministically. The result satisfies the
//! property that actually matters for accounting -- **the parts sum to the
//! whole, exactly** -- and stays within one minor unit of the ideal share for
//! every recipient.
//!
//! Largest-remainder is known to admit the Alabama paradox (adding to the
//! pool can reduce someone's allocation). For an apportionment of money
//! recomputed every period that is a curiosity rather than a problem, and the
//! alternative divisor methods trade it for a violation of quota, which is
//! worse here: a recipient who can see that they got less than their floor
//! will dispute it.
//!
//! # The micropayment problem
//!
//! Realistic per-subject amounts are far below any payment rail's minimum
//! economic transfer. A subject whose data earns 0.0004 EUR in a month cannot
//! be paid 0.0004 EUR: card and bank rails cost cents to tens of cents per
//! transfer, so paying immediately would destroy more value than it moves.
//!
//! DUAP's answer is not to pretend the amount is bigger, and not to round it
//! to zero. [`PayoutAccumulator`] carries exact balances forward at
//! nano-minor-unit precision and releases a payment only when the balance
//! crosses a threshold that the deployment sets from its own rail costs. The
//! balance is a liability on the clearing node's books from the moment it
//! accrues -- it is owed whether or not it has been paid -- and the ledger
//! carries it as one. What the protocol cannot fix is that some balances will
//! never cross the threshold; `ECONOMIC_MODEL.md` section "Dust" gives the
//! options (escheat, charitable assignment, expiry with notice) and takes no
//! position on which is right, because that is a legal question that differs
//! by jurisdiction.

use duap_model::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// One recipient's allocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Allocation<K> {
    pub recipient: K,
    pub share: Ratio,
    pub amount: Money,
}

/// Split `pool` among `shares` so the parts sum exactly to the whole.
///
/// `shares` need not sum to one: whatever fraction is unallocated stays in
/// the residual, which is returned. This matters because attribution
/// legitimately terminates (severed edges, negligible shares) and the
/// unattributed remainder belongs to the controller, not to the recipients.
pub fn distribute<K: Ord + Clone>(
    pool: Money,
    shares: &BTreeMap<K, Ratio>,
) -> Result<(Vec<Allocation<K>>, Money), ModelError> {
    if shares.is_empty() {
        return Ok((Vec::new(), pool));
    }
    let total = pool.minor;

    // exact = pool * share, kept as (floor, remainder numerator, denominator)
    struct Row<K> {
        recipient: K,
        share: Ratio,
        floor: i128,
        rem_num: i128,
        rem_den: i128,
    }

    let mut rows: Vec<Row<K>> = Vec::with_capacity(shares.len());
    let mut allocated = 0i128;
    for (k, s) in shares {
        if s.num < 0 {
            return Err(ModelError::Invalid {
                field: "share",
                reason: "a negative attribution share cannot be paid".into(),
            });
        }
        let num = total
            .checked_mul(s.num)
            .ok_or(ModelError::Overflow("distribute"))?;
        let den = s.den as i128;
        let floor = num.div_euclid(den);
        let rem = num.rem_euclid(den);
        allocated += floor;
        rows.push(Row {
            recipient: k.clone(),
            share: *s,
            floor,
            rem_num: rem,
            rem_den: den,
        });
    }

    // Units left over after flooring, bounded by the number of recipients.
    let exact_total: i128 = {
        // sum of floors plus the units implied by the remainders
        let mut sum_share_num = 0i128;
        let mut common: i128 = 1;
        for r in &rows {
            common = lcm(common, r.rem_den);
        }
        for r in &rows {
            sum_share_num += r.share.num * (common / r.share.den as i128);
        }
        // total * (sum of shares), floored
        (total * sum_share_num).div_euclid(common)
    };
    let mut leftover = exact_total - allocated;

    // Hand out the leftover units by descending remainder; ties by recipient
    // order, which is deterministic because the map is ordered.
    let mut order: Vec<usize> = (0..rows.len()).collect();
    order.sort_by(|&a, &b| {
        let ra = rows[a].rem_num * rows[b].rem_den;
        let rb = rows[b].rem_num * rows[a].rem_den;
        rb.cmp(&ra)
            .then_with(|| rows[a].recipient.cmp(&rows[b].recipient))
    });
    let mut extra: BTreeMap<usize, i128> = BTreeMap::new();
    let mut i = 0;
    while leftover > 0 && !order.is_empty() {
        *extra.entry(order[i % order.len()]).or_insert(0) += 1;
        leftover -= 1;
        i += 1;
    }

    let mut out = Vec::with_capacity(rows.len());
    let mut sum = 0i128;
    for (idx, r) in rows.into_iter().enumerate() {
        let amount = r.floor + extra.get(&idx).copied().unwrap_or(0);
        sum += amount;
        out.push(Allocation {
            recipient: r.recipient,
            share: r.share,
            amount: Money::new(pool.currency, amount),
        });
    }
    let residual = Money::new(pool.currency, total - sum);
    Ok((out, residual))
}

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    if a == 0 { 1 } else { a }
}

fn lcm(a: i128, b: i128) -> i128 {
    (a / gcd(a, b)).saturating_mul(b)
}

/// A recipient's running balance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Balance {
    /// Amount accrued and not yet paid, at computation scale.
    pub accrued: Precise,
    /// Total paid out to date.
    pub paid: Money,
    /// When the balance last changed.
    pub updated_at: Timestamp,
}

/// A payment the accumulator has released.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Payout<K> {
    pub recipient: K,
    pub amount: Money,
    /// Amount left behind because it is smaller than a minor unit.
    pub carried: Precise,
}

/// Carries small balances forward until they are worth paying.
#[derive(Debug, Clone)]
pub struct PayoutAccumulator<K: Ord + Clone> {
    currency: Currency,
    /// Minimum balance at which a payout is released.
    threshold: Money,
    rounding: Rounding,
    balances: BTreeMap<K, Balance>,
}

impl<K: Ord + Clone> PayoutAccumulator<K> {
    pub fn new(currency: Currency, threshold: Money) -> Self {
        PayoutAccumulator {
            currency,
            threshold,
            // Toward zero: the payer is never over-charged by rounding, and
            // the fraction stays with the recipient's balance.
            rounding: Rounding::TowardZero,
            balances: BTreeMap::new(),
        }
    }

    pub fn threshold(&self) -> Money {
        self.threshold
    }

    pub fn balance(&self, k: &K) -> Option<&Balance> {
        self.balances.get(k)
    }

    pub fn len(&self) -> usize {
        self.balances.len()
    }

    pub fn is_empty(&self) -> bool {
        self.balances.is_empty()
    }

    /// Credit a recipient.
    pub fn accrue(&mut self, k: K, amount: Precise, at: Timestamp) -> Result<(), ModelError> {
        if amount.currency != self.currency {
            return Err(ModelError::CurrencyMismatch {
                a: self.currency.to_string(),
                b: amount.currency.to_string(),
            });
        }
        let b = self.balances.entry(k).or_insert(Balance {
            accrued: Precise::zero(self.currency),
            paid: Money::zero(self.currency),
            updated_at: at,
        });
        b.accrued = b.accrued.add(&amount)?;
        b.updated_at = at;
        Ok(())
    }

    /// Total owed across every recipient. This is the liability the ledger
    /// must carry.
    pub fn total_outstanding(&self) -> Result<Precise, ModelError> {
        let mut t = Precise::zero(self.currency);
        for b in self.balances.values() {
            t = t.add(&b.accrued)?;
        }
        Ok(t)
    }

    /// Release payouts for every balance at or above the threshold.
    ///
    /// The sub-minor-unit fraction stays in the balance, so no value is lost
    /// and the accumulator's invariant holds:
    /// `sum(paid) + sum(accrued) == sum(everything ever accrued)`.
    pub fn release(&mut self, at: Timestamp) -> Result<Vec<Payout<K>>, ModelError> {
        let mut out = Vec::new();
        let threshold_nmu = Precise::from_money(self.threshold)?.nmu;
        for (k, b) in self.balances.iter_mut() {
            if b.accrued.nmu < threshold_nmu {
                continue;
            }
            let (money, residue) = b.accrued.round_to_money(self.rounding);
            if money.minor <= 0 {
                continue;
            }
            b.accrued = residue;
            b.paid = b.paid.add(&money)?;
            b.updated_at = at;
            out.push(Payout {
                recipient: k.clone(),
                amount: money,
                carried: residue,
            });
        }
        Ok(out)
    }

    /// Balances that have not moved since `before` and are below the
    /// threshold: the dust a deployment must have a policy for.
    pub fn dust(&self, before: Timestamp) -> Result<Vec<(K, Precise)>, ModelError> {
        let threshold_nmu = Precise::from_money(self.threshold)?.nmu;
        Ok(self
            .balances
            .iter()
            .filter(|(_, b)| {
                b.updated_at < before && b.accrued.nmu < threshold_nmu && b.accrued.nmu > 0
            })
            .map(|(k, b)| (k.clone(), b.accrued))
            .collect())
    }
}
