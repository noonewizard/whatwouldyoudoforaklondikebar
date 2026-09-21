# Project status register

**Status:** PRODUCTION (process artefact) · 2026-09-21
**Owner:** `chief-architect`. No subsystem's status changes without approval
recorded in the commit that changes it.

This is the single place a status is asserted. Before this register existed,
each crate declared its own maturity in its module header, and eleven of
fifteen claimed PRODUCTION — a status whose entry criteria none of them met.
That finding is VS-8 in `security/findings.md`, and this file is its fix:
one claim, in one place, graded against one table.

## The states

```
CONCEPT -> RESEARCH -> SPECIFIED -> PROTOTYPE -> REFERENCE
        -> EXPERIMENTAL
        -> PRODUCTION_CANDIDATE -> PRODUCTION
        -> DEPRECATED
        -> REJECTED
```

| State | Entry criteria |
|---|---|
| CONCEPT | Named, with a reason to exist. Nothing implemented. |
| RESEARCH | An open question is being investigated with a stated method |
| SPECIFIED | A written specification another team could implement from |
| PROTOTYPE | Implemented, compiles, has tests |
| REFERENCE | Tests cover the specified behaviour; conformance vectors exist |
| EXPERIMENTAL | Results published with methods, seeds and uncertainty |
| PRODUCTION_CANDIDATE | Red-teamed, benchmarked, documented, operable |
| PRODUCTION | Independently reviewed; for cryptographic subsystems, independently audited |
| DEPRECATED | Superseded; retained for compatibility, with the successor named |
| REJECTED | Investigated and not adopted, with the reason recorded |

Two readings that are applied consistently here:

- **"Conformance vectors exist"** is read strictly for subsystems whose
  behaviour is normative. A subsystem that is deliberately
  implementation-specific (the ledger's internals, the metering pipeline's
  stage order) has no vectors to write, so it cannot reach REFERENCE by that
  route and stays at PROTOTYPE. That is the correct outcome: REFERENCE means
  "another implementation can check itself against this", and for a
  non-normative subsystem there is nothing to check against.
- **PRODUCTION is currently unreachable for everything in this repository**,
  because no independent review has taken place and no audit has been
  commissioned. Nothing is at PRODUCTION except the three process artefacts
  at the bottom of this page, which are documents rather than software.

## Rust crates

| Subsystem | Status | Evidence | What blocks the next state |
|---|---|---|---|
| `duap-canon` | REFERENCE | L1 vectors; property tests for encoder/decoder agreement; reproduced by the Go implementation | No coverage-guided fuzzing of the decoder. It parses adversary-supplied bytes, so this is the highest-value missing test in the repository. |
| `duap-crypto` | REFERENCE | L2 vectors; Ed25519, ML-DSA-44/65 and the hybrid suite tested against them | Not audited. A cryptographic subsystem cannot reach PRODUCTION without an independent audit however well it is tested, and none has been commissioned. |
| `duap-model` | REFERENCE | Event and taxonomy vectors; round-trip and strict-decode tests | VS-5 open: nothing prevents a constant pseudonym salt. |
| `duap-auth` | REFERENCE | L4 vectors; `Authorization.tla` exhaustive within bounds; mirror tests | VS-6 and VS-7 open. VS-6 must close before the wire format freezes. |
| `duap-provenance` | REFERENCE | L3 Merkle vectors; RFC 6962 inclusion and consistency proofs; dataset-commitment tests | PERF-01 open: inclusion-proof generation is O(n), 14.99 ms in a 100,000-entry tree. Blocks PRODUCTION_CANDIDATE and must be fixed before network exposure. |
| `duap-receipt` | REFERENCE | L3 receipt vectors; anchored-receipt verification from the public key alone | No independent review. |
| `duap-valuation` | REFERENCE | L5 pricing vectors; apportionment and rounding tests | No independent review. The coefficients are parameters, not measurements, and no status raises that. |
| `duap-conformance` | REFERENCE | 93 vectors checked by `cargo test` (VS-9) and by a second implementation | The Go verifier is still run by hand; wiring it in needs CI. |
| `duap-meter` | PROTOTYPE | Dedup, double-count and gap-detection tests; the incremental accumulator is proven byte-identical to the recursive construction | No vectors, by design — counters are inputs to pricing, not protocol objects. Reaching REFERENCE requires deciding whether the operation fingerprint is normative, which the vertical-slice review left open. |
| `duap-ledger` | PROTOTYPE | Double-entry tests; `Accounting.tla` exhaustive within bounds; mirror tests | No vectors, by design — ledger internals are implementation-specific. What is normative (rounding mode, residue posting) is covered by the L5 set. |
| `duap-clearing` | PROTOTYPE | Exercised end to end by `duap-demo` and pinned by the golden transcript | No tests of its own. Single process: horizontal scaling, multi-region operation and durable replication are UNIMPLEMENTED. |
| `duap-sdk` | PROTOTYPE | Exercised end to end by `duap-demo` | No tests of its own. VS-5 is open against the pseudonym derivation it exposes and blocks any release. |
| `duap-gateway` | PROTOTYPE | Integration tests over the nine observable stages | Not a production HTTP stack (ADR-0011). PERF-01 makes `/v1/log/proof` a denial-of-service lever; it must be rate-limited in the default configuration, not only in prose. |
| `duap-cli` | PROTOTYPE | Compiles and runs; used to produce the vectors and the demonstration | No tests of its own. |
| `duap-bench` | PROTOTYPE | Produced `benchmarks/results/2026-09-21-ci-runner.md` | Single-threaded, in-memory, no network, no durable storage. A benchmark harness that cannot measure concurrency cannot support a scaling claim, and none is made. |
| `duap-demo` | REFERENCE | The golden transcript is diffed on every test run, so any behavioural change anywhere in the stack fails a test | It is a demonstration; REFERENCE is its terminal state. |

