//! Timestamps.
//!
//! STATUS: PRODUCTION.
//!
//! DUAP timestamps are unsigned microseconds since the Unix epoch, UTC. Not
//! RFC 3339 strings: a string has many encodings of the same instant (offsets,
//! fractional-second digits, leap-second representations), and a canonical
//! object must have exactly one. Rendering to RFC 3339 happens at the edges.
//!
//! Microsecond resolution is a deliberate compromise: nanoseconds overflow
//! `u64` in 2554 and are not meaningful across machines anyway, while
//! milliseconds are too coarse to order events inside a single request.
//!
//! **What a timestamp is not.** A timestamp in an event is an assertion by
//! whoever signed the event. It is evidence of ordering only when corroborated
//! by an independent anchor -- in DUAP, inclusion in the transparency log.

use crate::error::{ModelError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Microseconds since the Unix epoch, UTC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timestamp(pub u64);

/// One second in microseconds.
pub const SECOND: u64 = 1_000_000;
/// One minute in microseconds.
pub const MINUTE: u64 = 60 * SECOND;
/// One hour in microseconds.
pub const HOUR: u64 = 60 * MINUTE;
/// One day in microseconds.
pub const DAY: u64 = 24 * HOUR;

impl Timestamp {
    pub const EPOCH: Timestamp = Timestamp(0);
    /// A far-future sentinel used for open-ended validity.
    pub const MAX: Timestamp = Timestamp(u64::MAX);

    pub fn now() -> Timestamp {
        Timestamp(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_micros() as u64)
                .unwrap_or(0),
        )
    }

    pub fn from_secs(s: u64) -> Timestamp {
        Timestamp(s.saturating_mul(SECOND))
    }

    pub fn as_secs(self) -> u64 {
        self.0 / SECOND
    }

    pub fn saturating_add(self, micros: u64) -> Timestamp {
        Timestamp(self.0.saturating_add(micros))
    }

    pub fn saturating_sub(self, micros: u64) -> Timestamp {
        Timestamp(self.0.saturating_sub(micros))
    }

    /// Difference in microseconds, saturating at zero.
    pub fn since(self, earlier: Timestamp) -> u64 {
        self.0.saturating_sub(earlier.0)
    }

    /// Truncate to the start of the UTC day, for daily accounting windows.
    pub fn truncate_day(self) -> Timestamp {
        Timestamp(self.0 - self.0 % DAY)
    }

    /// Truncate to the start of the hour.
    pub fn truncate_hour(self) -> Timestamp {
        Timestamp(self.0 - self.0 % HOUR)
    }

    /// Render as an RFC 3339 UTC string with microsecond precision.
    pub fn to_rfc3339(self) -> String {
        let secs = (self.0 / SECOND) as i64;
        let micros = (self.0 % SECOND) as u32;
        match time::OffsetDateTime::from_unix_timestamp(secs) {
            Ok(dt) => {
                let dt = dt + time::Duration::microseconds(micros as i64);
                dt.format(&time::format_description::well_known::Rfc3339)
                    .unwrap_or_else(|_| format!("@{}", self.0))
            }
            Err(_) => format!("@{}", self.0),
        }
    }

    /// Parse an RFC 3339 string, truncating below microsecond resolution.
    pub fn parse_rfc3339(s: &str) -> Result<Timestamp> {
        let dt = time::OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)
            .map_err(|e| ModelError::Invalid {
                field: "timestamp",
                reason: format!("{s:?}: {e}"),
            })?;
        let micros = dt.unix_timestamp_nanos() / 1_000;
        if micros < 0 {
            return Err(ModelError::Invalid {
                field: "timestamp",
                reason: format!("{s:?} predates the Unix epoch"),
            });
        }
        Ok(Timestamp(micros as u64))
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_rfc3339())
    }
}

impl FromStr for Timestamp {
    type Err = ModelError;
    fn from_str(s: &str) -> Result<Timestamp> {
        if let Some(rest) = s.strip_prefix('@') {
            return rest.parse::<u64>().map(Timestamp).map_err(|_| ModelError::Invalid {
                field: "timestamp",
                reason: s.to_owned(),
            });
        }
        Timestamp::parse_rfc3339(s)
    }
}

/// A half-open interval `[start, end)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: Timestamp,
    pub end: Timestamp,
}

impl TimeRange {
    pub fn new(start: Timestamp, end: Timestamp) -> Result<TimeRange> {
        if end < start {
            return Err(ModelError::Invalid {
                field: "time_range",
                reason: format!("end {end} precedes start {start}"),
            });
        }
        Ok(TimeRange { start, end })
    }

    pub fn contains(&self, t: Timestamp) -> bool {
        t >= self.start && t < self.end
    }

    pub fn overlaps(&self, other: &TimeRange) -> bool {
        self.start < other.end && other.start < self.end
    }

    pub fn duration_micros(&self) -> u64 {
        self.end.since(self.start)
    }
}
