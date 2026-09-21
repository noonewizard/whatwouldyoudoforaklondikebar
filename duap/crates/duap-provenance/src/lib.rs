//! # duap-provenance
//!
//! The DUAP transparency log and derivation graph.
//!
//! STATUS: PRODUCTION (reference implementation).
//!
//! Two structures, two jobs:
//!
//! * [`log::TransparencyLog`] -- an append-only Merkle log of *commitments*
//!   (never content), with RFC 6962 inclusion and consistency proofs. It
//!   answers "did this object exist by time T, and has the history been
//!   rewritten since?".
//! * [`dag::ProvenanceGraph`] -- a content-addressed derivation graph with
//!   exact-rational attribution. It answers "what was this built from, and
//!   whose data does it carry?".
//!
//! Neither answers "is the recorded claim true". That needs audit sampling
//! and cross-checks; see `THREAT_MODEL.md`.

pub mod dag;
pub mod log;
pub mod merkle;

pub use dag::{
    DerivationPolicy, Edge, GraphError, NodeKind, ProvNode, ProvenanceGraph, WeightBasis,
};
pub use log::{
    EntryKind, InclusionWitness, LogEntry, LogError, LogMonitor, TransparencyLog, TreeHead,
};
pub use merkle::{ConsistencyProof, InclusionProof, MerkleLog, leaf_hash, node_hash, root_of};
