//! Deduplication, replay rejection, and sequence-gap detection.
//!
//! STATUS: PRODUCTION (in-memory reference; the durable index lives in
//! `duap-clearing`).
//!
//! Three distinct failures are handled here, and conflating them is a common
//! source of both over- and under-counting:
//!
//! * **Retransmission.** The same event arrives twice because a network
//!   failure made the sender retry. Must be idempotent: counted once, and the
//!   second arrival must not be an error.
//! * **Replay.** An attacker re-submits a captured, validly signed event to
//!   inflate a payout. Indistinguishable from retransmission by inspection --
//!   which is exactly why idempotent dedup is the defence, not signature
//!   checking.
//! * **Suppression.** An organisation silently omits inconvenient events. No
//!   amount of checking what *arrived* detects this; only a per-stream
//!   monotonic sequence makes the hole visible.
//!
//! # The dedup key
//!
//! Two events are the same event iff they have the same canonical digest.
//! Two *different* events that describe the same real-world operation are a
//! double count, which is a separate check ([`crate::doublecount`]).
//!
//! # Memory
//!
//! The reference index is a `BTreeSet` of digests within a bounded time
//! window. At protocol scale that is not viable, and the production design is
//! a time-partitioned key-value store with partitions dropped once the replay
//! window closes; `OPERATIONS.md` documents the sizing. The window is what
//! bounds the state: outside it, events are rejected on age rather than
//! deduplicated, so the index never has to be unbounded.

use duap_canon::digest::Digest;
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Why an event was not accepted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "snake_case")]
pub enum RejectReason {
    /// Already seen. Idempotent: the caller should treat this as success.
    Duplicate { first_seen: Timestamp },
    /// Older than the replay window.
    TooOld {
        occurred_at: Timestamp,
        horizon: Timestamp,
    },
    /// Dated further into the future than the tolerated skew.
    TooNew {
        occurred_at: Timestamp,
        horizon: Timestamp,
    },
    /// The sequence index was already used by a different event.
    SequenceConflict { stream: String, index: u64 },
    /// The sequence index went backwards within a stream.
    SequenceRegression {
        stream: String,
        index: u64,
        seen_up_to: u64,
    },
}

/// Outcome of offering an event to the meter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Admission {
    /// First time seen; count it.
    Accepted,
    /// Seen before or otherwise not countable.
    Rejected(RejectReason),
}

impl Admission {
    pub fn accepted(&self) -> bool {
        matches!(self, Admission::Accepted)
    }
}

/// Window parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowConfig {
    /// How far back an event may be dated, in microseconds.
    pub max_age: u64,
    /// How far into the future, in microseconds.
    pub max_skew: u64,
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig {
            max_age: 7 * DAY,
            max_skew: 5 * MINUTE,
        }
    }
}

/// Deduplication index over a bounded time window.
#[derive(Debug, Clone)]
pub struct DedupIndex {
    config: WindowConfig,
    /// digest -> first time it was admitted.
    seen: BTreeMap<String, Timestamp>,
    /// (stream, index) -> digest, for sequence conflict detection.
    sequenced: BTreeMap<(String, u64), String>,
    /// stream -> highest index seen and the set of gaps still open.
    streams: BTreeMap<String, StreamState>,
}

#[derive(Debug, Clone, Default)]
struct StreamState {
    high_water: Option<u64>,
    missing: BTreeSet<u64>,
    lowest_seen: Option<u64>,
}

/// A report of indices a stream never delivered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GapReport {
    pub stream: String,
    pub high_water: u64,
    pub missing: Vec<u64>,
}

impl DedupIndex {
    pub fn new(config: WindowConfig) -> DedupIndex {
        DedupIndex {
            config,
            seen: BTreeMap::new(),
            sequenced: BTreeMap::new(),
            streams: BTreeMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.seen.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seen.is_empty()
    }

    /// Offer an event, at wall-clock time `now`.
    pub fn admit(&mut self, event: &DataUsageEvent, digest: Digest, now: Timestamp) -> Admission {
        if event.occurred_at.0 + self.config.max_age < now.0 {
            return Admission::Rejected(RejectReason::TooOld {
                occurred_at: event.occurred_at,
                horizon: now.saturating_sub(self.config.max_age),
            });
        }
        if event.occurred_at.0 > now.0.saturating_add(self.config.max_skew) {
            return Admission::Rejected(RejectReason::TooNew {
                occurred_at: event.occurred_at,
                horizon: now.saturating_add(self.config.max_skew),
            });
        }
        let key = digest.to_string();
        if let Some(first) = self.seen.get(&key) {
            return Admission::Rejected(RejectReason::Duplicate { first_seen: *first });
        }

        if let Some(sq) = &event.sequence {
            let stream = sq.stream.to_string();
            let skey = (stream.clone(), sq.index);
            if let Some(other) = self.sequenced.get(&skey) {
                if *other != key {
                    return Admission::Rejected(RejectReason::SequenceConflict {
                        stream,
                        index: sq.index,
                    });
                }
            }
            let st = self.streams.entry(stream.clone()).or_default();
            match st.high_water {
                None => {
                    st.high_water = Some(sq.index);
                    st.lowest_seen = Some(sq.index);
                }
                Some(hw) if sq.index > hw => {
                    for missing in (hw + 1)..sq.index {
                        st.missing.insert(missing);
                    }
                    st.high_water = Some(sq.index);
                }
                Some(_) => {
                    // A late arrival fills a known gap, or is a regression.
                    if !st.missing.remove(&sq.index) {
                        let lowest = st.lowest_seen.unwrap_or(sq.index);
                        if sq.index < lowest {
                            st.lowest_seen = Some(sq.index);
                        } else {
                            return Admission::Rejected(RejectReason::SequenceRegression {
                                stream,
                                index: sq.index,
                                seen_up_to: st.high_water.unwrap_or(0),
                            });
                        }
                    }
                }
            }
            self.sequenced.insert(skey, key.clone());
        }

        self.seen.insert(key, now);
        Admission::Accepted
    }

    /// Drop state older than the replay window.
    ///
    /// Returns the number of entries released. Callers run this on a timer;
    /// without it the index grows without bound, which is the failure mode
    /// that makes naive dedup unusable at scale.
    pub fn evict_before(&mut self, horizon: Timestamp) -> usize {
        let before = self.seen.len();
        self.seen.retain(|_, t| *t >= horizon);
        let kept: BTreeSet<&String> = self.seen.keys().collect();
        self.sequenced.retain(|_, d| kept.contains(d));
        before - self.seen.len()
    }

    /// Streams with holes.
    pub fn gaps(&self) -> Vec<GapReport> {
        self.streams
            .iter()
            .filter(|(_, s)| !s.missing.is_empty())
            .map(|(k, s)| GapReport {
                stream: k.clone(),
                high_water: s.high_water.unwrap_or(0),
                missing: s.missing.iter().copied().collect(),
            })
            .collect()
    }

    /// Highest index observed on a stream.
    pub fn high_water(&self, stream: &str) -> Option<u64> {
        self.streams.get(stream).and_then(|s| s.high_water)
    }
}
