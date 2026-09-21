# ADR-0013: Per-controller pseudonyms derived from a subject root secret

**Status:** accepted · 2026-09-21

## Context

Attribution needs a stable handle for the subject an event is about.
Privacy forbids that handle being the same across controllers, because a
globally stable identifier is a join key and the harm it enables is
precisely what the protocol exists to price.

## Problem

How is a subject named?

## Alternatives

1. **A global subject identifier**, issued by a registry.
2. **A random per-controller identifier**, stored by the subject's agent.
3. **A derived per-controller pseudonym** from a root secret.
4. **An anonymous credential** with selective disclosure.

## Decision

`SubjectRef(org) = truncate_128(H("duap.pseudonym.v1", root || 0x00 ||
org_id))`, with a separate derivation for the per-controller signing key so
that publishing a public key does not reveal the pseudonym.

## Trade-offs

The root secret becomes a single point of failure: lose it and the subject
cannot recompute their pseudonyms or sign anything; disclose it and every
pseudonym is linkable. A random-per-controller scheme spreads that risk
across a table the subject must back up instead.

Derivation wins because the table is the harder thing to keep: a subject
with a hundred controllers must back up a hundred entries and keep them in
sync across devices, and a lost entry loses a relationship. A 256-bit seed
is one thing to back up, and recovery is recomputation.

## Consequences

- The agent must protect the root secret at least as well as a password
  manager protects a vault, and `PRIVACY.md` says so.
- Pseudonyms are recomputable offline, which is what makes a single subject
  dashboard possible without a central linkage table.
- The scheme provides no unlinkability at payout time, where some party
  must learn that pseudonyms belong together. That exposure is stated in
  `PRIVACY.md` and prototyped against in `research/`.

## Rejected alternatives

**A global identifier** is rejected outright. It is the tracking identifier
the protocol exists to make unnecessary.

**Random per-controller identifiers** are rejected on the backup argument
above, though they are otherwise equivalent and a deployment that prefers
them is conformant: the specification requires only that a reference be
stable per (subject, controller) and not derivable across controllers.

**Anonymous credentials** are rejected for the kernel, not in principle.
They would improve the payout story materially. They also add a
cryptographic dependency with far less deployment history than the
protocol's other primitives, to a layer that is already the hardest to
explain. Recorded as research, not as a rejection on the merits.
