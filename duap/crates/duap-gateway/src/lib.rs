//! # duap-gateway
//!
//! The DUAP reference gateway: an HTTP surface over a clearing node in which
//! every pipeline stage is observable.
//!
//! STATUS: REFERENCE. Suitable for conformance testing, integration work and
//! local demonstration. Not a production HTTP stack: see
//! `docs/adr/0011-gateway-http-stack.md` and `DEPLOYMENT.md`.

pub mod http;
pub mod observe;
pub mod service;

pub use http::{Request, Response, Server};
pub use observe::{Metrics, Stage, StageTimer};
pub use service::{Gateway, RateLimit};
