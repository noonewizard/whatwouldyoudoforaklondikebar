# ADR-0012: Deny-overrides with obligation union

**Status:** accepted · 2026-09-21

## Context

An authorization grant contains several terms. Some permit, some deny, some
attach conditions. Two or more may apply to the same event. The combining
algorithm decides what happens then, and the choice determines whether a
grant means what its author thought.

## Problem

Which combining algorithm?

## Alternatives

1. **First-applicable** (the first matching term decides).
2. **Permit-overrides.**
3. **Deny-overrides.**
4. **Ordered policy sets** with per-set algorithms (XACML-style).

## Decision

Deny-overrides, with the obligations of *all* matching permit terms unioned
and each checked. A single violated obligation makes the decision Deny.

## Trade-offs

Deny-overrides means a broad prohibition cannot be narrowed by a later,
more specific permission. A subject who writes "never for advertising" and
then "except contextual advertising" gets the first rule, not the second,
and must instead narrow the prohibition. That is surprising once and safe
always.

Obligation union means a permission can be harder to satisfy than its
author intended, when a second term with an unrelated safeguard also
matches. The alternative is worse: a broad permit silently erasing a narrow
term's safeguard.

## Consequences

- Term order carries no meaning, which removes a class of authoring
  mistake and removes re-ordering during amendment as an attack.
- The evaluator is a pure function with no ordering sensitivity, which is
  what makes it model-checkable.
- Authoring tools must explain the algorithm to subjects, because "add an
  exception" does not work.

## Rejected alternatives

**First-applicable** is rejected because term order becomes load-bearing.
An amendment that reorders terms changes the policy without changing any
term, and no diff makes that obvious.

**Permit-overrides** is rejected because it makes prohibitions advisory. A
subject's "never" must not be defeatable by adding a permission.

**XACML-style policy sets** are rejected as more expressive than the
problem needs, at the cost of a semantics that few people predict
correctly. The evidence from deployed XACML is that combining-algorithm
subtleties are a recurring source of policy error.
