# ADR-0017: A facts interface for context-dependent obligations

**Status:** accepted · 2026-09-21
**Decided by:** `chief-architect`, with `protocol-engineer`
**Closes:** vertical-slice review decision 7

## Context

The authorization evaluator is a pure function of a grant, an event and an
evaluation context. Purity is load-bearing: it is why `evaluate` can be
model-checked in `formal/Authorization.tla`, why INV-A3 (determinism) holds,
and why two implementations can be expected to agree.

VS-2 found that this purity had a gap. `MaxDerivationDepth` denied every
deriving operation, because derivation depth is a property of the
provenance graph and the evaluator has no graph. The fix added
`DerivationContext { input_depth: Option<u8>, epsilon_micro: Option<u64> }`
to `EvalContext`, and made `ClearingNode::ingest` populate it.

That worked, and the vertical-slice review flagged the shape as wrong
before it could spread.

## Problem

Several obligations need facts an event does not carry:
`MaxDerivationDepth` needs graph depth; `MaxEpsilonMicro` needs the privacy
budget actually spent; `MinCohort` needs the cohort size behind an
aggregate; a volume cap would need cumulative usage; a rate limit would
need recent history.

Adding one `Option<T>` field per obligation has three defects that compound:

1. **It does not scale.** Every new context-dependent obligation widens a
   struct every caller constructs.
2. **`Option` conflates two different situations.** `None` currently means
   both "the provider knows there is no such value" and "nobody asked".
   The obligation must deny on both, so a caller that simply forgot a field
   is indistinguishable from a genuine policy denial.
3. **The failure is silent at the call site.** A caller who forgets is not
   warned; they get denials that look like policy outcomes. VS-2 was
   exactly this failure, and it survived a full unit-test suite because
   every test of the evaluator passed.

## Alternatives

**A. Keep widening `EvalContext`.** Simple, no new concepts, and the
struct is small today.

**B. Give the evaluator a handle to the provenance graph and the meter.**
Obligations query what they need directly.

**C. A `EvalFacts` trait the evaluator queries, returning a three-valued
`Fact<T>`: `Known(T)`, `NotApplicable`, `Unavailable`.** The evaluator
stays pure with respect to its inputs; the caller supplies an
implementation; the default implementation returns `Unavailable` for
everything and obligations needing a fact deny on it.

## Decision

Take **C**.

```rust
pub enum Fact<T> {
    /// The provider answered.
    Known(T),
    /// The provider can answer and the answer is that no such value exists
    /// -- for example, an operation with no inputs has no input depth.
    NotApplicable,
    /// The provider cannot answer. Obligations that need this fact deny.
    Unavailable,
}

pub trait EvalFacts {
    fn input_depth(&self, ev: &DataUsageEvent) -> Fact<u8> { Fact::Unavailable }
    fn epsilon_micro(&self, ev: &DataUsageEvent) -> Fact<u64> { Fact::Unavailable }
    fn cohort_size(&self, ev: &DataUsageEvent) -> Fact<u64> { Fact::Unavailable }
}
```

Three rules make this safe rather than merely tidier:

1. **`Unavailable` denies, and says so.** The denial reason distinguishes
   "the obligation was violated" from "the fact needed to check it was not
   available", so a misconfigured caller sees a configuration error rather
   than a policy outcome. This is the defect VS-2 actually was.
2. **The default implementation returns `Unavailable` for everything**, so
   a caller that supplies nothing fails closed by construction rather than
   by remembering to.
3. **The evaluator remains pure.** It receives a `&dyn EvalFacts` and
   calls it; it does not reach into a store. Determinism now means
   determinism *given the same facts*, which is what INV-A3 must be
   restated to say.

## Trade-offs

**Determinism becomes conditional.** INV-A3 previously said that
`evaluate` is a function of `(grant, event, context)`. It now says it is a
function of `(grant, event, facts answered)`. That is a weaker statement
and it is the honest one: the fact was always an input, and encoding it in
a struct rather than a trait did not make it less so. A provider that
answers differently on two calls produces two decisions, and the model
cannot see that.

**A trait object is harder to model-check** than a struct of `Option`s. The
TLA+ model already abstracts obligations away entirely, so nothing is lost
there today — but that is a statement about the model's current bounds, not
a defence.

**One more concept.** `Fact<T>` is a third state where `Option` had two,
and readers must learn it. The distinction it draws is real and is the
whole point.

## Consequences

- `DerivationContext` is replaced by `EvalFacts` + `Fact<T>`.
  `EvalContext` carries `facts: &dyn EvalFacts`.
- `Obligation::check` takes `&dyn EvalFacts`.
- `DecisionReason` gains `FactUnavailable { term, obligation, fact }`,
  distinct from `ObligationViolated`.
- `ClearingNode` implements `EvalFacts` from its provenance graph, and the
  VS-2 rule survives: an input the node has never seen is `Known(0)`, not
  `Unavailable`, so withholding an upstream event cannot evade a depth
  obligation by making the fact unanswerable.
- `specs/invariants.md` INV-A3 is restated.
- `specs/protocol-v0.1.md` section 5.4 gains the three-valued semantics,
  because an implementation that treats `Unavailable` as permit is
  non-conforming in the most dangerous possible way.

## Rejected alternatives

**A, widening `EvalContext`.** Rejected on the three defects above. The
decisive one is not scale but silence: the current shape cannot tell a
caller they forgot something, and that is how VS-2 reached an end-to-end
run with a full green test suite.

**B, handing the evaluator a graph and a meter.** This is the option that
would make obligations easiest to write, and it was rejected on the
property that matters most. An evaluator that reaches into a store is no
longer a pure function: it cannot be reasoned about in isolation, its
decisions depend on when it ran, and two implementations with different
storage would diverge in ways no conformance vector could capture. The
cost of C — one more concept and a conditional determinism statement — is
much smaller than losing that.
