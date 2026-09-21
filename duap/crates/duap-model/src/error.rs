//! Errors for the DUAP object model.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ModelError>;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ModelError {
    #[error("{0}")]
    Taxonomy(#[from] TaxonomyError),

    #[error("malformed identifier: {0}")]
    BadId(String),

    #[error("invalid {field}: {reason}")]
    Invalid {
        field: &'static str,
        reason: String,
    },

    #[error("currency mismatch: {a} and {b}")]
    CurrencyMismatch { a: String, b: String },

    #[error("arithmetic overflow in {0}")]
    Overflow(&'static str),

    #[error("division by zero in {0}")]
    DivZero(&'static str),

    #[error("event failed validation: {0}")]
    InvalidEvent(String),

    #[error("canonical encoding error: {0}")]
    Canon(#[from] duap_canon::CanonError),

    #[error("cryptographic error: {0}")]
    Crypto(#[from] duap_crypto::CryptoError),

    #[error("entropy source failed: {0}")]
    Entropy(String),
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TaxonomyError {
    #[error("unknown {kind} code {code:?}")]
    UnknownCode { kind: &'static str, code: String },
}
