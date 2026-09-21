//! Error type for canonical encoding, decoding, and digesting.
//!
//! STATUS: REFERENCE. Every variant names what was rejected and why, so a
//! strict-decode failure tells an implementer which rule their encoder
//! broke rather than that something was wrong.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, CanonError>;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CanonError {
    #[error("unexpected end of input at byte {at}, needed {want} more byte(s)")]
    UnexpectedEof { at: usize, want: usize },

    #[error("malformed CBOR at byte {at}: {reason}")]
    Malformed { at: usize, reason: &'static str },

    #[error("non-canonical CBOR at byte {at}: {reason}")]
    NonCanonical { at: usize, reason: &'static str },

    #[error("unsupported CBOR construct at byte {at}: {reason}")]
    Unsupported { at: usize, reason: &'static str },

    #[error("duplicate map key {key:?}")]
    DuplicateKey { key: String },

    #[error("{consumed} of {total} bytes consumed; trailing data is not allowed")]
    TrailingBytes { consumed: usize, total: usize },

    #[error("input too large: {len} > {max}")]
    TooLarge { len: usize, max: usize },

    #[error("nesting deeper than {max}")]
    DepthLimit { max: usize },

    #[error("value cannot be represented in the DUAP data model: {0}")]
    OutOfModel(String),

    #[error("JSON view error: {0}")]
    JsonView(String),

    #[error("unknown hash algorithm {0:?}")]
    UnknownHashAlg(String),

    #[error("malformed digest string: {0}")]
    BadDigest(String),

    #[error("foreign codec error: {0}")]
    Foreign(String),
}
