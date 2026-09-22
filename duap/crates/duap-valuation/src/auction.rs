//! Sealed-bid auctions with commit-reveal.
//!
//! STATUS: PRODUCTION (mechanism); whether an auction is the right way to
//! price any particular data is a question the protocol does not answer.
//!
//! # Mechanism
//!
//! Second-price sealed-bid (Vickrey) with a reserve, run in two phases:
//!
//! 1. **Commit.** Each bidder submits `H(salt || amount || bidder)`. The
//!    commitments are published to the transparency log.
//! 2. **Reveal.** Each bidder opens its commitment. Unopened commitments are
//!    discarded; opened ones that do not match are rejected.
//!
//! The winner is the highest revealed bid at or above the reserve, and pays
//! the greater of the second-highest bid and the reserve.
//!
//! # Why commit-reveal
//!
//! Without it, the auctioneer sees every bid before the close and can insert
//! a bid just below the highest ("last look"), turning a second-price auction
//! into a first-price one and extracting the whole surplus. With it, the
//! auctioneer is committed before it learns anything, and because the
//! commitments are logged, a bidder can prove after the fact which bids
//! existed at close.
//!
//! # The two phases must not overlap
//!
//! A reveal is bounded *below* by `commit_closes` as well as above by
//! `reveal_closes`. Both bounds are load-bearing: a bid opened while
//! others can still commit is readable by them, and they can commit to one
//! unit more. That makes the auction open-ascending for every bidder who
//! has not yet committed, which is exactly what the sealing is for.
//!
//! Only the upper bound was present until finding AUC-01, which
//! `tests/auction_redteam.rs` demonstrates.
//!
//! # What the mechanism does *not* fix
//!
//! * **Shill bidding.** A colluding auctioneer and a sham bidder can still
//!   raise the second price, because a commitment says nothing about whether
//!   the bidder is real. The defence is identity: bidders are enrolled
//!   organisations with a stake, and the log makes a pattern of losing shill
//!   bids visible to auditors. This is deterrence, not prevention.
//!   `THREAT_MODEL.md` T-23.
//! * **Bidder collusion, and specifically withheld reveals.** A ring that
//!   agrees to suppress bids lowers the clearing price, and no
//!   single-round mechanism prevents it. What matters here is that this
//!   implementation makes it *costless*: the second price is computed over
//!   revealed bids only, reveals are published as they arrive so a bidder
//!   can decide after seeing others, and there is no deposit to forfeit.
//!   `tests/auction_redteam.rs` demonstrates one withheld reveal cutting a
//!   clearing price from 90 to 50. The only defence offered is that the
//!   withholding is counted in `unrevealed_commitments` and therefore
//!   visible -- and a withheld reveal is indistinguishable from a crashed
//!   bidder, so any enforcement must be an economic forfeit that treats
//!   both alike rather than a judgement about intent. Accepted as
//!   RISK-03 in `security/findings.md`.
//! * **Truthful bidding.** Vickrey's dominant-strategy result assumes
//!   independent private values and no budget constraints. Data lots are
//!   neither independent nor free of budget effects, so "bidders will bid
//!   their true value" is not a claim DUAP makes.

use duap_canon::digest::{Digest, HashAlg};
use duap_model::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const BID_DOMAIN: &str = "duap.bid-commitment.v1";

/// What is being auctioned.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lot {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "dc")]
    pub data_class: DataClass,
    #[serde(rename = "op")]
    pub operation: Operation,
    #[serde(rename = "pp")]
    pub purpose: Purpose,
    #[serde(rename = "u")]
    pub unit: Unit,
    /// Units offered.
    #[serde(rename = "q")]
    pub quantity: u64,
    #[serde(rename = "rs", default, skip_serializing_if = "Option::is_none")]
    pub reserve_unit_price: Option<Precise>,
    #[serde(rename = "cl")]
    pub commit_closes: Timestamp,
    #[serde(rename = "rv")]
    pub reveal_closes: Timestamp,
}

/// A published commitment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BidCommitment {
    #[serde(rename = "b")]
    pub bidder: OrgId,
    #[serde(rename = "c")]
    pub commitment: Digest,
    #[serde(rename = "t")]
    pub submitted_at: Timestamp,
}

