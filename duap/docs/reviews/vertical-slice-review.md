# Vertical-slice review

**Status:** REFERENCE (review record) · 2026-09-21
**Subject:** the first end-to-end slice, `crates/duap-demo`, commit range
`7e0615b..fad9633`
**Reviewers:** `chief-architect`, with findings from `red-team-engineer`,
`protocol-engineer`, `provenance-engineer`, `economics-engineer` and
`interop-engineer`
**Gate:** this is the review required before any further subsystem is
built. Nothing in `infrastructure/`, `sdk/python`, `sdk/typescript`,
`simulations/` or `dashboard/` may be implemented until the decisions below
are recorded and acted on.

## Resolution status, added 2026-09-21 after the review

This review is a point-in-time record and its body is not edited. What has
happened to its eight decisions since:

| Decision | Status |
|---|---|
| 1. The slice passes the gate | Recorded |
| 2. Fix PERF-01 before network exposure | **Done.** 14.99 ms → 3.3 µs (`benchmarks/results/2026-09-21-perf01-fix.md`), and the proof endpoints now carry a separate finite budget in the shipped default |
| 3. Absorb `PricingRule::PerUnit` before the wire format freezes | **Done.** ADR-0016; vectors regenerated, golden transcript moved by two digest lines and no economic figure (VS-6) |
| 4. Type-level control on the pseudonym salt | **Done.** `RootSecretSource`, a feature-gated `InsecureFixedSecret`, and a checker on the opt-in list (VS-5) |
| 5. Demote or model `negotiation.rs` | **Done.** Both: `formal/Negotiation.tla` written and checked, marker lowered (VS-7) |
| 6. Coverage-guided fuzzing of the decoder | **Done, and it was right.** Six targets; the first found a nine-byte denial-of-service defect in two minutes (FUZZ-01) |
| 7. Redesign the obligation context interface by ADR | **Done.** ADR-0017: an `EvalFacts` trait with a three-valued `Fact<T>`, defaulting to `Unavailable` so a caller fails closed by construction |
| 8. No subsystem advances while a blocker stands | In force; `docs/STATUS.md` is the register |

Two findings were raised *by* this review and are also closed: VS-7 above,
and VS-8, the maturity markers, corrected in a status-only commit.

---

## What the slice actually is

One synthetic subject, one controller (`org:duap/acme-example-corp`), one
processor, one clearing node, one buyer, one unlisted broker, and eighteen
steps that run in a single process with no network and no external service.
Its output is a golden transcript
(`crates/duap-demo/tests/golden/transcript.txt`) diffed on every test run,
so a behavioural change anywhere in the stack breaks a test rather than
drifting quietly.

It exercises the full abstraction — collect, authorize, identify, classify,
measure, process, derive, transfer, monetize, settle — and ends with a
balanced ledger, six anchored receipts, a settled dispute, and a payout of
2.28 EUR released after eleven simulated periods crossed a 1.00 EUR
threshold. 8 operations were accepted and 6 refused. An independent
verifier re-checked all six receipts holding nothing but the node's public
key.

That is the object under review. Everything below is about whether it was
the right thing to build.

---

## 1. What worked

### The kernel is smaller than the implementation, and provably separable

