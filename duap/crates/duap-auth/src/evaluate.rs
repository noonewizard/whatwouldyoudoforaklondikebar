//! The authorization evaluator.
//!
//! STATUS: PRODUCTION. The state machine it implements is model-checked in
//! `formal/Authorization.tla`; the invariants that specification asserts are
//! restated here as tests.
//!
//! # Combining algorithm
//!
//! **Deny overrides, obligations union.**
//!
//! 1. Every term whose matcher applies is collected.
//! 2. If any applicable term is a `Deny`, the decision is `Deny`. No `Permit`
//!    can overcome it and term order is irrelevant.
//! 3. Otherwise, if at least one applicable `Permit` remains after revocation
//!    filtering, the obligations of *all* of them are unioned and each is
//!    checked. A single violated obligation makes the decision `Deny`.
//! 4. Otherwise the grant's `default_effect` applies.
//!
//! Deny-overrides is chosen over first-applicable because term order then
//! carries no meaning, which removes an entire class of authoring mistake
//! (and of malicious re-ordering during grant amendment). Obligation union is
//! chosen over "obligations of the winning term" because the alternative lets
//! a broad permissive term silently erase a narrow term's safeguard.
//!
//! # Determinism
//!
//! Evaluation is a pure function of `(grant, revocations, event, context)`.
//! No clock, no I/O, no map iteration order dependence. Two implementations
//! that agree on the inputs must agree on the decision, which is what the
//! conformance vectors check.

use crate::grant::{Effect, Grant};
use crate::obligation::{DerivationContext, Obligation, ObligationStatus};
use crate::revocation::Revocation;
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};

/// Why a decision came out as it did.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "snake_case")]
pub enum DecisionReason {
    /// No term matched; the grant's default applied.
    DefaultEffect,
    /// A Deny term matched.
    DeniedByTerm { term: u32 },
    /// Permitted by the listed terms.
    PermittedByTerms { terms: Vec<u32> },
    /// A matching permit was withdrawn.
    RevokedTerm { term: u32 },
    /// An obligation attached to the permission was violated.
    ObligationViolated { term: u32, obligation: String, detail: String },
    /// The grant was not in force at the time of the operation.
    OutsideGrantWindow,
    /// The event referenced a different grant, epoch or digest.
    GrantMismatch { detail: String },
    /// The event is about a different subject or controller.
    PartyMismatch { detail: String },
}

/// The result of evaluating one event against one grant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub effect: Effect,
    pub reason: DecisionReason,
    /// Terms that matched and permitted, in ascending id order.
    pub permitting_terms: Vec<u32>,
    /// Obligations the controller now carries. Populated only on `Permit`.
    pub obligations: Vec<Obligation>,
    /// Obligations that cannot be checked at decision time and are being
    /// monitored instead.
    pub deferred: Vec<String>,
    /// Pricing rule that applies, if the decision is `Permit`.
    pub pricing: Option<PricingRule>,
    /// Currency the pricing rule is denominated in.
    pub currency: Option<Currency>,
}

impl Decision {
    pub fn permitted(&self) -> bool {
        self.effect == Effect::Permit
    }

    fn deny(reason: DecisionReason) -> Decision {
        Decision {
            effect: Effect::Deny,
            reason,
            permitting_terms: Vec::new(),
            obligations: Vec::new(),
            deferred: Vec::new(),
            pricing: None,
            currency: None,
        }
    }
}

/// Inputs beyond the event that the evaluator needs.
#[derive(Debug, Clone, Default)]
pub struct EvalContext {
    pub derivation: DerivationContext,
    /// Whether the caller has already verified the subject's signature over
    /// the grant. The evaluator refuses to permit anything when this is false,
    /// so that a caller cannot accidentally rely on an unauthenticated grant.
    pub grant_signature_verified: bool,
}

impl EvalContext {
    pub fn verified() -> EvalContext {
        EvalContext {
            derivation: DerivationContext::default(),
            grant_signature_verified: true,
        }
    }

    pub fn with_derivation(mut self, d: DerivationContext) -> Self {
        self.derivation = d;
        self
    }
}

