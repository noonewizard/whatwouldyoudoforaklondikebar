# ADR-0016: Absorb `PricingRule::PerUnit` into `UnitTable`

**Status:** accepted · 2026-09-21
**Decided by:** `chief-architect`, on `economics-engineer`'s finding
**Closes:** VS-6

## Context

VS-3 found that a single authorization term could carry only one per-unit
price, while real terms routinely span operations metered in records,
queries and inferences. The fix added `PricingRule::UnitTable`, a list of
`(Unit, Precise)` pairs ordered by unit code.

It was added *alongside* `PricingRule::PerUnit`, which prices exactly one
unit. A one-row unit table expresses exactly what `PerUnit` expresses:

```
PerUnit { unit: U, unit_price: P }   ==   UnitTable { prices: [(U, P)] }
```

The vertical-slice review recorded this as the one change in its list that
becomes impossible if deferred, because the wire format is not yet frozen
and after a freeze both spellings are permanent.

## Problem

Two wire representations of one concept. Should one be removed before the
format freezes?

## Alternatives

**A. Remove `PerUnit`; keep `UnitTable`.** One representation. Every
consumer handles one case. A convenience constructor keeps call sites
short.

**B. Remove `UnitTable`; keep `PerUnit` and allow several pricing rules per
term.** Also one representation, reached from the other direction: a term
carries a list of rules instead of a rule carrying a list of prices.

**C. Keep both.** `PerUnit` is the common case and reads better at a call
site; `UnitTable` handles the rest.

## Decision

Take **A**. Delete the `PerUnit` variant. Add
`PricingRule::per_unit(unit, price)`, a constructor returning a one-row
`UnitTable`, so call sites read the same as before and the wire format has
one spelling.

Adjust the accessors so behaviour is preserved rather than merely
compiling:

- `unit()` returns `Some(u)` for a table with exactly one row, and `None`
  for a table with several. Previously `PerUnit` returned `Some` and
  `UnitTable` always returned `None`; a former `PerUnit` value therefore
  answers identically.
- `covers_unit()` and `validate()` lose their `PerUnit` arms; the
  `UnitTable` arms already handle the one-row case, including the
  non-negative price check.

## Trade-offs

The wire format becomes slightly more verbose for the common case: a
one-price rule now encodes a tag, a one-element array and a pair, rather
than a tag and two fields. Measured on the conformance vectors, the
difference is a handful of bytes per term, against grants of 385 bytes and
event envelopes of 665. Not material.

The enum loses a variant that read more clearly at a match site. That cost
is real for anyone writing an implementation by hand, and is paid once,
against a cost every consumer would otherwise pay forever.

## Consequences

- `PricingRule::PerUnit` is gone from the model, the valuation engine, the
  SDK surface and every test and benchmark.
- The L5 pricing vectors are regenerated. This is a wire-format change and
  this ADR is its record; `crates/duap-conformance/tests/vectors_test.rs`
  fails the build until they are regenerated deliberately.
- The Go implementation is unaffected: it claims L1–L3 and pricing is L5.
- `specs/protocol-v0.1.md` drops the `per_unit` rule tag.
- VS-6 closes.

## Rejected alternatives

**B, a list of rules per term rather than a list of prices per rule.**
Arguably cleaner: it separates "what is priced" from "how it is priced",
and would let a term combine a tiered rule for queries with a flat rule for
records, which `UnitTable` cannot express. Rejected on scope and timing
rather than on merit — it is a larger change to the term structure, the
evaluator's rule-selection logic and the L4 vectors, and the review gate is
explicit that nothing beyond the listed decisions should be built before
stage 0 of the go-to-market plan returns an answer. It is recorded here
because it is the better long-term shape and should be reconsidered if
mixed-rule terms are ever actually requested. Absorbing `PerUnit` now does
not foreclose it.

**C, keeping both.** Rejected because it is the status quo and the status
quo is the defect. Two spellings of one concept mean every consumer
handles two cases, the vectors must cover both, and an implementation that
handles only the common one passes most tests. The readability argument is
real and is answered by the constructor, which is what call sites see.
