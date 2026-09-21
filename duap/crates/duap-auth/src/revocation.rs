//! Revocation.
//!
//! STATUS: PRODUCTION.
//!
//! # What revocation can and cannot do
//!
//! Revocation stops *future* authorised operations and starts the clock on
//! deletion obligations. It cannot undo processing already performed, and it
//! cannot reach into a derived object, a trained model, or a copy an
//! organisation made outside the protocol. A system that claims otherwise is
//! claiming to do something software cannot do; DUAP's contribution is that
//! the boundary is explicit and the record on either side of it is provable.
//!
//! The protocol therefore records three distinct things:
//!
//! 1. **Prospective effect** -- from `effective_from`, matching operations are
//!    denied. Checkable, enforced by the evaluator.
//! 2. **Deletion obligations** -- a revocation may demand deletion of data
//!    already held. Deferred: discharged by `lifecycle.delete` events, and
//!    unfulfilled deadlines become clearing-node exceptions.
//! 3. **Derivative reach** -- whether the revocation extends to objects
//!    already derived. Recorded as an assertion by the subject and as a
//!    separate, explicitly non-automatic obligation, because propagating a
//!    revocation through a model's weights is not generally possible. See
//!    `AI_ATTRIBUTION.md`.

use crate::grant::{Grant, RevocationPolicy};
use duap_canon::digest::Digest;
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};

/// Domain label for revocation digests and signatures.
pub const REVOCATION_DOMAIN: &str = "duap.revocation.v1";

/// How much of a grant a revocation withdraws.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", deny_unknown_fields)]
pub enum RevocationScope {
    /// The whole grant.
    #[serde(rename = "all")]
    All,
    /// Specific terms, by id.
    #[serde(rename = "terms")]
    Terms {
        #[serde(rename = "t")]
        term_ids: Vec<u32>,
    },
    /// Everything under the listed purposes.
    #[serde(rename = "purposes")]
    Purposes {
        #[serde(rename = "p")]
        purposes: Vec<Purpose>,
    },
    /// Everything touching the listed data classes.
    #[serde(rename = "classes")]
    Classes {
        #[serde(rename = "c")]
        classes: Vec<DataClass>,
    },
}

/// What the subject asks to happen to data already held.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetroactiveRequest {
    /// Stop future use only.
    None,
    /// Delete the source data.
    DeleteSource,
    /// Delete the source data and every derived object the controller can
    /// identify. Recorded as a request; the controller's response is itself an
    /// event, and the gap between request and response is what an auditor
    /// looks at.
    DeleteSourceAndDerived,
}

/// A signed withdrawal of authorization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Revocation {
    #[serde(rename = "v")]
    pub schema: u16,
    #[serde(rename = "g")]
    pub grant: GrantId,
    /// The epoch being revoked. A revocation applies to this epoch and all
    /// later ones, so amending a grant cannot escape a revocation in flight.
    #[serde(rename = "ep")]
    pub epoch: u32,
    /// Digest of the grant document being revoked.
    #[serde(rename = "gd")]
    pub grant_digest: Digest,
    #[serde(rename = "sc")]
    pub scope: RevocationScope,
    #[serde(rename = "at")]
    pub declared_at: Timestamp,
    /// When the revocation starts to bite, derived from the grant's
    /// revocation policy.
    #[serde(rename = "ef")]
    pub effective_from: Timestamp,
    #[serde(rename = "rq")]
    pub retroactive: RetroactiveRequest,
    #[serde(rename = "rs", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl Revocation {
    /// Build a revocation consistent with the grant's own notice policy.
    pub fn for_grant(
        grant: &Grant,
        scope: RevocationScope,
        declared_at: Timestamp,
        retroactive: RetroactiveRequest,
    ) -> Result<Revocation, ModelError> {
        Ok(Revocation {
            schema: 1,
            grant: grant.id,
            epoch: grant.epoch,
            grant_digest: grant.digest()?,
            scope,
            declared_at,
            effective_from: grant.revocation.effective_from(declared_at),
            retroactive,
            reason: None,
        })
    }

    pub fn digest(&self) -> Result<Digest, ModelError> {
        Ok(Digest::of_object(REVOCATION_DOMAIN, self)?)
    }

    /// Whether this revocation suppresses `term_id` for an event with the
    /// given class and purpose at time `t`.
    pub fn suppresses(
        &self,
        term_id: u32,
        class: DataClass,
        purpose: Purpose,
        t: Timestamp,
    ) -> bool {
        if t < self.effective_from {
            return false;
        }
        match &self.scope {
            RevocationScope::All => true,
            RevocationScope::Terms { term_ids } => term_ids.contains(&term_id),
            RevocationScope::Purposes { purposes } => purposes.iter().any(|p| p.covers(purpose)),
            RevocationScope::Classes { classes } => classes.contains(&class),
        }
    }

    /// Check internal consistency against the grant it names.
    pub fn validate_against(&self, grant: &Grant) -> Result<(), ModelError> {
        if self.grant != grant.id {
            return Err(ModelError::Invalid {
                field: "revocation.grant",
                reason: "revocation names a different grant".into(),
            });
        }
        let expected = grant.revocation.effective_from(self.declared_at);
        if self.effective_from < expected {
            return Err(ModelError::Invalid {
                field: "revocation.effective_from",
                reason: format!(
                    "effective_from {} precedes the policy-derived {expected}",
                    self.effective_from
                ),
            });
        }
        if matches!(grant.revocation, RevocationPolicy::Immediate)
            && self.effective_from > self.declared_at
        {
            // Permitted: a subject may choose to give more notice than the
            // policy requires. Recorded for the audit trail rather than
            // rejected.
        }
        Ok(())
    }
}
