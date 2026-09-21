//! Price multipliers.
//!
//! STATUS: PRODUCTION (mechanism); the coefficients are PARAMETERS, not
//! findings. Nothing in this file claims to know what data is worth.
//!
//! # What this is
//!
//! A base price answers "how much per record". A multiplier set answers "and
//! how much more, or less, for *this* record". DUAP separates them because
//! they come from different places: the base price is negotiated or
//! scheduled, while the adjustments are mechanical functions of properties
//! the protocol already records (sensitivity tier, re-identification prior,
//! retention, exclusivity, freshness).
//!
//! # The form
//!
//! ```text
//! price = base_unit_price x quantity x PI(m_i)
//! ```
//!
//! Multiplicative, not additive, because the factors are close to
//! independent proportional effects and because a multiplicative form cannot
//! drive a price negative. Each `m_i` is an exact rational, so the product is
//! exact and reproducible; the order of multiplication does not change the
//! result because rationals are associative and no rounding happens until
//! the end.
//!
//! # Where the numbers come from
//!
//! They are defaults chosen to be *ordinally* defensible -- more sensitive
//! costs more, exclusive costs more, stale costs less -- and nothing
//! stronger. `VALUATION.md` sets out the calibration procedure a deployment
//! should run against its own realised transaction data before using any of
//! them in anger, and records that DUAP's authors have not run it: there is
//! no market to calibrate against yet. A schedule may override every
//! coefficient.

use duap_model::prelude::*;
use serde::{Deserialize, Serialize};

/// Coefficients for the multiplier set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultiplierPolicy {
    /// Multiplier per sensitivity tier, indexed by rank 0..=4.
    pub sensitivity: [Ratio; 5],
    /// Multiplier applied per 10 points of re-identification prior above 50.
    pub reid_step: Ratio,
    /// Multiplier when the counterparty obtains exclusivity.
    pub exclusivity: Ratio,
    /// Multiplier for indefinite retention.
    pub indefinite_retention: Ratio,
    /// Multiplier for retention beyond a year.
    pub long_retention: Ratio,
    /// Multiplier for a transfer out of the jurisdiction of collection.
    pub cross_border: Ratio,
    /// Freshness half-life in days: data this old is worth half. Zero
    /// disables decay.
    pub freshness_half_life_days: u32,
    /// Floor on the freshness multiplier, so old data never becomes free.
    pub freshness_floor: Ratio,
    /// Multiplier for special-category data on top of its tier.
    pub special_category: Ratio,
}

impl Default for MultiplierPolicy {
    fn default() -> Self {
        let r = |n: i128, d: u64| Ratio::new(n, d).expect("non-zero denominator");
        MultiplierPolicy {
            // t0 non-personal, t1 low, t2 moderate, t3 high, t4 severe.
            sensitivity: [r(1, 2), r(1, 1), r(2, 1), r(4, 1), r(8, 1)],
            reid_step: r(105, 100),
            exclusivity: r(3, 1),
            indefinite_retention: r(5, 2),
            long_retention: r(3, 2),
            cross_border: r(5, 4),
            freshness_half_life_days: 180,
            freshness_floor: r(1, 8),
            special_category: r(3, 2),
        }
    }
}

/// One applied factor, retained so a price can be explained line by line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppliedMultiplier {
    pub name: String,
    pub factor: Ratio,
    pub because: String,
}

/// Inputs the multipliers need beyond the event itself.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MultiplierContext {
    /// Age of the data at the time of use, in days.
    pub age_days: Option<u32>,
    /// Whether the use crosses out of the jurisdiction of collection.
    pub cross_border: bool,
}

