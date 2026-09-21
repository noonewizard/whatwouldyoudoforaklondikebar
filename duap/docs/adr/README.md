# Architecture decision records

**Status:** PRODUCTION (process artefact) · 2026-09-21

An ADR is required for any change that alters a wire format, the crate
dependency graph, a protocol invariant, a security boundary, the economic
semantics of an object, or a subsystem's maturity claim
(`.claude/rules/05-no-silent-architecture-change.md`).

ADRs are never edited after acceptance except to add a supersession note. A
changed decision is a new ADR naming the one it replaces.

| ADR | Title | Status |
|---|---|---|
| [0000](0000-template.md) | ADR-0000: Template | template |
| [0001](0001-scope.md) | ADR-0001: Scope: an accounting layer, not a data platform | accepted |
| [0002](0002-canonical-encoding.md) | ADR-0002: Canonical encoding: a restricted deterministic CBOR | accepted |
| [0003](0003-cryptographic-agility.md) | ADR-0003: Cryptographic agility and the hybrid suite | accepted |
| [0004](0004-key-custody.md) | ADR-0004: Key custody is a deployment decision | accepted |
| [0005](0005-transparency-log-not-blockchain.md) | ADR-0005: A transparency log, not a blockchain | accepted |
| [0006](0006-organisation-identity.md) | ADR-0006: Organisation identity is delegated to an authority | accepted |
| [0007](0007-compact-wire-names.md) | ADR-0007: Compact wire names | accepted |
| [0008](0008-monetary-arithmetic.md) | ADR-0008: Monetary arithmetic: integers at two scales | accepted |
| [0009](0009-grant-epochs.md) | ADR-0009: Grant epochs and the hash chain | accepted |
| [0010](0010-event-storage.md) | ADR-0010: Event storage is an append-only blob store with derived indices | accepted |
| [0011](0011-gateway-http-stack.md) | ADR-0011: A hand-written HTTP stack in the reference gateway | accepted |
| [0012](0012-deny-overrides-with-obligation-union.md) | ADR-0012: Deny-overrides with obligation union | accepted |
| [0013](0013-pseudonym-derivation.md) | ADR-0013: Per-controller pseudonyms derived from a subject root secret | accepted |
| [0014](0014-repository-structure.md) | ADR-0014: Repository structure: a monorepo of small crates | accepted |