## Non-Rust subsystems

| Subsystem | Status | Evidence | What blocks the next state |
|---|---|---|---|
| `gateway/` (Go verifier) | REFERENCE | Passes all 77 L1–L3 vectors; written against the specification, not ported; found VS-4 | Covers three of five levels, and the same author wrote both implementations. A third-party implementation is what would make the independence claim unqualified. |
| `spec/vectors/` | REFERENCE | 93 vectors across 8 files, checked by `cargo test` and by the Go verifier | L4 and L5 have no second implementation. |
| `specs/protocol-v0.1.md` | SPECIFIED | An independent L1–L3 implementation was built from it | Not frozen. VS-6 must close before it is. |
| `specs/invariants.md` | SPECIFIED | 25 invariants, each mapped to an enforcement point and a test or model; `tools/check_invariant_tests.py` confirms all 59 cited tests and 7 cited model invariants exist | The checker verifies that a cited test exists, not that it tests the invariant. That judgement is a review responsibility and has not been independently exercised. |
| `formal/` | REFERENCE | Two models, exhaustive within stated bounds (15,237 and 1,920 states); non-vacuity guard | No refinement proof. The models are not extracted from or verified against the Rust, and `formal/Negotiation.tla` is referenced by code but does not exist (VS-7). |
| `ontology/` | REFERENCE | Generates the taxonomy for three languages from one source; regeneration checked | Regeneration is not yet checked in CI, because there is no CI. |
| `research/ai-attribution/` | EXPERIMENTAL | Five findings with measured numbers, seeds and cost; the headline result is negative | It is an experiment and stays one. Its conclusion — that no estimator is a defensible basis for payment — is what keeps attribution off the protocol's critical path. |
| `sdk/python` | CONCEPT | A generated taxonomy module and nothing else | No client, no signing, no tests. Must not be described as a Python SDK. |
| `sdk/typescript` | CONCEPT | A generated taxonomy module and nothing else | Same. |
| `infrastructure/` | CONCEPT | Empty | Nothing is deployable, and no deployment artefact should be written before PERF-01 closes. |
| `dashboard/` | CONCEPT | Empty | — |
| `compliance/` | CONCEPT | Empty | `COMPLIANCE.md` is unwritten; rule 04 governs what it may say. |
| `schemas/` | CONCEPT | Empty | JSON Schema, CDDL and protobuf artefacts are specified as deliverables and do not exist. |
| `simulations/` | CONCEPT | Does not exist | Economic questions requiring participants rather than simulation are listed in `docs/research/`. |

## Optional settlement rails

| Subsystem | Status | Reason |
|---|---|---|
| Fiat settlement interface | PROTOTYPE | `duap-ledger` records what is owed and hands execution to a payment system that already exists. No rail is implemented. |
| Tokenized settlement | RESEARCH | Investigated as one optional rail among several, per the directive's instruction not to require cryptocurrency. Nothing implemented; no protocol object depends on it. |
| Blockchain-based ledger | REJECTED | ADR-0005. Nothing in the vertical slice required consensus among mutually distrusting parties over a shared state machine. A transparency log supplies append-only evidence at 108 ns per append and 129 bytes per entry, with the detection-not-prevention limit stated. |

## Process artefacts

| Artefact | Status |
|---|---|
| `security/findings.md` | PRODUCTION |
| `docs/adr/` | PRODUCTION |
| `docs/STATUS.md` | PRODUCTION |

## History

| Date | Change | Evidence |
|---|---|---|
| 2026-09-21 | Register created. Eleven crates lowered from PRODUCTION to REFERENCE or PROTOTYPE. | VS-8; `docs/reviews/vertical-slice-review.md` §2 and decision 8. No crate had been independently reviewed, none audited, and three held open findings that rule 10 forbids at PRODUCTION. |
