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
| PERF-01 | perf | duap-provenance | fixed | Inclusion-proof generation was O(n) |
| VS-5 | low | duap-model | open | Pseudonym derivation accepts a salt with no control against a constant one |
| VS-6 | low | duap-auth | open | Two representations of the same pricing concept |
| VS-7 | low | duap-auth | fixed | `negotiation` referenced a formal model that did not exist |
| VS-8 | medium | all crates | open | Crate maturity markers claim PRODUCTION without meeting the entry criteria |
| VS-9 | medium | duap-conformance | fixed | The conformance vectors were not checked by `cargo test` |
| CI-01 | low | ontology | fixed | Generated code was not formatter-idempotent, so the staleness check could not work |
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

## PERF-01 — Inclusion-proof generation was O(n) (fixed)

**Found by:** the benchmark run of 2026-09-21.
**Severity:** perf, with a security consequence. Generating one audit path
in a 100,000-entry tree took 14.99 ms while verifying the same path took
2.5 us. `MerkleLog::inclusion_proof` recomputed sibling subtree roots from
leaves on demand, so proof generation was linear in tree size.

**Impact:** acceptable in a reference implementation; unacceptable on the
gateway's `/v1/log/proof` endpoint, where it let an attacker spend 15 ms of
server time per cheap request.

**Reproduction:** `benchmarks/results/2026-09-21-ci-runner.md`, scenario
`log.inclusion_proof`.

**Fix:** `MerkleLog` now caches the roots of complete subtrees level by
level, maintained in amortised O(1) on append. The RFC 6962 split rule
makes every left child of the proof recursion an aligned complete subtree,
so each level is a cache lookup and the recursion descends only the right
spine.

**Measured:** 14.99 ms to **3.3 us**, on a run where every untouched
scenario was slower than before, so the improvement is understated
(`benchmarks/results/2026-09-21-perf01-fix.md`).

**What it cost, recorded because a fix reported without its cost is half a
report:** appends went from 108 ns to 1.1 us, roughly 6-7x once the slower
machine is discounted. That is the amortised cost of hashing interior nodes
as they complete -- work the old code deferred and then repeated per proof.
The trade is right here because appends are internal and batched while
proofs are served on demand, so the expensive operation is now the one an
attacker cannot trigger; and at 1.1 us append is about 0.5% of the 207.4 us
ingest path. Memory roughly doubles.

**Why this is not a second definition of the tree:** `root_of` remains the
reference construction from RFC 6962 section 2.1, and
`crates/duap-provenance/tests/merkle_cache.rs` asserts the cached and
recursive constructions agree on every root, audit path and consistency
path for every tree size in 0..=130 -- the range covering every shape the
split rule can produce. If they ever disagree, the cache is wrong.

**Still required before network exposure:** rate-limiting on
`/v1/log/proof` in the shipped default configuration. 3.3 us is not a
denial-of-service lever, but an unlimited endpoint is still an unlimited
endpoint.

**Owner:** `provenance-engineer`. **Status:** fixed. No longer blocks
PRODUCTION_CANDIDATE for `duap-provenance`; independent review still does.

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

## VS-5 — Nothing prevents a constant pseudonym salt (open)

**Found by:** the vertical-slice review of 2026-09-21.
**Severity:** low as it stands, high if the pattern escapes. The
demonstration uses `DEMO_SALT: [u8; 32] = [0xA5; 32]` so its transcript is
deterministic, which is correct there. A constant salt anywhere else makes
per-controller pseudonyms identical across controllers, which destroys the
only unlinkability property the protocol offers: two controllers could join
their records on the pseudonym alone.

**Current control:** a comment saying real agents must use fresh salts. A
comment is not a control.

**Reproduction:** `crates/duap-demo/src/lib.rs`, `DEMO_SALT`. Constructing
`SubjectPseudonym` with the same salt under two different controller
identities yields values that are equal whenever the subject root is equal.

**Proposed fix:** require a `SaltSource` with no `Default`, no `const`
constructor, and one explicitly named test-only implementation, so the
compiler refuses the mistake rather than a reviewer catching it.

**Owner:** `privacy-engineer`. **Status:** open, blocking any SDK release.

## VS-6 — Two representations of one pricing concept (open)

**Found by:** the vertical-slice review of 2026-09-21.
**Severity:** low now, permanent later. VS-3's fix added
`PricingRule::UnitTable` alongside the existing single-price rule, and the
single-price form is expressible as a one-row table. Every consumer must
handle both, and the conformance vectors must cover both.

**Proposed fix:** absorb `PerUnit` into `UnitTable` before the wire format
is frozen. After freezing, it is permanent.

**Owner:** `economics-engineer`, with `protocol-engineer` for the vectors.
**Status:** open, must close before the L5 vectors are declared stable.

## VS-7 — Speculative machinery in a normative-looking crate (fixed)

**Found by:** the vertical-slice review of 2026-09-21.
**Severity:** low. `crates/duap-auth/src/negotiation.rs` referenced
`formal/Negotiation.tla`, which did not exist, and no slice step negotiates
anything. Sitting in the authorization crate's public surface at a
`PRODUCTION` marker, it implied a protocol interaction that had not been
specified.

**Fix:** the review offered two routes -- write the model, or demote the
module. Both were taken, because they answer different halves of the
finding.

