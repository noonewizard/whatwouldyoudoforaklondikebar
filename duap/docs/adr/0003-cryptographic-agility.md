# ADR-0003: Cryptographic agility and the hybrid suite

**Status:** accepted · 2026-09-21

## Context

Accounting records must remain verifiable for as long as they might be
disputed, which in tax and privacy contexts is measured in years and in
some jurisdictions in decades. Ed25519 is not believed to resist a
cryptanalytically relevant quantum computer. ML-DSA is standardised (FIPS
204) but has far less deployment history and no comparable body of
implementation review.

## Problem

Which signature scheme, given that the answer must hold for records signed
today and verified in 2045?

## Alternatives

1. **Ed25519 only**, and migrate later.
2. **ML-DSA only.**
3. **A hybrid**: both signatures, both must verify.
4. **A suite registry** with all of the above and policy deciding.

## Decision

A suite registry. Every key, signature and envelope names its suite;
verifiers dispatch on the name; policy decides which suites are acceptable
in which role. Four suites are registered: `ed25519`, `ml-dsa-44`,
`ml-dsa-65`, and the concatenated hybrid `ed25519+ml-dsa-44`.

The reference clearing node signs receipts with the hybrid.

## Trade-offs

The hybrid costs 2,484 signature bytes against Ed25519's 64, and signing is
about 15 times slower (measured: 760.6 µs against 47.4 µs). Verification is
only 3.3 times slower, which is the favourable direction for receipts:
signed once, verified by many parties over many years.

An event envelope signed with the hybrid is 3,096 bytes against 665 — 4.7
times — which is the reason events are signed with Ed25519 by default and
receipts with the hybrid. That is a policy choice a deployment can change.

## Consequences

- No primitive is hard-coded anywhere; adding a suite is a registry entry.
- Security arguments must be per-suite, and the hybrid's argument must be
  stated: forging the concatenation requires forging both halves, so it is
  unforgeable if *either* component is.
- The implementation must reject a suite it does not implement rather than
  accept it. The Go verifier demonstrates this: it implements Ed25519 only
  and skips ML-DSA vectors explicitly.

## Rejected alternatives

**Ed25519 only** is rejected because "migrate later" is not a plan for
records already signed. A signature made today with a scheme broken in
2040 is worthless in 2040, and no migration reaches backwards.

**ML-DSA only** is rejected because it concentrates risk in a scheme whose
implementations are young. A cryptanalytic result or a widespread
implementation flaw would invalidate every record at once; the hybrid
survives either failure.

**Hybrid everywhere, by mandate** is rejected on measurement: a 4.7x
storage penalty on every event, applied to the highest-volume object in the
protocol, is a large cost to impose on deployments that have made a
different risk assessment. The registry lets them choose and lets policy
enforce the choice.
