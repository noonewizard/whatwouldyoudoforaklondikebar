//! Key registry: enrolment, rotation, and revocation.
//!
//! STATUS: PRODUCTION (in-memory reference; persistence lives in
//! `duap-clearing`).
//!
//! # Revocation semantics
//!
//! Revocation in an accounting system cannot be "the key stops working now",
//! because receipts signed months ago must remain verifiable. DUAP therefore
//! separates two times:
//!
//! * `declared_at` -- when the revocation was published;
//! * `effective_from` -- the earliest time from which signatures by this key
//!   are no longer to be trusted.
//!
//! For an orderly retirement (`Superseded`, `Retired`) `effective_from` is the
//! moment of retirement and earlier signatures stay valid. For a suspected
//! compromise the holder sets `effective_from` to the earliest time the key
//! could have been exposed; if that is unknown it must be set to the key's
//! `not_before`, invalidating everything the key ever signed. There is no
//! honest middle option: a verifier cannot distinguish a signature made by the
//! holder at time T from one made by a thief who had the key at time T.
//!
//! This is only sound if `created` cannot be backdated, which a signature
//! alone does not prevent. DUAP closes the gap by requiring receipts to be
//! anchored in the transparency log: the log's signed tree heads bound
//! *existence* from above, and the clearing node refuses to settle a receipt
//! whose signature time is earlier than the anchor allows by more than the
//! configured skew. See `THREAT_MODEL.md` T-12 and `duap-provenance`.

use crate::error::{CryptoError, Result};
use crate::key::{KeyId, PublicKey};
use duap_canon::digest::Digest;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Why a key was revoked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevocationReason {
    /// Replaced by a newer key in the normal rotation schedule.
    Superseded,
    /// Withdrawn from service without suspicion.
    Retired,
    /// Private key believed to be exposed.
    Compromise,
    /// Withdrawn by the registry operator for policy reasons.
    PolicyViolation,
}

impl RevocationReason {
    /// Whether signatures made before `effective_from` remain trustworthy.
    pub const fn preserves_earlier_signatures(self) -> bool {
        match self {
            RevocationReason::Superseded | RevocationReason::Retired => true,
            RevocationReason::Compromise | RevocationReason::PolicyViolation => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Revocation {
    pub reason: RevocationReason,
    /// Microseconds since the Unix epoch.
    pub declared_at: u64,
    /// Microseconds since the Unix epoch. Signatures created at or after this
    /// instant are rejected.
    pub effective_from: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyStatus {
    Active,
    Rotated,
    Revoked,
}

/// The role a key is authorised to play. Roles are checked by callers, not by
/// this crate: a key valid for signing events must not thereby be able to
/// sign clearing-node tree heads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyRole {
    /// Signs data usage events (collection agents).
    EventSigner,
    /// Signs authorizations (data subjects and their agents).
    AuthorizationSigner,
    /// Signs receipts (clearing nodes).
    ReceiptSigner,
    /// Signs transparency-log tree heads.
    LogSigner,
    /// Signs settlement instructions.
    SettlementSigner,
    /// Signs registry updates.
    RegistryAdmin,
    /// Signs audit attestations.
    Auditor,
}

/// One entry in the registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyRecord {
    pub kid: KeyId,
    pub public_key: PublicKey,
    /// Identifier of the holder (organisation, agent, or pseudonymous subject
    /// handle). Opaque to this crate.
    pub holder: String,
    pub roles: Vec<KeyRole>,
    /// Microseconds since the Unix epoch.
    pub not_before: u64,
    /// Microseconds since the Unix epoch, exclusive. `None` means open-ended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_after: Option<u64>,
    pub status: KeyStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revocation: Option<Revocation>,
    /// Key this one replaces, for rotation chains.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<KeyId>,
}

impl KeyRecord {
    pub fn new(
        public_key: PublicKey,
        holder: impl Into<String>,
        roles: Vec<KeyRole>,
        not_before: u64,
        not_after: Option<u64>,
    ) -> KeyRecord {
        KeyRecord {
            kid: public_key.key_id(),
            public_key,
            holder: holder.into(),
            roles,
            not_before,
            not_after,
            status: KeyStatus::Active,
            revocation: None,
            supersedes: None,
        }
    }

    pub fn has_role(&self, role: KeyRole) -> bool {
        self.roles.contains(&role)
    }
}

/// Verification-time parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerificationContext {
    /// The verifier's current time, microseconds since the Unix epoch.
    pub now: u64,
    /// How far into the future a `created` timestamp may be before it is
    /// rejected as clock skew or backdating, in microseconds.
    pub max_future_skew: u64,
    /// How far in the past a `created` timestamp may be, in microseconds.
    /// `u64::MAX` disables the check (correct for archival verification).
    pub max_age: u64,
}

impl VerificationContext {
    /// Live ingestion: five minutes of future skew, one day of age.
    pub fn live(now: u64) -> Self {
        VerificationContext {
            now,
            max_future_skew: 5 * 60 * 1_000_000,
            max_age: 24 * 3600 * 1_000_000,
        }
    }

