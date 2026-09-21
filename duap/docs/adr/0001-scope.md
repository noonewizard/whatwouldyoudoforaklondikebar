# ADR-0001: Scope: an accounting layer, not a data platform

**Status:** accepted · 2026-09-21

## Context

Several adjacent systems already exist: consent management platforms, data
clearinghouses, privacy-enhancing computation frameworks, data marketplaces
and provenance systems for machine learning. Each solves part of the
problem and none composes with the others, because each defines its own
object model and each assumes custody of something.

At the same time, every organisation of any size already knows how much
data it processes. The information exists; it is not comparable, not
attributable, and not signed.

## Problem

Is DUAP a system that *holds* data, or a system that *accounts for* data
held elsewhere?

## Alternatives

1. **A data platform.** DUAP receives data, stores it, mediates access, and
   accounts for what it mediates. Complete control over the accounting;
   nothing escapes it.
2. **An accounting layer.** DUAP receives signed assertions about
   operations performed elsewhere. It never holds the data.
3. **A hybrid.** An accounting layer with an optional custody mode for
   participants who want it.

## Decision

DUAP is an accounting layer. The primitive is a signed assertion about a
use, not the data. No component of the protocol transports, stores or
mediates access to the data being accounted for.

## Trade-offs

The protocol cannot verify that an assertion is true. It sees only what it
is told, and an organisation that processes data outside the instrumented
path generates no events. Everything in `THREAT_MODEL.md` about
suppression and fabrication follows from this decision.

In exchange, DUAP can be adopted incrementally by an existing system
without changing where that system's data lives, which is the only adoption
path that is realistic at all. A protocol that requires custody requires a
migration, and a migration requires a reason that does not yet exist.

## Consequences

- Events carry classifications and quantities, never values. The strongest
  statement about content is a salted commitment.
- The transparency log commits to digests only.
- "Did this happen?" is out of scope for cryptography and in scope for
  audit, cross-checking and, ultimately, discovery.
- Every document must state this boundary rather than let a reader assume
  the stronger property.

## Rejected alternatives

**The data platform** is rejected on adoption grounds, not technical ones.
It would give stronger guarantees: if the platform mediates access, it can
count accesses rather than believe a report of them. But it requires an
organisation to move its data into infrastructure it does not control,
which is a larger ask than the accounting benefit justifies, and it makes
the operator a single point of both failure and compromise for exactly the
data the protocol exists to protect.

**The hybrid** is rejected because an optional custody mode is not optional
in practice: once it exists, the accounting semantics fork into "verified"
and "asserted", participants are compared across the fork, and the
asserted side is treated as second-class. One semantics, honestly limited,
is better than two.
