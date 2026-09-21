# ADR-0009: Grant epochs and the hash chain

**Status:** accepted · 2026-09-21

## Context

Authorizations change. A subject narrows a permission, adds a prohibition,
or accepts a new price. Meanwhile events already cite the authorization
they relied on, and those events are signed and stored.

## Problem

How does a grant change without invalidating the record of what was relied
on, and without letting a party claim to have relied on a version that did
not exist?

## Alternatives

1. **Mutable grants** with a version counter.
2. **Immutable grants with epochs**, each a separate signed document.
3. **A delta log**: a base grant plus signed amendments.

## Decision

Grants are immutable. An amendment is a new document with the same
identifier, `epoch + 1`, and a `previous` field holding the digest of the
epoch it replaces, forming a hash chain. Events cite `(grant, epoch,
digest)`; an evaluator recomputes the digest and rejects a mismatch.

A revocation naming epoch *n* binds every epoch ≥ *n*.

## Trade-offs

Storage grows with amendments, and a long-lived grant accumulates a chain
the store must retain to validate later links. The alternative — discarding
superseded epochs — would make old events unevaluable.

## Consequences

- A store must refuse an amendment that does not chain to the epoch it
  claims to replace.
- "We relied on the version that permitted it" becomes a checkable claim.
- `INV-A5` (amendment cannot escape revocation) is required and is
  model-checked.

## Rejected alternatives

**Mutable grants** are rejected because the record of what was relied on
would be a version number, and a version number is not evidence: the
controller and the subject can hold different documents both labelled v3.

**A delta log** is rejected because evaluation would require replaying
amendments in order, making the decision a function of a sequence rather
than of a document. That is more state, more failure modes, and a worse
story for an auditor who wants to check one decision.
