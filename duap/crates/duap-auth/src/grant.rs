//! Grants: the authorization document.
//!
//! STATUS: PRODUCTION.
//!
//! A grant answers the ten questions an accounting layer needs settled before
//! an operation happens: who is authorised, by whom, over what data, for what
//! purpose, in which jurisdictions, from when, until when, with what onward
//! and derivative rights, at what price, and how it can be withdrawn.
//!
//! # Deny by default
//!
//! `default_effect` is `Deny` and cannot be omitted from the encoding. A grant
//! that fails to mention an operation does not authorise it. This is the
//! opposite of the prevailing consent-string designs, where absence of a
//! signal is widely treated as permission.
//!
//! # Epochs, not mutation
//!
//! A grant is immutable. Changing terms produces a new grant document with the
//! same [`GrantId`] and `epoch + 1`, signed afresh. Events reference
//! `(grant, epoch, digest)`, so "we relied on the version that permitted it"
//! is a checkable claim rather than an argument. See
//! `docs/adr/0009-grant-epochs.md`.

use crate::matcher::Matcher;
use crate::obligation::Obligation;
use duap_canon::digest::Digest;
use duap_crypto::KeyId;
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Domain label for grant digests and signatures.
pub const GRANT_DOMAIN: &str = "duap.grant.v1";
/// Current grant schema version.
pub const GRANT_SCHEMA_VERSION: u16 = 1;

/// Whether a term permits or denies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Effect {
    Permit,
    Deny,
}

/// One rule in a grant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Term {
    /// Stable index, referenced from receipts so that a decision can be
    /// explained years later without re-deriving it.
    #[serde(rename = "i")]
    pub id: u32,
    #[serde(rename = "e")]
    pub effect: Effect,
    #[serde(rename = "m", default, skip_serializing_if = "is_default_matcher")]
    pub matcher: Matcher,
    #[serde(rename = "ob", default, skip_serializing_if = "Vec::is_empty")]
    pub obligations: Vec<Obligation>,
    /// Price that applies when this term is the basis for a permission.
    #[serde(rename = "pr", default, skip_serializing_if = "Option::is_none")]
    pub pricing: Option<PricingRule>,
    /// Free-text label shown to the subject. Never load-bearing.
    #[serde(rename = "l", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

fn is_default_matcher(m: &Matcher) -> bool {
    *m == Matcher::default()
}

impl Term {
    pub fn permit(id: u32, matcher: Matcher) -> Term {
        Term {
            id,
            effect: Effect::Permit,
            matcher,
            obligations: Vec::new(),
            pricing: None,
            label: None,
        }
    }

    pub fn deny(id: u32, matcher: Matcher) -> Term {
        Term {
            id,
            effect: Effect::Deny,
            matcher,
            obligations: Vec::new(),
            pricing: None,
            label: None,
        }
    }

    pub fn with_obligations(mut self, o: Vec<Obligation>) -> Term {
        self.obligations = o;
        self
    }

    pub fn with_pricing(mut self, p: PricingRule) -> Term {
        self.pricing = Some(p);
        self
    }

    pub fn with_label(mut self, l: impl Into<String>) -> Term {
        self.label = Some(l.into());
        self
    }
}

/// What withdrawing a grant does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", deny_unknown_fields)]
pub enum RevocationPolicy {
    /// Effective from the instant the revocation is signed.
    #[serde(rename = "immediate")]
    Immediate,
    /// Effective after a notice period, so that the controller can stop
    /// pipelines in flight. The maximum period is capped by the clearing
    /// node's policy; an unbounded notice period is a revocation in name only.
    #[serde(rename = "notice")]
    AfterNotice {
        #[serde(rename = "h")]
        hours: u32,
    },
}

impl RevocationPolicy {
    pub fn effective_from(&self, declared_at: Timestamp) -> Timestamp {
        match self {
            RevocationPolicy::Immediate => declared_at,
            RevocationPolicy::AfterNotice { hours } => {
                declared_at.saturating_add(*hours as u64 * HOUR)
            }
        }
    }
}

/// The authorization document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Grant {
    #[serde(rename = "v")]
    pub schema: u16,
    #[serde(rename = "id")]
    pub id: GrantId,
    /// Monotonic revision. Starts at 1.
    #[serde(rename = "ep")]
    pub epoch: u32,
    /// Digest of the previous epoch, forming a hash chain so that a grant's
    /// history cannot be rewritten. `None` for epoch 1.
    #[serde(rename = "pv", default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<Digest>,
    /// The subject, under the pseudonym they present to this controller.
    #[serde(rename = "sb")]
    pub subject: SubjectRef,
    /// The key the subject signs grants and revocations with.
    #[serde(rename = "sk")]
    pub subject_key: KeyId,
    /// The controller the grant is made to.
    #[serde(rename = "ct")]
    pub controller: OrgId,
    #[serde(rename = "is")]
    pub issued_at: Timestamp,
    #[serde(rename = "nb")]
    pub not_before: Timestamp,
    /// Expiry. `None` is permitted but the reference policy engine warns:
    /// an authorization with no end is hard for a subject to reason about.
    #[serde(rename = "ex", default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,
    #[serde(rename = "tm")]
    pub terms: Vec<Term>,
    /// Effect when no term matches. Encoded explicitly, always.
    #[serde(rename = "df")]
    pub default_effect: Effect,
    /// Price applied when a matching permit term states none.
    #[serde(rename = "pc", default, skip_serializing_if = "Option::is_none")]
    pub default_pricing: Option<PricingRule>,
    #[serde(rename = "rv")]
    pub revocation: RevocationPolicy,
    /// Currency in which this grant's prices are denominated.
    #[serde(rename = "cu")]
    pub currency: Currency,
    #[serde(rename = "xt", default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, duap_canon::Value>,
}

