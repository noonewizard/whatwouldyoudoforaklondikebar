# ADR-0010: Event storage is an append-only blob store with derived indices

**Status:** accepted · 2026-09-21

## Context

A clearing node must retain events: to re-derive an aggregate, to answer a
coverage challenge against a receipt, and to support a dispute years later.
The access patterns are narrow: append, point lookup by digest, and range
scan by occurrence time.

## Problem

What storage, and what is authoritative?

## Alternatives

1. **A relational schema** with one column per event field.
2. **A blob store** holding canonical bytes, with derived columns for the
   queried dimensions.
3. **An object store** with an external index.
4. **A distributed ledger.**

## Decision

Append-only storage of the canonical CBOR blob, with derived indexed
columns for digest and occurrence time. The blob is authoritative; the
columns are rebuildable from it.

`EventStore` is a trait. The reference implementations are in-memory (the
reference the others are tested against) and SQLite.

## Trade-offs

Querying on a dimension without a column means a scan, so adding a query
pattern means adding a column and a migration. The alternative — a column
per field — makes the schema the source of truth, which is exactly the
failure this decision avoids.

## Consequences

- No derived column may be written without the blob it derives from.
- Every store implementation is tested against the in-memory reference with
  the same differential suite.
- Retention and eviction are storage concerns with an explicit policy, so
  unbounded growth is impossible by construction rather than by operator
  discipline.

## Rejected alternatives

**A relational schema** is rejected because the signature covers the
canonical bytes. If the columns are authoritative, a schema migration can
silently change what a signature is understood to cover.

**An object store with an external index** is rejected for the reference
implementation only: it is a reasonable production choice and the trait
admits it, but it makes the local demonstration depend on two systems.

**A distributed ledger** is rejected for the reasons in ADR-0005, and
additionally because event bodies must not be published at all.
