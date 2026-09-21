//! The pricing engine.
//!
//! STATUS: PRODUCTION.
//!
//! Turns `(usage counter, pricing rule, policy)` into an exact amount plus a
//! breakdown that reproduces it. Determinism is the requirement: two parties
//! given the same inputs must compute the same number to the
//! nano-minor-unit, or every invoice becomes a dispute.

use crate::multiplier::{AppliedMultiplier, MultiplierContext, MultiplierPolicy};
use duap_meter::{UsageCounter, UsageKey};
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum PricingError {
    #[error("pricing rule prices {rule_unit} but the usage is measured in {usage_unit}")]
    UnitMismatch {
        rule_unit: &'static str,
        usage_unit: &'static str,
    },
    #[error("revenue-share pricing needs declared revenue and none was given")]
    RevenueNotDeclared,
    #[error("pricing rule refers to schedule {0}, which was not supplied")]
    ScheduleMissing(String),
    #[error("auction pricing requires a settled price from market {0}")]
    AuctionUnsettled(String),
    #[error("{0}")]
    Model(#[from] ModelError),
}

/// A price with its derivation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceBreakdown {
    /// Base price per unit before multipliers.
    #[serde(rename = "b")]
    pub base_unit_price: Precise,
    /// Units charged.
    #[serde(rename = "q")]
    pub quantity: u64,
    /// Multipliers applied, in order.
    #[serde(rename = "m")]
    pub multipliers: Vec<AppliedMultiplier>,
    /// Product of the multipliers.
    #[serde(rename = "f")]
    pub combined_factor: Ratio,
    /// Final amount at computation scale, before invoice-level rounding.
    #[serde(rename = "a")]
    pub amount: Precise,
    /// Human-readable summary of the rule that produced it.
    #[serde(rename = "r")]
    pub rule: String,
    /// Whether a minimum-price obligation raised the result.
    #[serde(rename = "fl", default, skip_serializing_if = "is_false")]
    pub floored: bool,
}

fn is_false(b: &bool) -> bool {
    !*b
}

impl PriceBreakdown {
    pub fn zero(currency: Currency, rule: &str) -> PriceBreakdown {
        PriceBreakdown {
            base_unit_price: Precise::zero(currency),
            quantity: 0,
            multipliers: Vec::new(),
            combined_factor: Ratio::ONE,
            amount: Precise::zero(currency),
            rule: rule.to_owned(),
            floored: false,
        }
    }
}

/// External inputs the engine cannot derive from the counter.
#[derive(Debug, Clone, Default)]
pub struct PricingInputs {
    /// Declared revenue for revenue-share rules.
    pub declared_revenue: Option<Money>,
    /// Settled market price per unit for auction rules.
    pub settled_auction_price: Option<Precise>,
    /// Whether the counterparty obtained exclusivity.
    pub exclusive: bool,
    /// Effective sensitivity, which may exceed the class default.
    pub sensitivity: Option<SensitivityTier>,
    /// Retention as declared on the events in the bucket.
    pub retention: Option<RetentionPolicy>,
    /// Multiplier context.
    pub multiplier_ctx: MultiplierContext,
    /// Minimum price obligation, if one attached to the authorization.
    pub minimum_unit_price: Option<Precise>,
}

/// The pricing engine.
#[derive(Debug, Clone)]
pub struct PriceEngine {
    pub policy: MultiplierPolicy,
    pub currency: Currency,
}

impl PriceEngine {
    pub fn new(currency: Currency) -> PriceEngine {
        PriceEngine {
            policy: MultiplierPolicy::default(),
            currency,
        }
    }

    pub fn with_policy(mut self, p: MultiplierPolicy) -> Self {
        self.policy = p;
        self
    }

