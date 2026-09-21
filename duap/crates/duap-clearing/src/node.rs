//! The clearing node.
//!
//! STATUS: PRODUCTION (reference implementation, single-process).
//!
//! # The nine ingest stages
//!
//! Every accepted event passes through the same ordered pipeline. The order
//! matters: each stage assumes the previous one has run, and skipping one
//! turns a later check into a false guarantee.
//!
//! 1. **Authenticate the sender** -- the envelope carries a signature by a
//!    key the registry accepts in the event-signer role.
//! 2. **Validate the schema** -- canonical encoding, known version, no
//!    unknown fields.
//! 3. **Validate the structure** -- the eleven rules in
//!    `DataUsageEvent::validate`.
//! 4. **Check authorization** -- evaluate the cited grant at the cited
//!    epoch, with the cited digest, against the revocations on file.
//! 5. **Prevent replay** -- digest dedup inside the replay window, plus
//!    sequence-gap tracking.
//! 6. **Detect double counting** -- reporter-independent fingerprint.
//! 7. **Assign canonical identity** -- the event's digest *is* its identity;
//!    the node assigns only a store sequence number.
//! 8. **Append** -- to the event store and, batched, to the transparency
//!    log.
//! 9. **Acknowledge** -- return a signed acknowledgement binding the event
//!    digest to a log position.
//!
//! # Acknowledgement is not a receipt
//!
//! An acknowledgement says "I have this event and it is in my log at this
//! position". A [`duap_receipt::Receipt`] says "over this period, this much
//! authorised usage occurred and this much is charged". Conflating them is a
//! category error: the first is per-event and immediate, the second is
//! per-period and involves pricing. Both are signed; only the second is an
//! accounting document.

use crate::store::{EventStore, MemoryStore, StoreError};
use duap_auth::{AuthorizationStore, Decision, DecisionReason, Effect, EvalContext};
use duap_canon::digest::{Digest, HashAlg};
use duap_crypto::{Envelope, KeyRegistry, KeyRole, SecretKey, SuitePolicy, VerificationContext};
use duap_ledger::{Account, AccountId, AccountKind, InvoiceBuilder, Ledger, NoTax, TaxPolicy};
use duap_meter::{
    MeterOutcome, MeterPipeline, ResolutionPolicy, UsageCounter, UsageKey, WindowConfig, WindowSize,
};
use duap_model::prelude::*;
use duap_provenance::{EntryKind, LogEntry, ProvenanceGraph, TransparencyLog};
use duap_receipt::{Coverage, DecisionSummary, LogAnchor, Receipt, ReceiptBuilder, events_root};
use duap_valuation::{PriceEngine, PricingInputs, ScheduleRegistry};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const ACK_DOMAIN: &str = "duap.ack.v1";

/// Signed acknowledgement of one ingested event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Acknowledgement {
    #[serde(rename = "v")]
    pub schema: u16,
    #[serde(rename = "nd")]
    pub node: OrgId,
    /// Canonical identity of the event: its digest.
    #[serde(rename = "ev")]
    pub event: Digest,
    /// Position in the node's event store.
    #[serde(rename = "sq")]
    pub store_sequence: u64,
    /// Log entry index, once the batch containing this event is sealed.
    #[serde(rename = "li", default, skip_serializing_if = "Option::is_none")]
    pub log_index: Option<u64>,
    #[serde(rename = "at")]
    pub accepted_at: Timestamp,
}

/// Why an event was not accepted, by pipeline stage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "stage", rename_all = "snake_case")]
pub enum RejectAt {
    Authentication { detail: String },
    Schema { detail: String },
    Structure { detail: String },
    Authorization { reason: DecisionReason },
    Replay { detail: String },
    DoubleCount { detail: String },
    Storage { detail: String },
}

/// Outcome of offering one envelope to the node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngestOutcome {
    Accepted {
        ack: Acknowledgement,
        decision: Box<Decision>,
    },
    /// Already held. Idempotent: not an error.
    Duplicate {
        event: Digest,
    },
    Rejected(RejectAt),
}

impl IngestOutcome {
    pub fn accepted(&self) -> bool {
        matches!(self, IngestOutcome::Accepted { .. })
    }
}

