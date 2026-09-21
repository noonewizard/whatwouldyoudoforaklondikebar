# Final assessment

**Status:** REFERENCE · 2026-09-21
**The question:** can a Data Rights Economy become a real global industry,
and can DUAP become the neutral interoperability layer that makes data
usage measurable, verifiable, accountable and settleable across
organizational boundaries?

**Evidence limits, first.** The technical claims below rest on code in
this repository that runs, is tested, and can be re-run. The market claims
rest on web-search summaries of pages this environment could not open —
its network policy permitted exactly one source host — and are labelled
throughout. Nobody has been asked whether they want this. Every commercial
statement is inference from public reporting.

---

## The answer

**Not as posed. A narrower version survives, and it is worth building.**

The thesis as stated — a global market where individuals and organisations
license data usage rights, with DUAP as the interoperability layer — does
not survive its own falsification exercise. Four of eleven kill attempts
succeeded. What remains is roughly a tenth the size of the original
concept and is specific enough to test.

### Classification

The directive asks for classification rather than a verdict.

| Dimension | Classification | Basis |
|---|---|---|
| Technical viability | **Viable** | The vertical slice runs end to end; 277 tests, 93 conformance vectors, three exhaustively model-checked specifications, a second implementation in another language |
| Technical constraints | **Constrained, and the constraints are known** | Evidence must aggregate or the economics fail; usage outside instrumentation is invisible; nothing proves a report is true |
| Economic promise | **Uncertain, and unresolvable by engineering** | Turns entirely on whether a buyer will meter its own internal use for a counterparty's benefit. Nobody has been asked |
| Strategic differentiation | **Differentiated but narrow** | Every deployed meter in this market is a trusted third party or a self-report. DUAP's claim reduces to evidence that does not require trusting its producer, for usage no intermediary can observe |
| Strategic redundancy | **Redundant where a gate exists** | RSL owns terms, Cloudflare owns the crawl gate. DUAP should never be proposed where an intermediary can sit in the path |
| Legal constraint | **Constrained for individuals, workable for organisations** | Data-protection rights are inalienable control rights, not property. Seven questions for counsel remain, one of which may determine whether a hosted EU product can exist |
| Standards dependence | **High, deliberately** | Three objects defined against nine standards bound. If that ratio inverts, the argument is lost |
| Adoption dependence | **Total** | Gaia-X is the precedent: specification quality has repeatedly failed to predict adoption in this domain |

---

## What was killed, and by what

Not hedged — these are the parts of the original concept that should not
be built.

**The consumer data marketplace.** Killed twice independently, which is
the strongest signal in the analysis. Reported per-person data value
clusters around $10/year with an ARPU upper bound of a few hundred
[search-summary], while this repository's own benchmarks put the cost of
per-event evidence orders of magnitude above the value of a fractional-cent
transaction. Economics and engineering reached the same verdict from
opposite directions. Datacoup's reported 2019 shutdown needs no additional
explanation.

**Individual data income as a product.** Same arithmetic, plus a rights
problem: for most data associated with a person, that person holds no
transferable economic right to license. A product promising income from it
is misrepresenting the law.

**Attribution-based pricing.** Killed by this project's own measurement.
Defensible estimators agree with exact Shapley at ρ ≈ +0.53 with 33% top-3
overlap — different estimators pay different people, and nothing
adjudicates. The negative result is what makes the remaining design
defensible.

**The AI web-content beachhead.** Killed by competitive research conducted
after the architecture was built. RSL 1.0 standardises the terms including
pay-per-crawl and pay-per-inference; Cloudflare meters the crawl boundary
at reportedly over a billion HTTP 402s per day; a dozen intermediaries
occupy the commercial layer [search-summary]. That market closed while
this protocol was being designed.

---

## What survives

One sentence:

> Signed, sequenced, cross-checkable usage evidence for data use occurring
> **inside a counterparty's infrastructure**, where no intermediary can sit
> in the path, linked to the licence that authorized it and aggregated into
> an obligation both parties can reconcile.