impl MultiplierPolicy {
    /// Compute the multiplier set for one usage key.
    ///
    /// Returns both the product and the itemised list, because an
    /// unexplainable price is a dispute waiting to happen.
    pub fn apply(
        &self,
        class: DataClass,
        sensitivity: SensitivityTier,
        retention: Option<&RetentionPolicy>,
        exclusive: bool,
        ctx: &MultiplierContext,
    ) -> Result<(Ratio, Vec<AppliedMultiplier>), ModelError> {
        let mut items = Vec::new();
        let mut product = Ratio::ONE;

        let tier = self.sensitivity[sensitivity.rank() as usize];
        product = product.mul(tier)?;
        items.push(AppliedMultiplier {
            name: "sensitivity".into(),
            factor: tier,
            because: format!("tier {}", sensitivity.code()),
        });

        let reid = class.reid_risk();
        if reid > 50 {
            let steps = (reid as u32 - 50) / 10;
            let mut f = Ratio::ONE;
            for _ in 0..steps {
                f = f.mul(self.reid_step)?;
            }
            if steps > 0 {
                product = product.mul(f)?;
                items.push(AppliedMultiplier {
                    name: "reidentification".into(),
                    factor: f,
                    because: format!("re-identification prior {reid} ({steps} steps above 50)"),
                });
            }
        }

        if class.is_special_category() {
            product = product.mul(self.special_category)?;
            items.push(AppliedMultiplier {
                name: "special_category".into(),
                factor: self.special_category,
                because: format!(
                    "tagged special category by {}",
                    class.special_categories().join(", ")
                ),
            });
        }

        if let Some(rp) = retention {
            match rp.basis {
                RetentionBasis::Indefinite => {
                    product = product.mul(self.indefinite_retention)?;
                    items.push(AppliedMultiplier {
                        name: "retention".into(),
                        factor: self.indefinite_retention,
                        because: "indefinite retention".into(),
                    });
                }
                RetentionBasis::FixedPeriod if rp.days.unwrap_or(0) > 365 => {
                    product = product.mul(self.long_retention)?;
                    items.push(AppliedMultiplier {
                        name: "retention".into(),
                        factor: self.long_retention,
                        because: format!("retention of {} days", rp.days.unwrap_or(0)),
                    });
                }
                _ => {}
            }
        }

        if exclusive {
            product = product.mul(self.exclusivity)?;
            items.push(AppliedMultiplier {
                name: "exclusivity".into(),
                factor: self.exclusivity,
                because: "counterparty obtains exclusive rights".into(),
            });
        }

        if ctx.cross_border {
            product = product.mul(self.cross_border)?;
            items.push(AppliedMultiplier {
                name: "cross_border".into(),
                factor: self.cross_border,
                because: "transfer leaves the jurisdiction of collection".into(),
            });
        }

        if let (Some(age), true) = (ctx.age_days, self.freshness_half_life_days > 0) {
            let f = self.freshness_factor(age)?;
            if f != Ratio::ONE {
                product = product.mul(f)?;
                items.push(AppliedMultiplier {
                    name: "freshness".into(),
                    factor: f,
                    because: format!(
                        "{age} days old, half-life {}",
                        self.freshness_half_life_days
                    ),
                });
            }
        }

        Ok((product, items))
    }

    /// Exponential decay approximated by exact halving per half-life plus
    /// linear interpolation within the period.
    ///
    /// A true exponential is irrational and cannot be represented exactly,
    /// and an accounting system must not depend on a floating-point
    /// approximation that two implementations might round differently. The
    /// piecewise-linear form is within 6% of `2^(-t/h)` everywhere, is exact
    /// in rational arithmetic, and is trivially reproducible.
    pub fn freshness_factor(&self, age_days: u32) -> Result<Ratio, ModelError> {
        let h = self.freshness_half_life_days;
        if h == 0 {
            return Ok(Ratio::ONE);
        }
        let periods = age_days / h;
        let rem = age_days % h;
        let mut f = Ratio::ONE;
        for _ in 0..periods.min(32) {
            f = f.mul(Ratio::new(1, 2)?)?;
        }
        if periods >= 32 {
            return Ok(self.freshness_floor);
        }
        // Linear interpolation from f down to f/2 across the period.
        if rem > 0 {
            let interp = Ratio::new((2 * h - rem) as i128, (2 * h) as u64)?;
            f = f.mul(interp)?;
        }
        // Compare f < floor exactly.
        if (f.num) * (self.freshness_floor.den as i128)
            < (self.freshness_floor.num) * (f.den as i128)
        {
            return Ok(self.freshness_floor);
        }
        Ok(f)
    }
}
