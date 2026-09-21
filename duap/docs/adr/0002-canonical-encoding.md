# ADR-0002: Canonical encoding: a restricted deterministic CBOR

**Status:** accepted · 2026-09-21

## Context

Every digest in the protocol is the identity of an object, and every
signature is over bytes. If two byte strings can decode to the same value,
or one value can encode two ways, then two parties can disagree about what
was signed. That is not a theoretical concern: it is the mechanism behind
a long list of real signature-bypass vulnerabilities.

## Problem

Which encoding, and how much of it?

## Alternatives

1. **JSON with a canonicalisation scheme** (RFC 8785 JCS).
2. **Protocol Buffers.**
3. **Full CBOR with RFC 8949 deterministic encoding.**
4. **A restricted subset of deterministic CBOR.**

## Decision

Canonical form is RFC 8949 §4.2.1 core deterministic encoding, restricted
to: null, boolean, unsigned integers, negative integers bounded at −2^63,
byte strings, text strings, arrays, and maps with unique text keys. Floats,
tags, indefinite lengths, `undefined` and non-text keys are outside the
model and rejected.

The JSON view is a lossless rendering for systems that cannot process
CBOR, and is never the hashing input.

## Trade-offs

Rejecting floats means no field can carry a real number, so money is
integers plus a scale, weights are rationals, and probabilities are
parts-per-million integers. That is more work at every call site, and it is
the right trade: a protocol that carries 0.1 in a float carries a different
0.1 in every language.

Rejecting tags means no standard date/time type, so timestamps are
integers. Rejecting bignums means amounts above 2^64 are decimal strings,
which is ugly and unambiguous.

## Consequences

- The decoder must be strict, and strictness must be tested with
  non-canonical inputs, not only with valid ones.
- A JSON-only implementation must still implement the CBOR encoder to
  compute a digest. The conformance vectors make that checkable.
- Negative integers are bounded at −2^63. This was not in the original
  design: the independent Go implementation could not parse
  `{"$n64":"-18446744073709551616"}` without arbitrary-precision
  arithmetic, for one value no object uses. See `security/findings.md` VS-4.

## Rejected alternatives

**JCS over JSON** is rejected because JSON numbers are the problem it does
not solve: JCS specifies a serialisation for doubles, which means the
canonical form of a large integer depends on IEEE-754 rounding. It also
has no byte-string type, so every digest and key would travel base64-encoded
inside a string, inflating every object by a third.

**Protocol Buffers** is rejected because proto3 serialisation is
explicitly not canonical — field ordering and default-value omission are
implementation choices — so a canonicalisation layer would be required
anyway, and unknown-field preservation conflicts with the strict rejection
this protocol wants.

**Full deterministic CBOR** is rejected because it admits floats and tags,
and every admitted construct is one an implementation must get exactly
right for interoperability. The subset is smaller than the union of what
implementations get right.
