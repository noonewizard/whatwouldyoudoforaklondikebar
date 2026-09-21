//! Windowed aggregation.
//!
//! STATUS: PRODUCTION.
//!
//! Aggregation exists because pricing and invoicing operate on periods, not
//! on individual events, and because a per-event ledger posting at protocol
//! volume would be five orders of magnitude more write traffic than the
//! accounting needs.
//!
//! # The key
//!
//! Counters are keyed by everything that can change the price:
//! `(controller, processor, subject scope, data class, operation, purpose,
//! jurisdiction, unit, window)`. Two events that differ in any of these are
//! never merged, so aggregation is lossless with respect to pricing. What it
//! does lose is per-event identity -- which is why the events themselves are
//! retained and committed to the log, and the aggregate carries the Merkle
//! root of the events that produced it.
//!
//! # Additivity
//!
//! Only additive units are summed. `Unit::Share` is an attribution share and
//! summing shares across scopes is meaningless; the aggregator refuses rather
//! than producing a plausible-looking wrong number.

use duap_canon::digest::{Digest, HashAlg};
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Errors from aggregation.
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum AggregateError {
    #[error("unit {0} is not additive and cannot be aggregated")]
    NonAdditive(&'static str),
    #[error("counter overflow for key {0}")]
    Overflow(String),
    #[error("{0}")]
    Model(#[from] ModelError),
}

/// The dimensions a counter is keyed by.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsageKey {
    #[serde(rename = "ct")]
    pub controller: OrgId,
    #[serde(rename = "pr", default, skip_serializing_if = "Option::is_none")]
    pub processor: Option<OrgId>,
    /// `None` for cohort or non-personal scopes; those are counted under
    /// `scope_tag` instead so that an aggregate never invents a subject.
    #[serde(rename = "sb", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<SubjectRef>,
    #[serde(rename = "st")]
    pub scope_tag: ScopeTag,
    #[serde(rename = "dc")]
    pub data_class: DataClass,
    #[serde(rename = "op")]
    pub operation: Operation,
    #[serde(rename = "pp")]
    pub purpose: Purpose,
    #[serde(rename = "ju")]
    pub country: String,
    #[serde(rename = "u")]
    pub unit: Unit,
    /// Start of the accounting window.
    #[serde(rename = "w")]
    pub window_start: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeTag {
    Subject,
    Cohort,
    NonPersonal,
}

/// Accumulated usage for one key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsageCounter {
    #[serde(rename = "n")]
    pub quantity: u64,
    #[serde(rename = "e")]
    pub event_count: u64,
    #[serde(rename = "f")]
    pub first: Timestamp,
    #[serde(rename = "l")]
    pub last: Timestamp,
    /// Merkle root over the digests of the events in this bucket, in arrival
    /// order. Lets an auditor demand the underlying events and check that the
    /// set produces this aggregate.
    #[serde(rename = "r")]
    pub evidence_root: Digest,
    /// Number of leaves under `evidence_root`.
    #[serde(rename = "c")]
    pub evidence_size: u64,
}

/// Window size for aggregation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowSize {
    Hour,
    Day,
    /// A billing period boundary supplied by the caller.
    Custom(u64),
}

impl WindowSize {
    pub fn micros(self) -> u64 {
        match self {
            WindowSize::Hour => HOUR,
            WindowSize::Day => DAY,
            WindowSize::Custom(m) => m.max(1),
        }
    }

    pub fn start_of(self, t: Timestamp) -> Timestamp {
        let m = self.micros();
        Timestamp(t.0 - t.0 % m)
    }
}

/// Windowed usage aggregator.
#[derive(Debug, Clone)]
pub struct Aggregator {
    window: WindowSize,
    counters: BTreeMap<UsageKey, UsageCounter>,
    evidence: BTreeMap<UsageKey, duap_provenance_merkle::MerkleLog>,
}

/// Local alias so this crate does not depend on the whole provenance crate
/// just for a Merkle accumulator.
mod duap_provenance_merkle {
    pub use crate::evidence::MerkleLog;
}

impl Aggregator {
    pub fn new(window: WindowSize) -> Aggregator {
        Aggregator {
            window,
            counters: BTreeMap::new(),
            evidence: BTreeMap::new(),
        }
    }

    pub fn window(&self) -> WindowSize {
        self.window
    }