/// Node configuration.
#[derive(Debug, Clone)]
pub struct ClearingConfig {
    pub currency: Currency,
    /// Share of each charge that flows to the data subject.
    ///
    /// A single global number here is a *reference default*, not a claim
    /// about what the split should be. Real deployments set it per grant
    /// through pricing terms; this exists so the demonstration has a value.
    pub default_subject_share: Ratio,
    pub replay_window: WindowConfig,
    pub aggregation_window: WindowSize,
    pub double_count_policy: ResolutionPolicy,
    /// Minimum balance before a subject payout is released.
    pub payout_threshold: Money,
    pub rounding: Rounding,
    /// Verification-time tolerances.
    pub max_future_skew: u64,
}

impl ClearingConfig {
    pub fn reference(currency: Currency) -> ClearingConfig {
        ClearingConfig {
            currency,
            default_subject_share: Ratio::new(1, 2).expect("non-zero denominator"),
            replay_window: WindowConfig::default(),
            aggregation_window: WindowSize::Day,
            double_count_policy: ResolutionPolicy::ControllerWins,
            payout_threshold: Money::new(currency, 100),
            rounding: Rounding::HalfEven,
            max_future_skew: 5 * MINUTE,
        }
    }
}

/// Running counts for observability.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeStats {
    pub offered: u64,
    pub accepted: u64,
    pub duplicates: u64,
    pub rejected_auth: u64,
    pub rejected_authz: u64,
    pub rejected_schema: u64,
    pub rejected_structure: u64,
    pub rejected_replay: u64,
    pub rejected_double_count: u64,
    pub receipts_issued: u64,
    pub invoices_issued: u64,
}

/// A DUAP clearing node.
pub struct ClearingNode {
    pub org: OrgId,
    pub config: ClearingConfig,
    pub registry: KeyRegistry,
    pub suite_policy: SuitePolicy,
    pub authorizations: AuthorizationStore,
    pub meter: MeterPipeline,
    pub log: TransparencyLog,
    pub provenance: ProvenanceGraph,
    pub engine: PriceEngine,
    pub schedules: ScheduleRegistry,
    pub ledger: Ledger,
    pub payouts: duap_valuation::PayoutAccumulator<SubjectRef>,
    pub stats: NodeStats,
    store: Box<dyn EventStore>,
    signing_key: SecretKey,
    /// Events accepted in the current period, grouped by usage key.
    pending: BTreeMap<UsageKey, Vec<Digest>>,
    /// Decisions, so a receipt can name the terms that permitted the usage.
    decisions: BTreeMap<String, Decision>,
}

