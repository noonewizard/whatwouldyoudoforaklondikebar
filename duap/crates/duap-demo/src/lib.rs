//! # duap-demo
//!
//! A complete, deterministic, synthetic DUAP transaction.
//!
//! STATUS: REFERENCE.
//!
//! # What this is
//!
//! One data subject, one controller (Acme Example Corp), one processor, one
//! buyer, one clearing node, and every stage of the protocol exercised end
//! to end in a single process with no network and no external services.
//!
//! Everything is synthetic. The "personal data" is a hard-coded string in a
//! commitment; no real person's data appears anywhere in this repository,
//! and `.claude/rules/` forbids it.
//!
//! # What it demonstrates
//!
//! 1. A subject issues an authorization with priced terms and explicit
//!    prohibitions.
//! 2. Acme collects data under it.
//! 3. Acme processes, derives, transfers and uses the data for AI training.
//! 4. An attempt to use the data outside the authorization is refused.
//! 5. A second reporter's duplicate claim on one operation is detected.
//! 6. The subject revokes; subsequent usage is refused.
//! 7. The period closes: usage is priced, receipts are issued and anchored,
//!    an invoice is raised and the ledger is posted.
//! 8. A dispute is filed, evidence is checked, an adjustment is posted.
//! 9. Settlement moves money and the subject's balance is credited.
//! 10. An independent verifier re-checks every receipt from the artefacts
//!     alone, with no access to the clearing node's internal state.
//!
//! Determinism: every key is derived from a fixed seed and every timestamp
//! is fixed, so the whole run reproduces byte for byte. `tests/` asserts
//! that, which is what makes the artefacts usable as conformance vectors.

use duap_auth::prelude::*;
use duap_canon::digest::{Digest, HashAlg};
use duap_clearing::{ClearingConfig, ClearingNode, IngestOutcome, PeriodResult, RejectAt};
use duap_crypto::{
    Envelope, KeyRecord, KeyRegistry, KeyRole, SecretKey, SuiteId, SuitePolicy, VerificationContext,
};
use duap_ledger::{
    AccountId, Claim as DisputeClaim, Dispute, DisputeState, Evidence, JournalEntry, NoTax,
    Posting, Rail, SettlementInstruction, SettlementState,
};
use duap_model::prelude::*;
use duap_provenance::DerivationPolicy;
use duap_sdk::prelude::*;
use serde::Serialize;

/// Fixed clock: 2026-09-21T00:00:00Z.
pub const T0: u64 = 1_758_412_800;

pub fn t(offset_secs: u64) -> Timestamp {
    Timestamp::from_secs(T0 + offset_secs)
}

/// A line in the demonstration transcript.
#[derive(Debug, Clone, Serialize)]
pub struct Step {
    pub n: u32,
    pub stage: String,
    pub detail: String,
    /// Machine-checkable facts asserted by this step.
    pub facts: Vec<String>,
}

/// The complete result, serialisable so tests and CI can diff it.
#[derive(Debug, Serialize)]
pub struct DemoResult {
    pub steps: Vec<Step>,
    pub events_accepted: u64,
    pub events_refused: u64,
    pub receipts: usize,
    pub invoice_total: String,
    pub subject_share: String,
    pub trial_balance_zero: bool,
    pub log_size: u64,
    pub log_root: String,
    pub attribution: Vec<(String, String)>,
    pub independent_verification_passed: bool,
    pub subject_paid: String,
    pub subject_outstanding: String,
}

struct Cast {
    subject: SubjectAgent,
    subject_pseudonym: SubjectRef,
    acme: OrgId,
    vendor: OrgId,
    buyer: OrgId,
    clearing: OrgId,
    acme_key: SecretKey,
    vendor_key: SecretKey,
    node_key: SecretKey,
}

fn cast() -> Cast {
    let acme: OrgId = "org:duap/acme-example-corp".parse().expect("valid org id");
    let vendor: OrgId = "org:duap/vendor-analytics".parse().expect("valid org id");
    let buyer: OrgId = "org:duap/buyer-mediaco".parse().expect("valid org id");
    let clearing: OrgId = "org:duap/clearing-eu-1".parse().expect("valid org id");
    let root = SubjectRoot::from_secret([0x11; 32]);
    let pseudonym = root.pseudonym_for(&acme);
    let subject_key = root.key_for(&acme, SuiteId::Ed25519);
    Cast {
        subject: SubjectAgent::new(root, subject_key),
        subject_pseudonym: pseudonym,
        acme,
        vendor,
        buyer,
        clearing,
        acme_key: SecretKey::from_seed(SuiteId::Ed25519, [0x22; 32]),
        vendor_key: SecretKey::from_seed(SuiteId::Ed25519, [0x33; 32]),
        // The clearing node signs with a hybrid suite: its signatures must
        // remain verifiable for as long as accounting records are kept.
        node_key: SecretKey::from_seed(SuiteId::Ed25519MlDsa44, [0x44; 32]),
    }
}

