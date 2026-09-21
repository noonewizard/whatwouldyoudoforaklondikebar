//! Obligations: the conditional half of an authorization term.
//!
//! STATUS: PRODUCTION.
//!
//! An obligation is a condition attached to a permission. DUAP divides them by
//! what a verifier can actually do with them, which is the distinction most
//! consent systems blur:
//!
//! * **Checkable** -- decidable from the event alone at decision time
//!   (`NoAiTraining`, `TransferOnlyTo`, `MaxRetentionDays`). A violation makes
//!   the decision `Deny`; there is no "permitted but non-compliant" state.
//! * **Deferred** -- a promise about the future (`DeleteBy`, `NotifyOnUse`).
//!   The protocol records the promise, timestamps it, and makes the later
//!   breach *provable*; it cannot prevent the breach. A `DeleteBy` obligation
//!   is discharged by a matching `lifecycle.delete` event, and the clearing
//!   node raises an exception when the deadline passes with none.
//! * **Economic** -- constraints on price (`MinimumPrice`). Enforced by the
//!   valuation engine, and a violation is a billing dispute rather than an
//!   authorization failure.
//!
//! Pretending a deferred obligation is enforced is the single most common
//! overclaim in this space. See `THREAT_MODEL.md` T-30 and the "limits of
//! enforcement" section of `PRIVACY.md`.

use duap_model::prelude::*;
use serde::{Deserialize, Serialize};

/// A condition attached to a permission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", deny_unknown_fields)]
pub enum Obligation {
    /// Retention must not exceed `days` from collection.
    #[serde(rename = "max_retention")]
    MaxRetentionDays {
        #[serde(rename = "d")]
        days: u32,
    },
    /// The data must be deleted by an absolute deadline.
    #[serde(rename = "delete_by")]
    DeleteBy {
        #[serde(rename = "t")]
        at: Timestamp,
    },
    /// No derived object may be created from this data.
    #[serde(rename = "no_derivative")]
    NoDerivative,
    /// Derived objects may be created, but only to a bounded depth in the
    /// provenance graph.
    #[serde(rename = "max_derivation_depth")]
    MaxDerivationDepth {
        #[serde(rename = "d")]
        depth: u8,
    },
    /// No transfer outside the controller.
    #[serde(rename = "no_onward_transfer")]
    NoOnwardTransfer,
    /// Transfers only to the listed organisations.
    #[serde(rename = "transfer_allowlist")]
    TransferOnlyTo {
        #[serde(rename = "o")]
        orgs: Vec<OrgId>,
    },
    /// No use of the data in any AI training operation.
    #[serde(rename = "no_ai_training")]
    NoAiTraining,
    /// AI operations allowed only for the listed operations (typically
    /// evaluation and inference, excluding training).
    #[serde(rename = "ai_allowlist")]
    AiOnlyFor {
        #[serde(rename = "o")]
        operations: Vec<Operation>,
    },
    /// Data must not leave the listed countries.
    #[serde(rename = "residency")]
    DataResidency {
        #[serde(rename = "c")]
        countries: Vec<String>,
    },
    /// Aggregate releases must cover at least `k` subjects.
    #[serde(rename = "min_cohort")]
    MinCohort {
        #[serde(rename = "n")]
        k: u64,
    },
    /// Differentially private releases must use at most this epsilon,
    /// expressed in micro-units (1_000_000 == epsilon 1.0).
    #[serde(rename = "max_epsilon")]
    MaxEpsilonMicro {
        #[serde(rename = "e")]
        epsilon_micro: u64,
    },
    /// The subject must be notified of each use.
    #[serde(rename = "notify")]
    NotifyOnUse,
    /// The use must be priced at or above a floor.
    #[serde(rename = "min_price")]
    MinimumPrice {
        #[serde(rename = "p")]
        price: Precise,
        #[serde(rename = "u")]
        unit: Unit,
    },
    /// The event must carry a commitment to the underlying value, so that a
    /// dispute can be resolved by opening it.
    #[serde(rename = "require_commitment")]
    RequireCommitment,
    /// The event must name a processor (no anonymous sub-processing).
    #[serde(rename = "require_processor")]
    RequireNamedProcessor,
}

