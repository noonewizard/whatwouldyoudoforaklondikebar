//! Pricing *declarations*.
//!
//! STATUS: PRODUCTION (declarations); the engine that evaluates them lives in
//! `duap-valuation`.
//!
//! These types say what a party has committed to, not what something is
//! worth. DUAP takes no position on the value of data: the protocol's job is
//! to make the commitment machine-readable, bind it into the authorization
//! that was relied on, and reproduce the arithmetic later. Sections 10 and 11
//! of `ECONOMIC_MODEL.md` explain why a single universal price function is not
//! a design goal but a design error.

use crate::error::{ModelError, Result};
use crate::money::{Precise, Ratio};
use crate::taxonomy::Unit;
use serde::{Deserialize, Serialize};

/// One step of a volume-tiered schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tier {
    /// Upper bound of this tier in units, inclusive. `None` means unbounded,
    /// and only the last tier may be unbounded.
    #[serde(rename = "u", default, skip_serializing_if = "Option::is_none")]
    pub up_to: Option<u64>,
    /// Price for each unit falling in this tier.
    #[serde(rename = "p")]
    pub unit_price: Precise,
}

/// How a use is priced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", deny_unknown_fields)]
pub enum PricingRule {
    /// Authorised at zero price. Distinct from "no rule": an explicit zero is
    /// a decision, and the participant dashboard shows it as one.
    #[serde(rename = "free")]
    Free,

    /// A fixed price per metered unit.
    #[serde(rename = "per_unit")]
    PerUnit {
        #[serde(rename = "u")]
        unit: Unit,
        #[serde(rename = "p")]
        unit_price: Precise,
    },

    /// A volume schedule. Tiers are marginal: units are charged at the rate of
    /// the tier they fall into, not the whole volume at the last tier's rate.
    #[serde(rename = "tiered")]
    Tiered {
        #[serde(rename = "u")]
        unit: Unit,
        #[serde(rename = "t")]
        tiers: Vec<Tier>,
    },

    /// A share of declared revenue attributable to the use.
    ///
    /// Only meaningful where the controller declares revenue in the event's
    /// economic context. Where it does not, the engine falls back to `floor`
    /// if present and otherwise reports the use as unpriced rather than
    /// guessing.
    #[serde(rename = "revenue_share")]
    RevenueShare {
        #[serde(rename = "s")]
        share: Ratio,
        #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
        floor: Option<Precise>,
    },

    /// Defer to a published schedule identified by `id`, whose content digest
    /// is pinned so that the schedule cannot be changed retroactively.
    #[serde(rename = "schedule")]
    Schedule {
        #[serde(rename = "i")]
        id: String,
        #[serde(rename = "d")]
        digest: duap_canon::Digest,
    },

    /// A bilaterally negotiated price recorded by reference.
    #[serde(rename = "negotiated")]
    Negotiated {
        #[serde(rename = "r")]
        reference: String,
        #[serde(rename = "u")]
        unit: Unit,
        #[serde(rename = "p")]
        unit_price: Precise,
    },

    /// Price discovered in a market. The protocol records the market and the
    /// reserve; clearing uses the settled price reported by the market with
    /// its own signature.
    #[serde(rename = "auction")]
    Auction {
        #[serde(rename = "m")]
        market: String,
        #[serde(rename = "u")]
        unit: Unit,
        #[serde(rename = "r", default, skip_serializing_if = "Option::is_none")]
        reserve: Option<Precise>,
    },
}

impl PricingRule {
    /// The unit this rule prices, where it has one.
    pub fn unit(&self) -> Option<Unit> {
        match self {
            PricingRule::PerUnit { unit, .. }
            | PricingRule::Tiered { unit, .. }
            | PricingRule::Negotiated { unit, .. }
            | PricingRule::Auction { unit, .. } => Some(*unit),
            PricingRule::Free | PricingRule::RevenueShare { .. } | PricingRule::Schedule { .. } => {
                None
            }
        }
    }

    /// Structural checks that do not need market data.
    pub fn validate(&self) -> Result<()> {
        match self {
            PricingRule::Tiered { tiers, .. } => {
                if tiers.is_empty() {
                    return Err(ModelError::Invalid {
                        field: "pricing.tiers",
                        reason: "a tiered rule needs at least one tier".into(),
                    });
                }
                let mut last: Option<u64> = None;
                for (i, t) in tiers.iter().enumerate() {
                    match t.up_to {
                        None if i + 1 != tiers.len() => {
                            return Err(ModelError::Invalid {
                                field: "pricing.tiers",
                                reason: "only the last tier may be unbounded".into(),
                            });
                        }
                        Some(u) => {
                            if let Some(l) = last {
                                if u <= l {
                                    return Err(ModelError::Invalid {
                                        field: "pricing.tiers",
                                        reason: "tier bounds must strictly increase".into(),
                                    });
                                }
                            }
                            last = Some(u);
                        }
                        None => {}
                    }
                    if t.unit_price.nmu < 0 {
                        return Err(ModelError::Invalid {
                            field: "pricing.tiers",
                            reason: "a tier price must not be negative".into(),
                        });
                    }
                }
                Ok(())
            }
            PricingRule::PerUnit { unit_price, .. }
            | PricingRule::Negotiated { unit_price, .. } => {
                if unit_price.nmu < 0 {
                    return Err(ModelError::Invalid {
                        field: "pricing.unit_price",
                        reason: "a unit price must not be negative".into(),
                    });
                }
                Ok(())
            }
            PricingRule::RevenueShare { share, .. } => {
                if share.num < 0 {
                    return Err(ModelError::Invalid {
                        field: "pricing.share",
                        reason: "a revenue share must not be negative".into(),
                    });
                }
                if share.num as i128 > share.den as i128 {
                    return Err(ModelError::Invalid {
                        field: "pricing.share",
                        reason: "a revenue share above 100% is rejected as a modelling error"
                            .into(),
                    });
                }
                Ok(())
            }
            PricingRule::Free | PricingRule::Schedule { .. } | PricingRule::Auction { .. } => Ok(()),
        }
    }
}
