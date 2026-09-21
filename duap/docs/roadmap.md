# Roadmap

**Status:** REFERENCE (a plan with gates, not a forecast) · 2026-09-21

The directive asks for a ten-year industry roadmap and then says: do not
assume these milestones will occur, and specify measurable gates. This
document takes the second instruction more seriously than the first.

**The honest framing.** Everything past stage 1 is contingent on a
question nobody has asked a real counterparty yet
(`docs/market/go-to-market.md` stage 0). A ten-year plan whose first step
is unvalidated is a plan about a hypothesis, and is written as one here:
each stage has an **exit condition that can fail**, and a stated
consequence of failing. There are no dates, because a date on an
unvalidated stage is a fiction that later gets defended.

## Where the project actually is

| | |
|---|---|
| Protocol | SPECIFIED, wire format not frozen but no longer blocked from freezing |
| Reference implementation | 277 tests, 93 conformance vectors, three model-checked specifications |
| Second implementation | L1–L3 in Go, 77/77, same author |
| Vertical slice | Runs end to end; all eight review decisions closed |
| Findings | 9 fixed, 0 open, 2 accepted risks |
| Adopters | **None. Nobody has been asked.** |

The last row is the one that matters and is the reason for stage 0.

---

## Stage 0 — Find out whether the problem is real

**Work:** ask AI companies and rights-holders operating under RSL
pay-per-inference terms one question: *what do you do today to establish
what is owed?*

**Exit condition:** an answer other than "we estimate and both sides
accept it."

**If it fails:** stop. Falsification report stopping condition 7. This is
the cheapest possible refutation and it should be sought first, not last.

**Measure:** number of counterparties asked; distribution of answers. Not
a vanity metric — three "we estimate" answers from three serious parties
ends the project, and that is worth knowing in a month rather than a year.

---

## Stage 1 — One pair reconciling

**Work:** the RSL binding and the OpenTelemetry collector processor, both
currently UNIMPLEMENTED and both named in `INTEROPERABILITY.md` as the two
that matter. Freeze the wire format.

**Exit condition:** two organisations reconcile one real metered licence
against shared evidence, and at least one of them says the evidence
changed an outcome — a corrected invoice, a closed dispute, an audit
satisfied.

**If it fails:** the accounting object is not the binding constraint.
Publish the specification, contribute what is useful to RSL or IDSA, and
stop building.

**Measure:** disputes raised; disputes resolved from evidence rather than
negotiation; reconciliation time before and after. Note that
`economics/results.md` finding 4 makes the **dispute rate** the single
most valuable unknown in the model — high leverage, no public data — so
stage 1 is also the experiment that resolves it.

---

## Stage 2 — Someone else implements it

**Exit condition:** an implementation this author did not write passes
L1–L5 without help.

**Why this is a stage and not a nice-to-have:** it is the architectural
test in `docs/architecture/minimal-protocol.md` §36 — the kernel must be
implementable without adopting this stack. The Go verifier is evidence and
not proof, because the same author wrote both, and it is stated that way
everywhere the claim appears.

**If it fails after eighteen months:** falsification stopping condition 5.
The kernel is not as small or as clear as claimed, and the specification
needs rewriting rather than promoting.

**Measure:** independent implementations; conformance level reached;
defects they find in the specification. The last is the real metric —
VS-4 was worth more than the 77 passing vectors.

---

## Stage 3 — Netting across counterparties

**Exit condition:** a participant with obligations to several
counterparties says multilateral netting is cheaper than bilateral
settlement, in money or in operational effort.

**Why it matters disproportionately:** `docs/business/business-model.md`
§5 identifies the clearing relationship as the *only* structurally
defensible moat this project has. Every other candidate was assessed and
found weak, largely by design — public vectors and portable evidence
prevent lock-in on purpose.

**If it fails:** the business is a good infrastructure business with no
moat, which is a legitimate outcome and must be said to investors in those
words rather than discovered by them.

---

## Stage 4 — A standards body takes it

**Exit condition:** a work item accepted somewhere that is not this
repository — IDSA, the Linux Foundation, a W3C community group, or the
RSL Collective.

**Why not a foundation of our own:** `GOVERNANCE.md` §3 argues against it.
Standing one up is expensive, its neutrality is nominal until it has
members with divergent interests, and the precedents for single-vendor
foundations are poor. Contributing to a body that already has convening
power is more likely to work.

**If it fails:** the protocol is a product feature, not a standard. It can
still be useful; it cannot be called neutral infrastructure.

---

## What a longer horizon would require, stated as conditions

The directive asks for years five to ten. Rather than invent milestones,
here is what would have to be true for them to exist at all. Each is a
**hypothesis**, not a projection.

| For this to happen | This must first be true |
|---|---|
| Multiple marketplaces using DUAP evidence | Stage 2 — evidence portable between implementations nobody coordinates |
| Cross-industry adoption beyond AI data | Stage 1 in a second vertical, most plausibly industrial telemetry under the EU Data Act |
| Accounting-system integration | Obligations material enough that finance functions care, which requires volume no one has demonstrated |
| Regulatory reference | An authority citing usage evidence in guidance. Not plannable, not to be assumed, and `docs/market/why-not-already.md` warns against a strategy that needs it |
| Settlement infrastructure | Stage 3, plus a legal answer to `docs/legal/regulatory-matrix.md` question 1 |

**What this repository will not do** is state a revenue figure, a market
size or a valuation for these. `economics/results.md` stops at unit
economics deliberately: the product of two assumptions carries no
information, and a ten-year projection is the product of many.

---

## Success metrics

The directive asks for objective metrics and warns against vanity ones.
The distinction applied here: a metric is vanity if it can grow while the
thesis is failing.

### Metrics that indicate adoption

| Metric | Why it cannot be gamed into meaninglessness |
|---|---|
| **Independent implementations passing L1–L5** | Requires someone else to spend real effort on the specification |
| **Specification defects found by others** | Can only rise if someone is reading it adversarially |
| **Counterparty *pairs* reconciling** | Needs two parties who disagree about money to both adopt |
| **Disputes resolved from evidence** | The thing the system is for. Directly measurable, and `economics/results.md` says it dominates cost |
| **Receipts verified by a party other than the issuer** | Distinguishes evidence from record-keeping |
| **Netted volume as a share of gross obligations** | Only rises with genuine multilateral participation |

### Metrics that are vanity here, and why

| Metric | Why it misleads |
|---|---|
| Events ingested | Grows with one enthusiastic adopter's log volume. Measures verbosity |
| Receipts issued | Same. A node can issue receipts nobody reads |
| Organisations "integrated" | Integration without reconciliation is a pilot that went nowhere |
| GitHub stars, downloads | Measure interest, not use |
| Total value "represented" | Can be large while nothing is settled, and invites the take-rate model `docs/business/business-model.md` rejects |

### Metrics for the protocol itself

Measured continuously, not aspirationally: conformance vectors passing
(93), model-check state counts within bounds (15,237 / 1,920 / 244),
fuzzing executions without a finding (31.3M), open findings (0), and
subsystems at each status in `docs/STATUS.md`. These say whether the
artefact is sound. They say nothing about whether anyone wants it, and
must never be reported as though they did.