    pub fn len(&self) -> usize {
        self.counters.len()
    }

    pub fn is_empty(&self) -> bool {
        self.counters.is_empty()
    }

    /// The key an event falls into.
    pub fn key_for(&self, ev: &DataUsageEvent) -> UsageKey {
        let (subject, scope_tag) = match &ev.subject {
            SubjectScope::Subject { subject } => (Some(*subject), ScopeTag::Subject),
            SubjectScope::Cohort { .. } => (None, ScopeTag::Cohort),
            SubjectScope::NonPersonal => (None, ScopeTag::NonPersonal),
        };
        UsageKey {
            controller: ev.controller.clone(),
            processor: ev.processor.clone(),
            subject,
            scope_tag,
            data_class: ev.data_class,
            operation: ev.operation,
            purpose: ev.purpose,
            country: ev.jurisdiction.country.clone(),
            unit: ev.quantity.unit,
            window_start: self.window.start_of(ev.occurred_at),
        }
    }

    /// Add an event to its bucket.
    pub fn add(&mut self, ev: &DataUsageEvent, digest: Digest) -> Result<(), AggregateError> {
        if !ev.quantity.unit.additive() {
            return Err(AggregateError::NonAdditive(ev.quantity.unit.code()));
        }
        let key = self.key_for(ev);
        let log = self.evidence.entry(key.clone()).or_insert_with(|| {
            duap_provenance_merkle::MerkleLog::new(HashAlg::Sha2_256)
        });
        log.append(&digest.bytes);
        let root = log.root();
        let size = log.len();

        let c = self.counters.entry(key.clone()).or_insert(UsageCounter {
            quantity: 0,
            event_count: 0,
            first: ev.occurred_at,
            last: ev.occurred_at,
            evidence_root: root,
            evidence_size: size,
        });
        c.quantity = c
            .quantity
            .checked_add(ev.quantity.amount)
            .ok_or_else(|| AggregateError::Overflow(format!("{key:?}")))?;
        c.event_count += 1;
        c.first = c.first.min(ev.occurred_at);
        c.last = c.last.max(ev.occurred_at);
        c.evidence_root = root;
        c.evidence_size = size;
        Ok(())
    }

    /// Reverse an event previously added, for double-count resolution.
    ///
    /// The evidence root is *not* rewound: the event really was submitted,
    /// and erasing it would hide the reversal. Instead the counter's quantity
    /// is reduced and the reversal is itself recorded as evidence, so the
    /// root advances. An auditor replaying the bucket sees both.
    pub fn reverse(
        &mut self,
        ev: &DataUsageEvent,
        digest: Digest,
    ) -> Result<(), AggregateError> {
        let key = self.key_for(ev);
        if let Some(c) = self.counters.get_mut(&key) {
            c.quantity = c.quantity.saturating_sub(ev.quantity.amount);
            c.event_count = c.event_count.saturating_sub(1);
            if let Some(log) = self.evidence.get_mut(&key) {
                let mut marker = Vec::with_capacity(33);
                marker.push(0xffu8); // reversal marker
                marker.extend_from_slice(&digest.bytes);
                log.append(&marker);
                c.evidence_root = log.root();
                c.evidence_size = log.len();
            }
        }
        Ok(())
    }

    pub fn get(&self, key: &UsageKey) -> Option<&UsageCounter> {
        self.counters.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&UsageKey, &UsageCounter)> {
        self.counters.iter()
    }

    /// Remove and return every counter whose window ended before `horizon`.
    pub fn drain_closed(&mut self, horizon: Timestamp) -> Vec<(UsageKey, UsageCounter)> {
        let w = self.window.micros();
        let closed: Vec<UsageKey> = self
            .counters
            .keys()
            .filter(|k| k.window_start.0 + w <= horizon.0)
            .cloned()
            .collect();
        let mut out = Vec::with_capacity(closed.len());
        for k in closed {
            if let Some(c) = self.counters.remove(&k) {
                self.evidence.remove(&k);
                out.push((k, c));
            }
        }
        out
    }

    /// Total quantity across every counter in a unit.
    pub fn total(&self, unit: Unit) -> u128 {
        self.counters
            .iter()
            .filter(|(k, _)| k.unit == unit)
            .map(|(_, c)| c.quantity as u128)
            .sum()
    }
}
