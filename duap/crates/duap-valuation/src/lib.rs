//! # duap-valuation
//!
//! Pricing, auctions, and royalty distribution.
//!
//! STATUS: REFERENCE (mechanism). Tests cover the specified behaviour and
//! L5 pricing vectors exist; no independent review. The *coefficients* in
//! [`multiplier::MultiplierPolicy`] are parameters chosen to be ordinally
//! defensible, not measurements of what data is worth. No claim is made that
//! any number in this crate is the right price for anything.
//!
//! The crate separates three things that get confused:
//!
//! * **A rule** ([`duap_model::pricing::PricingRule`]) -- what the parties
//!   agreed.
//! * **A price** ([`engine::PriceEngine`]) -- what the rule yields for a
//!   specific metered quantity, with an itemised derivation.
//! * **A distribution** ([`distribution`]) -- how a pool is split among the
//!   subjects an object is attributed to, exactly, with the micropayment
//!   problem handled rather than ignored.

pub mod auction;
pub mod distribution;
pub mod engine;
pub mod multiplier;
pub mod schedule;

pub use auction::{AuctionResult, BidCommitment, BidReveal, Lot, SealedBidAuction, bid_commitment};
pub use distribution::{Allocation, Balance, PayoutAccumulator, Payout, distribute};
pub use engine::{PriceBreakdown, PriceEngine, PricingError, PricingInputs};
pub use multiplier::{AppliedMultiplier, MultiplierContext, MultiplierPolicy};
pub use schedule::{PricingSchedule, ScheduleEntry, ScheduleRegistry};
