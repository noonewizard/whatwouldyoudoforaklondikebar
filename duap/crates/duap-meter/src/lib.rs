//! # duap-meter
//!
//! Metering: turning a stream of signed events into countable, non-duplicated
//! usage.
//!
//! STATUS: PROTOTYPE. Implemented and tested; no conformance vectors,
//! because the counters this crate produces are inputs to pricing rather
//! than protocol objects.
//!
//! The pipeline is four independent checks, in this order, because each
//! assumes the previous one has run:
//!
//! 1. **Structural validation** (`duap-model`) -- is this a well-formed
//!    event?
//! 2. **Admission** ([`dedup`]) -- have we already counted it, is it inside
//!    the replay window, does its sequence number make sense?
//! 3. **Double-count detection** ([`doublecount`]) -- does another party
//!    already claim this operation?
//! 4. **Aggregation** ([`aggregate`]) -- add it to the right counter.
//!
//! Authorization is checked separately (`duap-auth`) and pricing afterwards
//! (`duap-valuation`); the meter is deliberately ignorant of both, so that a
//! metering bug cannot become an authorization bypass.

pub mod aggregate;
pub mod dedup;
pub mod doublecount;
pub mod evidence;
pub mod pipeline;

pub use aggregate::{
    AggregateError, Aggregator, ScopeTag, UsageCounter, UsageKey, WindowSize,
};
pub use dedup::{Admission, DedupIndex, GapReport, RejectReason, WindowConfig};
pub use doublecount::{
    Collision, CountDecision, DoubleCountDetector, ResolutionPolicy, fingerprint,
};
pub use pipeline::{MeterOutcome, MeterPipeline, MeterStats};