/// An opened bid.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BidReveal {
    #[serde(rename = "b")]
    pub bidder: OrgId,
    #[serde(rename = "p")]
    pub unit_price: Precise,
    #[serde(rename = "s", with = "salt_hex")]
    pub salt: [u8; 32],
    #[serde(rename = "t")]
    pub revealed_at: Timestamp,
}

mod salt_hex {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &[u8; 32], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_bytes(v)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 32], D::Error> {
        let v = <serde_bytes_compat::Bytes as Deserialize>::deserialize(d)?;
        v.0.try_into()
            .map_err(|_| serde::de::Error::custom("salt must be 32 bytes"))
    }

    mod serde_bytes_compat {
        use serde::de::{Error, SeqAccess, Visitor};
        use serde::{Deserialize, Deserializer};
        use std::fmt;

        pub struct Bytes(pub Vec<u8>);

        impl<'de> Deserialize<'de> for Bytes {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                struct V;
                impl<'de> Visitor<'de> for V {
                    type Value = Bytes;
                    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.write_str("a byte string")
                    }
                    fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Bytes, E> {
                        Ok(Bytes(v.to_vec()))
                    }
                    fn visit_byte_buf<E: Error>(self, v: Vec<u8>) -> Result<Bytes, E> {
                        Ok(Bytes(v))
                    }
                    fn visit_seq<A: SeqAccess<'de>>(self, mut s: A) -> Result<Bytes, A::Error> {
                        let mut o = Vec::new();
                        while let Some(b) = s.next_element::<u8>()? {
                            o.push(b);
                        }
                        Ok(Bytes(o))
                    }
                }
                d.deserialize_bytes(V)
            }
        }
    }
}