The concrete instance is **pay-per-inference**. RSL standardises the term.
Parties are reportedly agreeing it. Inference happens inside the AI
company, where no crawl gate, CDN or intermediary can observe it — so
nothing deployed can measure what is owed under a term an official
standard already defines. That is a named, currently-unserved gap rather
than a category of opportunity, and it is small enough to test in weeks.

### Why the technical thesis held

The parts that survived did so by refusing claims, not by making them:

- Measurement stayed separate from valuation, so a dispute re-priced
  0.01 EUR without re-ingesting an event.
- The receipt publishes its own limits as machine-readable data — five
  established claims and five explicitly not — so tooling surfaces them
  without anyone reading a document.
- Inclusion is claimed; influence and economic contribution are refused.
  That refusal turned out to align with what EU AI Act art. 53(1)(d) asks
  for [search-summary].
- No blockchain. Nothing in eighteen end-to-end steps required consensus
  among mutually distrusting parties.

### Why "verifiable" is not overstated here

The protocol detects; it does not prevent. A clearing node can fabricate
(RISK-01). Usage outside instrumentation is invisible (RISK-02). A
transparency log detects equivocation *given auditor gossip*, and no
auditor is implemented — which three separate operational documents
independently identified as the binding gap. What is actually delivered is
non-repudiation, gap detection and cross-reporter double-count detection.
That is worth money in a dispute and is not proof.

---

## The two things that could still kill it

Neither is technical, which is the finding.

**Will a buyer meter its own internal use for a counterparty's benefit?**
A buyer asked to make itself auditable has an obvious reason to decline,
and in a concentrated market it can. The counter-argument is the FOCUS
precedent — the concentrated side adopted a common schema because
integrating N counterparties cost more than standardising — but FOCUS
normalised a report providers already had to produce. DUAP asks for one
they currently do not. Unresolved.

**Is there a governance structure both neutral enough to be adopted and
funded enough to be maintained?** A company-owned protocol is not neutral.
A foundation with no engineering capacity publishes documents and waits.
`GOVERNANCE.md` argues against creating a new foundation and for
contributing to a body that already has convening power — which is an
argument against this project owning the thing it built.

---

## The minimum viable path

Five steps. The first is not engineering.

1. **Ask.** Put one question to parties operating under RSL
   pay-per-inference terms: what do you do today to establish what is
   owed? If the answer is "we estimate and both sides accept it", stop.
2. **Bind, don't build.** The RSL binding and the OpenTelemetry collector
   processor. Both are UNIMPLEMENTED and both are bindings to things
   adopters already have. Freeze the wire format.
3. **One pair reconciling.** Two organisations, one real metered licence,
   shared evidence — and evidence that changed an outcome. This is also
   the experiment that resolves the dispute rate, which the economic model
   identifies as high-leverage and entirely unmeasured.
4. **Someone else implements it.** The kernel must be implementable
   without adopting this stack. The Go verifier is evidence, not proof:
   same author.
5. **Give it away.** A work item accepted by a body that is not this
   repository.

Each step can fail, and `docs/roadmap.md` states what failure means rather
than how to route around it.

---

## The uncomfortable summary

The engineering worked. The vertical slice found four defects, the second
implementation found a fifth that only a second implementation could, and
a coverage-guided fuzzer found a ninth in under two minutes that property
tests, 269 unit tests, a second implementation and three formal models had
all missed. Every one is fixed and every one is recorded with its
reproduction.

The economics did not work as posed. The consumer thesis failed to
arithmetic. The beachhead failed to a market that solved the problem
differently, with a trusted intermediary, while this was being designed —
and that solution is *good enough*, which is the part worth sitting with.
"Cryptographically verifiable" has lost to "already deployed and nobody is
lying yet" many times.

What is left is a real gap, precisely stated, in a market that exists. It
is much smaller than a global data economy. It is testable in weeks rather
than years, and the test is a conversation rather than a codebase.

**The most useful thing this repository produced is not the protocol. It
is the set of things it establishes cannot be done** — attribution cannot
ground payment, individuals cannot be paid enough to cover the cost of
paying them, revocation cannot un-train a model, and no signature makes a
self-report true. Each of those is written into the code as a refusal
rather than into a document as a caveat, which is the only form in which
such a limit survives contact with a product manager.
