# ADR-0006: Organisation identity is delegated to an authority

**Status:** accepted · 2026-09-21

## Context

Accounting requires an accountable party. That party must be nameable in a
way that is stable across years, comparable across clearing nodes, and
auditable. It does not have to be a name the protocol itself vouches for.

## Problem

Does DUAP operate an organisation registry?

## Alternatives

1. **Operate a registry** with admission criteria and legal verification.
2. **Use an existing identifier scheme** exclusively (LEI, DID, domain).
3. **Delegate to a named authority**, carried in the identifier.

## Decision

An organisation identifier is `org:<authority>/<local>`. The authority
names the registry that vouches for the binding: `duap` for the protocol's
own registry, `did` for a W3C DID, `lei` for a Legal Entity Identifier,
and others as adopted.

The protocol requires only that the binding be stable and auditable. It
does not verify it.

## Trade-offs

Two clearing nodes may know the same legal person under two identifiers
from two authorities, and nothing in the protocol reconciles them. That is
a real cost: cross-node aggregation of one organisation's behaviour needs
an out-of-band mapping.

In exchange, DUAP does not become a know-your-customer operator, which is
a regulated activity with a jurisdictional footprint that would make the
protocol un-deployable in several of the places it most needs to work.

## Consequences

- `OrgId` validates syntax only: a lowercase authority, a non-empty local
  part, no control characters or spaces.
- `COMPLIANCE.md` records that identity assurance is an authority's
  responsibility and names it as an interpretation-required area.
- A clearing node's own admission policy is an operational control, not a
  protocol one.

## Rejected alternatives

**Operating a registry** is rejected because verifying that an identifier
corresponds to a real legal person is a regulated activity, and because it
would make the protocol's adoption depend on the operator's jurisdiction.

**A single existing scheme** is rejected because none covers the range:
LEIs cover financial-market participants, DIDs cover self-asserted
identity with no verification, domains cover organisations with domains
and conflate identity with a lease.