/// Evaluate `event` against `grant`, honouring `revocations`.
///
/// `revocations` need not be filtered by the caller; entries naming another
/// grant are ignored.
pub fn evaluate(
    grant: &Grant,
    revocations: &[Revocation],
    event: &DataUsageEvent,
    ctx: &EvalContext,
) -> Decision {
    if !ctx.grant_signature_verified {
        return Decision::deny(DecisionReason::GrantMismatch {
            detail: "the grant's signature has not been verified".into(),
        });
    }

    // The event must actually reference this grant, at this epoch, with this
    // document digest. Without the digest check, a controller could amend a
    // grant and claim the amended text was what it relied on.
    let aref = &event.authorization;
    if aref.grant != grant.id {
        return Decision::deny(DecisionReason::GrantMismatch {
            detail: format!("event cites grant {} but was evaluated against {}", aref.grant, grant.id),
        });
    }
    if aref.epoch != grant.epoch {
        return Decision::deny(DecisionReason::GrantMismatch {
            detail: format!("event cites epoch {} but the grant is at epoch {}", aref.epoch, grant.epoch),
        });
    }
    match grant.digest() {
        Ok(d) if d == aref.grant_digest => {}
        Ok(d) => {
            return Decision::deny(DecisionReason::GrantMismatch {
                detail: format!("event cites grant digest {} but the grant hashes to {d}", aref.grant_digest),
            });
        }
        Err(e) => {
            return Decision::deny(DecisionReason::GrantMismatch {
                detail: format!("grant could not be hashed: {e}"),
            });
        }
    }

    if event.controller != grant.controller {
        return Decision::deny(DecisionReason::PartyMismatch {
            detail: format!(
                "event controller {} is not the grantee {}",
                event.controller, grant.controller
            ),
        });
    }
    if let Some(s) = event.subject.subject_ref() {
        if s != grant.subject {
            return Decision::deny(DecisionReason::PartyMismatch {
                detail: "event subject does not match the grant's subject".into(),
            });
        }
    }

    if !grant.in_window(event.occurred_at) {
        return Decision::deny(DecisionReason::OutsideGrantWindow);
    }

    // 1. Collect applicable terms.
    let applicable: Vec<&crate::grant::Term> = grant
        .terms
        .iter()
        .filter(|t| t.matcher.matches(event))
        .collect();

    // 2. Deny overrides.
    if let Some(d) = applicable.iter().find(|t| t.effect == Effect::Deny) {
        return Decision::deny(DecisionReason::DeniedByTerm { term: d.id });
    }

    // 3. Revocation filtering over the permits.
    let mut permits = Vec::new();
    let mut first_revoked = None;
    for t in applicable.iter().filter(|t| t.effect == Effect::Permit) {
        let revoked = revocations.iter().any(|r| {
            r.grant == grant.id
                && grant.epoch >= r.epoch
                && r.suppresses(t.id, event.data_class, event.purpose, event.occurred_at)
        });
        if revoked {
            first_revoked.get_or_insert(t.id);
        } else {
            permits.push(*t);
        }
    }

    if permits.is_empty() {
        if let Some(term) = first_revoked {
            return Decision::deny(DecisionReason::RevokedTerm { term });
        }
        return match grant.default_effect {
            Effect::Deny => Decision::deny(DecisionReason::DefaultEffect),
            Effect::Permit => Decision {
                effect: Effect::Permit,
                reason: DecisionReason::DefaultEffect,
                permitting_terms: Vec::new(),
                obligations: Vec::new(),
                deferred: Vec::new(),
                pricing: grant.default_pricing.clone(),
                currency: Some(grant.currency),
            },
        };
    }

    // 4. Union obligations and check them.
    let mut obligations: Vec<Obligation> = Vec::new();
    let mut deferred: Vec<String> = Vec::new();
    for t in &permits {
        for o in &t.obligations {
            if !obligations.contains(o) {
                match o.check(event, &ctx.derivation) {
                    ObligationStatus::Satisfied => obligations.push(o.clone()),
                    ObligationStatus::Economic => obligations.push(o.clone()),
                    ObligationStatus::Deferred(note) => {
                        deferred.push(format!("{}: {note}", o.label()));
                        obligations.push(o.clone());
                    }
                    ObligationStatus::Violated(detail) => {
                        return Decision::deny(DecisionReason::ObligationViolated {
                            term: t.id,
                            obligation: o.label().to_owned(),
                            detail,
                        });
                    }
                }
            }
        }
    }

    // Pricing: among the permitting terms that state a rule, prefer one
    // whose rule can actually price the unit this event is metered in, then
    // the most specific, then the lowest term id. Unit coverage comes first
    // because a more specific rule in the wrong unit prices nothing at all;
    // the tie-breaks keep the choice deterministic. (Vertical-slice
    // finding VS-3.)
    let mut best: Option<(&crate::grant::Term, (bool, u32))> = None;
    for t in &permits {
        if let Some(rule) = &t.pricing {
            let score = (rule.covers_unit(event.quantity.unit), t.matcher.specificity());
            match best {
                None => best = Some((t, score)),
                Some((bt, bs)) if score > bs || (score == bs && t.id < bt.id) => {
                    best = Some((t, score))
                }
                _ => {}
            }
        }
    }
    let pricing = best
        .map(|(t, _)| t.pricing.clone().expect("checked"))
        .or_else(|| grant.default_pricing.clone());

    let mut ids: Vec<u32> = permits.iter().map(|t| t.id).collect();
    ids.sort_unstable();

    Decision {
        effect: Effect::Permit,
        reason: DecisionReason::PermittedByTerms { terms: ids.clone() },
        permitting_terms: ids,
        obligations,
        deferred,
        pricing,
        currency: Some(grant.currency),
    }
}

