//! Double-count detection.
//!
//! STATUS: PRODUCTION (detection); resolution is a clearing-node policy and
//! ultimately a dispute.
//!
//! Deduplication catches the *same* event arriving twice. It does nothing
//! about two *different* events describing the same real-world operation,
//! which is what happens when a controller and its processor both instrument
//! the same pipeline, when an SDK is installed twice, or when a party inflates
//! a claim on purpose.
//!
//! The defence is a fingerprint over the facts that identify an operation
//! independently of who reported it:
//!
//! ```text
//! fingerprint = H( controller, subject, data class, operation, purpose,
//!                  occurred_at rounded to the collision window,
//!                  quantity, provenance output )
//! ```
//!
//! Reporter identity is deliberately excluded, so two reporters of the same
//! operation collide. Time is rounded, because two instruments will not agree
//! to the microsecond.
//!
//! # False positives are expected
//!
//! Two genuinely distinct operations can share a fingerprint -- a subject
//! whose record is read twice in the same millisecond for the same purpose.
//! The meter therefore *flags* rather than *drops*: a collision produces a
//! [`Collision`] that the clearing node resolves by policy (prefer the
//! controller's report, prefer the earliest, or suspend both pending a
//! dispute). Silently dropping would under-count real usage, which is the
//! error that favours the reporting organisation. `ECONOMIC_MODEL.md` section
//! "Double counting" gives the resolution rules and their bias.

use duap_canon::digest::{Digest, HashAlg};
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Domain label for operation fingerprints.
pub const FINGERPRINT_DOMAIN: &str = "duap.op-fingerprint.v1";

/// Time granularity at which two reports are considered simultaneous.
pub const DEFAULT_COLLISION_WINDOW: u64 = 1_000; // 1 millisecond

/// An identity for a real-world operation, independent of who reported it.
pub fn fingerprint(ev: &DataUsageEvent, window_micros: u64) -> Digest {
    let bucket = if window_micros == 0 {
        ev.occurred_at.0
    } else {
        ev.occurred_at.0 - ev.occurred_at.0 % window_micros
    };
    let subject = match &ev.subject {
        SubjectScope::Subject { subject } => subject.to_string(),
        SubjectScope::Cohort { handle, size } => format!("cohort:{handle}:{size}"),
        SubjectScope::NonPersonal => "nonpersonal".to_owned(),
    };
    let parts = duap_canon::Value::map([
        ("ct", duap_canon::Value::text(ev.controller.as_str())),
        ("sb", duap_canon::Value::text(subject)),
        ("dc", duap_canon::Value::text(ev.data_class.code())),
        ("op", duap_canon::Value::text(ev.operation.code())),
        ("pp", duap_canon::Value::text(ev.purpose.code())),
        ("tb", duap_canon::Value::Uint(bucket)),
        ("qu", duap_canon::Value::text(ev.quantity.unit.code())),
        ("qn", duap_canon::Value::Uint(ev.quantity.amount)),
        (
            "po",
            match &ev.provenance.output {
                Some(o) => duap_canon::Value::text(o.to_string()),
                None => duap_canon::Value::Null,
            },
        ),
    ]);
    Digest::of(
        HashAlg::Sha2_256,
        FINGERPRINT_DOMAIN,
        &duap_canon::encode(&parts),
    )
}

/// Two events that appear to describe the same operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Collision {
    pub fingerprint: Digest,
    /// Digest of the event already recorded.
    pub incumbent: Digest,
    /// Digest of the event that collided with it.
    pub challenger: Digest,
    /// Reporters, for the resolution policy.
    pub incumbent_reporter: OrgId,
    pub challenger_reporter: OrgId,
}

/// How the clearing node resolves a collision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionPolicy {
    /// Count the first report, flag the second. Biased toward whoever
    /// reports fastest.
    FirstWins,
    /// Count the controller's own report in preference to a processor's.
    /// Biased toward the accountable party, which is usually right and is
    /// always explicable.
    ControllerWins,
    /// Count neither and raise an exception for human resolution. Safest for
    /// the subject, most expensive to operate.
    SuspendBoth,
}

/// Outcome of offering an event to the double-count detector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountDecision {
    /// No collision; count it.
    Count,
    /// Collides, and policy says this event is the one to count. The
    /// incumbent must be reversed.
    CountAndReverse {
        reversed: Digest,
        collision: Collision,
    },
    /// Collides, and policy says not to count this one.
    Suppress(Collision),
    /// Collides, and policy says to count neither.
    SuspendBoth(Collision),
}

/// Fingerprint index with a resolution policy.
#[derive(Debug, Clone)]
pub struct DoubleCountDetector {
    window: u64,
    policy: ResolutionPolicy,
    seen: BTreeMap<String, (Digest, OrgId, Option<OrgId>)>,
}

impl DoubleCountDetector {
    pub fn new(policy: ResolutionPolicy) -> Self {
        DoubleCountDetector {
            window: DEFAULT_COLLISION_WINDOW,
            policy,
            seen: BTreeMap::new(),
        }
    }

    pub fn with_window(mut self, micros: u64) -> Self {
        self.window = micros;
        self
    }

    pub fn len(&self) -> usize {
        self.seen.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seen.is_empty()
    }

    /// The reporter of an event: the processor if one is named, else the
    /// controller.
    fn reporter(ev: &DataUsageEvent) -> OrgId {
        ev.processor
            .clone()
            .unwrap_or_else(|| ev.controller.clone())
    }

    pub fn offer(&mut self, ev: &DataUsageEvent, digest: Digest) -> CountDecision {
        let fp = fingerprint(ev, self.window);
        let key = fp.to_string();
        let reporter = Self::reporter(ev);
        let is_controller_report = ev.processor.is_none();

        match self.seen.get(&key).cloned() {
            None => {
                self.seen
                    .insert(key, (digest, reporter, ev.processor.clone()));
                CountDecision::Count
            }
            Some((incumbent, inc_reporter, inc_processor)) => {
                if incumbent == digest {
                    // Identical event: dedup already handled this, but be
                    // idempotent if called directly.
                    return CountDecision::Count;
                }
                let collision = Collision {
                    fingerprint: fp,
                    incumbent,
                    challenger: digest,
                    incumbent_reporter: inc_reporter.clone(),
                    challenger_reporter: reporter.clone(),
                };
                match self.policy {
                    ResolutionPolicy::FirstWins => CountDecision::Suppress(collision),
                    ResolutionPolicy::SuspendBoth => CountDecision::SuspendBoth(collision),
                    ResolutionPolicy::ControllerWins => {
                        let incumbent_is_controller = inc_processor.is_none();
                        if is_controller_report && !incumbent_is_controller {
                            self.seen
                                .insert(key, (digest, reporter, ev.processor.clone()));
                            CountDecision::CountAndReverse {
                                reversed: incumbent,
                                collision,
                            }
                        } else {
                            CountDecision::Suppress(collision)
                        }
                    }
                }
            }
        }
    }

    /// Drop fingerprints outside the retention horizon. The caller supplies
    /// the surviving digests, since the detector does not track time itself.
    pub fn retain(&mut self, keep: &dyn Fn(&Digest) -> bool) {
        self.seen.retain(|_, (d, _, _)| keep(d));
    }
}
