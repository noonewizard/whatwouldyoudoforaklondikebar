//! Matchers: the predicate half of an authorization term.
//!
//! STATUS: PRODUCTION.
//!
//! A matcher is a conjunction of independent dimension tests. Every dimension
//! defaults to "any", so an empty matcher matches everything and a term is
//! narrowed by adding dimensions -- never widened by adding them. That
//! monotonicity is what makes the evaluator's behaviour predictable to a
//! non-specialist reading a grant in a dashboard.
//!
//! Purposes are matched through the lattice (`Purpose::covers`), never by
//! string equality: a grant for `marketing` covers
//! `marketing.advertising.behavioral` without having to enumerate it, and a
//! grant for `marketing.advertising.contextual` does not.

use duap_model::prelude::*;
use serde::{Deserialize, Serialize};

/// A set over a dimension. `Any` is the identity for conjunction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", deny_unknown_fields)]
#[derive(Default)]
pub enum Selector<T> {
    /// Matches everything on this dimension.
    #[serde(rename = "any")]
    #[default]
    Any,
    /// Matches exactly the listed values.
    #[serde(rename = "in")]
    In {
        #[serde(rename = "v")]
        values: Vec<T>,
    },
    /// Matches everything except the listed values.
    #[serde(rename = "not_in")]
    NotIn {
        #[serde(rename = "v")]
        values: Vec<T>,
    },
}

impl<T: PartialEq> Selector<T> {
    pub fn matches(&self, v: &T) -> bool {
        match self {
            Selector::Any => true,
            Selector::In { values } => values.contains(v),
            Selector::NotIn { values } => !values.contains(v),
        }
    }

    pub fn is_any(&self) -> bool {
        matches!(self, Selector::Any)
    }
}

/// Selector over data classes, with namespace support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", deny_unknown_fields)]
#[derive(Default)]
pub enum ClassSelector {
    #[serde(rename = "any")]
    #[default]
    Any,
    #[serde(rename = "in")]
    In {
        #[serde(rename = "v")]
        values: Vec<DataClass>,
    },
    #[serde(rename = "not_in")]
    NotIn {
        #[serde(rename = "v")]
        values: Vec<DataClass>,
    },
    /// Every class in the listed dotted namespaces, e.g. `location`.
    #[serde(rename = "ns")]
    Namespace {
        #[serde(rename = "v")]
        namespaces: Vec<String>,
    },
    /// Every class at or below a sensitivity tier.
    #[serde(rename = "max_tier")]
    MaxTier {
        #[serde(rename = "v")]
        tier: SensitivityTier,
    },
}

impl ClassSelector {
    pub fn matches(&self, c: DataClass) -> bool {
        match self {
            ClassSelector::Any => true,
            ClassSelector::In { values } => values.contains(&c),
            ClassSelector::NotIn { values } => !values.contains(&c),
            ClassSelector::Namespace { namespaces } => {
                namespaces.iter().any(|n| n == c.namespace())
            }
            ClassSelector::MaxTier { tier } => c.sensitivity().rank() <= tier.rank(),
        }
    }
}

/// Selector over purposes, lattice-aware.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", deny_unknown_fields)]
#[derive(Default)]
pub enum PurposeSelector {
    #[serde(rename = "any")]
    #[default]
    Any,
    /// Matches the listed purposes and everything below them in the lattice.
    #[serde(rename = "under")]
    Under {
        #[serde(rename = "v")]
        roots: Vec<Purpose>,
    },
    /// Matches exactly the listed purposes, with no descent.
    #[serde(rename = "exact")]
    Exact {
        #[serde(rename = "v")]
        values: Vec<Purpose>,
    },
    /// Matches everything except the listed purposes and their descendants.
    #[serde(rename = "not_under")]
    NotUnder {
        #[serde(rename = "v")]
        roots: Vec<Purpose>,
    },
    /// Matches every purpose flagged commercial (or, negated, non-commercial).
    #[serde(rename = "commercial")]
    Commercial {
        #[serde(rename = "v")]
        value: bool,
    },
}

impl PurposeSelector {
    pub fn matches(&self, p: Purpose) -> bool {
        match self {
            PurposeSelector::Any => true,
            PurposeSelector::Under { roots } => roots.iter().any(|r| r.covers(p)),
            PurposeSelector::Exact { values } => values.contains(&p),
            PurposeSelector::NotUnder { roots } => !roots.iter().any(|r| r.covers(p)),
            PurposeSelector::Commercial { value } => p.commercial() == *value,
        }
    }
}