/// A store of grants and revocations, keyed so that the clearing node can
/// resolve an event's `AuthorizationRef` without scanning.
#[derive(Debug, Default, Clone)]
pub struct AuthorizationStore {
    grants: std::collections::BTreeMap<(String, u32), Grant>,
    revocations: std::collections::BTreeMap<String, Vec<Revocation>>,
}

impl AuthorizationStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_grant(&mut self, g: Grant) -> Result<(), ModelError> {
        g.validate()?;
        // An amendment must chain to the epoch it replaces.
        if g.epoch > 1 {
            let prev = self.grants.get(&(g.id.to_string(), g.epoch - 1));
            match (prev, g.previous) {
                (Some(p), Some(d)) if p.digest()? == d => {}
                (Some(_), _) => {
                    return Err(ModelError::Invalid {
                        field: "grant.previous",
                        reason: "amendment does not chain to the stored predecessor".into(),
                    });
                }
                (None, _) => {
                    return Err(ModelError::Invalid {
                        field: "grant.previous",
                        reason: format!("predecessor epoch {} is not stored", g.epoch - 1),
                    });
                }
            }
        }
        self.grants.insert((g.id.to_string(), g.epoch), g);
        Ok(())
    }

    pub fn insert_revocation(&mut self, r: Revocation) -> Result<(), ModelError> {
        if let Some(g) = self.grant(&r.grant, r.epoch) {
            r.validate_against(g)?;
        }
        self.revocations
            .entry(r.grant.to_string())
            .or_default()
            .push(r);
        Ok(())
    }

    pub fn grant(&self, id: &GrantId, epoch: u32) -> Option<&Grant> {
        self.grants.get(&(id.to_string(), epoch))
    }

    pub fn latest_epoch(&self, id: &GrantId) -> Option<&Grant> {
        self.grants
            .range((id.to_string(), 0)..=(id.to_string(), u32::MAX))
            .next_back()
            .map(|(_, g)| g)
    }

    pub fn revocations_for(&self, id: &GrantId) -> &[Revocation] {
        self.revocations
            .get(&id.to_string())
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn grant_count(&self) -> usize {
        self.grants.len()
    }

    /// Resolve the event's authorization reference and evaluate.
    pub fn decide(&self, event: &DataUsageEvent, ctx: &EvalContext) -> Decision {
        let aref = &event.authorization;
        match self.grant(&aref.grant, aref.epoch) {
            Some(g) => evaluate(g, self.revocations_for(&aref.grant), event, ctx),
            None => Decision::deny(DecisionReason::GrantMismatch {
                detail: format!("no stored grant {} at epoch {}", aref.grant, aref.epoch),
            }),
        }
    }
}
