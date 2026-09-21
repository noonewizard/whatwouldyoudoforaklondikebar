# ADR-0014: Repository structure: a monorepo of small crates

**Status:** accepted · 2026-09-21

## Context

The design brief proposes a directory per subsystem (`/crypto`,
`/identity`, `/authorization`, …). The implementation is a Rust workspace
plus a Go module, a Python package, a TypeScript package and a set of
specifications.

## Problem

How is the repository organised so that the protocol kernel is visibly
separable from the reference implementation?

## Alternatives

1. **Directory per subsystem**, language-agnostic, as the brief proposes.
2. **One crate**, modules inside.
3. **A workspace of small crates**, one per subsystem, plus per-language
   SDK directories.
4. **Multiple repositories**, one per subsystem.

## Decision

A workspace of small crates under `crates/`, one per subsystem, with the
subsystem names the brief proposes. Specifications, schemas, vectors,
ontology, formal models, infrastructure and research sit at the project
root as their own directories, because they are not Rust.

The mapping from the brief's structure is:

| Brief | Here |
|---|---|
| `/protocol`, `/crypto`, `/identity`, `/authorization`, `/provenance`, `/metering`, `/valuation`, `/accounting`, `/clearing`, `/settlement` | `crates/duap-*` |
| `/spec`, `/schemas` | `specs/`, `schemas/`, `spec/vectors/` |
| `/sdk/*` | `crates/duap-sdk`, `sdk/python`, `sdk/typescript`, `gateway/` |
| `/gateway`, `/agents`, `/connectors` | `crates/duap-gateway`, `gateway/` |
| `/cli` | `crates/duap-cli` |
| everything else | same name at the project root |

## Trade-offs

A workspace is one version number and one release cadence for ten crates,
which couples their lifecycles more tightly than separate repositories
would. Cross-crate refactoring is easy, which is a risk as well as a
convenience: it makes it *easy* to violate a boundary, so
`.claude/rules/09-architecture-authority.md` has to do the work the
repository layout does not.

## Consequences

- The dependency graph is enforced by Cargo and must stay acyclic.
- Every crate must be usable on its own: an organisation can meter without
  clearing, or verify receipts without metering. That is the property that
  makes independent implementation of parts possible.
- The whole project lives under `duap/` in a shared repository. If it is
  extracted, `duap/` becomes the root and nothing moves.

## Rejected alternatives

**Directory per subsystem, language-agnostic** is rejected because it
fights the build tools of every language in it. A Rust crate is a
directory with a manifest; splitting one subsystem across
`/crypto/rust/` and `/crypto/go/` adds a layer that neither toolchain
wants.

**One crate** is rejected because it makes the dependency graph invisible.
Small crates force the boundaries to be declared, and a boundary violation
becomes a compile error rather than a code-review observation.

**Multiple repositories** are rejected at this stage. The protocol is not
yet stable enough for the cross-repository version negotiation that split
repositories require; `GOVERNANCE.md` revisits this at SPECIFIED status.
