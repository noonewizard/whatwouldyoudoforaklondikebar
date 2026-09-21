//! The metering pipeline.
//!
//! STATUS: PRODUCTION.

use crate::aggregate::{AggregateError, Aggregator, WindowSize};
use crate::dedup::{Admission, DedupIndex, RejectReason, WindowConfig};
use crate::doublecount::{Collision, CountDecision, DoubleCountDetector, ResolutionPolicy};
use duap_canon::digest::Digest;
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};

/// What the pipeline did with an event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeterOutcome {
    /// Counted.
    Counted { digest: Digest },
    /// Counted, and an earlier conflicting event was reversed.
    CountedWithReversal { digest: Digest, collision: Box<Collision> },
    /// Not counted, and why.
    Rejected(RejectReason),
    /// Not counted because another party already claimed the operation.
    Suppressed(Box<Collision>),
    /// Neither this nor the incumbent is counted; an exception was raised.
    Suspended(Box<Collision>),
    /// Structurally invalid.
    Invalid(String),
}

impl MeterOutcome {
    pub fn counted(&self) -> bool {
        matches!(
            self,
            MeterOutcome::Counted { .. } | MeterOutcome::CountedWithReversal { .. }
        )
    }
}

/// Running counts, for observability and for the SLO on rejection rates.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeterStats {
    pub offered: u64,
    pub counted: u64,
    pub duplicates: u64,
    pub out_of_window: u64,
    pub sequence_problems: u64,
    pub suppressed: u64,
    pub suspended: u64,
    pub invalid: u64,
    pub reversals: u64,
}

/// The four-stage metering pipeline.
pub struct MeterPipeline {
    pub dedup: DedupIndex,
    pub detector: DoubleCountDetector,
    pub aggregator: Aggregator,
    pub stats: MeterStats,
    /// Events retained by digest so that a reversal can undo the right
    /// quantity. Bounded by the same window as the dedup index.
    recent: std::collections::BTreeMap<String, DataUsageEvent>,
}

impl MeterPipeline {
    pub fn new(window: WindowConfig, agg: WindowSize, policy: ResolutionPolicy) -> MeterPipeline {
        MeterPipeline {
            dedup: DedupIndex::new(window),
            detector: DoubleCountDetector::new(policy),
            aggregator: Aggregator::new(agg),
            stats: MeterStats::default(),
            recent: std::collections::BTreeMap::new(),
        }
    }

    /// Offer one event.
    pub fn offer(
        &mut self,
        ev: &DataUsageEvent,
        now: Timestamp,
    ) -> Result<MeterOutcome, AggregateError> {
        self.stats.offered += 1;
        if let Err(e) = ev.validate() {
            self.stats.invalid += 1;
            return Ok(MeterOutcome::Invalid(e.to_string()));
        }
        let digest = ev.digest()?;

        match self.dedup.admit(ev, digest, now) {
            Admission::Rejected(r) => {
                match &r {
                    RejectReason::Duplicate { .. } => self.stats.duplicates += 1,
                    RejectReason::TooOld { .. } | RejectReason::TooNew { .. } => {
                        self.stats.out_of_window += 1
                    }
                    RejectReason::SequenceConflict { .. }
                    | RejectReason::SequenceRegression { .. } => {
                        self.stats.sequence_problems += 1
                    }
                }
                return Ok(MeterOutcome::Rejected(r));
            }
            Admission::Accepted => {}
        }

        match self.detector.offer(ev, digest) {
            CountDecision::Count => {
                self.aggregator.add(ev, digest)?;
                self.recent.insert(digest.to_string(), ev.clone());
                self.stats.counted += 1;
                Ok(MeterOutcome::Counted { digest })
            }
            CountDecision::CountAndReverse {
                reversed,
                collision,
            } => {
                if let Some(old) = self.recent.get(&reversed.to_string()).cloned() {
                    self.aggregator.reverse(&old, reversed)?;
                }
                self.aggregator.add(ev, digest)?;
                self.recent.insert(digest.to_string(), ev.clone());
                self.stats.counted += 1;
                self.stats.reversals += 1;
                Ok(MeterOutcome::CountedWithReversal {
                    digest,
                    collision: Box::new(collision),
                })
            }
            CountDecision::Suppress(c) => {
                self.stats.suppressed += 1;
                Ok(MeterOutcome::Suppressed(Box::new(c)))
            }
            CountDecision::SuspendBoth(c) => {
                if let Some(old) = self.recent.get(&c.incumbent.to_string()).cloned() {
                    self.aggregator.reverse(&old, c.incumbent)?;
                    self.stats.reversals += 1;
                }
                self.stats.suspended += 1;
                Ok(MeterOutcome::Suspended(Box::new(c)))
            }
        }
    }

    /// Release state older than `horizon`.
    pub fn evict_before(&mut self, horizon: Timestamp) -> usize {
        let dropped = self.dedup.evict_before(horizon);
        self.recent.retain(|_, e| e.occurred_at >= horizon);
        let keep: std::collections::BTreeSet<String> = self.recent.keys().cloned().collect();
        self.detector.retain(&|d| keep.contains(&d.to_string()));
        dropped
    }
}
