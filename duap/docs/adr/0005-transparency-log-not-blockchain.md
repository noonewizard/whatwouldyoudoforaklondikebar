# ADR-0005: A transparency log, not a blockchain

**Status:** accepted · 2026-09-21

## Context

The protocol needs an append-only, globally consistent, third-party
auditable history of commitments. The obvious contemporary answer is a
distributed ledger, and the question is asked of every project in this
space.

## Problem

Does DUAP need consensus?

## Alternatives

1. **A conventional database** with append-only discipline.
2. **A signed append-only log** without Merkle structure.
3. **A Merkle transparency log** (RFC 6962 style).
4. **A permissioned blockchain.**
5. **A public blockchain.**

## Decision

A Merkle transparency log. One hash per entry, O(log n) inclusion and
consistency proofs, signed tree heads, no consensus, no tokens, no
fork-choice rule.

## Trade-offs

A transparency log gives **detection** of equivocation, not prevention. A
log operator that shows different histories to different parties is caught
only when those parties compare tree heads, which requires gossip. A
blockchain with honest-majority consensus gives prevention instead.

DUAP accepts detection. Every document says "detected", not "prevented",
and `THREAT_MODEL.md` T-15 states the exposure: between equivocation and
its discovery, a dishonest log can show a relying party a history that
omits an entry.

## Consequences

- Relying parties must actually check consistency proofs, and the reference
  `LogMonitor` refuses a head that regresses or forks.
- Gossip between auditors is a protocol-adjacent requirement, not an
  optional extra, and `GOVERNANCE.md` makes independent auditors a
  governance role rather than a nicety.
- Settlement is a separate rail. Nothing in the accounting requires a
  token.

## Rejected alternatives

**A conventional database** is rejected because "append-only" enforced by
policy is not evidence: a third party cannot distinguish a database that
was never modified from one that was.

**A signed log without Merkle structure** is rejected because proving that
one entry is in a million-entry history requires sending the history.

**A permissioned blockchain** is rejected on cost. It buys agreement on
ordering between mutually distrusting validators, which DUAP does not need:
every entry is signed by its author, and the ordering that matters is the
log's own sequence, which consistency proofs already pin. The throughput
penalty is several orders of magnitude, and the validator set is a
governance problem as hard as the one it solves.

**A public blockchain** is rejected for the same reason plus three more:
per-entry cost at the protocol's target volume is prohibitive; settlement
finality is probabilistic where accounting needs it deterministic; and
publishing to a permissionless network makes jurisdictional data-residency
requirements unsatisfiable even for digests, because the argument that a
digest is not personal data is not one every regulator has accepted.
