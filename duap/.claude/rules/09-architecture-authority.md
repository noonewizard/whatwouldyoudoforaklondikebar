# Architecture authority

## The rule

No agent changes an interface owned by another agent. It proposes; the owner
decides; an unresolved disagreement becomes an ADR for `chief-architect`.

## Ownership

Ownership is declared in each agent definition's **Scope** section. Where two
scopes appear to overlap, they do not: one of them is wrong and the
resolution is an ADR, not a negotiation in code.

## What "interface" means here

- A public type or function in another subsystem's crate.
- A wire format or domain label.
- A protocol invariant.
- A file another agent owns (specifications, threat model, compliance
  mapping).

## Escalation

1. Agent A needs a change in agent B's scope.
2. A writes the requirement, with the reason and the alternatives it
   considered, and hands it to B.
3. B implements it, proposes an alternative, or declines with a reason.
4. If A and B disagree, `chief-architect` decides by ADR.

Bypassing this by editing the other subsystem directly is the specific
failure the rule exists to prevent: it produces two architectures that both
work locally and cannot be reconciled.

## The red team's exemption

`red-team-engineer` may write a failing test anywhere in the repository to
demonstrate a finding. It may not write the fix.
