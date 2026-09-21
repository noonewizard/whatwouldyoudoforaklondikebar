//! The data subject's side of the protocol.
//!
//! STATUS: PRODUCTION.
//!
//! A subject agent holds the subject's root secret, derives a distinct
//! pseudonym and signing key per controller, and issues and withdraws
//! grants. It never sends personal data anywhere: the only things it
//! produces are signed authorization documents.

use crate::controller::SdkError;
use duap_auth::{
    Grant, GrantBuilder, RetroactiveRequest, Revocation, RevocationPolicy, RevocationScope, Term,
};
use duap_crypto::{Envelope, SecretKey};
use duap_model::prelude::*;

/// An agent acting for one data subject.
pub struct SubjectAgent {
    root: SubjectRoot,
    /// The key used to sign grants. In the reference agent this is derived
    /// from the root; a hardware-backed deployment supplies its own.
    key: SecretKey,
}

/// The policy parameters of a grant, grouped so they cannot be transposed
/// positionally.
///
/// STATUS: PROTOTYPE, with the crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrantPolicy {
    /// When the grant stops authorizing, regardless of revocation.
    pub expires_at: Timestamp,
    /// How withdrawal takes effect.
    pub revocation: RevocationPolicy,
    /// The currency every price in the grant's terms is denominated in.
    pub currency: Currency,
}

impl SubjectAgent {
    pub fn new(root: SubjectRoot, key: SecretKey) -> SubjectAgent {
        SubjectAgent { root, key }
    }

    /// The pseudonym this subject presents to `controller`.
    pub fn pseudonym_for(&self, controller: &OrgId) -> SubjectRef {
        self.root.pseudonym_for(controller)
    }

    pub fn public_key(&self) -> duap_crypto::PublicKey {
        self.key.public_key()
    }

    /// Issue a grant to `controller`, signed.
    ///
    /// Returns both the grant and its sealed envelope: the grant is what
    /// gets evaluated, the envelope is what gets transmitted and logged.
    pub fn authorize(
        &mut self,
        controller: &OrgId,
        id: GrantId,
        terms: Vec<Term>,
        at: Timestamp,
        currency: Currency,
    ) -> Result<(Grant, Envelope), SdkError> {
        let grant = GrantBuilder::new(
            id,
            self.root.pseudonym_for(controller),
            self.key.key_id(),
            controller.clone(),
            at,
            currency,
        )
        .terms(terms)
        .build()?;
        let mut env = Envelope::seal(duap_auth::GRANT_DOMAIN, &grant)?;
        env.sign(&self.key, at.0, None)?;
        Ok((grant, env))
    }

    /// Issue a grant with an explicit expiry and revocation policy.
    ///
    /// The policy parameters travel together in [`GrantPolicy`] rather than
    /// as a widening positional list: an expiry and a revocation policy
    /// are both `Timestamp`-adjacent and easy to transpose at a call site,
    /// and a transposed grant is one that expires when it should have been
    /// revocable.
    pub fn authorize_with(
        &mut self,
        controller: &OrgId,
        id: GrantId,
        terms: Vec<Term>,
        at: Timestamp,
        policy: GrantPolicy,
    ) -> Result<(Grant, Envelope), SdkError> {
        let grant = GrantBuilder::new(
            id,
            self.root.pseudonym_for(controller),
            self.key.key_id(),
            controller.clone(),
            at,
            policy.currency,
        )
        .terms(terms)
        .expires_at(policy.expires_at)
        .revocation(policy.revocation)
        .build()?;
        let mut env = Envelope::seal(duap_auth::GRANT_DOMAIN, &grant)?;
        env.sign(&self.key, at.0, None)?;
        Ok((grant, env))
    }

    /// Withdraw an authorization, wholly or in part.
    ///
    /// The returned revocation takes effect at the time the grant's own
    /// notice policy implies, which may be later than `at`. That is a
    /// property of the grant the subject agreed to, and the agent surfaces
    /// it rather than pretending withdrawal is always instant.
    pub fn revoke_authorization(
        &mut self,
        grant: &Grant,
        scope: RevocationScope,
        retroactive: RetroactiveRequest,
        at: Timestamp,
    ) -> Result<(Revocation, Envelope), SdkError> {
        let rev = Revocation::for_grant(grant, scope, at, retroactive)?;
        rev.validate_against(grant)?;
        let mut env = Envelope::seal(duap_auth::REVOCATION_DOMAIN, &rev)?;
        env.sign(&self.key, at.0, None)?;
        Ok((rev, env))
    }
}
