# Findings register

**Status:** PRODUCTION (process artefact) · maintained by `red-team-engineer`
and `security-engineer`

Every finding is recorded here with a reproduction and tracked to closure or
to an explicitly accepted risk with a named accepter. A finding is never
closed because it is "documented as a limitation" unless the documentation
is accurate, prominent, and in the artefact a reader would consult.

Severity: **critical** (breaks an invariant in production), **high**
(exploitable with realistic capability), **medium** (exploitable with
unusual capability or produces a wrong but bounded result), **low**
(quality or hardening), **perf** (performance defect).

| ID | Severity | Component | Status | Summary |
|---|---|---|---|---|
| VS-1 | high | duap-receipt | fixed | Log anchor was inside the digest the log commits to |
| VS-2 | high | duap-clearing | fixed | Derivation-depth obligations always failed closed |
| VS-3 | medium | duap-auth | fixed | Pricing selection ignored the metered unit |
| VS-4 | medium | duap-canon | fixed | JSON view required arbitrary-precision parsing for one value |
| PERF-01 | perf | duap-provenance | open | Inclusion-proof generation is O(n) |
| RISK-01 | n/a | protocol | accepted | A dishonest clearing node can issue a receipt for usage it never evaluated |
| RISK-02 | n/a | protocol | accepted | Usage outside instrumentation is invisible |

---

## VS-1 — Log anchor inside the anchored digest (fixed)

**Found by:** building the vertical slice.
**Severity:** high. The receipt could not be anchored at all: the log entry
commits to the receipt's digest, but attaching the resulting inclusion proof
changed that digest, so verification failed for every anchored receipt.

**Reproduction (before the fix):** `an_anchored_receipt_verifies_its_log_position`
in `crates/duap-receipt/tests/receipt_tests.rs` failed with a digest
mismatch.

**Fix:** `Receipt::digest` now hashes the anchor-free core
(`Receipt::core`), making the anchor a detachable proof *about* the receipt
rather than part of it. A holder can also upgrade to a fresher proof without
changing the receipt's identity.

## VS-2 — Derivation-depth obligations always failed closed (fixed)

**Found by:** the first end-to-end run.
**Severity:** high. Any grant carrying `MaxDerivationDepth` denied every
deriving operation with "derivation depth is unknown", because the
authorization evaluator has no access to the provenance graph and nothing
supplied the depth.

**Reproduction (before the fix):** the demonstration's fine-tuning step was
rejected with `ObligationViolated { obligation: "max_derivation_depth" }`.

**Fix:** `ClearingNode::ingest` computes the depth from its provenance graph
and passes it in the evaluation context. An input the node has never seen is
treated as depth 0 rather than unknown, so an organisation cannot evade a
depth obligation by withholding the upstream event.

## VS-3 — Pricing selection ignored the metered unit (fixed)

**Found by:** the first end-to-end run.
**Severity:** medium. A term covering several operations could only carry one
per-unit price, so usage metered in a different unit was either unpriced or
rejected with a unit mismatch. Real grants routinely span operations metered
in records, queries and inferences.

**Fix:** added `PricingRule::UnitTable`, and made the evaluator prefer a
priced term whose rule can actually price the event's unit before falling
back to specificity and term order.

## VS-4 — JSON view required arbitrary-precision parsing (fixed)

**Found by:** the independent Go implementation, which could not parse the
`canon/encode/neg_large` vector.
**Severity:** medium. `Nint(u64::MAX)` renders as `{"$n64":"-18446744073709551616"}`,
whose magnitude is 2^64 and does not fit an unsigned 64-bit integer. Every
implementation would have needed big-integer arithmetic for one value that
no DUAP object uses.

**Fix:** negative integers are bounded at `i64::MIN` throughout the codec,
the JSON view, the serde bridge and the property generators. Both
implementations now reject anything below it.

**Lesson recorded:** this is the class of defect that only a second
implementation finds, which is the argument for keeping `gateway/` honest
rather than porting the Rust.

## PERF-01 — Inclusion-proof generation is O(n) (open)

**Found by:** the benchmark run of 2026-09-21.
**Severity:** perf. Generating one audit path in a 100,000-entry tree takes
14.99 ms; verifying the same path takes 2.5 us. `MerkleLog::inclusion_proof`
recomputes sibling subtree roots on demand instead of caching them, so proof
generation is linear in the tree size.

**Impact:** acceptable in the reference implementation and in tests;
unacceptable on the gateway's `/v1/log/proof` endpoint, where it is a
denial-of-service lever — an attacker can spend 15 ms of server time per
cheap request.

**Reproduction:** `benchmarks/results/2026-09-21-ci-runner.md`, scenario
`log.inclusion_proof`.

**Proposed fix:** cache complete-subtree roots as entries are appended (the
incremental accumulator in `duap-meter::evidence` already maintains exactly
these), or adopt the tile-based layout used by production transparency logs.
Until then the gateway should rate-limit the proof endpoint separately.

**Owner:** `provenance-engineer`. **Status:** open, not blocking the
vertical-slice review, blocking PRODUCTION_CANDIDATE for `duap-provenance`.

## RISK-01 — A dishonest clearing node (accepted)

A clearing node can issue a receipt for usage no event ever described, or
decline to issue one for usage that occurred. No signature prevents this,
because the node is the signer.

**Mitigations that exist:** every receipt commits to an event-set root that
the subject or an auditor can demand and check
(`Receipt::verify_coverage`); every receipt is anchored, so a node cannot
back-date; the log's consistency proofs make selective history rewriting
detectable given gossip.

**Residual risk:** fabrication and omission remain possible and are detected
only by cross-checking against a party who holds independent records.

**Accepted by:** `chief-architect`, recorded in
`docs/architecture/minimal-protocol.md` §5 and in `Receipt::claims()`, which
returns "the described operations actually took place -- NOT ESTABLISHED".

## RISK-02 — Usage outside instrumentation (accepted)

The protocol accounts for what it is told. An organisation that copies data
out of an instrumented system and processes it elsewhere generates no
events, and no cryptography detects that.

**Mitigations that exist:** per-stream sequence numbers make *partial*
suppression visible; gap reports are exposed on `/v1/stats`.

**Residual risk:** complete non-participation, and any processing on a
copy, is invisible to the protocol. This is a property of the problem, not
of the design.

**Accepted by:** `chief-architect`, recorded in
`docs/research/0003-uninstrumented-participants.md`.