`formal/Negotiation.tla` now models the six states and every transition,
the digest chain binding an offer to its request and an acceptance to its
offer, counter-offers superseding the live offer, replay rejection, and
expiry with the rejection exemption. Six invariants hold exhaustively over
244 distinct states with none left on the queue, and
`Negotiation_NonVacuity.cfg` asserts a grant is unreachable and must fail,
which it does. `check.sh` runs both and gates on the expected outcome of
each.

`crates/duap-auth/tests/negotiation_model_mirror.rs` mirrors the six
invariants by name, plus two behaviours the model abstracts: that a
counter-offer supersedes the previous one so the superseded offer can no
longer be accepted, and that expiry binds every message except rejection.
Eight tests.

The marker moved from `PRODUCTION` to `REFERENCE`, with the reason stated
in the module header: nothing in the vertical slice negotiates, so the
module is exercised by its own tests and not end to end.

**What is still not established:** that the Rust implements the model. The
mirror tests are agreement on specific cases, not a refinement proof, and
`formal/README.md` says so for every model in the repository.

## VS-8 — Maturity markers claim more than the evidence supports (open)

**Found by:** the vertical-slice review of 2026-09-21, auditing crate
markers against `.claude/rules/10-status-discipline.md`.
**Severity:** medium. Eleven crates carry `STATUS: PRODUCTION` in their
module documentation. Rule 10's entry criteria for PRODUCTION are
"independently reviewed; for cryptographic subsystems, independently
audited". No crate in this repository has been independently reviewed and
none has been audited. Rule 10 also forbids PRODUCTION while an open
red-team finding against the subsystem is unresolved, which is separately
violated by `duap-provenance` (PERF-01), `duap-auth` (VS-6, VS-7) and
`duap-model` (VS-5).

This is the failure mode rule 01 exists to prevent — code that looks
finished and is not — appearing in the markers meant to prevent it. The
markers were written subsystem by subsystem, each meaning "complete and
tested", which is the REFERENCE criterion, not the PRODUCTION one.

**Reproduction:**

```
for c in crates/*/src/lib.rs; do grep -m1 "STATUS:" "$c"; done
```

**Fix:** lower every crate to the status its evidence supports, in a
status-only commit as rule 10 requires, and record the register in
`docs/STATUS.md` so the claim lives in one place rather than in fifteen
module headers that drift independently.

**Owner:** `chief-architect`. **Status:** open at the time of the review;
closed by the status-only commit that follows it.

## VS-9 — The conformance vectors were not checked by `cargo test` (fixed)

**Found by:** the vertical-slice review of 2026-09-21, while assessing
whether `duap-conformance` met its claimed status.
**Severity:** medium. The 93 vectors in `spec/vectors/` were checked only by
running `cargo run -p duap-conformance -- check` by hand. A change that
altered a vector left `cargo test --workspace` green. Since a changed vector
*is* a wire-format change, this is the one regression that most needs to
fail loudly and was the one that failed silently.

There is no CI in this repository yet, so "the pipeline would have caught
it" was not available as an answer.

**Reproduction (before the fix):** edit any digest in
`spec/vectors/canonical-encoding.json`; `cargo test --workspace` passes.

**Fix:** `crates/duap-conformance/tests/vectors_test.rs`, three tests:
every committed vector passes the self-check; every committed file is
byte-identical to what the generator produces, with the error message
naming the ADR requirement and the regeneration command; and every claimed
conformance level has at least one vector, so a level cannot be advertised
without being verifiable.

**Remaining gap:** the Go verifier in `gateway/cmd/duap-verify` is still run
by hand. Wiring it in needs a CI workflow with a Go toolchain, which does
not exist yet and is tracked as part of the CI work rather than as a
separate finding.

## CI-01 — Generated code was not formatter-idempotent (fixed)

**Found by:** writing the CI workflow and running its checks over the whole
tree for the first time, 2026-09-21.
**Severity:** low in effect, structural in kind. Rule 06 requires generated
files to be regenerated rather than hand-edited, with the regeneration
checked in CI. The check could not have worked: `ontology/gen_code.py`
emitted valid but unformatted source, `cargo fmt` reformatted
`crates/duap-model/src/taxonomy.rs` in place, and the next regeneration
undid the formatting. A staleness check would have reported a 161-line
diff on every run that touched formatting, which is the false alarm that
teaches people to ignore a check.

The Go emitter had the same defect in a more visible form: the committed
`gateway/internal/taxonomy/taxonomy.go` had never been `gofmt`-clean, which
`gofmt -l` confirms against the pre-fix file.

**Fix:** the generator now runs the language's own formatter over its
output before writing or comparing, so generated output is a fixed point.
A missing formatter is reported and not fatal, because the generator must
still work on a machine with only Python. CI runs
`python3 ontology/gen_code.py --check` and a `gofmt -l` gate.

**Related findings in the same session, both fixed in place:** the
personal-data hook's payment-card pattern used `\b`, which matches after a
decimal point, so the float `2.5560241107087633` in a research results file
read as a card number; and the maturity-marker hook's vocabulary omitted
`PROTOTYPE`, silently disabling itself across four crates (ADR-0015).

**The general lesson, recorded because it generalises past these three:**
all three defects existed because the checks only ever inspected staged
diffs. A check that has never been run over the whole tree has never been
tested against the tree, and the first full run found a defect in every
one of them.