/// The result of checking one obligation against one event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObligationStatus {
    /// Decidable now, and satisfied.
    Satisfied,
    /// A fact needed to decide this obligation was not available. The
    /// evaluator denies, distinctly from a violation: this is a
    /// misconfiguration on the caller's side, not a policy outcome.
    FactUnavailable(&'static str),
    /// Decidable now, and violated.
    Violated(String),
    /// A promise about the future. Recorded, monitored, not enforced here.
    Deferred(&'static str),
    /// Decided outside the authorization engine (pricing).
    Economic,
}

/// A fact the evaluator asked for, in three states rather than two.
///
/// `Option` conflates "the answer is none" with "nobody asked", and an
/// obligation must treat those differently: the first is a policy outcome,
/// the second is a misconfiguration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fact<T> {
    /// The provider answered.
    Known(T),
    /// The provider can answer, and the answer is that no such value
    /// exists -- an operation with no inputs has no input depth.
    NotApplicable,
    /// The provider cannot answer. An obligation that needs this fact
    /// denies, and says *this* rather than "violated", so a caller that
    /// forgot to supply a provider sees a configuration error instead of
    /// what looks like a policy outcome. That confusion was VS-2.
    Unavailable,
}

/// Facts the evaluator may need that an event does not carry.
///
/// The evaluator stays a pure function of its inputs: it calls this trait
/// and never reaches into a store. Determinism therefore means determinism
/// *given the same answers* (INV-A3), which is the honest statement -- the
/// fact was always an input, and carrying it in a struct rather than a
/// trait never made it less so.
///
/// Every method defaults to [`Fact::Unavailable`], so a caller who
/// implements nothing fails closed by construction rather than by
/// remembering to. See ADR-0017.
pub trait EvalFacts: std::fmt::Debug {
    /// Depth of the deepest input in the provenance graph.
    fn input_depth(&self, _ev: &DataUsageEvent) -> Fact<u8> {
        Fact::Unavailable
    }

    /// Privacy budget actually spent by this release, in micro-units.
    fn epsilon_micro(&self, _ev: &DataUsageEvent) -> Fact<u64> {
        Fact::Unavailable
    }
}

/// A provider that knows nothing.
///
/// Every context-dependent obligation denies under it. This is the correct
/// default for a caller with no provenance graph -- a decision made without
/// the facts should not be a permit.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoFacts;

impl EvalFacts for NoFacts {}