    /// Price one aggregated usage counter.
    pub fn price(
        &self,
        key: &UsageKey,
        counter: &UsageCounter,
        rule: &PricingRule,
        inputs: &PricingInputs,
    ) -> Result<PriceBreakdown, PricingError> {
        let sensitivity = inputs
            .sensitivity
            .unwrap_or_else(|| key.data_class.sensitivity());

        let (factor, items) = self.policy.apply(
            key.data_class,
            sensitivity,
            inputs.retention.as_ref(),
            inputs.exclusive,
            &inputs.multiplier_ctx,
        )?;

        let (base, rule_label) = match rule {
            PricingRule::Free => {
                return Ok(PriceBreakdown {
                    base_unit_price: Precise::zero(self.currency),
                    quantity: counter.quantity,
                    multipliers: Vec::new(),
                    combined_factor: Ratio::ONE,
                    amount: Precise::zero(self.currency),
                    rule: "free".into(),
                    floored: false,
                });
            }
            PricingRule::PerUnit { unit, unit_price } => {
                self.check_unit(*unit, key.unit)?;
                (
                    *unit_price,
                    format!("per_unit {unit_price} per {}", unit.code()),
                )
            }
            PricingRule::UnitTable { prices } => {
                match prices.iter().find(|(u, _)| *u == key.unit) {
                    Some((u, p)) => (*p, format!("unit_table {p} per {}", u.code())),
                    None => {
                        return Err(PricingError::UnitMismatch {
                            rule_unit: "unit_table",
                            usage_unit: key.unit.code(),
                        });
                    }
                }
            }
            PricingRule::Negotiated {
                unit,
                unit_price,
                reference,
            } => {
                self.check_unit(*unit, key.unit)?;
                (*unit_price, format!("negotiated ({reference})"))
            }
            PricingRule::Tiered { unit, tiers } => {
                self.check_unit(*unit, key.unit)?;
                // Marginal tiering: walk the tiers, charging each slice at
                // its own rate. Computed before multipliers so that the
                // multipliers apply to the whole line, not per slice.
                let mut remaining = counter.quantity;
                let mut consumed = 0u64;
                let mut total = Precise::zero(self.currency);
                for t in tiers {
                    if remaining == 0 {
                        break;
                    }
                    let slice = match t.up_to {
                        Some(up) if up > consumed => remaining.min(up - consumed),
                        Some(_) => 0,
                        None => remaining,
                    };
                    if slice > 0 {
                        total = total.add(&t.unit_price.mul_u64(slice)?)?;
                        remaining -= slice;
                        consumed += slice;
                    }
                }
                let amount = total.mul_ratio(factor)?;
                let (amount, floored) =
                    self.apply_floor(amount, counter.quantity, inputs, factor)?;
                return Ok(PriceBreakdown {
                    base_unit_price: Precise::zero(self.currency),
                    quantity: counter.quantity,
                    multipliers: items,
                    combined_factor: factor,
                    amount,
                    rule: format!("tiered ({} tiers)", tiers.len()),
                    floored,
                });
            }
            PricingRule::RevenueShare { share, floor } => match inputs.declared_revenue {
                Some(rev) => {
                    let p = Precise::from_money(rev)?.mul_ratio(*share)?;
                    let amount = p.mul_ratio(factor)?;
                    let (amount, floored) =
                        self.apply_floor(amount, counter.quantity, inputs, factor)?;
                    return Ok(PriceBreakdown {
                        base_unit_price: Precise::zero(self.currency),
                        quantity: counter.quantity,
                        multipliers: items,
                        combined_factor: factor,
                        amount,
                        rule: format!("revenue_share {share} of {rev}"),
                        floored,
                    });
                }
                None => match floor {
                    Some(f) => (*f, "revenue_share fallback to floor".to_owned()),
                    None => return Err(PricingError::RevenueNotDeclared),
                },
            },
            PricingRule::Auction {
                market,
                unit,
                reserve,
            } => {
                self.check_unit(*unit, key.unit)?;
                match inputs.settled_auction_price {
                    Some(p) => (p, format!("auction {market} settled")),
                    None => match reserve {
                        Some(r) => (*r, format!("auction {market} at reserve")),
                        None => return Err(PricingError::AuctionUnsettled(market.clone())),
                    },
                }
            }
            PricingRule::Schedule { id, .. } => {
                return Err(PricingError::ScheduleMissing(id.clone()));
            }
        };

        let line = base.mul_u64(counter.quantity)?;
        let amount = line.mul_ratio(factor)?;
        let (amount, floored) = self.apply_floor(amount, counter.quantity, inputs, factor)?;

        Ok(PriceBreakdown {
            base_unit_price: base,
            quantity: counter.quantity,
            multipliers: items,
            combined_factor: factor,
            amount,
            rule: rule_label,
            floored,
        })
    }

    fn check_unit(&self, rule_unit: Unit, usage_unit: Unit) -> Result<(), PricingError> {
        if rule_unit != usage_unit {
            return Err(PricingError::UnitMismatch {
                rule_unit: rule_unit.code(),
                usage_unit: usage_unit.code(),
            });
        }
        Ok(())
    }

    /// A minimum-price obligation is a floor on the *final* amount, after
    /// multipliers: a subject who insisted on a penny a record gets a penny
    /// a record even if every multiplier points down.
    fn apply_floor(
        &self,
        amount: Precise,
        quantity: u64,
        inputs: &PricingInputs,
        _factor: Ratio,
    ) -> Result<(Precise, bool), PricingError> {
        match inputs.minimum_unit_price {
            Some(min) => {
                let floor = min.mul_u64(quantity)?;
                if floor.nmu > amount.nmu {
                    Ok((floor, true))
                } else {
                    Ok((amount, false))
                }
            }
            None => Ok((amount, false)),
        }
    }
}