fn enrol(reg: &mut KeyRegistry, key: &SecretKey, holder: &OrgId, roles: Vec<KeyRole>) {
    reg.enroll(KeyRecord::new(
        key.public_key(),
        holder.to_string(),
        roles,
        0,
        None,
    ))
    .expect("enrolment of a freshly generated key cannot fail");
}

/// Run the whole demonstration.
pub fn run() -> Result<DemoResult, Box<dyn std::error::Error>> {
    let mut c = cast();
    let mut steps: Vec<Step> = Vec::new();
    let mut n = 0u32;
    let mut step = |stage: &str, detail: String, facts: Vec<String>| {
        n += 1;
        steps.push(Step {
            n,
            stage: stage.to_owned(),
            detail,
            facts,
        });
    };

    // -----------------------------------------------------------------
    // 1. The clearing node and the key registry
    // -----------------------------------------------------------------
    let mut node = ClearingNode::new(
        c.clearing.clone(),
        SecretKey::from_seed(SuiteId::Ed25519MlDsa44, [0x44; 32]),
        ClearingConfig::reference(Currency::EUR),
    );
    enrol(
        &mut node.registry,
        &c.acme_key,
        &c.acme,
        vec![KeyRole::EventSigner],
    );
    enrol(
        &mut node.registry,
        &c.vendor_key,
        &c.vendor,
        vec![KeyRole::EventSigner],
    );
    enrol(
        &mut node.registry,
        &c.node_key,
        &c.clearing,
        vec![KeyRole::ReceiptSigner, KeyRole::LogSigner],
    );
    // The subject's key, so the node can check the grant's signature.
    node.registry.enroll(KeyRecord::new(
        c.subject.public_key(),
        format!("subject:{}", c.subject_pseudonym),
        vec![KeyRole::AuthorizationSigner],
        0,
        None,
    ))?;
    step(
        "setup",
        format!(
            "Clearing node {} enrolled 4 keys; node signs with {}",
            c.clearing, c.node_key.suite
        ),
        vec![
            format!("registry_size={}", node.registry.len()),
            format!("node_suite={}", c.node_key.suite),
            format!("node_suite_pq={}", c.node_key.suite.quantum_resistant()),
        ],
    );

    // -----------------------------------------------------------------
    // 2. Authorization
    // -----------------------------------------------------------------
    // 0.002 minor units per query.
    let _per_query = PricingRule::per_unit(Unit::Query, Precise::new(Currency::EUR, 2_000_000));
    // 0.05 minor units per record.
    let per_record = PricingRule::per_unit(Unit::Record, Precise::new(Currency::EUR, 50_000_000));
    let per_inference =
        PricingRule::per_unit(Unit::Inference, Precise::new(Currency::EUR, 20_000_000));
    let per_token = PricingRule::per_unit(Unit::Token, Precise::new(Currency::EUR, 100_000));

    let terms = vec![
        Term::permit(
            1,
            Matcher::any()
                .classes(ClassSelector::Namespace {
                    namespaces: vec!["location".into(), "behavior".into(), "transaction".into()],
                })
                .purposes(PurposeSelector::Under {
                    roots: vec![Purpose::Service],
                }),
        )
        // One term covering several operations needs prices for each unit
        // those operations are metered in.
        .with_pricing(PricingRule::unit_table([
            (Unit::Query, Precise::new(Currency::EUR, 2_000_000)),
            (Unit::Record, Precise::new(Currency::EUR, 30_000_000)),
            (Unit::SubjectDay, Precise::new(Currency::EUR, 5_000_000)),
        ]))
        .with_obligations(vec![Obligation::MaxRetentionDays { days: 90 }])
        .with_label("Service delivery"),
        Term::permit(
            2,
            Matcher::any()
                .classes(ClassSelector::In {
                    values: vec![DataClass::BehaviorWebBrowsing, DataClass::LocationCoarse],
                })
                .purposes(PurposeSelector::Under {
                    roots: vec![Purpose::MarketingAdvertisingContextual],
                }),
        )
        .with_pricing(per_inference.clone())
        .with_label("Contextual advertising only"),
        Term::permit(
            3,
            Matcher::any()
                .classes(ClassSelector::In {
                    values: vec![DataClass::TransactionPurchase],
                })
                .operations(Selector::In {
                    values: vec![Operation::ProcessAggregate, Operation::TransferLicense],
                }),
        )
        .with_pricing(per_record.clone())
        .with_obligations(vec![Obligation::TransferOnlyTo {
            orgs: vec![c.buyer.clone()],
        }])
        .with_label("Aggregate purchase data, licensable to one named buyer"),
        Term::permit(
            4,
            Matcher::any()
                .classes(ClassSelector::In {
                    values: vec![DataClass::ContentUserGenerated],
                })
                .operations(Selector::In {
                    values: vec![Operation::AiFinetune],
                }),
        )
        .with_pricing(per_token.clone())
        .with_obligations(vec![Obligation::MaxDerivationDepth { depth: 4 }])
        .with_label("Fine-tuning on my reviews, priced per token"),
        // Explicit prohibitions. A Deny always wins.
        Term::deny(
            10,
            Matcher::any().classes(ClassSelector::In {
                values: vec![DataClass::LocationPrecise, DataClass::HealthInferred],
            }),
        )
        .with_label("Never precise location or inferred health"),
        Term::deny(
            11,
            Matcher::any().purposes(PurposeSelector::Under {
                roots: vec![Purpose::RiskInsurance, Purpose::RiskCredit],
            }),
        )
        .with_label("Never insurance or credit"),
        Term::deny(
            12,
            Matcher::any().operations(Selector::In {
                values: vec![Operation::TransferSale],
            }),
        )
        .with_label("Never sold"),
    ];

    let (grant, grant_env) = c.subject.authorize_with(
        &c.acme,
        GrantId([0x55; 16]),
        terms,
        t(0),
        duap_sdk::GrantPolicy {
            expires_at: t(365 * 86_400),
            revocation: RevocationPolicy::AfterNotice { hours: 1 },
            currency: Currency::EUR,
        },
    )?;
    // The node verifies the grant's signature before storing it.
    let grant_signers = grant_env.verify(
        &node.registry,
        &SuitePolicy::draft_default(),
        &VerificationContext::archival(t(1).0),
    )?;
    node.authorizations.insert_grant(grant.clone())?;
    node.log.append(duap_provenance::LogEntry {
        kind: duap_provenance::EntryKind::Grant,
        object: grant.digest()?,
        submitter: c.acme.clone(),
        sequenced_at: t(1),
        shard: Some("eu".into()),
    })?;
    step(
        "authorization",
        format!(
            "Subject {} granted {} four priced permissions and three prohibitions",
            c.subject_pseudonym, c.acme
        ),
        vec![
            format!("grant={}", grant.id),
            format!("epoch={}", grant.epoch),
            format!("terms={}", grant.terms.len()),
            format!("default_effect={:?}", grant.default_effect),
            format!("signers={}", grant_signers.len()),
            format!("digest={}", grant.digest()?),
        ],
    );

    // -----------------------------------------------------------------
    // 3. Instrumented collection
    // -----------------------------------------------------------------
    let mut acme_agent = ControllerAgent::new(
        c.acme.clone(),
        AgentRef::new("duap-sdk-rust", env!("CARGO_PKG_VERSION")),
        Jurisdiction::new("DE")?.with_regimes([Regime::EuGdpr, Regime::EuDataAct]),
        SecretKey::from_seed(SuiteId::Ed25519, [0x22; 32]),
    );
    acme_agent.use_authorization(grant.reference()?);

    let subject_scope = SubjectScope::Subject {
        subject: c.subject_pseudonym,
    };

    // A salted commitment stands in for the value, which never leaves Acme.
    //
    // The salt is fixed here so the whole run reproduces byte for byte and
    // the artefacts can serve as conformance vectors. A real agent MUST use
    // a fresh random salt: a predictable salt destroys the hiding property
    // for low-entropy values, which is the whole point of salting.
    const DEMO_SALT: [u8; 32] = [0xA5; 32];
    let commitment = Commitment::with_salt(b"synthetic:city=Berlin;district=Mitte", DEMO_SALT);

    let mut accepted = 0u64;
    let mut refused = 0u64;
    let mut envelopes: Vec<(String, Envelope)> = Vec::new();

    let offer = |node: &mut ClearingNode,
                 _label: &str,
                 env: Envelope,
                 at: Timestamp|
     -> Result<IngestOutcome, Box<dyn std::error::Error>> {
        let o = node.ingest(&env, at)?;
        Ok(o)
    };

    // Collection: 1 record.
    let (_ev, _d, env) = acme_agent.record_usage(
        UsageRecord::new(
            subject_scope.clone(),
            DataClass::LocationCoarse,
            Operation::CollectObserve,
            Purpose::ServiceCore,
            1,
            t(10),
        )
        .collection(CollectionMethod::DeviceSensor)
        .retention(RetentionPolicy {
            basis: RetentionBasis::FixedPeriod,
            days: Some(30),
            until: None,
        })
        .commitment(commitment),
        t(10),
    )?;
    let out = offer(&mut node, "collect", env.clone(), t(11))?;
    assert!(out.accepted(), "collection must be authorised: {out:?}");
    accepted += 1;
    envelopes.push(("collect".into(), env));
    step(
        "collection",
        "Acme collected one coarse-location record under term 1".into(),
        vec![
            format!("unit={}", Operation::CollectObserve.meter().code()),
            format!("commitment={}", commitment.0),
            "raw_value_transmitted=false".into(),
        ],
    );

    // Service queries: 4200 of them, batched into one event per hour is the
    // realistic pattern; here two events for brevity.
    for (i, (amount, at)) in [(2_500u64, t(3_600)), (1_700, t(7_200))].iter().enumerate() {
        let (_e, _d, env) = acme_agent.record_usage(
            UsageRecord::new(
                subject_scope.clone(),
                DataClass::LocationCoarse,
                Operation::AccessQuery,
                Purpose::ServiceCore,
                *amount,
                *at,
            )
            .retention(RetentionPolicy {
                basis: RetentionBasis::FixedPeriod,
                days: Some(30),
                until: None,
            }),
            *at,
        )?;
        let o = offer(&mut node, "query", env.clone(), at.saturating_add(SECOND))?;
        assert!(o.accepted(), "service query {i} must be authorised: {o:?}");
        accepted += 1;
        envelopes.push((format!("query-{i}"), env));
    }
    step(
        "processing",
        "4200 service queries recorded across two metering intervals".into(),
        vec!["queries=4200".into(), "term=1".into()],
    );

    // -----------------------------------------------------------------
    // 4. Derivation: a contextual-advertising inference
    // -----------------------------------------------------------------
    let profile = ContentId::of_bytes("duap.object.v1", b"acme:contextual-segment:v1");
    let (_e, _d, env) = acme_agent.record_usage(
        UsageRecord::new(
            subject_scope.clone(),
            DataClass::BehaviorWebBrowsing,
            Operation::ProcessProfile,
            Purpose::MarketingAdvertisingContextual,
            120,
            t(10_800),
        )
        .provenance(Provenance {
            output: Some(profile),
            transform: Some("segmenter@1.4.2".into()),
            ..Default::default()
        }),
        t(10_800),
    )?;
    let o = offer(&mut node, "derive", env.clone(), t(10_801))?;
    assert!(
        o.accepted(),
        "contextual profiling must be authorised: {o:?}"
    );
    accepted += 1;
    envelopes.push(("derive".into(), env));
    step(
        "derivation",
        "120 contextual-advertising inferences produced a derived segment".into(),
        vec![
            format!("output={profile}"),
            format!("unit={}", Operation::ProcessProfile.meter().code()),
        ],
    );

    // -----------------------------------------------------------------
    // 5. A refused use: behavioural advertising is outside the grant
    // -----------------------------------------------------------------
    let (_e, _d, env) = acme_agent.record_usage(
        UsageRecord::new(
            subject_scope.clone(),
            DataClass::BehaviorWebBrowsing,
            Operation::CommercialAdvertise,
            Purpose::MarketingAdvertisingBehavioral,
            500,
            t(11_000),
        ),
        t(11_000),
    )?;
    let refused_behavioural = offer(&mut node, "behavioural", env, t(11_001))?;
    assert!(!refused_behavioural.accepted());
    refused += 1;
    step(
        "refusal",
        "Behavioural advertising refused: no term permits that purpose".into(),
        vec![format!("outcome={refused_behavioural:?}")],
    );

    // A prohibited class.
    let (_e, _d, env) = acme_agent.record_usage(
        UsageRecord::new(
            subject_scope.clone(),
            DataClass::LocationPrecise,
            Operation::AccessQuery,
            Purpose::ServiceCore,
            1,
            t(11_100),
        ),
        t(11_100),
    )?;
    let refused_precise = offer(&mut node, "precise", env, t(11_101))?;
    assert!(!refused_precise.accepted());
    refused += 1;
    let denied_by_term_10 = matches!(
        &refused_precise,
        IngestOutcome::Rejected(RejectAt::Authorization {
            reason: DecisionReason::DeniedByTerm { term: 10 }
        })
    );
    step(
        "refusal",
        "Precise location refused by the subject's explicit prohibition".into(),
        vec![format!("denied_by_term_10={denied_by_term_10}")],
    );
    assert!(denied_by_term_10, "the Deny term must be the stated reason");

    // A prohibited sale.
    let (_e, _d, env) = acme_agent.record_usage(
        UsageRecord::new(
            subject_scope.clone(),
            DataClass::TransactionPurchase,
            Operation::TransferSale,
            Purpose::CommerceSale,
            10,
            t(11_200),
        )
        .economics(EconomicContext {
            counterparty: Some(c.buyer.clone()),
            ..Default::default()
        }),
        t(11_200),
    )?;
    let refused_sale = offer(&mut node, "sale", env, t(11_201))?;
    assert!(!refused_sale.accepted());
    refused += 1;
    step(
        "refusal",
        "Sale refused: term 12 denies transfer.sale outright".into(),
        vec![format!("outcome_is_denial={}", !refused_sale.accepted())],
    );

    // -----------------------------------------------------------------
    // 6. A permitted transfer, and a rejected one to the wrong buyer
    // -----------------------------------------------------------------
    let aggregate = ContentId::of_bytes("duap.object.v1", b"acme:purchase-aggregate:2025-09");
    let (_e, _d, env) = acme_agent.record_usage(
        UsageRecord::new(
            subject_scope.clone(),
            DataClass::TransactionPurchase,
            Operation::ProcessAggregate,
            Purpose::ImprovementAnalytics,
            40,
            t(14_400),
        )
        .provenance(Provenance {
            output: Some(aggregate),
            transform: Some("rollup@2.0.0".into()),
            ..Default::default()
        }),
        t(14_400),
    )?;
    let o = offer(&mut node, "aggregate", env.clone(), t(14_401))?;
    assert!(o.accepted(), "aggregation must be authorised: {o:?}");
    accepted += 1;
    envelopes.push(("aggregate".into(), env));

    let (_e, _d, env) = acme_agent.record_usage(
        UsageRecord::new(
            subject_scope.clone(),
            DataClass::TransactionPurchase,
            Operation::TransferLicense,
            Purpose::CommerceLicensing,
            40,
            t(18_000),
        )
        .economics(EconomicContext {
            counterparty: Some(c.buyer.clone()),
            ..Default::default()
        }),
        t(18_000),
    )?;
    let o = offer(&mut node, "license", env.clone(), t(18_001))?;
    assert!(
        o.accepted(),
        "licensing to the allowlisted buyer must pass: {o:?}"
    );
    accepted += 1;
    envelopes.push(("license".into(), env));

    let stranger: OrgId = "org:duap/unlisted-broker".parse()?;
    let (_e, _d, env) = acme_agent.record_usage(
        UsageRecord::new(
            subject_scope.clone(),
            DataClass::TransactionPurchase,
            Operation::TransferLicense,
            Purpose::CommerceLicensing,
            40,
            t(18_100),
        )
        .economics(EconomicContext {
            counterparty: Some(stranger.clone()),
            ..Default::default()
        }),
        t(18_100),
    )?;
    let refused_transfer = offer(&mut node, "license-stranger", env, t(18_101))?;
    assert!(!refused_transfer.accepted());
    refused += 1;
    step(
        "transfer",
        format!(
            "Licensed 40 aggregate records to {}; an identical transfer to {} was refused by the allowlist obligation",
            c.buyer, stranger
        ),
        vec![
            "allowlisted_transfer=accepted".into(),
            "unlisted_transfer=refused".into(),
        ],
    );

    // -----------------------------------------------------------------
    // 7. AI training
    // -----------------------------------------------------------------
    let model = ContentId::of_bytes("duap.object.v1", b"acme:review-model:ft-2025-09");
    let (_e, _d, env) = acme_agent.record_usage(
        UsageRecord::new(
            subject_scope.clone(),
            DataClass::ContentUserGenerated,
            Operation::AiFinetune,
            Purpose::ImprovementAiTraining,
            18_400,
            t(21_600),
        )
        .provenance(Provenance {
            inputs: vec![profile],
            output: Some(model),
            transform: Some("lora-r16@0.9".into()),
        }),
        t(21_600),
    )?;
    let o = offer(&mut node, "finetune", env.clone(), t(21_601))?;
    assert!(o.accepted(), "fine-tuning must be authorised: {o:?}");
    accepted += 1;
    envelopes.push(("finetune".into(), env));
    step(
        "ai_training",
        "18400 tokens of the subject's reviews entered a fine-tuning run".into(),
        vec![
            format!("model={model}"),
            format!("unit={}", Operation::AiFinetune.meter().code()),
            "attribution_claim=inclusion_only".into(),
        ],
    );

    // -----------------------------------------------------------------
    // 8. Double counting: the vendor reports the same aggregation
    // -----------------------------------------------------------------
    let mut vendor_agent = ControllerAgent::new(
        c.acme.clone(), // the vendor acts for Acme as a processor
        AgentRef::new("vendor-collector", "3.1.0"),
        Jurisdiction::new("DE")?.with_regimes([Regime::EuGdpr]),
        SecretKey::from_seed(SuiteId::Ed25519, [0x33; 32]),
    );
    vendor_agent.use_authorization(grant.reference()?);
    let (_e, _d, env) = vendor_agent.record_usage(
        UsageRecord::new(
            subject_scope.clone(),
            DataClass::TransactionPurchase,
            Operation::ProcessAggregate,
            Purpose::ImprovementAnalytics,
            40,
            t(14_400),
        )
        .processor(c.vendor.clone())
        .provenance(Provenance {
            output: Some(aggregate),
            transform: Some("rollup@2.0.0".into()),
            ..Default::default()
        }),
        t(14_402),
    )?;
    let dup = offer(&mut node, "vendor-duplicate", env, t(14_403))?;
    let caught = matches!(dup, IngestOutcome::Rejected(RejectAt::DoubleCount { .. }));
    if !caught {
        refused += 0;
    } else {
        refused += 1;
    }
    step(
        "double_counting",
        "The processor reported the same aggregation the controller had already reported".into(),
        vec![
            format!("detected={caught}"),
            "policy=controller_wins".into(),
        ],
    );
    assert!(
        caught,
        "a second reporter of one operation must be detected"
    );

    // -----------------------------------------------------------------
    // 9. Revocation
    // -----------------------------------------------------------------
    let (revocation, rev_env) = c.subject.revoke_authorization(
        &grant,
        RevocationScope::Purposes {
            purposes: vec![Purpose::ImprovementAiTraining],
        },
        RetroactiveRequest::DeleteSourceAndDerived,
        t(25_200),
    )?;
    rev_env.verify(
        &node.registry,
        &SuitePolicy::draft_default(),
        &VerificationContext::archival(t(25_201).0),
    )?;
    node.authorizations.insert_revocation(revocation.clone())?;
    node.log.append(duap_provenance::LogEntry {
        kind: duap_provenance::EntryKind::Revocation,
        object: revocation.digest()?,
        submitter: "org:duap/subject-agent".to_string().parse()?,
        sequenced_at: t(25_201),
        shard: Some("eu".into()),
    })?;

    // Training after the notice period must now fail.
    let (_e, _d, env) = acme_agent.record_usage(
        UsageRecord::new(
            subject_scope.clone(),
            DataClass::ContentUserGenerated,
            Operation::AiFinetune,
            Purpose::ImprovementAiTraining,
            5_000,
            t(25_200 + 2 * 3600),
        )
        .provenance(Provenance {
            output: Some(ContentId::of_bytes(
                "duap.object.v1",
                b"acme:review-model:ft-later",
            )),
            ..Default::default()
        }),
        t(25_200 + 2 * 3600),
    )?;
    let after = offer(
        &mut node,
        "finetune-after-revocation",
        env,
        t(25_200 + 2 * 3600 + 1),
    )?;
    assert!(
        !after.accepted(),
        "post-revocation training must be refused"
    );
    refused += 1;

    // Service queries are untouched: the revocation was scoped.
    let (_e, _d, env) = acme_agent.record_usage(
        UsageRecord::new(
            subject_scope.clone(),
            DataClass::LocationCoarse,
            Operation::AccessQuery,
            Purpose::ServiceCore,
            300,
            t(25_200 + 3 * 3600),
        )
        .retention(RetentionPolicy {
            basis: RetentionBasis::FixedPeriod,
            days: Some(30),
            until: None,
        }),
        t(25_200 + 3 * 3600),
    )?;
    let still_ok = offer(
        &mut node,
        "query-after-revocation",
        env.clone(),
        t(25_200 + 3 * 3600 + 1),
    )?;
    assert!(
        still_ok.accepted(),
        "a scoped revocation must not stop unrelated use"
    );
    accepted += 1;
    envelopes.push(("query-after-revocation".into(), env));

    step(
        "revocation",
        "Subject withdrew AI-training permission with one hour's notice; service use continues"
            .into(),
        vec![
            format!("effective_from={}", revocation.effective_from),
            format!("retroactive={:?}", revocation.retroactive),
            "training_after=refused".into(),
            "service_after=accepted".into(),
            "already_trained_model=NOT_UNLEARNED".into(),
        ],
    );

    // -----------------------------------------------------------------
    // 10. Seal the batch and close the period
    // -----------------------------------------------------------------
    let sealed = node.seal_batch(t(86_000))?;
    let period = TimeRange::new(t(0), t(2 * 86_400))?;
    let result: PeriodResult = node.close_period(period, &NoTax, t(86_400))?;
    assert!(
        result.unpriced.is_empty(),
        "every counter must carry a price: {:?}",
        result.unpriced
    );
    step(
        "accounting",
        format!(
            "Period closed: {} receipts, {} invoice(s), total {}",
            result.receipts.len(),
            result.invoices.len(),
            result.total_charged
        ),
        vec![
            format!(
                "batch_root={}",
                sealed.map(|(_, r)| r.to_string()).unwrap_or_default()
            ),
            format!("total_charged={}", result.total_charged),
            format!("subject_share={}", result.total_subject_share),
            format!("journal_entries={}", result.journal_entries.len()),
            format!("trial_balance_zero={}", node.ledger.is_balanced()),
        ],
    );

    let invoice = result.invoices.first().expect("one payer").clone();
    invoice.check_arithmetic()?;
    step(
        "billing",
        format!(
            "Invoice {} to {} for {} across {} lines",
            invoice.id,
            invoice.payer,
            invoice.total,
            invoice.lines.len()
        ),
        invoice
            .lines
            .iter()
            .map(|l| {
                format!(
                    "line{}: {} -> {} (subject {})",
                    l.line_no,
                    l.description,
                    l.amount,
                    l.subject_share
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "-".into())
                )
            })
            .collect(),
    );

    // -----------------------------------------------------------------
    // 11. Independent verification of every receipt
    // -----------------------------------------------------------------
    // A verifier that holds only the public key registry and the receipts.
    let mut verifier_registry = KeyRegistry::new();
    verifier_registry.enroll(KeyRecord::new(
        c.node_key.public_key(),
        c.clearing.to_string(),
        vec![KeyRole::ReceiptSigner, KeyRole::LogSigner],
        0,
        None,
    ))?;
    let mut all_verified = true;
    let mut claim_counts = (0usize, 0usize);
    for (_r, env) in &result.receipts {
        match duap_sdk::verify_receipt(
            env,
            &verifier_registry,
            &SuitePolicy::draft_default(),
            t(90_000),
        ) {
            Ok((v, claims)) => {
                claim_counts.0 += claims.iter().filter(|c| c.established).count();
                claim_counts.1 += claims.iter().filter(|c| !c.established).count();
                if !v.anchored {
                    all_verified = false;
                }
            }
            Err(_) => all_verified = false,
        }
    }
    step(
        "audit",
        "An independent verifier re-checked every receipt from the public key alone".into(),
        vec![
            format!("receipts={}", result.receipts.len()),
            format!("all_verified_and_anchored={all_verified}"),
            format!("claims_established={}", claim_counts.0),
            format!("claims_explicitly_not_established={}", claim_counts.1),
        ],
    );
    assert!(
        all_verified,
        "every issued receipt must verify and be anchored"
    );

    // -----------------------------------------------------------------
    // 12. Provenance and attribution
    // -----------------------------------------------------------------
    let (shares, dropped) =
        duap_sdk::query_provenance(&node.provenance, &model, &DerivationPolicy::default())?;
    let attribution: Vec<(String, String)> = shares
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    step(
        "provenance",
        format!(
            "The fine-tuned model attributes to {} subject(s); {} of the weight terminated",
            shares.len(),
            dropped
        ),
        vec![
            format!("model={model}"),
            format!("depth={}", node.provenance.depth(&model)?),
            format!(
                "descendants_of_profile={}",
                node.provenance.descendants(&profile).len()
            ),
            "claim=inclusion, NOT influence, NOT economic contribution".into(),
        ],
    );

    // -----------------------------------------------------------------
    // 13. Dispute
    // -----------------------------------------------------------------
    let disputed_receipt = &result.receipts[0].0;
    let mut dispute = Dispute::file(
        DisputeId([0x66; 16]),
        format!("subject:{}", c.subject_pseudonym),
        c.acme.clone(),
        DisputeClaim::QuantityWrong {
            line: 1,
            asserted: disputed_receipt.coverage.quantity.amount,
            claimed: disputed_receipt.coverage.quantity.amount / 2,
        },
        vec![
            Evidence::Receipt {
                digest: disputed_receipt.digest()?,
            },
            Evidence::LogInclusion {
                log: node.log.log_id.clone(),
                entry: disputed_receipt.digest()?,
                size: node.log.size(),
            },
            Evidence::Assertion {
                by: "subject".into(),
                statement: "My device was offline for part of the period".into(),
            },
        ],
        t(90_000),
    )?;
    dispute.invoice = Some(invoice.id);
    dispute.transition(DisputeState::UnderReview, t(90_100), None)?;
    // The controller's counter-evidence: the event set behind the receipt.
    dispute.counter_evidence.push(Evidence::Event {
        digest: disputed_receipt.coverage.events_root,
    });
    let adjustment = Money::new(Currency::EUR, 1);
    dispute.propose_adjustment(adjustment);
    dispute.transition(
        DisputeState::Settled,
        t(90_200),
        Some("Goodwill adjustment; the event set was verifiable but the subject's account of the period was credible".into()),
    )?;
    if let Some(adj) = dispute.to_adjustment_entry(&node.ledger, t(90_300))? {
        node.ledger.post(adj)?;
    }
    step(
        "dispute",
        "A quantity dispute was filed, weighed against verifiable evidence, and settled".into(),
        vec![
            format!(
                "verifiable_evidence={}",
                dispute.verifiable_evidence_count()
            ),
            format!("assertions={}", 1),
            format!("state={:?}", dispute.state),
            format!("adjustment={adjustment}"),
            format!("trial_balance_zero={}", node.ledger.is_balanced()),
        ],
    );

    // -----------------------------------------------------------------
    // 14. Settlement
    // -----------------------------------------------------------------
    // Acme pays the invoice.
    node.ledger.post(JournalEntry::new(
        "je:demo:payment",
        t(95_000),
        format!("Payment received from {}", c.acme),
        vec![
            Posting::debit(
                AccountId::settlement_cash(Currency::EUR),
                invoice.total.sub(&adjustment)?,
            ),
            Posting::credit(
                AccountId::receivable(&c.acme),
                invoice.total.sub(&adjustment)?,
            ),
        ],
    ))?;

    // A single period leaves the subject with a fraction of a euro, which no
    // payment rail will move economically. That is the micropayment problem,
    // and the protocol's answer is to carry the balance as a liability until
    // it is worth settling. To show the settlement path actually executing,
    // the demonstration accrues eleven further identical periods -- labelled
    // as a simulation, not as observed usage -- and then releases.
    let per_period = result.total_subject_share;
    let simulated_periods = 11i128;
    let extra = Money::new(Currency::EUR, per_period.minor * simulated_periods);
    node.ledger.post(JournalEntry::new(
        "je:demo:simulated-periods",
        t(95_500),
        format!("{simulated_periods} further periods, simulated for the settlement demonstration"),
        vec![
            Posting::debit(AccountId::settlement_cash(Currency::EUR), extra),
            Posting::credit(AccountId::subject_payable(&c.subject_pseudonym), extra),
        ],
    ))?;
    node.payouts
        .accrue(c.subject_pseudonym, Precise::from_money(extra)?, t(95_500))?;

    // The node releases the subject's balance if it crosses the threshold.
    let released = node.payouts.release(t(96_000))?;
    let mut paid = Money::zero(Currency::EUR);
    for p in &released {
        let si = SettlementInstruction::new(
            SettlementId([0x77; 16]),
            AccountId::settlement_cash(Currency::EUR).to_string(),
            format!("subject:{}", p.recipient),
            p.amount,
            Rail::Internal,
            t(96_000),
        );
        let entry = si.to_journal_entry(
            AccountId::subject_payable(&p.recipient),
            AccountId::settlement_cash(Currency::EUR),
            t(96_100),
        );
        node.ledger.post(entry)?;
        paid = paid.add(&p.amount)?;
        assert_eq!(si.state, SettlementState::Created);
    }
    let outstanding = node.payouts.total_outstanding()?;
    step(
        "settlement",
        if released.is_empty() {
            format!(
                "Subject balance {} is below the {} payout threshold and was carried forward",
                outstanding,
                node.payouts.threshold()
            )
        } else {
            format!("Released {paid} to the subject; {outstanding} carried forward")
        },
        vec![
            format!("payouts={}", released.len()),
            format!("paid={paid}"),
            format!("outstanding={outstanding}"),
            format!("threshold={}", node.payouts.threshold()),
            format!("simulated_extra_periods={simulated_periods}"),
            format!("trial_balance_zero={}", node.ledger.is_balanced()),
        ],
    );

    // -----------------------------------------------------------------
    // Result
    // -----------------------------------------------------------------
    let head = node.log.head(t(96_200));
    Ok(DemoResult {
        steps,
        events_accepted: accepted,
        events_refused: refused,
        receipts: result.receipts.len(),
        invoice_total: invoice.total.to_string(),
        subject_share: result.total_subject_share.to_string(),
        trial_balance_zero: node.ledger.is_balanced(),
        log_size: head.size,
        log_root: head.root.to_string(),
        attribution,
        independent_verification_passed: all_verified,
        subject_paid: paid.to_string(),
        subject_outstanding: outstanding.to_string(),
    })
}

/// Verify that the opening of the demonstration's commitment checks out.
/// Included because a commitment nobody ever opens proves nothing.
pub fn check_commitment() -> bool {
    let (c, o) =
        Commitment::commit(b"synthetic:city=Berlin;district=Mitte").expect("entropy available");
    c.verify(b"synthetic:city=Berlin;district=Mitte", &o)
        && !c.verify(b"synthetic:city=Munich;district=Mitte", &o)
}

/// Digest of the demonstration's declared inputs, for reproducibility
/// checking in CI.
pub fn scenario_digest() -> Digest {
    Digest::of(
        HashAlg::Sha2_256,
        "duap.demo-scenario.v1",
        format!("acme-example-corp|t0={T0}|v={}", env!("CARGO_PKG_VERSION")).as_bytes(),
    )
}