The strongest result is negative in form: `gateway/`, a Go implementation
of conformance levels L1–L3 written against `specs/protocol-v0.1.md`
rather than translated from the Rust, passes all 77 vectors in its range
and shares no code with the Rust stack. The directive's defining
architectural test (§36 — "small enough that an independent organization
could implement it without adopting our entire software stack") has been
met for three of five levels by construction rather than by assertion.

The bound on that claim matters and is stated in
`docs/architecture/minimal-protocol.md` §10: the same author wrote both, and
L4–L5 have no second implementation. It is evidence of independent
implementability, not proof of it.

### Canonical encoding held under adversarial input

`duap-canon` is the layer everything else rests on, and it survived the
attacks aimed at it. The property tests establish round-trip fidelity and
encoder/decoder agreement: the strict decoder rejects every byte sequence
the encoder would not emit, including non-minimal integer encodings,
out-of-order map keys, indefinite-length items and duplicate keys. The
length-first key ordering (ADR-0002) is mechanically checkable and the Go
implementation reproduced it from prose alone.

Domain separation did its job. `H("DUAP/1" || 0x00 || domain || 0x00 ||
payload)` makes the (domain, payload) pair injective, and the attempt to
present a grant digest where an event digest was expected fails at the
domain byte rather than at a type check that could be forgotten.

### Separating measurement from valuation was correct and is load-bearing

The directive made this separation mandatory (§20) and it turned out to be
the decision that keeps the system honest. `duap-meter` produces counters
that state what happened; `duap-valuation` converts counters to money using
coefficients that are explicitly labelled parameters, not discoveries.
Because nothing in the metering path knows a price, the invoice can be
recomputed under a different policy from the same evidence — which is
exactly what the dispute step does, adjusting 0.01 EUR without touching a
single event.

Had these been fused, the dispute would have required re-ingesting events,
and the system would have implied that its prices were measurements.

### Exact integer money conserved value across every operation

Eleven periods of accrual, largest-remainder apportionment, netting,
rounding to a payable amount, and a settled dispute, and the trial balance
is zero at every checkpoint with a residue of 0.000000000 minor EUR
outstanding. `Precise::round_to_money` returning the residue rather than
discarding it (ADR-0008) is the mechanism; the conservation invariant
(INV-L1) is checked in both the Rust tests and the TLA+ accounting model.

No floating point appears anywhere on the money path.

### The receipt publishes its own limits

`Receipt::claims()` returns ten claims as data — five established, five
explicitly not established. The audit step of the transcript prints "claims
established=30, claims explicitly not established=30" across six receipts.
A verifier cannot read the receipt without also reading what it refuses to
prove. This is the mechanism that keeps RISK-01 from being quietly forgotten
in prose.

### Model checking found the right shape of bug

Restructuring the invariants to quantify over decisions computable in the
current state, rather than over an accumulated decision log, took the
authorization model from 20M+ states to 15,237 and made it exhaustive. The
non-vacuity check (`NoPermitIsEverReachable`, which must *fail*) guards
against the classic formal-methods self-deception of proving properties of a
system that can never do anything.

---

## 2. What failed

Four functional defects, one performance defect, and one process defect.
Three of the four functional defects were found by running the slice; the
fourth was found by the second implementation, and that distinction is the
most useful thing in this section. VS-5, VS-6 and VS-7 are raised in §5 and
§6 below as design consequences rather than as defects in behaviour, and are
in the findings register under those numbers.

| ID | Where | How it was found |
|---|---|---|
| VS-1 | `duap-receipt` | building the slice |
| VS-2 | `duap-clearing` | first end-to-end run |
| VS-3 | `duap-auth` | first end-to-end run |
| VS-4 | `duap-canon` | the Go implementation |
| PERF-01 | `duap-provenance` | the benchmark run |
| VS-8 | every crate | auditing markers against rule 10 during this review |

### VS-1 — the anchor was inside the thing it anchored

The transparency log commits to a receipt's digest; attaching the resulting
inclusion proof to the receipt changed that digest. Every anchored receipt
failed verification. This is a circular-dependency error that no amount of
design review had caught, because on paper "the receipt is anchored in the
log" is obviously true and says nothing about what is inside the hash.

The fix — `Receipt::core()`, hashing the anchor-free core so the anchor
becomes a detachable proof *about* the receipt — is strictly better than
what was designed, because a holder can now upgrade to a fresher proof
without changing the receipt's identity. The slice produced a better design
than the specification did.

### VS-2 — an obligation that could never be satisfied

Any grant carrying `MaxDerivationDepth` denied every deriving operation,
because the authorization evaluator is a pure function with no access to the
provenance graph and nothing was supplying the depth. Failing closed meant
the feature was *safe* and *useless*, which is the failure mode that survives
unit testing: every test of the evaluator passed.

The fix keeps the evaluator pure — the clearing node computes depth from its
graph and passes it in `EvalContext`. The security-relevant detail is that
an input the node has never seen is treated as depth 0 rather than unknown,
so withholding an upstream event cannot evade a depth obligation.

### VS-3 — one price per term could not cover a multi-operation term

A term spanning operations metered in records, queries and inferences could
carry only one per-unit price. Real grants routinely span such operations,
so the data model was wrong rather than merely incomplete. `PricingRule::UnitTable`
and unit-aware rule selection fixed it.

### VS-4 — the defect only a second implementation finds

`Nint(u64::MAX)` renders in the JSON view as `{"$n64":"-18446744073709551616"}`,
magnitude 2^64, which does not fit an unsigned 64-bit integer. Every
conforming implementation would have needed arbitrary-precision arithmetic
for exactly one value that no DUAP object ever uses. The Rust never noticed
because `i128` made it free.

Negative integers are now bounded at `i64::MIN` across the codec, the JSON
view, the serde bridge and the property generators in both implementations.

This finding is the entire argument for `gateway/` existing. A port of the
Rust would have reproduced the requirement instead of objecting to it.

### PERF-01 — the proof endpoint is a denial-of-service lever (open)

Generating one audit path in a 100,000-entry tree takes 14.99 ms; verifying
the same path takes 2.5 µs. `MerkleLog::inclusion_proof` recomputes sibling
subtree roots on demand. In a reference implementation this is a curiosity;
on the gateway's `/v1/log/proof` endpoint it lets an attacker spend 15 ms of
server time per cheap request.

Not fixed. Open against `provenance-engineer`, blocking PRODUCTION_CANDIDATE
for `duap-provenance`. The fix is known (cache complete-subtree roots as
entries are appended — the incremental accumulator in
`duap-meter::evidence` already maintains exactly these) and was deliberately
not applied inside the slice, because applying it would have mixed an
optimisation into the correctness work the slice exists to validate.

### VS-8 — the markers meant to prevent overclaiming were overclaiming

Eleven crates carry `STATUS: PRODUCTION`. Rule 10 defines PRODUCTION as
"independently reviewed; for cryptographic subsystems, independently
audited". Neither has happened for anything here. Each marker was written
while finishing that subsystem and meant "complete and tested" — which is
the REFERENCE criterion.

Three crates additionally hold open findings, which rule 10 forbids at
PRODUCTION outright: `duap-provenance` (PERF-01), `duap-auth` (VS-6, VS-7),
`duap-model` (VS-5).

The lesson is narrow and worth stating: a per-subsystem marker written by
whoever finished that subsystem drifts toward optimism, because the author
is grading their own work against a criterion they are reading charitably.
The register in `docs/STATUS.md` exists so the claim is made once, in one
place, against the criteria table.

---

## 3. What assumptions were wrong

### "Authorization can be evaluated from the grant and the event alone"

This was the assumption behind making `evaluate()` a pure function, and it
was wrong for the whole class of obligations that quantify over history:
derivation depth, cumulative volume caps, rate limits, retention windows.
VS-2 was the first instance; the others are latent.

The correction taken is not to give the evaluator a database. It is to
classify obligations by what they need, and to make the caller responsible
for supplying it:

| Class | Decidable from | Status |
|---|---|---|
| Checkable | grant + event | implemented |
| Context-dependent | grant + event + caller-supplied context | implemented for depth only |
| Deferred | observation after the decision | recorded, not enforced |
| Economic | settlement | priced, not enforced |

The middle row is the honest statement of where the protocol now is: the
mechanism exists, one obligation uses it, and every other history-dependent
obligation is unimplemented rather than working. This is recorded in
`specs/protocol-v0.1.md` and must not be described as complete.

### "Revocation is a state change on a grant"

Modelling revocation as an epoch transition is correct for *future*
authorization and says nothing about what has already happened. The slice
made this concrete in a way the design did not: step 12 revokes AI-training
permission, training after the effective time is refused, service use
continues, and the transcript prints `already_trained_model=NOT_UNLEARNED`.

The assumption that needed killing was the implicit one that a
`DeleteSourceAndDerived` retroactive policy *means* the derived artefact is
gone. It means an obligation to delete was recorded and is checkable only by
observing the controller's subsequent behaviour. The directive warned about
this (§24) and the slice is where the warning became a printed line in the
output rather than a paragraph in a document.

### "A per-unit price is the natural pricing primitive"

VS-3. The natural primitive is a price *table keyed by unit*, because a
permission is granted over a purpose and a data class, and the operations
that serve one purpose are metered in different units. The original model
came from thinking about pricing before thinking about grants.

### "The demonstration needs randomness to be realistic"

Early versions seeded event identifiers from a random source, which made the
golden transcript unusable. `EventId::for_sequence(stream, index)` replaced
it. The wrong assumption was that realism required non-determinism; what it
required was a realistic *structure*, and determinism is worth more than
variety because it turns the whole slice into a regression test.

### An assumption that survived: no blockchain

The directive told us not to reach for a blockchain reflexively (§16) and
the slice confirms the transparency log was sufficient. Nothing in eighteen
steps needed consensus among mutually distrusting parties over a shared
state machine. What was needed — append-only evidence with inclusion and
consistency proofs — RFC 6962 provides at 108 ns per append against
9.2M appends/s, and 129 bytes per entry.

The honest limit, stated in ADR-0005 and in `SECURITY.md`: a transparency
log *detects* equivocation given gossip between auditors. It does not
prevent it. No step of the slice demonstrates the gossip, because no gossip
is implemented.

---

## 4. What cannot be proven

This section exists because the directive's §45 forbids "this is secure"
without a security argument. These are the claims the system cannot make,
and the reason each fails.

### That the described operations actually took place

RISK-01, accepted. The clearing node signs the receipt, so the node can
fabricate usage or omit it. The mitigations are real but partial: receipts
commit to an event-set root a subject can demand and check; anchoring
prevents back-dating; consistency proofs make selective history rewriting
detectable. Fabrication and omission remain detectable only by cross-checking
against a party holding independent records, and no such party exists in the
slice.

### That all usage was reported

RISK-02, accepted. An organisation that copies data out of an instrumented
system and processes it elsewhere generates no events. Per-stream sequence
numbers make *partial* suppression visible; complete non-participation is
invisible. This is a property of the problem. No cryptography addresses it,
and a document claiming otherwise would be lying.

### That the Rust implements the model

The TLA+ models are checked exhaustively within stated bounds (15,237 states
for authorization; 1,920 for accounting). Nothing is extracted from or
verified against the source. The mirror tests in
`crates/duap-auth/tests/model_mirror.rs` and
`crates/duap-ledger/tests/model_mirror.rs` are agreement on specific cases,
not a refinement proof. The defensible sentence is "the authorization and
accounting algorithms are model-checked; the implementation is tested
against the same invariants", and `formal/README.md` says exactly that.

### That pseudonyms are unlinkable

Per-controller pseudonym derivation gives unlinkability *against identifiers*
and nothing else. Two controllers cannot join on the pseudonym. They can
trivially join on content, timing, or any quasi-identifier in the payload,
and the protocol does not see payloads. `PRIVACY.md` states the mechanism and
this limit in the same paragraph, per rule 03.

### That a subject's payment reflects their contribution

`research/ai-attribution/RESULTS.md`, five findings, all measured. The
decisive one: leave-one-out and the influence function agree with each other
at Spearman ρ = +0.947 and with exact Shapley at ρ ≈ +0.53, with top-3
overlap of 33%. Choosing among them changes who gets paid the most. There is
no principled basis in the literature or in this experiment for preferring
one, so the protocol prices *metered usage*, which is observable, and makes
no claim about contribution, which is not.

### That the benchmarks describe a production system

Every figure in `benchmarks/results/2026-09-21-ci-runner.md` is
single-threaded, in-memory, on a shared cloud runner, with no network and no
durable storage. Ingest is 7,829 events/s on one core. Nothing in this
repository multiplies that by a core count, and the size arithmetic (665
bytes/event → 665 GB at 10^9 events) is labelled as arithmetic.

### That anything here is audited

No independent security audit. No coverage-guided fuzzing has been run
against the decoder, which is the single most valuable missing test given
that `duap-canon` parses adversary-supplied bytes. This is the largest
known gap in the evidence base and is the first item in §7 below.

---

## 5. What should be redesigned

### `MerkleLog::inclusion_proof` — required before any network exposure

PERF-01. Cache complete-subtree roots on append. The accumulator in
`duap-meter::evidence` is already the right structure and is proven
byte-identical to the recursive construction, so this is a substitution
rather than a new design. Until it lands, `/v1/log/proof` must be rate-limited
separately from the rest of the gateway, and that limit must be in the
gateway's default configuration rather than in its documentation.

### Obligation evaluation needs the context interface generalised

VS-2 was fixed by adding one field to `EvalContext`. Adding a field per
history-dependent obligation does not scale and will produce a context
struct that every caller must populate correctly or fail open. The
redesign: a `ContextProvider` trait the evaluator queries lazily, with a
default implementation that returns "unavailable" and an evaluator that
denies on unavailable. This keeps the evaluator pure with respect to its
inputs while making the failure mode explicit at the call site rather than
implicit in a `None`.

This is an ADR, not a patch. It changes a protocol-adjacent interface and
needs `protocol-engineer` and `chief-architect` agreement before code.

### Canonicalisation cost should be re-examined, not yet changed

11.1 µs per event, 9% of ingest, because the serde bridge encodes with a
general CBOR codec and then normalises. A direct canonical serialiser would
remove most of it and would introduce a second definition of the encoding —
the exact thing ADR-0002 exists to prevent. The recommendation is to leave
it and revisit only if a measured workload makes 9% matter, because a second
encoder is a second place for the encoder/decoder agreement property to
break, and that property is the foundation of everything else.

Recorded as a deliberate non-change so it is not silently "optimised" later.

### Dispute resolution is under-specified at the protocol level

The slice settles a dispute and produces a correct adjustment, but the
weighing of "verifiable evidence" (3) against "assertions" (1) is a policy
the reference implementation chose. `duap-ledger::dispute` should expose the
evidence classification and leave the weighing to a pluggable policy, and
`specs/protocol-v0.1.md` should specify only the classification and the
requirement that adjustments be reversing entries.

---

## 6. What should be removed

### Nothing from the kernel

No object, field or rule in the five-object kernel was found to be
unnecessary by the slice. Each is exercised. That is a weak result — a
single scenario cannot demonstrate minimality — but no candidate for removal
emerged.

### `PricingRule::PerUnit` should be absorbed, not kept alongside `UnitTable`

VS-3 added `UnitTable` next to the existing single-price rule. Two
representations of the same concept is one too many, and the single-price
form is expressible as a one-row table. Keeping both means every consumer
handles two cases and the vectors must cover both. Absorb `PerUnit` into
`UnitTable` before the wire format is frozen; after freezing, it is
permanent.

This is the clearest instance of the slice leaving a seam that must be
closed now because it becomes un-closable later.

### The `Negotiation` module should be demoted, not deleted

`crates/duap-auth/src/negotiation.rs` references a `formal/Negotiation.tla`
that does not exist, and no step of the slice negotiates anything. It is
speculative machinery for a protocol interaction that has not been
specified. Either the model is written and a slice step exercises it, or the
module is marked `EXPERIMENTAL` and moved out of the authorization crate's
public surface. It must not sit in a `PRODUCTION-CANDIDATE` crate implying
it is part of the protocol.

### The demonstration's hard-coded salt must never be promoted

`DEMO_SALT: [u8; 32] = [0xA5; 32]` is correct in a deterministic
demonstration and catastrophic anywhere else — a fixed salt makes
per-controller pseudonyms linkable across controllers, destroying the only
unlinkability property the protocol offers. The comment says so. A comment
is not a control. Before any SDK ships, pseudonym derivation must require a
salt from a `SaltSource` that has no default and no const constructor, so
the compiler refuses the mistake.

Filed as a finding rather than left in this review.

---

## 7. What should become protocol-level, and what should stay implementation-specific

The test applied: *would two independent implementations that disagree about
this produce incompatible evidence?* If yes, it is protocol. If they would
merely behave differently while producing evidence each can verify, it is
implementation.

### Protocol-level (normative, in `specs/protocol-v0.1.md`)

| Thing | Why |
|---|---|
| Canonical encoding and the strict decoder's rejection rules | Two encoders that disagree produce different digests for the same object; every signature breaks |
| Domain-separation labels and the digest construction | A label mismatch makes signatures unverifiable across implementations |
| The signature input structure `duap.sig-input.v1` | Same |
| The five kernel objects' wire fields and their names | Same |
| Suite identifiers and the hybrid concatenation order | A reversed concatenation is an unverifiable signature |
| Key identifier derivation | Self-certifying identifiers only work if derived identically |
| Merkle leaf/node prefixes (0x00/0x01) and proof construction | Proofs must verify across implementations |
| Deny-overrides with obligation union | Two nodes reaching opposite decisions on one grant is the worst possible interoperability failure |
| The purpose lattice's descent rule | Determines whether a decision is Permit or Deny |
| Grant epochs and the epoch hash chain | Determines which grant version a receipt refers to |
| Rounding mode (half-even) and the residue-posting rule | Two implementations that round differently produce different invoices from identical evidence |
| Receipt claim semantics — what a receipt does and does not establish | A receipt whose meaning varies by implementation is worthless as evidence |
| Obligation classification (Checkable / Context-dependent / Deferred / Economic) | Determines whether a decision may be made at all |

### Implementation-specific (not normative)

| Thing | Why |
|---|---|
| Storage engine and schema | Evidence is verifiable from the objects; how they are stored is invisible |
| The nine-stage ingest pipeline's internal ordering | Observable behaviour is the decision and the receipt, not the stages |
| Aggregation window sizes and period boundaries | A policy choice; the receipt states the period it covers |
| Multiplier coefficients and every valuation parameter | Explicitly parameters, not discoveries — §20's separation depends on these *not* being protocol |
| Auction mechanism | One optional settlement mechanism among several |
| Micropayment threshold and dust handling | Policy, as long as conservation holds and the residue is stated |
| Netting strategy | The ledger's invariants constrain the result; the path does not matter |
| HTTP surface, endpoint names, transport | Nothing in the evidence depends on them |
| Double-count arbitration policy (`controller_wins`) | Must be *stated* in the receipt; need not be uniform |
| Dispute evidence weighing | Per §5 above, classification is protocol, weighing is not |
| Pseudonym root-secret custody | The derivation is protocol; where the secret lives is operational |

### One case that is genuinely undecided

**Whether the operation fingerprint used for double-count detection is
protocol-level.** It is currently reporter-independent by construction, which
means two implementations that compute it differently would fail to detect a
double count across an organisational boundary — an argument for protocol.
Against: it is a heuristic, its false-positive behaviour is not
characterised, and freezing a heuristic into a wire format is how protocols
acquire permanent mistakes.

Recommendation: specify it as OPTIONAL with a defined construction, so
implementations that compute it agree, and implementations that do not are
still conforming. Requires an ADR. Not resolved by this review.

---

## Decisions this review records

1. **The slice passes the gate.** The architecture survived contact with an
   end-to-end run; the four defects it exposed were fixed, and the fixes
   improved the design in three of four cases.
2. **PERF-01 must be fixed before the gateway is exposed to a network.**
   Not before further protocol work, but before any deployment artefact is
   built.
3. **`PricingRule::PerUnit` is absorbed into `UnitTable` before the wire
   format is frozen.** This is the only change in this review that becomes
   impossible if deferred.
4. **Pseudonym salt derivation gets a type-level control** before any SDK
   ships.
5. **`negotiation.rs` is demoted to `EXPERIMENTAL`** or given a model and a
   slice step. It does not remain implicitly normative.
6. **Coverage-guided fuzzing of `duap-canon`'s decoder is the highest-value
   missing test** and precedes new subsystems.
7. **The obligation context interface is redesigned by ADR**, not by adding
   fields.
8. **Every crate's maturity marker is lowered to the status its evidence
   supports.** Eleven crates claim `PRODUCTION`; none has been
   independently reviewed and none audited, which are rule 10's entry
   criteria. Recorded as VS-8 and corrected in the status-only commit that
   follows this review.
9. **No subsystem advances past its current status** in `docs/STATUS.md`
   until the item above that blocks it is closed.

## What this review does not cover

Concurrency, durability, network behaviour, key custody in practice,
operational failure modes, multi-node clearing, cross-jurisdiction conflict
of law, and every economic question that requires participants rather than
simulation. None of these was in the slice; none is assessed here; and the
absence of a finding about them is not evidence that they are sound.
