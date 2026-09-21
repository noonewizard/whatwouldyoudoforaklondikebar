//! Errors for key handling, signing, and verification.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, CryptoError>;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CryptoError {
    #[error("unknown signature suite {0:?}")]
    UnknownSuite(String),

    #[error("key material has wrong length for {suite}: expected {expected}, got {actual}")]
    KeyLength {
        suite: &'static str,
        expected: usize,
        actual: usize,
    },

    #[error("signature has wrong length for {suite}: expected {expected}, got {actual}")]
    SignatureLength {
        suite: &'static str,
        expected: usize,
        actual: usize,
    },

    #[error("malformed key material: {0}")]
    MalformedKey(String),

    #[error("signature verification failed: {0}")]
    BadSignature(&'static str),

    #[error("signing failed: {0}")]
    SigningFailed(&'static str),

    #[error("key {kid} is not known to the registry")]
    UnknownKey { kid: String },

    #[error("key {kid} was not valid at {at}: {reason}")]
    KeyNotValid {
        kid: String,
        at: u64,
        reason: &'static str,
    },

    #[error("key identifier does not match the public key it names")]
    KeyIdMismatch,

    #[error("suite {suite} is not accepted by policy")]
    SuiteRejected { suite: &'static str },

    #[error("envelope payload digest does not match the signed digest")]
    PayloadMismatch,

    #[error("envelope carries no signature satisfying the policy")]
    NoAcceptableSignature,

    #[error(
        "signature by {kid} was created at {created}, outside the accepted skew window [{lo}, {hi}]"
    )]
    ClockSkew {
        kid: String,
        created: u64,
        lo: u64,
        hi: u64,
    },

    #[error("canonical encoding error: {0}")]
    Canon(#[from] duap_canon::CanonError),

    #[error("entropy source failed: {0}")]
    Entropy(String),
}
