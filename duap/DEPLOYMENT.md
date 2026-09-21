# Deployment

**Status:** SPECIFIED (design) / UNIMPLEMENTED (everything) · 2026-09-21

**Nothing in this document is built.** `infrastructure/` is empty.
`docs/STATUS.md` records it as CONCEPT. There is no container image, no
chart, no module, no deployment of any kind. This file is the design that
would be implemented, written down so the gap between what exists and what
is described is visible rather than inferred.

**There is also a gate in front of it.** `docs/reviews/vertical-slice-review.md`
decision 2 requires PERF-01 fixed before the gateway is exposed to a
network: inclusion-proof generation is O(n), 14.99 ms in a 100,000-entry
tree against 2.5 µs to verify, which makes `/v1/log/proof` a
denial-of-service lever at roughly 15 ms of server time per cheap request.
No deployment artefact should be written before that closes.

## Topology

```
   reporting agents (SDK / OTel collector)
             |  signed events over HTTPS
             v
      +--------------+        +------------------+
      | DUAP gateway | -----> |  clearing node   |
      +--------------+        +------------------+
             |                        |
             |                   +---------+
             |                   | storage |
             |                   +---------+
             |                        |
             v                        v
        metrics/logs        transparency log + auditors
```

Four components, each deployable independently:

| Component | Role | Status |
|---|---|---|
| Gateway | Terminates HTTPS, authenticates agents, applies the nine-stage pipeline | PROTOTYPE, not a production HTTP stack (ADR-0011) |
| Clearing node | Evaluates, meters, prices, receipts, anchors, invoices | PROTOTYPE, single process |
| Storage | Events, counters, grants, ledger | In-memory or SQLite; no durable deployment exercised |
| Log auditors | Fetch and check consistency proofs | **UNIMPLEMENTED**, and see below |

## The dependency nobody should overlook

The transparency log **detects** equivocation; it does not prevent it, and
detection is conditional on auditors independently fetching signed tree
heads and checking consistency between them. **No auditor is implemented,
and none is deployed.**

Until at least two independent auditors exist and gossip, the log's
detection property is a design property and not an operational one. A
deployment that presents the log as a guarantee without running auditors
is claiming a property it does not have, and `SECURITY.md` states the
condition. This is the single most important thing on this page.

## Scaling, as far as measurement supports

Measured: ingest is 7,829 events/s single-threaded at 127.7 µs per event,
of which Ed25519 verification is 39%
(`benchmarks/results/2026-09-21-ci-runner.md`).

**No concurrency has been measured.** Every benchmark scenario is
single-threaded, in-memory, with no network and no durable storage. Rule 02
forbids multiplying a per-core figure by a core count and calling it
throughput, so this document does not state a system capacity.

What can be said about the shape, marked as design rather than measurement:

- Ingest is embarrassingly parallel per stream; ordering matters only
  within a stream, because sequence-gap detection is per-stream.
- Dedup requires a shared index keyed on canonical digest, which is the
  first component that would need to be distributed and the first place a
  real design would meet a hard problem.
- Period close is a single-writer operation per grant.
- The log is append-only with a single writer; read scaling is a caching
  problem, and PERF-01 must be fixed first.

## Key management

| Key | Held by | Rotation |
|---|---|---|
| Agent signing key | The reporting system | Per deployment. Compromise invalidates later signatures, not earlier ones — unless the revocation reason is Compromise or PolicyViolation, in which case `KeyRegistry::check_usable` stops honouring earlier signatures too |
| Clearing node signing key | The node | Rare, high-consequence. Every receipt depends on it |
| Log signing key | The log operator | Rare. Rotation must be published in the log itself |
| Pseudonym root secret | The subject's agent | Never leaves it. A constant root would make pseudonyms linkable across controllers and destroy the only unlinkability the protocol offers; since VS-5 closed, a crate that has not enabled `insecure-fixed-secret` cannot construct one at all |

**No key-management implementation exists.** There is no HSM integration,
no KMS integration, and no key-rotation procedure that has been executed.
The suite registry supports Ed25519, ML-DSA-44/65 and a concatenated
hybrid; choosing among them is a deployment decision, and the measured
trade-off is in the benchmark results — post-quantum cost is concentrated
in signing (690.7 µs vs 47.4 µs) and the 4.7× envelope-size penalty
dominates the storage decision more than the CPU one.

## Configuration that must not be defaulted wrongly

| Setting | Requirement |
|---|---|
| `/v1/log/proof` rate limit | Must be in the shipped default configuration, not in prose, while PERF-01 is open |
| Subject root secret | Must come from `OsEntropy`. A deployment whose dependency graph enables `insecure-fixed-secret` has made a mistake; `tools/check_insecure_features.py` is the check |
| Dedup window | Must exceed the maximum expected reporting delay, or legitimate events are rejected as replays |
| Retention | Must outlive the dispute window. `economics/results.md` models 12–84 months; the correct value is a legal question |
| Log auditor endpoints | At least two independent ones, or the detection property is not operational |

## What would have to be built

1. Container images for gateway and clearing node.
2. A durable storage deployment — the SQLite path exists in code and has
   never been exercised under load or failure.
3. Log auditor, and a second operated by someone else. Without this the
   transparency log is a log.
4. Key management against a real KMS or HSM.
5. Health, readiness and the metrics already exposed in Prometheus text
   format by `duap-gateway::observe`.
6. `OPERATIONS.md`'s runbooks, which are also unimplemented.

None of this should start before PERF-01 closes and before
`docs/market/go-to-market.md` stage 0 returns an answer. Building
deployment infrastructure for a protocol nobody has agreed to adopt is the
most expensive way to be wrong.
