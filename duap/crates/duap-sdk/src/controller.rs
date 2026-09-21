//! The controller's side of the protocol.
//!
//! STATUS: PRODUCTION.
//!
//! A controller agent is what an instrumented system embeds. It turns an
//! application-level fact ("we ran a personalisation inference for this
//! user") into a signed, validated, sequenced Data Usage Event, and it
//! refuses to produce one that would fail the clearing node's checks --
//! failing locally, loudly, at integration time, rather than remotely and
//! silently in production.

use duap_canon::digest::Digest;
use duap_crypto::{Envelope, SecretKey};
use duap_model::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum SdkError {
    #[error("{0}")]
    Model(#[from] ModelError),
    #[error("{0}")]
    Crypto(#[from] duap_crypto::CryptoError),
    #[error("{0}")]
    Canon(#[from] duap_canon::CanonError),
    #[error("{0}")]
    Receipt(#[from] duap_receipt::ReceiptError),
    #[error("{0}")]
    Graph(#[from] duap_provenance::GraphError),
    #[error("{0}")]
    Pricing(#[from] duap_valuation::PricingError),
    #[error("{0}")]
    Ledger(#[from] duap_ledger::LedgerError),
    #[error("the agent has no authorization on file for {0}")]
    NoAuthorization(String),
    #[error("configuration error: {0}")]
    Config(String),
}

/// One usage to record.
///
/// Everything not in this struct is either derived (the metering unit, the
/// sensitivity floor, the record time) or held by the agent (the controller
/// identity, the agent version, the grant reference), which is what keeps
/// the call site short without hiding anything load-bearing.
#[derive(Debug, Clone)]
pub struct UsageRecord {
    pub subject: SubjectScope,
    pub data_class: DataClass,
    pub operation: Operation,
    pub purpose: Purpose,
    pub amount: u64,
    pub occurred_at: Timestamp,
    pub collection: Option<CollectionMethod>,
    pub retention: Option<RetentionPolicy>,
    pub provenance: Provenance,
    pub economics: Option<EconomicContext>,
    pub commitment: Option<Commitment>,
    pub processor: Option<OrgId>,
    /// Raise the sensitivity above the class default where context demands.
    pub sensitivity: Option<SensitivityTier>,
}

impl UsageRecord {
    pub fn new(
        subject: SubjectScope,
        data_class: DataClass,
        operation: Operation,
        purpose: Purpose,
        amount: u64,
        occurred_at: Timestamp,
    ) -> UsageRecord {
        UsageRecord {
            subject,
            data_class,
            operation,
            purpose,
            amount,
            occurred_at,
            collection: None,
            retention: None,
            provenance: Provenance::default(),
            economics: None,
            commitment: None,
            processor: None,
            sensitivity: None,
        }
    }

    pub fn collection(mut self, c: CollectionMethod) -> Self {
        self.collection = Some(c);
        self
    }
    pub fn retention(mut self, r: RetentionPolicy) -> Self {
        self.retention = Some(r);
        self
    }
    pub fn provenance(mut self, p: Provenance) -> Self {
        self.provenance = p;
        self
    }
    pub fn economics(mut self, e: EconomicContext) -> Self {
        self.economics = Some(e);
        self
    }
    pub fn commitment(mut self, c: Commitment) -> Self {
        self.commitment = Some(c);
        self
    }
    pub fn processor(mut self, p: OrgId) -> Self {
        self.processor = Some(p);
        self
    }
}

/// An agent embedded in an instrumented system.
pub struct ControllerAgent {
    controller: OrgId,
    agent: AgentRef,
    jurisdiction: Jurisdiction,
    key: SecretKey,
    authorization: Option<AuthorizationRef>,
    stream: ContentId,
    next_index: u64,
}

impl ControllerAgent {
    pub fn new(
        controller: OrgId,
        agent: AgentRef,
        jurisdiction: Jurisdiction,
        key: SecretKey,
    ) -> ControllerAgent {
        // The stream identifier is derived from the controller, the agent
        // build and the key, so two agent instances never share a sequence
        // space and a restart with the same key resumes the same stream.
        let stream = ContentId::of_bytes(
            "duap.stream.v1",
            format!(
                "{}|{}|{}|{}",
                controller.as_str(),
                agent.name,
                agent.version,
                key.key_id()
            )
            .as_bytes(),
        );
        ControllerAgent {
            controller,
            agent,
            jurisdiction,
            key,
            authorization: None,
            stream,
            next_index: 0,
        }
    }

    /// Resume a sequence after a restart. The caller must persist the last
    /// index; resuming at zero creates conflicts the clearing node will
    /// reject, which is the correct failure -- a silently restarted sequence
    /// would hide dropped events.
    pub fn resume_at(mut self, index: u64) -> Self {
        self.next_index = index;
        self
    }

    pub fn public_key(&self) -> duap_crypto::PublicKey {
        self.key.public_key()
    }

    pub fn stream_id(&self) -> ContentId {
        self.stream
    }

    pub fn next_index(&self) -> u64 {
        self.next_index
    }

    /// Attach the authorization this agent is operating under.
    pub fn use_authorization(&mut self, a: AuthorizationRef) {
        self.authorization = Some(a);
    }

    /// Record one usage: build, validate, sequence and sign an event.
    pub fn record_usage(
        &mut self,
        record: UsageRecord,
        recorded_at: Timestamp,
    ) -> Result<(DataUsageEvent, Digest, Envelope), SdkError> {
        let auth = self
            .authorization
            .clone()
            .ok_or_else(|| SdkError::NoAuthorization(self.controller.to_string()))?;

        let mut b = EventBuilder::new(
            EventId::for_sequence(&self.stream, self.next_index),
            self.agent.clone(),
            self.controller.clone(),
            record.subject,
            self.jurisdiction.clone(),
            record.data_class,
            record.operation,
            record.purpose,
            auth,
            record.amount,
            record.occurred_at,
        )
        .recorded_at(recorded_at.max(record.occurred_at))
        .sequence(EventSequence {
            stream: self.stream,
            index: self.next_index,
        })
        .provenance(record.provenance);

        if let Some(c) = record.collection {
            b = b.collection(c);
        }
        if let Some(r) = record.retention {
            b = b.retention(r);
        }
        if let Some(e) = record.economics {
            b = b.economics(e);
        }
        if let Some(c) = record.commitment {
            b = b.commitment(c);
        }
        if let Some(p) = record.processor {
            b = b.processor(p);
        }
        if let Some(s) = record.sensitivity {
            b = b.sensitivity(s);
        }

        let event = b.build()?;
        let digest = event.digest()?;
        let mut env = Envelope::seal(duap_model::event::EVENT_DOMAIN, &event)?;
        env.sign(&self.key, recorded_at.0, None)?;
        self.next_index += 1;
        Ok((event, digest, env))
    }
}
