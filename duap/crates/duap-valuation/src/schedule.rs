//! Published pricing schedules.
//!
//! STATUS: PRODUCTION.
//!
//! A schedule is a signed, content-addressed price list that many grants can
//! reference. Pinning the digest in the grant is what stops a schedule being
//! edited after the fact: a grant that cites schedule digest `D` is priced by
//! the document that hashes to `D`, and nothing else.
//!
//! Schedules are anchored in the transparency log, so a controller cannot
//! show one price list to a subject and another to the clearing node.

use duap_canon::digest::Digest;
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SCHEDULE_DOMAIN: &str = "duap.pricing-schedule.v1";

/// A rule keyed by the dimensions it applies to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScheduleEntry {
    #[serde(rename = "dc", default, skip_serializing_if = "Option::is_none")]
    pub data_class: Option<DataClass>,
    /// Applies to every class in this namespace when `data_class` is absent.
    #[serde(rename = "ns", default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "op", default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    #[serde(rename = "pp", default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<Purpose>,
    #[serde(rename = "r")]
    pub rule: PricingRule,
}

impl ScheduleEntry {
    /// How specific this entry is; the most specific match wins.
    pub fn specificity(&self) -> u32 {
        self.data_class.is_some() as u32 * 4
            + self.namespace.is_some() as u32 * 2
            + self.operation.is_some() as u32
            + self.purpose.is_some() as u32
    }

    pub fn matches(&self, class: DataClass, op: Operation, purpose: Purpose) -> bool {
        if let Some(c) = self.data_class {
            if c != class {
                return false;
            }
        }
        if let Some(ns) = &self.namespace {
            if ns != class.namespace() {
                return false;
            }
        }
        if let Some(o) = self.operation {
            if o != op {
                return false;
            }
        }
        if let Some(p) = self.purpose {
            if !p.covers(purpose) {
                return false;
            }
        }
        true
    }
}

/// A published price list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PricingSchedule {
    #[serde(rename = "v")]
    pub schema: u16,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "pb")]
    pub publisher: OrgId,
    #[serde(rename = "cu")]
    pub currency: Currency,
    #[serde(rename = "ef")]
    pub effective_from: Timestamp,
    #[serde(rename = "ex", default, skip_serializing_if = "Option::is_none")]
    pub effective_until: Option<Timestamp>,
    #[serde(rename = "en")]
    pub entries: Vec<ScheduleEntry>,
    #[serde(rename = "df", default, skip_serializing_if = "Option::is_none")]
    pub default_rule: Option<PricingRule>,
    #[serde(rename = "xt", default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, duap_canon::Value>,
}

impl PricingSchedule {
    pub fn digest(&self) -> Result<Digest, ModelError> {
        Ok(Digest::of_object(SCHEDULE_DOMAIN, self)?)
    }

    pub fn validate(&self) -> Result<(), ModelError> {
        for e in &self.entries {
            e.rule.validate()?;
            if e.data_class.is_some() && e.namespace.is_some() {
                return Err(ModelError::Invalid {
                    field: "schedule.entry",
                    reason: "an entry names either a class or a namespace, not both".into(),
                });
            }
        }
        if let Some(d) = &self.default_rule {
            d.validate()?;
        }
        if let Some(u) = self.effective_until {
            if u <= self.effective_from {
                return Err(ModelError::Invalid {
                    field: "schedule.effective_until",
                    reason: "expiry must follow the effective date".into(),
                });
            }
        }
        Ok(())
    }

    /// The rule that applies, most specific first; ties broken by position so
    /// that resolution is deterministic.
    pub fn resolve(
        &self,
        class: DataClass,
        op: Operation,
        purpose: Purpose,
    ) -> Option<&PricingRule> {
        let mut best: Option<(&ScheduleEntry, u32, usize)> = None;
        for (i, e) in self.entries.iter().enumerate() {
            if !e.matches(class, op, purpose) {
                continue;
            }
            let s = e.specificity();
            match best {
                None => best = Some((e, s, i)),
                Some((_, bs, bi)) if s > bs || (s == bs && i < bi) => best = Some((e, s, i)),
                _ => {}
            }
        }
        best.map(|(e, _, _)| &e.rule).or(self.default_rule.as_ref())
    }

    pub fn in_force(&self, t: Timestamp) -> bool {
        t >= self.effective_from && self.effective_until.is_none_or(|u| t < u)
    }
}

/// A set of schedules, resolved by pinned digest.
#[derive(Debug, Clone, Default)]
pub struct ScheduleRegistry {
    by_digest: BTreeMap<String, PricingSchedule>,
}

impl ScheduleRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish(&mut self, s: PricingSchedule) -> Result<Digest, ModelError> {
        s.validate()?;
        let d = s.digest()?;
        self.by_digest.insert(d.to_string(), s);
        Ok(d)
    }

    /// Resolve by the digest a grant pinned. A schedule that has been edited
    /// hashes differently and simply will not be found, which is the point.
    pub fn get(&self, d: &Digest) -> Option<&PricingSchedule> {
        self.by_digest.get(&d.to_string())
    }

    pub fn len(&self) -> usize {
        self.by_digest.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_digest.is_empty()
    }
}
