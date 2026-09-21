# Changelog

All notable changes to DUAP are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions
follow the protocol versioning rules in `GOVERNANCE.md` §4 rather than
plain semantic versioning: a change to canonical encoding, digest
construction or the signature input is a new major version, because a
mismatched implementation must fail to verify rather than silently
disagree.

## [Unreleased]

Nothing is released. There is no tagged version, no published artefact and
no crates.io publication. The protocol is at v0.1 and **the wire format is
not frozen** — VS-6 in `security/findings.md` must close first, because
absorbing `PricingRule::PerUnit` into `UnitTable` becomes impossible after
a freeze.

### Added

- Canonical encoding (`duap-canon`) with length-first deterministic CBOR,
  strict decoding, and domain-separated digests.
- Cryptographic core (`duap-crypto`): Ed25519, ML-DSA-44/65, a
  concatenated hybrid suite, self-certifying key identifiers, and
  revocation semantics that distinguish reasons preserving earlier
  signatures from reasons that do not.
- Canonical object model (`duap-model`) and a generated taxonomy.
- The Usage Authorization Language and its evaluator (`duap-auth`), with
  deny-overrides and obligation union.
- Transparency log with RFC 6962 inclusion and consistency proofs, and the
  derivation graph (`duap-provenance`).
- Metering pipeline with dedup, sequence-gap and double-count detection
  (`duap-meter`).
- Valuation engine and double-entry ledger (`duap-valuation`,
  `duap-ledger`).
- Receipts, clearing node, Rust SDK, and a working vertical slice
  (`duap-receipt`, `duap-clearing`, `duap-sdk`, `duap-demo`).
- 93 conformance vectors across L1–L5, and an independent Go
  implementation of L1–L3 written against the specification.
- `duax` CLI and a reference gateway.
- Engineering configuration: 24 agents, 10 rules, hooks, commands, skills.
- Three model-checked TLA+ specifications with two non-vacuity guards.
- Benchmark harness and measured results.
- Normative specification, 14 ADRs, and the root document set.
- AI-attribution experiment, dataset commitments, and the research
  register.
- Vertical-slice review, standards landscape, falsification report,
  competitive landscape, market analysis, unit-economics model, legal
  issue map, governance proposal, business model.
- `formal/Negotiation.tla` and its mirror tests (VS-7).

### Changed

- Eleven crates lowered from `PRODUCTION` to `REFERENCE` or `PROTOTYPE`
  (VS-8). None had been independently reviewed, none audited, and three
  held open findings that rule 10 forbids at that status.
- `docs/STATUS.md` became the single place a status is asserted.
- The beachhead changed from AI web-content licensing to pay-per-inference
  accounting, on competitive evidence (falsification kill attempt 11).

### Fixed

- **VS-1** Receipt anchor was inside the digest the log commits to.
  `Receipt::core()` detaches it, which also lets a holder upgrade to a
  fresher proof without changing the receipt's identity.
- **VS-2** Derivation-depth obligations always failed closed. The clearing
  node now computes depth from its graph; an unseen input is treated as
  depth 0, so withholding an upstream event cannot evade the obligation.
- **VS-3** Pricing selection ignored the metered unit. Added
  `PricingRule::UnitTable`.
- **VS-4** The JSON view required arbitrary-precision parsing for one
  value no DUAP object uses. Found by the Go implementation; negative
  integers are now bounded at `i64::MIN` in both.
- **VS-7** `negotiation` referenced a formal model that did not exist. The
  model is written and checked; the module marker is now honest.
- **VS-9** Conformance vectors were not checked by `cargo test`. A changed
  vector now fails the build.

### Known open

- **PERF-01** Inclusion-proof generation is O(n). Blocks
  `PRODUCTION_CANDIDATE` for `duap-provenance` and any network exposure.
- **VS-5** Nothing prevents a constant pseudonym salt. Blocks any SDK
  release.
- **VS-6** Two representations of one pricing concept. Must close before
  the wire format freezes.
- **RISK-01, RISK-02** Accepted: a dishonest clearing node, and usage
  outside instrumentation.
- No independent security audit. No coverage-guided fuzzing of the
  decoder, which is the highest-value missing test in the repository.
