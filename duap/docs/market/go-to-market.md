# Go-to-market

**Status:** REFERENCE · 2026-09-21
**Owner:** `venture-strategy`, reviewed by `chief-architect`
**Supersedes:** any earlier plan naming AI web-content licensing as the
entry point. That beachhead was removed by kill attempt 11 in
`docs/reviews/falsification.md`.

## 1. The constraint set this plan must satisfy

Everything that follows is bounded by findings established elsewhere in
this repository, not by preference. The plan exists to fit inside them.

| Constraint | Source |
|---|---|
| Not consumers. Not individuals. Not a wallet | falsification §2, §8 |
| Not where a gate can sit — a trusted intermediary is cheaper and deployed | falsification §11 |
| Evidence must aggregate; per-event receipts are uneconomic | falsification §4; `economics/results.md` finding 2 |
| Inclusion, never influence | `AI_ATTRIBUTION.md` |
| Bind RSL, ODRL, the Dataspace Protocol; define only the usage object | `docs/standards/prior-art.md` |
| Infrastructure cost is not the constraint; adoption is | `economics/results.md` finding 1 |
| Human dispute cost dominates machine cost | `economics/results.md` finding 3 |

## 2. The beachhead: pay-per-inference accounting

**The claim.** Really Simple Licensing 1.0 standardises a
**pay-per-inference** compensation term. Parties can express it, and
reportedly are adopting RSL. Nothing deployed can measure it, because
inference happens inside the AI company's infrastructure, where no crawler
gate, CDN or intermediary can observe it. Cloudflare's meter stops at the
crawl boundary; TollBit counts pages; ProRata apportions ad revenue. None
of them sees an inference.

So there is a named, standardised, commercially agreed term with **no
accounting mechanism at all**. That is not a market DUAP has to create. It
is an obligation that already exists and currently gets settled by
estimate, assertion or nothing.

**Why it fits every constraint in §1.** It is enterprise-to-enterprise. No
gate can sit in the path, which is the precondition for DUAP being the
right tool rather than an expensive one. Inference counts aggregate
naturally into periodic counters. The claim required is inclusion — *this
content was retrieved and used to ground this many responses* — not
influence. And the integration is a binding to RSL rather than a competing
licence format.

**Why it is uncomfortable, stated plainly.** It requires an AI company to
meter and sign its own internal usage for a counterparty's benefit. That is
kill attempt 7, unresolved, and this plan does not resolve it — it
proposes the cheapest available test of it.

### The specific first experiment

One AI company, one publisher or rights-holder group, one RSL licence with
a pay-per-inference term, and the question: *what do you currently do to
establish what is owed?*

Three possible answers, each of which is worth knowing:

1. **"We estimate and both sides accept it."** Then there is no problem to
   solve, stopping condition 7 in the falsification report is met, and the
   project stops. This is the cheapest possible refutation and should be
   sought first, not last.
2. **"We report a number and they have to trust it."** Then the product is
   a signed, sequenced, gap-detectable version of the number they already
   send — a small integration, a real improvement, and a wedge.
3. **"It is a source of dispute."** Then `economics/results.md` finding 3
   applies directly: dispute cost dominates everything, and shared evidence
   is worth more than its infrastructure cost by orders of magnitude.

**This is a research question before it is a sales motion**, and the
correct next action for this project is to get that answer, not to write
more code.

## 3. Why not the alternatives

Assessed against the constraints in §1 rather than scored arbitrarily.

| Candidate | Why not |
|---|---|
| AI web-content crawl licensing | Closed. RSL owns terms, Cloudflare owns the gate (falsification §11) |
| Consumer data wallet | Removed. Unit economics and rights (falsification §2, §8) |
| Data marketplaces | They are venues; accounting is internal to the venue and they experience no pain from its absence (`docs/market/competitive-landscape.md` §3) |
| Industrial telemetry / IoT | Genuine candidate. The EU Data Act's connected-product access and compensation provisions, applicable from 12 September 2025, create a real obligation to make data available on reasonable terms. Deferred rather than rejected: the regulatory driver is real but the commercial urgency is UNKNOWN to this author, and it is a slower cycle than the AI vertical |
| Scientific / research data | Low willingness to pay, high rights complexity, no budget holder |
| Healthcare | Highest rights complexity, longest sales cycle, worst possible first market for an unaudited protocol |
| Enterprise internal data governance | Not a DUAP problem. Intra-organizational usage tracking has products; the boundary crossing is what DUAP is for |

Industrial telemetry under the EU Data Act is the strongest second
candidate and should be re-examined once the §2 question is answered.

## 4. Adoption mechanics

**The integration must cost hours, not quarters.** The measured facts
support it — ingest is 127.7 µs per event and infrastructure cost is under
a dollar per customer per month at base assumptions — but the binding
constraint is engineering effort at the adopter, not compute.

Two integration paths, in priority order:

1. **An OpenTelemetry collector processor** that turns spans into signed
   DUAP events. For any organisation already emitting OTel, this is a
   configuration change rather than an instrumentation project. This is the
   single highest-leverage unbuilt item in the repository and is currently
   UNIMPLEMENTED (`docs/standards/prior-art.md` §10).
2. **An RSL binding**: derive a DUAP grant from an RSL licence document so
   that terms already published as RSL become machine-enforceable and
   machine-accountable without re-expressing them. Also UNIMPLEMENTED.

Both are bindings to things adopters already have. Neither asks anyone to
adopt a new rights language, which is the mistake that would guarantee
failure.

## 5. Sequence

Each stage has an exit condition that can fail.

| Stage | Work | Exit condition |
|---|---|---|
| 0 — now | Answer the §2 question with real counterparties | An answer other than "we estimate and both sides accept it" |
| 1 | RSL binding; OTel processor; fix PERF-01 before any network exposure | One counterparty pair reconciling a real metered licence |
| 2 | Second independent implementation by someone else | Interop against the L1–L5 vectors without this author's help |
| 3 | Multi-counterparty netting | Netting demonstrably cheaper than bilateral settlement for a real participant |
| 4 | Standardisation via an existing body | A work item accepted somewhere that is not this repository |

**Stage 0 gates everything.** Building stage 1 before stage 0 answers is
the specific failure the vertical-slice review gate exists to prevent, one
level up.

## 6. What this plan does not claim

- That DUAP becomes a global standard. Stage 4 is a hypothesis with no
  evidence behind it.
- That the AI vertical is large enough alone. Reported AI data licensing
  market figures are secondary-source and were not verifiable from this
  environment.
- That adoption follows from quality. Gaia-X is the counter-example and
  `docs/standards/prior-art.md` treats it as a precedent rather than a
  competitor.
- That anyone has been asked. **No potential adopter, counterparty or
  buyer has been contacted.** Every commercial statement in this document
  is inference from public reporting, and the plan's first stage exists
  because of that.
