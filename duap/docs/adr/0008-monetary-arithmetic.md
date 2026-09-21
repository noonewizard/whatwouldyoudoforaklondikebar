# ADR-0008: Monetary arithmetic: integers at two scales

**Status:** accepted · 2026-09-21

## Context

Per-event prices in this protocol are tiny: a plausible rate for a service
query is 0.0000125 EUR. Settlement, by contrast, happens in whole cents.
Rounding each event to a cent would either zero every charge or inflate it
by four orders of magnitude.

## Problem

How are amounts represented, and where does rounding happen?

## Alternatives

1. **Floating point** throughout.
2. **Decimal floating point** (IEEE 754-2008 decimal128).
3. **Integers at the minor unit**, rounding per event.
4. **Integers at two scales**, rounding once per invoice line.

## Decision

Two integer scales. `Money` holds signed minor units; `Precise` holds
signed nano-minor-units (10^−9 of a minor unit). Pricing computes in
`Precise`; rounding happens exactly once per invoice line, via a function
that returns both the rounded money and the residue. The residue is posted
to a rounding account so the ledger can prove conservation.

Amounts are encoded on the wire as canonical decimal strings, because the
canonical data model caps integers at 64 bits and aggregate sums exceed it.

## Trade-offs

Two scales mean two types and explicit conversion, which is more code at
every boundary. Decimal-string encoding is larger than a CBOR integer and
requires a canonical-spelling check on parse (no leading zeros, no `-0`).

## Consequences

- `INV-L3` (rounding conserves value) is testable and is tested as a
  property.
- Default rounding is half-even, because half-up drifts upward over many
  lines; subject shares round toward zero, so the payer is never
  over-charged by rounding and the fraction stays with the recipient.
- No code path may use a float for an amount. `to_canonical_cbor` rejects
  floats, so the type system and the encoder both enforce it.

## Rejected alternatives

**Floating point** is rejected outright: 0.01 is not representable, and at
trillions of operations the accumulated error is real money that cannot be
reconciled against a bank statement.

**Decimal128** is rejected because it is not in the canonical data model
(it would require a CBOR tag), support varies by language, and its
rounding behaviour is another thing two implementations must agree on
exactly. Integers plus an explicit scale need no agreement beyond the
scale.

**Integers at the minor unit only** is rejected on the arithmetic above: it
makes per-event pricing impossible without either losing the charge or
inventing one.