impl Grant {
    pub fn digest(&self) -> Result<Digest, ModelError> {
        Ok(Digest::of_object(GRANT_DOMAIN, self)?)
    }

    pub fn to_canonical(&self) -> Result<Vec<u8>, ModelError> {
        Ok(duap_canon::to_canonical_cbor(self)?)
    }

    pub fn from_canonical(b: &[u8]) -> Result<Grant, ModelError> {
        Ok(duap_canon::from_canonical_cbor(b)?)
    }

    /// A reference usable in an event.
    pub fn reference(&self) -> Result<AuthorizationRef, ModelError> {
        Ok(AuthorizationRef {
            grant: self.id,
            grant_digest: self.digest()?,
            epoch: self.epoch,
        })
    }

    /// Structural validation.
    pub fn validate(&self) -> Result<(), ModelError> {
        let bad = |r: String| {
            Err(ModelError::Invalid {
                field: "grant",
                reason: r,
            })
        };
        if self.schema != GRANT_SCHEMA_VERSION {
            return bad(format!("unsupported grant schema {}", self.schema));
        }
        if self.epoch == 0 {
            return bad("epoch starts at 1".into());
        }
        if self.epoch == 1 && self.previous.is_some() {
            return bad("epoch 1 must not chain to a previous epoch".into());
        }
        if self.epoch > 1 && self.previous.is_none() {
            return bad(format!(
                "epoch {} must chain to its predecessor",
                self.epoch
            ));
        }
        if let Some(ex) = self.expires_at {
            if ex <= self.not_before {
                return bad("expiry must follow not_before".into());
            }
        }
        if self.terms.len() > 1024 {
            return bad("a grant may carry at most 1024 terms".into());
        }
        let mut ids = std::collections::BTreeSet::new();
        for t in &self.terms {
            if !ids.insert(t.id) {
                return bad(format!("duplicate term id {}", t.id));
            }
            if let Some(p) = &t.pricing {
                p.validate()?;
            }
            if t.effect == Effect::Deny && !t.obligations.is_empty() {
                return bad(format!(
                    "term {} is a Deny and cannot carry obligations",
                    t.id
                ));
            }
        }
        if let Some(p) = &self.default_pricing {
            p.validate()?;
        }
        if let RevocationPolicy::AfterNotice { hours } = self.revocation {
            if hours > 720 {
                return bad(
                    "a notice period beyond 30 days is rejected: it makes withdrawal illusory"
                        .into(),
                );
            }
        }
        for k in self.extensions.keys() {
            if !k.contains('.') || k.starts_with("duap.") {
                return bad(format!("invalid extension key {k:?}"));
            }
        }
        Ok(())
    }

    /// Whether the grant is within its own validity window at `t`.
    pub fn in_window(&self, t: Timestamp) -> bool {
        t >= self.not_before && self.expires_at.is_none_or(|e| t < e)
    }
}

/// Builder for grants.
pub struct GrantBuilder {
    g: Grant,
}

impl GrantBuilder {
    pub fn new(
        id: GrantId,
        subject: SubjectRef,
        subject_key: KeyId,
        controller: OrgId,
        issued_at: Timestamp,
        currency: Currency,
    ) -> GrantBuilder {
        GrantBuilder {
            g: Grant {
                schema: GRANT_SCHEMA_VERSION,
                id,
                epoch: 1,
                previous: None,
                subject,
                subject_key,
                controller,
                issued_at,
                not_before: issued_at,
                expires_at: None,
                terms: Vec::new(),
                default_effect: Effect::Deny,
                default_pricing: None,
                revocation: RevocationPolicy::Immediate,
                currency,
                extensions: BTreeMap::new(),
            },
        }
    }

    pub fn term(mut self, t: Term) -> Self {
        self.g.terms.push(t);
        self
    }
    pub fn terms(mut self, t: impl IntoIterator<Item = Term>) -> Self {
        self.g.terms.extend(t);
        self
    }
    pub fn not_before(mut self, t: Timestamp) -> Self {
        self.g.not_before = t;
        self
    }
    pub fn expires_at(mut self, t: Timestamp) -> Self {
        self.g.expires_at = Some(t);
        self
    }
    pub fn default_effect(mut self, e: Effect) -> Self {
        self.g.default_effect = e;
        self
    }
    pub fn default_pricing(mut self, p: PricingRule) -> Self {
        self.g.default_pricing = Some(p);
        self
    }
    pub fn revocation(mut self, r: RevocationPolicy) -> Self {
        self.g.revocation = r;
        self
    }

    pub fn build(self) -> Result<Grant, ModelError> {
        self.g.validate()?;
        Ok(self.g)
    }
}

impl Grant {
    /// Produce the next epoch of this grant with amended terms.
    pub fn amend(&self, terms: Vec<Term>, at: Timestamp) -> Result<Grant, ModelError> {
        let mut next = self.clone();
        next.epoch = self
            .epoch
            .checked_add(1)
            .ok_or(ModelError::Overflow("grant epoch"))?;
        next.previous = Some(self.digest()?);
        next.terms = terms;
        next.issued_at = at;
        next.validate()?;
        Ok(next)
    }
}
