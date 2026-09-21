//! Machine-to-machine authorization negotiation.
//!
//! STATUS: REFERENCE. The state machine and message types are implemented,
//! model-checked in `formal/Negotiation.tla` and mirrored by
//! `tests/negotiation_model_mirror.rs`. Transport bindings live in
//! `duap-sdk` and the gateway. No step of the vertical slice negotiates
//! anything, so this module is exercised by its own tests and not
//! end to end -- which is why it is REFERENCE and not higher.
//!
//! Two parties that have never spoken need to agree on an authorization
//! without a human in the loop. The exchange is deliberately small:
//!
//! ```text
//!   requester                                responder (subject's agent)
//!      |-- Request  (what, why, how long, offered price) ---->|
//!      |<------------------------- Offer (terms, price) ------|
//!      |-- Accept (signs the grant) ------------------------->|
//!      |<------------------------- Granted (signed grant) ----|
//! ```
//!
//! Either side may `Reject` or `Withdraw` at any point before `Granted`, and a
//! responder may answer a `Request` with a counter-`Offer` that narrows the
//! matcher or raises the price. Every message is carried in a signed envelope,
//! and the accepted `Offer`'s digest is recorded in the resulting grant's
//! extensions, so the negotiation that produced an authorization is auditable.
//!
//! The state machine below rejects out-of-order and replayed messages. It is
//! the same structure as `formal/Negotiation.tla`, whose six invariants are
//! checked exhaustively within bounds and mirrored as Rust tests of the same
//! names. The model does not cover signatures, message content, transport or
//! concurrent negotiations; `formal/README.md` states the bounds.

use crate::grant::{Grant, Term};
use crate::matcher::Matcher;
use duap_canon::digest::Digest;
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};

pub const NEGOTIATION_DOMAIN: &str = "duap.negotiation.v1";

/// Opaque negotiation identifier.
pub type NegotiationId = duap_model::ids::BatchId;

/// A request for authorization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessRequest {
    #[serde(rename = "id")]
    pub id: NegotiationId,
    #[serde(rename = "rq")]
    pub requester: OrgId,
    /// The subject whose authorization is sought, under the pseudonym the
    /// requester already holds. Absent for a request addressed to a
    /// population rather than an individual.
    #[serde(rename = "sb", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<SubjectRef>,
    /// What the requester wants to be allowed to do.
    #[serde(rename = "wt")]
    pub wants: Matcher,
    /// Plain-language purpose disclosure, shown to a human if one is asked.
    #[serde(rename = "ds")]
    pub disclosure: String,
    /// Requested duration in hours.
    #[serde(rename = "du")]
    pub duration_hours: u32,
    /// Price the requester proposes to pay.
    #[serde(rename = "pp", default, skip_serializing_if = "Option::is_none")]
    pub proposed_pricing: Option<PricingRule>,
    #[serde(rename = "cu")]
    pub currency: Currency,
    #[serde(rename = "at")]
    pub sent_at: Timestamp,
    #[serde(rename = "ex")]
    pub expires_at: Timestamp,
}

/// A response proposing concrete terms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessOffer {
    #[serde(rename = "id")]
    pub id: NegotiationId,
    /// Digest of the request this answers.
    #[serde(rename = "rd")]
    pub request_digest: Digest,
    #[serde(rename = "tm")]
    pub terms: Vec<Term>,
    #[serde(rename = "du")]
    pub duration_hours: u32,
    #[serde(rename = "cu")]
    pub currency: Currency,
    #[serde(rename = "at")]
    pub sent_at: Timestamp,
    #[serde(rename = "ex")]
    pub expires_at: Timestamp,
    /// Whether this narrows or re-prices the request rather than matching it.
    #[serde(rename = "ct", default, skip_serializing_if = "is_false")]
    pub counter: bool,
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// Acceptance of an offer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Acceptance {
    #[serde(rename = "id")]
    pub id: NegotiationId,
    #[serde(rename = "od")]
    pub offer_digest: Digest,
    #[serde(rename = "at")]
    pub sent_at: Timestamp,
}

/// Refusal, with a machine-readable code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rejection {
    #[serde(rename = "id")]
    pub id: NegotiationId,
    #[serde(rename = "cd")]
    pub code: RejectCode,
    #[serde(rename = "at")]
    pub sent_at: Timestamp,
    #[serde(rename = "ms", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RejectCode {
    /// The subject declines this purpose outright.
    PurposeRefused,
    /// The price is unacceptable.
    PriceRefused,
    /// The requested duration is too long.
    DurationRefused,
    /// The data class is not on offer at any price.
    ClassRefused,
    /// The requester is not acceptable as a counterparty.
    CounterpartyRefused,
    /// The request expired before it was answered.
    Expired,
    /// Refused without stating a reason.
    Unspecified,
}

/// Any message in the exchange.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "t", rename_all = "snake_case")]
pub enum NegotiationMessage {
    Request(AccessRequest),
    Offer(AccessOffer),
    Accept(Acceptance),
    Reject(Rejection),
    Withdraw(Rejection),
}

impl NegotiationMessage {
    pub fn id(&self) -> NegotiationId {
        match self {
            NegotiationMessage::Request(m) => m.id,
            NegotiationMessage::Offer(m) => m.id,
            NegotiationMessage::Accept(m) => m.id,
            NegotiationMessage::Reject(m) | NegotiationMessage::Withdraw(m) => m.id,
        }
    }

    pub fn digest(&self) -> Result<Digest, ModelError> {
        Ok(Digest::of_object(NEGOTIATION_DOMAIN, self)?)
    }

    pub fn sent_at(&self) -> Timestamp {
        match self {
            NegotiationMessage::Request(m) => m.sent_at,
            NegotiationMessage::Offer(m) => m.sent_at,
            NegotiationMessage::Accept(m) => m.sent_at,
            NegotiationMessage::Reject(m) | NegotiationMessage::Withdraw(m) => m.sent_at,
        }
    }
}

