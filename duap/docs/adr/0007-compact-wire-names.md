# ADR-0007: Compact wire names

**Status:** accepted · 2026-09-21

## Context

Every DUAP object is encoded as a CBOR map with text keys. At the
protocol's target volume — trillions of events — the encoded size of those
keys is a material fraction of total storage and bandwidth.

A minimal event is 475 bytes canonical (measured). With descriptive field
names (`data_class`, `authorization`, `occurred_at`, …) the same event is
roughly 620 bytes: about 30% larger.

## Problem

Readable field names, or small ones?

## Alternatives

1. **Descriptive names** (`data_class`).
2. **Integer keys** (`3`).
3. **Short text names** (`dc`).

## Decision

Short text names of two or three characters, fixed by the specification,
generated into every SDK, and documented in a field table in
`specs/protocol-v0.1.md` §4.2.

## Trade-offs

A raw encoded object is not self-describing to a human. That cost is paid
by tooling: `duax canon decode` renders the JSON view, and the JSON view
uses the same short names, so the table is needed either way.

## Consequences

- The field table is normative and must be kept in the specification, not
  only in code.
- Every SDK generates its field mapping from one source, so they cannot
  drift.
- A renamed field is a breaking change requiring a new domain label.

## Rejected alternatives

**Descriptive names** are rejected on the measurement: 30% of the largest
object class, permanently, for readability that tooling supplies anyway.

**Integer keys** are rejected although they are smaller still. The DUAP
data model requires text map keys, which makes canonical ordering trivially
specifiable and the JSON view mechanical. Admitting integer keys would
reintroduce an ordering rule that differs between CBOR and JSON, and the
saving over two-character text keys is one byte per field.
