//! # duap-conformance
//!
//! Cross-implementation conformance vectors.
//!
//! STATUS: REFERENCE. The 93 committed vectors are checked by
//! `cargo test` and by a second implementation. Not PRODUCTION: no
//! independent review, and the Go verifier is still run by hand.
//!
//! # Why this crate is the most important one in the repository
//!
//! The architectural test set out in `docs/architecture/minimal-protocol.md`
//! is whether an independent organisation can implement DUAP without
//! adopting this software. A specification alone does not settle that:
//! prose is ambiguous, and both parties discover the ambiguity only when
//! their digests disagree in production.
//!
//! Conformance vectors settle it mechanically. Each vector is an input and
//! the exact output a conforming implementation must produce. They are
//! generated from the reference implementation, committed as JSON, and
//! checked by:
//!
//! * the reference implementation itself (regression: a change that alters
//!   a vector is a wire-format change and must be deliberate);
//! * a deliberately independent Go verifier in `gateway/cmd/duap-verify`,
//!   written against the specification rather than against this code;
//! * any third-party implementation that wants to claim conformance.
//!
//! # Levels
//!
//! Not every implementation needs every capability. The vectors are graded,
//! and an implementation states which level it claims:
//!
//! | level | capability |
//! |---|---|
//! | L1 | canonical encoding, decoding and digests |
//! | L2 | L1 plus signature verification |
//! | L3 | L2 plus Merkle proofs and receipt verification |
//! | L4 | L3 plus authorization evaluation |
//! | L5 | L4 plus pricing arithmetic |
//!
//! A browser extension that only checks receipts needs L3. A clearing node
//! needs L5.

use serde::{Deserialize, Serialize};

pub mod vectors;

/// Conformance level a vector belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Level {
    L1,
    L2,
    L3,
    L4,
    L5,
}

/// One test vector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    /// Stable identifier, referenced from the specification.
    pub id: String,
    pub level: Level,
    /// What a conforming implementation must do with this vector.
    pub requirement: String,
    /// Input, in a shape specific to the group.
    pub input: serde_json::Value,
    /// Expected output.
    pub expect: serde_json::Value,
}

/// A named group of vectors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorFile {
    pub group: String,
    pub protocol: String,
    pub description: String,
    pub vectors: Vec<Vector>,
}

impl VectorFile {
    pub fn new(group: &str, description: &str, vectors: Vec<Vector>) -> VectorFile {
        VectorFile {
            group: group.to_owned(),
            protocol: duap_canon::PROTOCOL_ID.to_owned(),
            description: description.to_owned(),
            vectors,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("vectors are serialisable") + "\n"
    }
}

/// Result of checking one vector.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckResult {
    pub id: String,
    pub passed: bool,
    pub detail: String,
}

/// Summary of a conformance run.
#[derive(Debug, Clone, Default)]
pub struct Summary {
    pub passed: usize,
    pub failed: usize,
    pub failures: Vec<CheckResult>,
}

impl Summary {
    pub fn record(&mut self, r: CheckResult) {
        if r.passed {
            self.passed += 1;
        } else {
            self.failed += 1;
            self.failures.push(r);
        }
    }

    pub fn ok(&self) -> bool {
        self.failed == 0
    }
}