/// The conjunction of dimension tests that decides whether a term applies.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Matcher {
    #[serde(rename = "dc", default, skip_serializing_if = "is_any_class")]
    pub data_classes: ClassSelector,
    #[serde(rename = "op", default, skip_serializing_if = "Selector::is_any")]
    pub operations: Selector<Operation>,
    #[serde(rename = "of", default, skip_serializing_if = "Selector::is_any")]
    pub operation_families: Selector<OperationFamily>,
    #[serde(rename = "pp", default, skip_serializing_if = "is_any_purpose")]
    pub purposes: PurposeSelector,
    /// ISO 3166-1 alpha-2 country codes.
    #[serde(rename = "ju", default, skip_serializing_if = "Selector::is_any")]
    pub countries: Selector<String>,
    #[serde(rename = "cp", default, skip_serializing_if = "Selector::is_any")]
    pub counterparties: Selector<OrgId>,
    #[serde(rename = "cm", default, skip_serializing_if = "Selector::is_any")]
    pub collection_methods: Selector<CollectionMethod>,
    #[serde(rename = "pr", default, skip_serializing_if = "Selector::is_any")]
    pub processors: Selector<OrgId>,
    /// Upper bound on the effective sensitivity of a matching event.
    #[serde(rename = "mt", default, skip_serializing_if = "Option::is_none")]
    pub max_sensitivity: Option<SensitivityTier>,
    /// Restrict the term to a window of occurrence times.
    #[serde(rename = "tw", default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TimeRange>,
}

fn is_any_class(c: &ClassSelector) -> bool {
    matches!(c, ClassSelector::Any)
}

fn is_any_purpose(p: &PurposeSelector) -> bool {
    matches!(p, PurposeSelector::Any)
}

impl Matcher {
    /// Matches everything.
    pub fn any() -> Matcher {
        Matcher::default()
    }

    pub fn classes(mut self, c: ClassSelector) -> Self {
        self.data_classes = c;
        self
    }
    pub fn operations(mut self, o: Selector<Operation>) -> Self {
        self.operations = o;
        self
    }
    pub fn families(mut self, f: Selector<OperationFamily>) -> Self {
        self.operation_families = f;
        self
    }
    pub fn purposes(mut self, p: PurposeSelector) -> Self {
        self.purposes = p;
        self
    }
    pub fn countries(mut self, c: Selector<String>) -> Self {
        self.countries = c;
        self
    }
    pub fn counterparties(mut self, c: Selector<OrgId>) -> Self {
        self.counterparties = c;
        self
    }
    pub fn collection_methods(mut self, c: Selector<CollectionMethod>) -> Self {
        self.collection_methods = c;
        self
    }
    pub fn max_sensitivity(mut self, t: SensitivityTier) -> Self {
        self.max_sensitivity = Some(t);
        self
    }
    pub fn window(mut self, w: TimeRange) -> Self {
        self.window = Some(w);
        self
    }

    /// Whether this matcher applies to `ev`.
    pub fn matches(&self, ev: &DataUsageEvent) -> bool {
        if !self.data_classes.matches(ev.data_class) {
            return false;
        }
        if !self.operations.matches(&ev.operation) {
            return false;
        }
        if !self.operation_families.matches(&ev.operation.family()) {
            return false;
        }
        if !self.purposes.matches(ev.purpose) {
            return false;
        }
        if !self.countries.matches(&ev.jurisdiction.country) {
            return false;
        }
        let counterparty = ev.economics.as_ref().and_then(|e| e.counterparty.clone());
        match (&self.counterparties, &counterparty) {
            (Selector::Any, _) => {}
            (_, None) => {
                // A term that constrains counterparties cannot apply to an
                // event with none. Being conservative here means a
                // counterparty-scoped Permit does not accidentally cover
                // internal use, and a counterparty-scoped Deny does not
                // accidentally block it.
                return false;
            }
            (sel, Some(cp)) => {
                if !sel.matches(cp) {
                    return false;
                }
            }
        }
        match (&self.processors, &ev.processor) {
            (Selector::Any, _) => {}
            (_, None) => return false,
            (sel, Some(p)) => {
                if !sel.matches(p) {
                    return false;
                }
            }
        }
        match (&self.collection_methods, &ev.collection) {
            (Selector::Any, _) => {}
            (_, None) => return false,
            (sel, Some(c)) => {
                if !sel.matches(c) {
                    return false;
                }
            }
        }
        if let Some(t) = self.max_sensitivity {
            if ev.sensitivity.rank() > t.rank() {
                return false;
            }
        }
        if let Some(w) = self.window {
            if !w.contains(ev.occurred_at) {
                return false;
            }
        }
        true
    }

    /// Rough specificity score, used only to order terms in explanations.
    pub fn specificity(&self) -> u32 {
        let mut n = 0;
        if !is_any_class(&self.data_classes) {
            n += 1;
        }
        if !self.operations.is_any() {
            n += 1;
        }
        if !self.operation_families.is_any() {
            n += 1;
        }
        if !is_any_purpose(&self.purposes) {
            n += 1;
        }
        if !self.countries.is_any() {
            n += 1;
        }
        if !self.counterparties.is_any() {
            n += 1;
        }
        if !self.collection_methods.is_any() {
            n += 1;
        }
        if !self.processors.is_any() {
            n += 1;
        }
        if self.max_sensitivity.is_some() {
            n += 1;
        }
        if self.window.is_some() {
            n += 1;
        }
        n
    }
}