impl Obligation {
    /// Check this obligation against an event.
    pub fn check(&self, ev: &DataUsageEvent, facts: &dyn EvalFacts) -> ObligationStatus {
        use ObligationStatus as S;
        match self {
            Obligation::MaxRetentionDays { days } => match &ev.retention {
                None => S::Violated("no retention policy was declared".into()),
                Some(rp) => match rp.basis {
                    RetentionBasis::Transient => S::Satisfied,
                    RetentionBasis::FixedPeriod => match rp.days {
                        Some(d) if d <= *days => S::Satisfied,
                        Some(d) => S::Violated(format!(
                            "declared retention {d} days exceeds the permitted {days}"
                        )),
                        None => S::Violated("fixed-period retention without a period".into()),
                    },
                    RetentionBasis::Indefinite => {
                        S::Violated("indefinite retention exceeds any bounded permission".into())
                    }
                    RetentionBasis::AccountLifetime | RetentionBasis::LegalHold => {
                        // Unbounded in principle. Permitted only if the
                        // controller also states a hard deadline within the
                        // allowance.
                        match rp.until {
                            Some(u) if u.since(ev.occurred_at) <= *days as u64 * DAY => {
                                S::Satisfied
                            }
                            _ => S::Violated(format!(
                                "retention basis {:?} is unbounded and no deadline within {days} days was given",
                                rp.basis
                            )),
                        }
                    }
                },
            },
            Obligation::DeleteBy { .. } => {
                S::Deferred("deletion deadline recorded; discharged by a lifecycle.delete event")
            }
            Obligation::NoDerivative => {
                if ev.operation.derives() {
                    S::Violated(format!(
                        "operation {} creates a derived object",
                        ev.operation.code()
                    ))
                } else {
                    S::Satisfied
                }
            }
            Obligation::MaxDerivationDepth { depth } => {
                if !ev.operation.derives() {
                    return S::Satisfied;
                }
                // An operation with no recorded inputs still produces a
                // derivative, so it sits at depth 0 and the result at 1.
                match facts.input_depth(ev) {
                    Fact::Known(d) if d.saturating_add(1) > *depth => S::Violated(format!(
                        "derivation would reach depth {} beyond the permitted {depth}",
                        d.saturating_add(1)
                    )),
                    Fact::Known(_) => S::Satisfied,
                    Fact::NotApplicable if 1 > *depth => S::Violated(format!(
                        "derivation would reach depth 1 beyond the permitted {depth}"
                    )),
                    Fact::NotApplicable => S::Satisfied,
                    Fact::Unavailable => S::FactUnavailable("input_depth"),
                }
            }
            Obligation::NoOnwardTransfer => {
                if ev.operation.family() == OperationFamily::Transfer
                    && ev.operation != Operation::TransferInternal
                {
                    S::Violated(format!(
                        "operation {} transfers outside the controller",
                        ev.operation.code()
                    ))
                } else {
                    S::Satisfied
                }
            }
            Obligation::TransferOnlyTo { orgs } => {
                if ev.operation.family() != OperationFamily::Transfer
                    || ev.operation == Operation::TransferInternal
                {
                    return S::Satisfied;
                }
                match ev.economics.as_ref().and_then(|e| e.counterparty.as_ref()) {
                    Some(cp) if orgs.contains(cp) => S::Satisfied,
                    Some(cp) => S::Violated(format!("{cp} is not on the transfer allowlist")),
                    None => S::Violated("transfer without a named counterparty".into()),
                }
            }
            Obligation::NoAiTraining => {
                const TRAINING: [Operation; 7] = [
                    Operation::AiPretrain,
                    Operation::AiFinetune,
                    Operation::AiInstructionTune,
                    Operation::AiPreferenceData,
                    Operation::AiRlEnvironment,
                    Operation::AiDistill,
                    Operation::AiSynthesize,
                ];
                if TRAINING.contains(&ev.operation) {
                    S::Violated(format!(
                        "operation {} is a training use",
                        ev.operation.code()
                    ))
                } else {
                    S::Satisfied
                }
            }
            Obligation::AiOnlyFor { operations } => {
                if ev.operation.family() == duap_model::taxonomy::OperationFamily::Ai
                    && !operations.contains(&ev.operation)
                {
                    S::Violated(format!(
                        "AI operation {} is not on the allowlist",
                        ev.operation.code()
                    ))
                } else {
                    S::Satisfied
                }
            }
            Obligation::DataResidency { countries } => {
                if countries.contains(&ev.jurisdiction.country) {
                    S::Satisfied
                } else {
                    S::Violated(format!(
                        "processing in {} is outside the permitted residency set",
                        ev.jurisdiction.country
                    ))
                }
            }
            Obligation::MinCohort { k } => match &ev.subject {
                SubjectScope::Cohort { size, .. } if size >= k => S::Satisfied,
                SubjectScope::Cohort { size, .. } => {
                    S::Violated(format!("cohort size {size} is below the required {k}"))
                }
                SubjectScope::Subject { .. } => {
                    S::Violated("an individual-level event cannot satisfy a cohort floor".into())
                }
                SubjectScope::NonPersonal => S::Satisfied,
            },
            Obligation::MaxEpsilonMicro { epsilon_micro } => {
                let is_dp = ev.operation == Operation::ProcessDpRelease;
                match facts.epsilon_micro(ev) {
                    Fact::Known(e) if e <= *epsilon_micro => S::Satisfied,
                    Fact::Known(e) => S::Violated(format!(
                        "epsilon {e} micro exceeds the permitted {epsilon_micro} micro"
                    )),
                    Fact::NotApplicable if is_dp => S::Violated(
                        "a differentially private release must report its epsilon".into(),
                    ),
                    Fact::NotApplicable => S::Satisfied,
                    Fact::Unavailable if is_dp => S::FactUnavailable("epsilon_micro"),
                    Fact::Unavailable => S::Satisfied,
                }
            }
            Obligation::NotifyOnUse => {
                S::Deferred("notification recorded as an obligation on the controller")
            }
            Obligation::MinimumPrice { .. } => S::Economic,
            Obligation::RequireCommitment => {
                if ev.commitment.is_some() {
                    S::Satisfied
                } else {
                    S::Violated("no value commitment was recorded".into())
                }
            }
            Obligation::RequireNamedProcessor => {
                if ev.processor.is_some() {
                    S::Satisfied
                } else {
                    S::Violated("no processor was named".into())
                }
            }
        }
    }

    /// Short stable label, used in receipts and dashboards.
    pub fn label(&self) -> &'static str {
        match self {
            Obligation::MaxRetentionDays { .. } => "max_retention",
            Obligation::DeleteBy { .. } => "delete_by",
            Obligation::NoDerivative => "no_derivative",
            Obligation::MaxDerivationDepth { .. } => "max_derivation_depth",
            Obligation::NoOnwardTransfer => "no_onward_transfer",
            Obligation::TransferOnlyTo { .. } => "transfer_allowlist",
            Obligation::NoAiTraining => "no_ai_training",
            Obligation::AiOnlyFor { .. } => "ai_allowlist",
            Obligation::DataResidency { .. } => "residency",
            Obligation::MinCohort { .. } => "min_cohort",
            Obligation::MaxEpsilonMicro { .. } => "max_epsilon",
            Obligation::NotifyOnUse => "notify",
            Obligation::MinimumPrice { .. } => "min_price",
            Obligation::RequireCommitment => "require_commitment",
            Obligation::RequireNamedProcessor => "require_processor",
        }
    }
}