    /// Archival re-verification: no age limit.
    pub fn archival(now: u64) -> Self {
        VerificationContext {
            now,
            max_future_skew: 5 * 60 * 1_000_000,
            max_age: u64::MAX,
        }
    }
}

/// An in-memory key registry.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyRegistry {
    keys: BTreeMap<String, KeyRecord>,
}

impl KeyRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enrol a key. Fails if the record's `kid` does not match its public key
    /// (the self-certifying property) or if the identifier is already taken by
    /// a different key.
    pub fn enroll(&mut self, record: KeyRecord) -> Result<()> {
        if record.public_key.key_id() != record.kid {
            return Err(CryptoError::KeyIdMismatch);
        }
        let k = record.kid.to_string();
        if let Some(existing) = self.keys.get(&k) {
            if existing.public_key != record.public_key {
                return Err(CryptoError::KeyIdMismatch);
            }
        }
        self.keys.insert(k, record);
        Ok(())
    }

    pub fn get(&self, kid: &KeyId) -> Option<&KeyRecord> {
        self.keys.get(&kid.to_string())
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &KeyRecord> {
        self.keys.values()
    }

    pub fn by_holder<'a>(&'a self, holder: &'a str) -> impl Iterator<Item = &'a KeyRecord> + 'a {
        self.keys.values().filter(move |r| r.holder == holder)
    }

    /// Mark a key rotated, recording the successor.
    pub fn rotate(&mut self, old: &KeyId, new: &KeyId, at: u64) -> Result<()> {
        let old_key = self.keys.get(&old.to_string()).cloned().ok_or_else(|| {
            CryptoError::UnknownKey {
                kid: old.to_string(),
            }
        })?;
        if !self.keys.contains_key(&new.to_string()) {
            return Err(CryptoError::UnknownKey {
                kid: new.to_string(),
            });
        }
        let mut updated = old_key;
        updated.status = KeyStatus::Rotated;
        updated.not_after = Some(at);
        updated.revocation = Some(Revocation {
            reason: RevocationReason::Superseded,
            declared_at: at,
            effective_from: at,
            note: None,
        });
        self.keys.insert(old.to_string(), updated);
        if let Some(n) = self.keys.get_mut(&new.to_string()) {
            n.supersedes = Some(*old);
        }
        Ok(())
    }

    /// Revoke a key.
    pub fn revoke(&mut self, kid: &KeyId, revocation: Revocation) -> Result<()> {
        let rec = self
            .keys
            .get_mut(&kid.to_string())
            .ok_or_else(|| CryptoError::UnknownKey {
                kid: kid.to_string(),
            })?;
        rec.status = KeyStatus::Revoked;
        rec.revocation = Some(revocation);
        Ok(())
    }

    /// Check that `kid` may be relied on for a signature created at `created`.
    pub fn check_usable(
        &self,
        kid: &KeyId,
        created: u64,
        ctx: &VerificationContext,
    ) -> Result<&KeyRecord> {
        let rec = self.get(kid).ok_or_else(|| CryptoError::UnknownKey {
            kid: kid.to_string(),
        })?;
        if created > ctx.now.saturating_add(ctx.max_future_skew) {
            return Err(CryptoError::ClockSkew {
                kid: kid.to_string(),
                created,
                lo: ctx.now.saturating_sub(ctx.max_age),
                hi: ctx.now.saturating_add(ctx.max_future_skew),
            });
        }
        if ctx.max_age != u64::MAX && created < ctx.now.saturating_sub(ctx.max_age) {
            return Err(CryptoError::ClockSkew {
                kid: kid.to_string(),
                created,
                lo: ctx.now.saturating_sub(ctx.max_age),
                hi: ctx.now.saturating_add(ctx.max_future_skew),
            });
        }
        if created < rec.not_before {
            return Err(CryptoError::KeyNotValid {
                kid: kid.to_string(),
                at: created,
                reason: "signature predates the key's validity period",
            });
        }
        if let Some(na) = rec.not_after {
            if created >= na {
                return Err(CryptoError::KeyNotValid {
                    kid: kid.to_string(),
                    at: created,
                    reason: "signature postdates the key's validity period",
                });
            }
        }
        if let Some(rev) = &rec.revocation {
            if created >= rev.effective_from {
                return Err(CryptoError::KeyNotValid {
                    kid: kid.to_string(),
                    at: created,
                    reason: "signature is at or after the revocation's effective time",
                });
            }
            if !rev.reason.preserves_earlier_signatures() && rev.effective_from <= rec.not_before {
                return Err(CryptoError::KeyNotValid {
                    kid: kid.to_string(),
                    at: created,
                    reason: "key is fully revoked (compromise time unknown)",
                });
            }
        }
        Ok(rec)
    }

    /// Digest of the whole registry state, for anchoring in the transparency
    /// log so that relying parties can detect a registry that shows different
    /// contents to different verifiers.
    pub fn state_digest(&self) -> Result<Digest> {
        Ok(Digest::of_object("duap.key-registry.v1", self)?)
    }
}
