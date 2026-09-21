# ADR-0011: A hand-written HTTP stack in the reference gateway

**Status:** accepted · 2026-09-21

## Context

The gateway is the protocol's trust boundary: it is where unauthenticated
bytes from the network first meet parsing, signature verification and
authorization. The reference implementation's purpose is to be inspectable
by people evaluating whether the protocol is sound.

## Problem

Which HTTP stack for the reference gateway?

## Alternatives

1. **An async framework** (axum on tokio, or equivalent).
2. **A minimal synchronous framework.**
3. **A hand-written blocking HTTP/1.1 server.**

## Decision

A hand-written blocking HTTP/1.1 server, about 200 lines, with a fixed
thread pool and a connection ceiling. Marked REFERENCE. A production
deployment terminates TLS and HTTP at a mature reverse proxy and speaks to
this service over loopback or a service mesh; `DEPLOYMENT.md` says so.

## Trade-offs

No TLS, no HTTP/2, no keep-alive, no graceful backpressure beyond the
connection ceiling, and throughput far below a tuned async stack. None of
those is what the reference implementation is for.

## Consequences

- The gateway crate's dependency surface is the protocol crates plus serde,
  clap and hex. A reviewer can read every line that touches a network byte.
- Any performance claim about the gateway must say that the HTTP layer is
  not the one a deployment would use.
- The connection ceiling is the only DoS control at this layer, and
  `THREAT_MODEL.md` records that.

## Rejected alternatives

**An async framework** is rejected for the reference implementation, not in
general. It is the right choice for production. But it brings a runtime and
a dependency tree of a few hundred thousand lines that a reviewer must take
on trust at precisely the point where the protocol's security argument
begins. The trade is deliberate and narrow: auditability at the trust
boundary, in exchange for performance the reference implementation does not
claim.

**A minimal synchronous framework** is rejected as the worst of both: a
dependency to audit, and still not a production stack.