#[derive(Debug, thiserror::Error)]
pub enum ClearingError {
    #[error("{0}")]
    Store(#[from] StoreError),
    #[error("{0}")]
    Model(#[from] ModelError),
    #[error("{0}")]
    Canon(#[from] duap_canon::CanonError),
    #[error("{0}")]
    Crypto(#[from] duap_crypto::CryptoError),
    #[error("{0}")]
    Ledger(#[from] duap_ledger::LedgerError),
    #[error("{0}")]
    Receipt(#[from] duap_receipt::ReceiptError),
    #[error("{0}")]
    Log(#[from] duap_provenance::LogError),
    #[error("{0}")]
    Pricing(#[from] duap_valuation::PricingError),
    #[error("{0}")]
    Aggregate(#[from] duap_meter::AggregateError),
    #[error("no pricing rule applies to {0}")]
    Unpriced(String),
}

impl ClearingNode {
    pub fn new(org: OrgId, signing_key: SecretKey, config: ClearingConfig) -> ClearingNode {
        let currency = config.currency;
        let mut ledger = Ledger::new();
        // The system accounts every node needs, opened up front so that a
        // posting can never fail for want of an account.
        for (id, kind) in [
            (AccountId::clearing_fee_revenue(), AccountKind::Revenue),
            (AccountId::settlement_cash(currency), AccountKind::Asset),
            (AccountId::rounding(currency), AccountKind::Rounding),
            (AccountId::suspense(currency), AccountKind::Suspense),
        ] {
            let name = id.to_string();
            ledger.open_account(Account::new(id, kind, currency, name));
        }
        ClearingNode {
            org: org.clone(),
            registry: KeyRegistry::new(),
            suite_policy: SuitePolicy::draft_default(),
            authorizations: AuthorizationStore::new(),
            meter: MeterPipeline::new(
                config.replay_window,
                config.aggregation_window,
                config.double_count_policy,
            ),
            log: TransparencyLog::new(format!("log:{}", org.as_str()), HashAlg::Sha2_256),
            provenance: ProvenanceGraph::new(),
            engine: PriceEngine::new(currency),
            schedules: ScheduleRegistry::new(),
            ledger,
            payouts: duap_valuation::PayoutAccumulator::new(currency, config.payout_threshold),
            stats: NodeStats::default(),
            store: Box::new(MemoryStore::new()),
            signing_key,
            pending: BTreeMap::new(),
            decisions: BTreeMap::new(),
            config,
        }
    }

    /// Replace the event store (for a durable backend).
    pub fn with_store(mut self, store: Box<dyn EventStore>) -> Self {
        self.store = store;
        self
    }

    pub fn public_key(&self) -> duap_crypto::PublicKey {
        self.signing_key.public_key()
    }

    pub fn store_len(&self) -> Result<u64, StoreError> {
        self.store.len()
    }

    /// Stage 1-9. The single entry point for an event.
    pub fn ingest(
        &mut self,
        envelope: &Envelope,
        now: Timestamp,
    ) -> Result<IngestOutcome, ClearingError> {
        self.stats.offered += 1;

        // --- 1. authenticate the sender -----------------------------------
        let vctx = VerificationContext {
            now: now.0,
            max_future_skew: self.config.max_future_skew,
            max_age: self.config.replay_window.max_age,
        };
        let signers = match envelope.verify(&self.registry, &self.suite_policy, &vctx) {
            Ok(s) => s,
            Err(e) => {
                self.stats.rejected_auth += 1;
                return Ok(IngestOutcome::Rejected(RejectAt::Authentication {
                    detail: e.to_string(),
                }));
            }
        };
        for kid in &signers {
            let rec = self.registry.get(kid).expect("verified implies present");
            if !rec.has_role(KeyRole::EventSigner) {
                self.stats.rejected_auth += 1;
                return Ok(IngestOutcome::Rejected(RejectAt::Authentication {
                    detail: format!("{kid} is not authorised to sign events"),
                }));
            }
        }

        // --- 2. validate the schema ---------------------------------------
        if envelope.domain != duap_model::event::EVENT_DOMAIN {
            self.stats.rejected_schema += 1;
            return Ok(IngestOutcome::Rejected(RejectAt::Schema {
                detail: format!("envelope domain {:?} is not an event", envelope.domain),
            }));
        }
        let event: DataUsageEvent = match envelope.open() {
            Ok(e) => e,
            Err(e) => {
                self.stats.rejected_schema += 1;
                return Ok(IngestOutcome::Rejected(RejectAt::Schema {
                    detail: e.to_string(),
                }));
            }
        };

        // --- 3. validate the structure ------------------------------------
        if let Err(e) = event.validate() {
            self.stats.rejected_structure += 1;
            return Ok(IngestOutcome::Rejected(RejectAt::Structure {
                detail: e.to_string(),
            }));
        }

        // The signer must be acting for the controller or the named
        // processor. Without this, any enrolled organisation could report
        // usage in another's name.
        let expected: Vec<String> = [
            Some(event.controller.to_string()),
            event.processor.as_ref().map(|p| p.to_string()),
        ]
        .into_iter()
        .flatten()
        .collect();
        let holder_ok = signers.iter().any(|kid| {
            self.registry
                .get(kid)
                .is_some_and(|r| expected.contains(&r.holder))
        });
        if !holder_ok {
            self.stats.rejected_auth += 1;
            return Ok(IngestOutcome::Rejected(RejectAt::Authentication {
                detail: "no signer is the controller or the named processor".into(),
            }));
        }

        // --- 4. check authorization ---------------------------------------
        //
        // The evaluator cannot see the provenance graph, so the node has to
        // supply the facts a depth- or epsilon-bounded obligation needs.
        // Discovered by the first vertical slice: without this, every
        // `MaxDerivationDepth` obligation failed closed with "depth is
        // unknown", which is safe but useless. See
        // `docs/reviews/vertical-slice-review.md` finding VS-2.
        let mut derivation = duap_auth::DerivationContext::default();
        if !event.provenance.inputs.is_empty() {
            let mut deepest: Option<u8> = None;
            for i in &event.provenance.inputs {
                if let Ok(d) = self.provenance.depth(i) {
                    deepest = Some(deepest.map_or(d, |x: u8| x.max(d)));
                }
            }
            // An input the node has never seen is treated as a source at
            // depth 0 rather than as unknown: refusing to price a
            // derivation because the node has not yet been told about its
            // input would let an organisation evade obligations by
            // withholding the upstream event.
            derivation.input_depth = Some(deepest.unwrap_or(0));
        } else if event.operation.derives() {
            derivation.input_depth = Some(0);
        }
        if let Some(duap_canon::Value::Uint(e)) = event.extensions.get("dp.epsilon_micro") {
            derivation.epsilon_micro = Some(*e);
        }
        let decision = self
            .authorizations
            .decide(&event, &EvalContext::verified().with_derivation(derivation));
        if decision.effect != Effect::Permit {
            self.stats.rejected_authz += 1;
            return Ok(IngestOutcome::Rejected(RejectAt::Authorization {
                reason: decision.reason,
            }));
        }

        // --- 5/6. replay and double counting -------------------------------
        let digest = event.digest()?;
        match self.meter.offer(&event, now)? {
            MeterOutcome::Counted { .. } => {}
            MeterOutcome::CountedWithReversal { collision, .. } => {
                // The incumbent's contribution has been reversed in the
                // aggregator; drop it from the pending set too so the
                // receipt's coverage matches the counter.
                let reversed = collision.incumbent.to_string();
                for v in self.pending.values_mut() {
                    v.retain(|d| d.to_string() != reversed);
                }
            }
            MeterOutcome::Rejected(r) => {
                let detail = format!("{r:?}");
                if matches!(r, duap_meter::RejectReason::Duplicate { .. }) {
                    self.stats.duplicates += 1;
                    return Ok(IngestOutcome::Duplicate { event: digest });
                }
                self.stats.rejected_replay += 1;
                return Ok(IngestOutcome::Rejected(RejectAt::Replay { detail }));
            }
            MeterOutcome::Suppressed(c) | MeterOutcome::Suspended(c) => {
                self.stats.rejected_double_count += 1;
                return Ok(IngestOutcome::Rejected(RejectAt::DoubleCount {
                    detail: format!(
                        "operation already claimed by {} (fingerprint {})",
                        c.incumbent_reporter, c.fingerprint
                    ),
                }));
            }
            MeterOutcome::Invalid(d) => {
                self.stats.rejected_structure += 1;
                return Ok(IngestOutcome::Rejected(RejectAt::Structure { detail: d }));
            }
        }

        // --- 7/8. canonical identity and append -----------------------------
        let seq = self.store.append(digest, &event)?;
        let key = self.meter.aggregator.key_for(&event);
        self.pending.entry(key).or_default().push(digest);
        self.decisions.insert(digest.to_string(), decision.clone());

        // Record provenance where the event declares it.
        self.record_provenance(&event, &digest)?;

        // --- 9. acknowledge --------------------------------------------------
        self.stats.accepted += 1;
        Ok(IngestOutcome::Accepted {
            ack: Acknowledgement {
                schema: 1,
                node: self.org.clone(),
                event: digest,
                store_sequence: seq,
                log_index: None,
                accepted_at: now,
            },
            decision: Box::new(decision),
        })
    }

    fn record_provenance(
        &mut self,
        event: &DataUsageEvent,
        _digest: &Digest,
    ) -> Result<(), ClearingError> {
        use duap_provenance::{Edge, NodeKind, ProvNode};
        if let Some(out) = event.provenance.output {
            // Ensure every declared input exists as a node; an input the
            // node has never seen is recorded as a source attributed to the
            // event's subject, which is the conservative reading.
            let mut edges = Vec::new();
            let inputs = if event.provenance.inputs.is_empty() {
                // No explicit inputs: the derivation is from the subject's
                // own data in this event.
                vec![ContentId::of_bytes(
                    "duap.object.v1",
                    event.id.to_string().as_bytes(),
                )]
            } else {
                event.provenance.inputs.clone()
            };
            let w = Ratio::new(1, inputs.len() as u64)?;
            for i in &inputs {
                if self.provenance.get(i).is_none() {
                    if let Some(s) = event.subject.subject_ref() {
                        let _ = self.provenance.insert(ProvNode::source(
                            *i,
                            event.controller.clone(),
                            s,
                            event.occurred_at,
                        ));
                    } else {
                        continue;
                    }
                }
                edges.push(Edge::uniform(*i, w));
            }
            if !edges.is_empty() {
                let kind = match event.operation {
                    Operation::AiPretrain
                    | Operation::AiFinetune
                    | Operation::AiInstructionTune
                    | Operation::AiDistill => NodeKind::Model,
                    Operation::ProcessAggregate => NodeKind::Aggregate,
                    Operation::ProcessDpRelease => NodeKind::DpRelease,
                    Operation::AiSynthesize => NodeKind::Synthetic,
                    _ => NodeKind::Derived,
                };
                let _ = self.provenance.insert(ProvNode::derived(
                    out,
                    kind,
                    event.controller.clone(),
                    event.occurred_at,
                    edges,
                    event.operation,
                ));
            }
        }
        Ok(())
    }

    /// Seal the pending events into a transparency-log batch.
    ///
    /// Batching keeps the log's write rate independent of the event rate:
    /// one entry per batch, not one per event, with the batch's Merkle root
    /// as the committed object. An auditor who wants per-event inclusion
    /// gets it from the batch root plus the event set.
    pub fn seal_batch(&mut self, at: Timestamp) -> Result<Option<(u64, Digest)>, ClearingError> {
        let mut all: Vec<Digest> = self.pending.values().flatten().copied().collect();
        if all.is_empty() {
            return Ok(None);
        }
        all.sort();
        let root = events_root(HashAlg::Sha2_256, &all);
        let idx = self.log.append(LogEntry {
            kind: EntryKind::EventBatch,
            object: root,
            submitter: self.org.clone(),
            sequenced_at: at,
            shard: None,
        })?;
        Ok(Some((idx, root)))
    }
}

/// Everything produced by closing a period.
#[derive(Debug)]
pub struct PeriodResult {
    pub receipts: Vec<(Receipt, Envelope)>,
    pub invoices: Vec<duap_ledger::Invoice>,
    pub journal_entries: Vec<String>,
    pub unpriced: Vec<UsageKey>,
    pub total_charged: Money,
    pub total_subject_share: Money,
}

impl ClearingNode {
    /// Close an accounting period: price the counters, issue receipts,
    /// assemble invoices, post the ledger, accrue subject balances.
    pub fn close_period<T: TaxPolicy>(
        &mut self,
        period: TimeRange,
        tax: &T,
        at: Timestamp,
    ) -> Result<PeriodResult, ClearingError> {
        let counters: Vec<(UsageKey, UsageCounter)> = self
            .meter
            .aggregator
            .iter()
            .filter(|(k, _)| period.contains(k.window_start))
            .map(|(k, c)| (k.clone(), c.clone()))
            .collect();

        let mut receipts = Vec::new();
        let mut unpriced = Vec::new();
        let mut total = Money::zero(self.config.currency);
        let mut subject_total = Money::zero(self.config.currency);
        let mut by_payer: BTreeMap<String, InvoiceBuilder> = BTreeMap::new();
        let mut payer_ids: BTreeMap<String, OrgId> = BTreeMap::new();

        for (key, counter) in counters {
            if counter.quantity == 0 {
                continue;
            }
            let digests = self.pending.get(&key).cloned().unwrap_or_default();
            let Some(first) = digests.first() else {
                continue;
            };
            let decision = self
                .decisions
                .get(&first.to_string())
                .cloned()
                .unwrap_or_else(|| unreachable_decision());

            let Some(rule) = decision.pricing.clone() else {
                unpriced.push(key.clone());
                continue;
            };
            let inputs = PricingInputs::default();
            let breakdown = self.engine.price(&key, &counter, &rule, &inputs)?;
            let (charge, _residue) = breakdown.amount.round_to_money(self.config.rounding);
            let subject_share = match key.subject {
                Some(_) => {
                    let (m, _) = Precise::from_money(charge)?
                        .mul_ratio(self.config.default_subject_share)?
                        .round_to_money(Rounding::TowardZero);
                    Some(m)
                }
                None => None,
            };

            let root = events_root(HashAlg::Sha2_256, &digests);
            let coverage = Coverage {
                data_class: key.data_class,
                operation: key.operation,
                purpose: key.purpose,
                country: key.country.clone(),
                quantity: Quantity::new(key.unit, counter.quantity),
                event_count: digests.len() as u64,
                events_root: root,
            };
            let subject_scope = match key.subject {
                Some(s) => SubjectScope::Subject { subject: s },
                None => SubjectScope::NonPersonal,
            };
            let stored = self.store.get(first)?;
            let authorization = stored
                .as_ref()
                .map(|s| s.event.authorization.clone())
                .unwrap_or_else(|| AuthorizationRef {
                    grant: GrantId([0u8; 16]),
                    grant_digest: Digest::of(HashAlg::Sha2_256, "duap.grant.v1", b""),
                    epoch: 0,
                });
            let provenance_output = stored.as_ref().and_then(|s| s.event.provenance.output);

            let mut builder = ReceiptBuilder::new(
                self.org.clone(),
                key.controller.clone(),
                subject_scope,
                period,
                coverage,
                authorization,
                DecisionSummary {
                    permitting_terms: if decision.permitting_terms.is_empty() {
                        vec![0]
                    } else {
                        decision.permitting_terms.clone()
                    },
                    obligations: decision
                        .obligations
                        .iter()
                        .map(|o| o.label().to_owned())
                        .collect(),
                    deferred: decision.deferred.clone(),
                },
                charge,
                Digest::of_object("duap.price.v1", &breakdown)?,
                at,
            );
            if let Some(p) = &key.processor {
                builder = builder.processor(p.clone());
            }
            if let Some(s) = subject_share {
                builder = builder.subject_share(s);
            }
            if let Some(o) = provenance_output {
                builder = builder.provenance_output(o);
            }
            let receipt = builder.build()?;

            // Anchor the receipt in the log, then attach the proof.
            let core_digest = receipt.digest()?;
            let idx = self.log.append(LogEntry {
                kind: EntryKind::Receipt,
                object: core_digest,
                submitter: self.org.clone(),
                sequenced_at: at,
                shard: None,
            })?;
            let head = self.log.head(at);
            let proof = self
                .log
                .inclusion_proof(idx, self.log.size())
                .expect("just appended");
            let mut anchored = receipt.clone();
            anchored.anchor = Some(LogAnchor {
                log_id: self.log.log_id.clone(),
                index: idx,
                proof,
                head,
            });
            let mut env = anchored.seal()?;
            env.sign(&self.signing_key, at.0, None)?;
            receipts.push((anchored, env));
            self.stats.receipts_issued += 1;

            let payer = key.controller.clone();
            let pk = payer.to_string();
            payer_ids.insert(pk.clone(), payer.clone());
            let b = by_payer.entry(pk).or_insert_with(|| {
                InvoiceBuilder::new(
                    InvoiceId::from_digest(&Digest::of(
                        HashAlg::Sha2_256,
                        "duap.invoice-id.v1",
                        format!("{}|{}|{}", self.org, payer, period.start).as_bytes(),
                    )),
                    self.org.clone(),
                    payer.clone(),
                    period,
                    self.config.currency,
                )
                .rounding(self.config.rounding)
            });
            b.line(
                key.clone(),
                root,
                breakdown,
                format!(
                    "{} x {} of {} for {}",
                    counter.quantity,
                    key.unit.code(),
                    key.data_class.code(),
                    key.purpose.code()
                ),
                key.subject.map(|_| self.config.default_subject_share),
            )?;

            total = total.add(&charge)?;
            if let Some(s) = subject_share {
                subject_total = subject_total.add(&s)?;
                if let Some(sub) = key.subject {
                    self.payouts.accrue(sub, Precise::from_money(s)?, at)?;
                }
            }
        }

        let mut invoices = Vec::new();
        let mut entry_ids = Vec::new();
        for (pk, builder) in by_payer {
            let payer = payer_ids.remove(&pk).expect("payer recorded");
            let inv = builder.build(tax, at, at.saturating_add(30 * DAY))?;
            inv.check_arithmetic()?;
            for (id, kind) in inv.required_accounts() {
                let name = id.to_string();
                self.ledger
                    .open_account(Account::new(id, kind, self.config.currency, name));
            }
            self.ledger.ensure_account(
                AccountId::receivable(&payer),
                AccountKind::Asset,
                self.config.currency,
            );
            let entry = inv.to_journal_entry(at)?;
            let id = entry.id.clone();
            self.ledger.post(entry)?;
            entry_ids.push(id);
            self.log.append(LogEntry {
                kind: EntryKind::Invoice,
                object: inv.digest()?,
                submitter: self.org.clone(),
                sequenced_at: at,
                shard: None,
            })?;
            self.stats.invoices_issued += 1;
            invoices.push(inv);
        }

        Ok(PeriodResult {
            receipts,
            invoices,
            journal_entries: entry_ids,
            unpriced,
            total_charged: total,
            total_subject_share: subject_total,
        })
    }

    /// Default tax policy: none, stated explicitly.
    pub fn close_period_untaxed(
        &mut self,
        period: TimeRange,
        at: Timestamp,
    ) -> Result<PeriodResult, ClearingError> {
        self.close_period(period, &NoTax, at)
    }
}

fn unreachable_decision() -> Decision {
    Decision {
        effect: Effect::Deny,
        reason: DecisionReason::DefaultEffect,
        permitting_terms: vec![],
        obligations: vec![],
        deferred: vec![],
        pricing: None,
        currency: None,
    }
}