/// Negotiation states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NegotiationState {
    /// Nothing sent yet.
    Idle,
    /// A request is outstanding.
    Requested,
    /// An offer is on the table.
    Offered,
    /// The offer was accepted; the grant is being issued.
    Accepted,
    /// Terminal: a grant exists.
    Granted,
    /// Terminal: refused or withdrawn.
    Closed,
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum NegotiationError {
    #[error("message {got:?} is not valid in state {state:?}")]
    OutOfOrder {
        state: NegotiationState,
        got: &'static str,
    },
    #[error("message belongs to negotiation {got}, not {want}")]
    WrongNegotiation { want: String, got: String },
    #[error("message replays a digest already seen in this negotiation")]
    Replay,
    #[error("message at {at} arrived after the exchange expired at {expiry}")]
    Expired { at: Timestamp, expiry: Timestamp },
    #[error("acceptance names offer digest {got}, but the outstanding offer is {want}")]
    OfferMismatch { want: String, got: String },
    #[error("offer answers request digest {got}, but the outstanding request is {want}")]
    RequestMismatch { want: String, got: String },
    #[error("{0}")]
    Model(#[from] ModelError),
}

/// A negotiation in progress.
#[derive(Debug, Clone)]
pub struct Negotiation {
    pub id: NegotiationId,
    pub state: NegotiationState,
    pub request: Option<AccessRequest>,
    pub request_digest: Option<Digest>,
    pub offer: Option<AccessOffer>,
    pub offer_digest: Option<Digest>,
    pub grant: Option<Grant>,
    seen: Vec<Digest>,
    expiry: Option<Timestamp>,
}

impl Negotiation {
    pub fn new(id: NegotiationId) -> Negotiation {
        Negotiation {
            id,
            state: NegotiationState::Idle,
            request: None,
            request_digest: None,
            offer: None,
            offer_digest: None,
            grant: None,
            seen: Vec::new(),
            expiry: None,
        }
    }

    /// Apply a message, advancing the state machine.
    pub fn apply(&mut self, msg: &NegotiationMessage) -> Result<NegotiationState, NegotiationError> {
        if msg.id() != self.id {
            return Err(NegotiationError::WrongNegotiation {
                want: self.id.to_string(),
                got: msg.id().to_string(),
            });
        }
        let d = msg.digest()?;
        if self.seen.contains(&d) {
            return Err(NegotiationError::Replay);
        }
        if let Some(x) = self.expiry {
            if msg.sent_at() > x && !matches!(msg, NegotiationMessage::Reject(_)) {
                return Err(NegotiationError::Expired {
                    at: msg.sent_at(),
                    expiry: x,
                });
            }
        }

        use NegotiationState as S;
        let next = match (self.state, msg) {
            (S::Idle, NegotiationMessage::Request(r)) => {
                self.request = Some(r.clone());
                self.request_digest = Some(d);
                self.expiry = Some(r.expires_at);
                S::Requested
            }
            (S::Requested | S::Offered, NegotiationMessage::Offer(o)) => {
                let want = self.request_digest.expect("a request precedes an offer");
                if o.request_digest != want {
                    return Err(NegotiationError::RequestMismatch {
                        want: want.to_string(),
                        got: o.request_digest.to_string(),
                    });
                }
                self.offer = Some(o.clone());
                self.offer_digest = Some(d);
                self.expiry = Some(o.expires_at);
                S::Offered
            }
            (S::Offered, NegotiationMessage::Accept(a)) => {
                let want = self.offer_digest.expect("an offer precedes acceptance");
                if a.offer_digest != want {
                    return Err(NegotiationError::OfferMismatch {
                        want: want.to_string(),
                        got: a.offer_digest.to_string(),
                    });
                }
                S::Accepted
            }
            (S::Requested | S::Offered | S::Accepted, NegotiationMessage::Reject(_))
            | (S::Requested | S::Offered, NegotiationMessage::Withdraw(_)) => S::Closed,
            (state, other) => {
                return Err(NegotiationError::OutOfOrder {
                    state,
                    got: match other {
                        NegotiationMessage::Request(_) => "Request",
                        NegotiationMessage::Offer(_) => "Offer",
                        NegotiationMessage::Accept(_) => "Accept",
                        NegotiationMessage::Reject(_) => "Reject",
                        NegotiationMessage::Withdraw(_) => "Withdraw",
                    },
                });
            }
        };
        self.seen.push(d);
        self.state = next;
        Ok(next)
    }

    /// Record the grant that settles an accepted negotiation.
    ///
    /// The grant must carry the accepted offer's digest in its extensions
    /// under `duap.negotiation` -- except that the reserved `duap.` namespace
    /// is closed to extensions, so the binding uses `negotiation.offer`.
    pub fn settle(&mut self, grant: Grant) -> Result<(), NegotiationError> {
        if self.state != NegotiationState::Accepted {
            return Err(NegotiationError::OutOfOrder {
                state: self.state,
                got: "Granted",
            });
        }
        let want = self.offer_digest.expect("accepted implies an offer");
        let got = grant
            .extensions
            .get("negotiation.offer")
            .and_then(|v| v.as_text().map(|s| s.to_owned()));
        match got {
            Some(s) if s == want.to_string() => {}
            other => {
                return Err(NegotiationError::OfferMismatch {
                    want: want.to_string(),
                    got: other.unwrap_or_else(|| "<absent>".into()),
                });
            }
        }
        self.grant = Some(grant);
        self.state = NegotiationState::Granted;
        Ok(())
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self.state, NegotiationState::Granted | NegotiationState::Closed)
    }
}