/// Compute the commitment for a bid.
pub fn bid_commitment(bidder: &OrgId, unit_price: Precise, salt: &[u8; 32]) -> Digest {
    let v = duap_canon::Value::map([
        ("b", duap_canon::Value::text(bidder.as_str())),
        ("p", duap_canon::Value::text(unit_price.nmu.to_string())),
        ("c", duap_canon::Value::text(unit_price.currency.as_str())),
        ("s", duap_canon::Value::Bytes(salt.to_vec())),
    ]);
    Digest::of(HashAlg::Sha2_256, BID_DOMAIN, &duap_canon::encode(&v))
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum AuctionError {
    #[error("the commit phase for lot {0} has closed")]
    CommitClosed(String),
    #[error("the reveal phase for lot {0} has closed")]
    RevealClosed(String),
    #[error("the commit phase for lot {0} has not closed; revealing now would break the seal")]
    RevealTooEarly(String),
    #[error("lot {0} has a reveal deadline at or before its commit deadline")]
    InvalidWindow(String),
    #[error("bidder {0} has no commitment in this auction")]
    NoCommitment(String),
    #[error("the reveal from {0} does not open its commitment")]
    RevealMismatch(String),
    #[error("bidder {0} has already committed")]
    DuplicateCommitment(String),
    #[error("currency {got} does not match the lot's {want}")]
    CurrencyMismatch { want: String, got: String },
    #[error("{0}")]
    Model(#[from] ModelError),
}

/// The outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuctionResult {
    pub lot: String,
    /// `None` if no bid met the reserve.
    pub winner: Option<OrgId>,
    /// The price the winner pays per unit.
    pub clearing_unit_price: Option<Precise>,
    /// The winning bid, for audit. Never charged; recorded so that the gap
    /// between bid and price is visible.
    pub winning_bid: Option<Precise>,
    pub revealed_bids: u64,
    pub unrevealed_commitments: u64,
}

/// A running auction.
#[derive(Debug, Clone)]
pub struct SealedBidAuction {
    pub lot: Lot,
    pub currency: Currency,
    commitments: BTreeMap<String, BidCommitment>,
    reveals: BTreeMap<String, BidReveal>,
}

impl SealedBidAuction {
    pub fn new(lot: Lot, currency: Currency) -> Self {
        SealedBidAuction {
            lot,
            currency,
            commitments: BTreeMap::new(),
            reveals: BTreeMap::new(),
        }
    }

    /// Both phases must be well ordered before either accepts anything.
    ///
    /// A lot whose reveal deadline is at or before its commit deadline has
    /// no valid reveal window, and every reveal against it would be
    /// simultaneously too early and too late. Rejecting the lot outright
    /// is clearer than rejecting every bid with a confusing reason.
    fn check_window(&self) -> Result<(), AuctionError> {
        if self.lot.reveal_closes <= self.lot.commit_closes {
            return Err(AuctionError::InvalidWindow(self.lot.id.clone()));
        }
        Ok(())
    }

    pub fn commit(&mut self, c: BidCommitment) -> Result<(), AuctionError> {
        self.check_window()?;
        if c.submitted_at > self.lot.commit_closes {
            return Err(AuctionError::CommitClosed(self.lot.id.clone()));
        }
        let k = c.bidder.to_string();
        if self.commitments.contains_key(&k) {
            return Err(AuctionError::DuplicateCommitment(k));
        }
        self.commitments.insert(k, c);
        Ok(())
    }

    pub fn reveal(&mut self, r: BidReveal) -> Result<(), AuctionError> {
        self.check_window()?;
        if r.revealed_at > self.lot.reveal_closes {
            return Err(AuctionError::RevealClosed(self.lot.id.clone()));
        }
        // A reveal inside the commit window breaks the seal for everyone
        // who has not yet committed: they can read the opened bid and
        // commit to one unit more, which turns a sealed-bid auction into
        // an open ascending one. Bounding the reveal below is as necessary
        // as bounding it above, and only the upper bound was there.
        // Finding AUC-01.
        if r.revealed_at <= self.lot.commit_closes {
            return Err(AuctionError::RevealTooEarly(self.lot.id.clone()));
        }
        if r.unit_price.currency != self.currency {
            return Err(AuctionError::CurrencyMismatch {
                want: self.currency.to_string(),
                got: r.unit_price.currency.to_string(),
            });
        }
        let k = r.bidder.to_string();
        let c = self
            .commitments
            .get(&k)
            .ok_or_else(|| AuctionError::NoCommitment(k.clone()))?;
        if bid_commitment(&r.bidder, r.unit_price, &r.salt) != c.commitment {
            return Err(AuctionError::RevealMismatch(k));
        }
        self.reveals.insert(k, r);
        Ok(())
    }

    pub fn commitment_count(&self) -> usize {
        self.commitments.len()
    }

    /// Settle the auction.
    ///
    /// Ties at the top are broken by the bidder identifier's lexicographic
    /// order. That is arbitrary but deterministic and publicly checkable,
    /// which is what matters; a random tie-break would let the auctioneer
    /// choose.
    pub fn settle(&self) -> AuctionResult {
        let reserve = self
            .lot
            .reserve_unit_price
            .unwrap_or_else(|| Precise::zero(self.currency));

        let mut bids: Vec<(&String, &BidReveal)> = self
            .reveals
            .iter()
            .filter(|(_, r)| r.unit_price.nmu >= reserve.nmu)
            .collect();
        bids.sort_by(|a, b| {
            b.1.unit_price
                .nmu
                .cmp(&a.1.unit_price.nmu)
                .then_with(|| a.0.cmp(b.0))
        });

        let unrevealed = (self.commitments.len() - self.reveals.len()) as u64;

        match bids.first() {
            None => AuctionResult {
                lot: self.lot.id.clone(),
                winner: None,
                clearing_unit_price: None,
                winning_bid: None,
                revealed_bids: self.reveals.len() as u64,
                unrevealed_commitments: unrevealed,
            },
            Some((_, top)) => {
                let second = bids
                    .get(1)
                    .map(|(_, r)| r.unit_price.nmu)
                    .unwrap_or(reserve.nmu);
                let clearing = Precise::new(self.currency, second.max(reserve.nmu));
                AuctionResult {
                    lot: self.lot.id.clone(),
                    winner: Some(top.bidder.clone()),
                    clearing_unit_price: Some(clearing),
                    winning_bid: Some(top.unit_price),
                    revealed_bids: self.reveals.len() as u64,
                    unrevealed_commitments: unrevealed,
                }
            }
        }
    }
}
