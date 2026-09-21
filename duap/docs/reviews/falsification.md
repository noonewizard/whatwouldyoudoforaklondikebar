# Falsification report

**Status:** REFERENCE · 2026-09-21
**Owner:** `red-team-lead`, with the economic, standards, legal, privacy and
competitive red teams
**Instruction this document follows:** "Do not protect the thesis. Destroy
it if necessary."

**Evidence health warning.** This document argues from market figures,
and on 2026-09-21 this environment's network policy blocked outbound HTTPS
to every source host except `github.com`. One source — the IDS Dataspace
Protocol specification repository — was fetched and read. Every other
figure below reached this document through a web-search tool's generated
summary of pages it could not open. Those are marked **[search-summary]**,
and the URL beside one is where the claim came from, not a document this
author read; a search summary can misattribute a number between sources or
drop a qualifier. The verdicts are written so that none of them turns on
the precision of such a figure — kill attempt 2 succeeds whether the number
is $10 or $200 per person per year — but every one must be checked against
primary sources before it is relied on commercially.
`docs/standards/prior-art.md` states the same constraint for the standards
analysis.

This document tries to kill DUAP. It is organised as a set of kill
attempts, each stated as a proposition that, if true, ends the project or
forces it to become something else. Each is argued as strongly as the
evidence allows before it is answered — and three of them are not
answered, because they are correct.

## Verdict up front

| # | Kill attempt | Verdict |
|---|---|---|
| 1 | Existing standards already do this | **Survives**, narrowly and specifically |
| 2 | A consumer data marketplace cannot work | **FATAL to the consumer thesis.** The consumer marketplace should not be built. |
| 3 | Bilateral contracts and audit rights are sufficient | **Partially fatal.** True for the largest deals; false at the margin that is growing |
| 4 | Verification costs exceed transaction value | **FATAL at consumer scale**, manageable at enterprise scale |
| 5 | Usage cannot be proven, so the evidence is theatre | **Survives**, only because the protocol already refuses the claim |
| 6 | AI attribution is impossible, so the AI vertical collapses | **Survives**, by abandoning attribution and selling inclusion |
| 7 | The largest buyers will route around any accounting layer | **Unresolved and serious** |
| 8 | Legal rights are too unclear to license | **Fatal for individuals, manageable for organisations** |
| 9 | A company-controlled protocol will not be adopted; a foundation will not be funded | **Unresolved and serious** |
| 10 | Privacy law makes the ledger itself a liability | **Manageable**, at a cost already paid in the design |

Three verdicts are fatal. They remove the consumer marketplace, the
individual-payout product and the mass-market wallet from the plan.
Two more are unresolved and could remove the rest. What survives is
narrower than the original concept and is stated in §12.

---

## 1. "Existing standards already do this"

**The strongest form of the argument.** The IDS Dataspace Protocol reached
a final 1.0.0 release in July 2025, is on its way to ISO, has a test
compatibility kit, and covers cataloguing, contract negotiation and
transfer initiation across organizational boundaries with usage control as
a stated concern [search-summary]
([IDSA](https://internationaldataspaces.org/dataspace-protocol-2025-1-on-track-for-iso-submission/)).
FinOps FOCUS normalises usage and cost data across providers and has
genuine multi-vendor adoption [search-summary]
([FOCUS](https://focus.finops.org/focus-specification/)). ODRL expresses
rights. PROV expresses lineage. OpenTelemetry meters everything. ISO 20022
settles. Between them, what is left?

**Why it fails, specifically.** The Dataspace Protocol's own specification
repository states it does not address metering, billing, accounting,
clearing or settlement, and does not define a verifiable usage receipt; it
governs the initiation and decommissioning of transfers rather than what
happens after. **This is the one claim in this document resting on a
document actually read** (retrieved 2026-09-21) — fortunate, because it is
the claim kill attempt 1 turns on. FOCUS covers metering and accounting
but only inside an existing provider-to-customer billing relationship, with
no rights linkage and an evidence model that consists of trusting the
seller's billing system. The gap is not "rights" or "metering"; it is a
usage record that is evidence-bearing, rights-linked and survives crossing
an organizational boundary.

**What this survival is worth.** Less than it appears. A gap in a standards
landscape is a necessary condition for a new standard and nowhere near a
sufficient one. Gaia-X occupies a genuine gap and its own community frames
adoption, not specification, as the open question [search-summary]
([Forrester](https://www.forrester.com/blogs/gaia-x-is-back-but-only-adoption-will-signal-success)).
This kill attempt fails; kill attempts 7 and 9 are where the same energy
should go.

**What would make it succeed:** an IDSA work item on usage accounting, or
FOCUS extending past provider-to-customer billing. Either should be treated
as a reason to stop, not a competitor to fight.

---

## 2. "A consumer data marketplace cannot work"

**This one succeeds. It is the most important finding in this document.**

**The argument from unit economics.** Take the most favourable available
numbers. Meta's global ARPU was reported at roughly $57 for 2025, with US
and Canada at roughly $233 and rest-of-world at roughly $14 [search-summary]
([stockanalysis.com](https://stockanalysis.com/stocks/meta/metrics/average-revenue-per-user/);
[MacroMicro](https://en.macromicro.me/charts/32591/fb-arpu)). Estimates of
the value of one person's personal data to a platform cluster around $10
per year, with whole-industry estimates reaching a few hundred dollars per
year for the highest-value users and data-broker prices spanning $0.10 to
$1,200 depending entirely on who is buying and what is included
[search-summary] ([Bruegel](https://www.bruegel.org/blog-post/economic-value-personal-data-online-platforms-firms-and-consumers)).

These are search-summary reports of secondary-source estimates with
incompatible methodologies, none of which this author could open, and
should not be treated as precise. They do not need to be
precise. Every one of them is small, and ARPU is an upper bound that
massively overstates the data component, because it includes the platform's
product, engineering, sales and distribution.

Now put a protocol underneath it. A person in the most valuable market
might plausibly be owed single-digit to low-double-digit dollars per year
across all their data, spread over thousands of events, each of which must
be metered, authorized, receipted, anchored, priced, invoiced, disputed
where necessary, and settled, with a payout threshold high enough that
payment fees do not consume it. `duap-demo` settles 2.28 EUR after eleven
simulated periods against a 1.00 EUR threshold — and that is a
demonstration tuned to produce a payout at all.

**The argument from history.** Datacoup, the best-known attempt to pay
individuals for their data, operated from 2012 and shut down in November
2019; commentary attributes the failure to the inability to offer users
meaningful money and the resulting inability to accumulate a pool worth
selling [search-summary]
([Data marketplace, Wikipedia](https://en.wikipedia.org/wiki/Data_marketplace)).
The shutdown date is reported rather than established here, and the
reasons are commentary rather than a company statement. But the pattern
recurs across the category, and the arithmetic above explains it without
needing any additional cause.

**The argument from consent.** Even if the money worked, a per-use consent
interface at the frequency real data collection occurs is consent fatigue
by construction, and the alternative — a standing policy the user sets once
— is the thing every cookie banner already is and that nobody reads. The
directive forbids dark patterns; honouring that forbids the interaction
model the product needs.

**Verdict: FATAL to the consumer marketplace.** Not "hard", not "later" —
the unit economics do not close, and no protocol improvement changes an
arithmetic in which the gross value per participant is smaller than the
cost of transacting it.

**What survives.** Individuals as *beneficiaries* of collective
arrangements, where a cooperative or union negotiates once on behalf of
many and DUAP accounts for the aggregate; and individuals as *auditors* of
usage under rights they already hold in law, where the product is
transparency rather than income. Neither is a marketplace and neither
should be described as one. `docs/market/consumer.md` holds what is left.

**The falsifying evidence I would need to reverse this:** a demonstrated
willingness to pay, per individual per year, above roughly $50 net of
verification and settlement cost, in a real market with real buyers.
Nothing in the sources reviewed suggests it exists.

---

## 3. "Bilateral contracts and audit rights are sufficient"

**The argument.** Every large AI training-data deal already exists as a
contract with audit rights. Shutterstock's individual deals with large
technology companies have been reported in the $25–50 million range, with
company-level AI licensing revenue of about $104 million in 2023 and an
expected $138 million in 2024 [search-summary]
([Quartz](https://qz.com/ai-training-data-pricing-licensing-deals-market-052126)).
At that deal size, both parties can afford lawyers, an audit clause and a
spreadsheet. A shared protocol adds process to a negotiation that is
already working.

**Why it partly succeeds.** It is correct for the top of the market. A
$50 million deal between two parties with legal departments does not need
an interoperability standard, and claiming otherwise would be the kind of
unearned claim the project rules forbid.

**Why it does not fully succeed.** The deal structure is reported to be
changing in exactly the direction that breaks spreadsheets. Attribution and
live-access deals — where publishers are paid when AI systems fetch or
ground their content in real time, rather than once for a training dump —
are reported as growing from 2 in 2023 to 11 in 2024 to 18 in 2025, with 34
projected for 2026, and Reddit is reported to be discussing dynamic pricing
structures with Google and OpenAI where the payment scales with usage
[search-summary] ([Digiday timeline](https://digiday.com/media/a-timeline-of-the-major-deals-between-publishers-and-ai-tech-companies-in-2025/);
[Quartz](https://qz.com/ai-training-data-pricing-licensing-deals-market-052126)).

These counts come from trade press and industry trackers rather than
filings, and reached this document as a search summary rather than as
articles read; the 2026 figure is a projection. Of every number here these
are the ones a reader should most doubt, because they are the ones the
surviving thesis most depends on — the wrong way round, stated rather than
hidden.

**Why the distinction matters more than the numbers.** A one-time licence
needs a contract. A usage-metered licence needs a meter both parties
believe, and there is currently no standard one. That is a smaller claim
than "the data economy needs DUAP" and it is the one that is actually
supported.

**Verdict: partially fatal.** It removes the largest deals from the
addressable market and leaves the metered, recurring, many-counterparty
middle — which is where the growth is reported to be. This is the
strongest available argument for the beachhead in
`docs/market/go-to-market.md`, and it is an argument about a trend that
could reverse.

---

## 4. "Verification costs exceed transaction value"

**The argument, with this repository's own measurements.** Ingest costs
127.7 µs per event single-threaded, of which Ed25519 verification is 49.8 µs
(39%); receipt verification is 209.1 µs; an event envelope is 665 bytes with
Ed25519 and 3,096 bytes with the hybrid post-quantum suite
(`benchmarks/results/2026-09-21-ci-runner.md`). Storing 10⁹ events is
665 GB before indices or replication — arithmetic on a measured size, not a
measurement.

Against a transaction worth a thousandth of a cent, that is absurd. The
`duap-demo` invoice prices 4,500 queries at 0.18 EUR. The cryptographic
cost of proving those 4,500 events exceeds the value of the line item by
orders of magnitude in CPU alone, before storage, dispute handling or a
human ever looking at it.

**Why it succeeds at consumer scale.** It does, completely, and it is the
same conclusion as kill attempt 2 arrived at from the technical side. Two
independent arguments reaching the same verdict is the strongest signal in
this document.

**Why it fails at enterprise scale.** The costs above are per event, and
the protocol does not require one event per record. Aggregation is a
first-class part of the metering design: a counter covering 20,000 events
closes into one priced, receipted, anchored invoice in 7.44 ms. The correct
unit of evidence is the *aggregate over a period*, with per-event detail
retained only where a dispute needs it. A monthly settlement between two
organisations over millions of records is a handful of receipts, and there
the verification cost is rounding error against the deal value.

**The design consequence, which is real and is a constraint the protocol
must carry:** DUAP is economically viable only where evidence can be
aggregated. Any product framing that implies per-event receipts for
individuals is selling something the cost model does not support.

**Verdict: fatal at consumer scale, manageable at enterprise scale, with a
mandatory design constraint attached.**

---

## 5. "Usage cannot be proven, so the evidence is theatre"

**The argument.** A clearing node signs a receipt describing usage it did
not observe; it was told about it. An organisation that copies data out of
an instrumented system and processes it elsewhere generates no events. The
cryptography proves that someone signed a statement, not that the statement
is true. Selling this as proof of usage is selling a lie with a hash
attached.

**Why it fails, and only just.** Every clause of the argument is correct,
and the protocol already concedes all of it in the artefacts a reader would
actually consult. `Receipt::claims()` returns ten claims as data, five
established and five explicitly not — the demonstration prints
"claims established=30, claims explicitly not established=30" across six
receipts. RISK-01 and RISK-02 in `security/findings.md` are accepted risks
with a named accepter, not footnotes.

What remains after the concessions is smaller and real: non-repudiation of
a report, detectable equivocation given gossip, detectable partial
suppression via per-stream sequences, and detectable double-counting across
reporters. "Alice cannot later deny having told us this, and cannot quietly
change what she told us" is worth money in a dispute. It is not proof that
Alice told the truth.

**Verdict: survives, conditionally.** The condition is that no artefact
ever claims more. The moment a marketing page says "cryptographically
proven data usage", this kill attempt succeeds retroactively and the
project deserves it.

---

## 6. "AI attribution is impossible, so the AI vertical collapses"

**The argument.** The AI vertical is the one with money in it. It depends
on being able to say what a dataset contributed. That cannot be done.

**The evidence, measured in this repository rather than asserted.**
`research/ai-attribution/experiment.py` runs exact Shapley, Monte-Carlo
Shapley, leave-one-out and an influence-function proxy against synthetic
data with known structure. Leave-one-out and the influence function agree
with each other at Spearman ρ = +0.947 and with exact Shapley at ρ ≈ +0.53,
with top-3 overlap of 33% (`research/ai-attribution/RESULTS.md`). Different
defensible estimators pay different people. There is no principled basis
for choosing one, and the exact computation does not scale.

**Why it fails anyway.** It kills attribution-based pricing, which was
never the product. `AI_ATTRIBUTION.md` splits the question into three
claims of different standing: inclusion is cryptographic, influence is
badly estimable, economic contribution is not establishable. Only inclusion
is in the protocol, enforced by
`crates/duap-provenance/tests/dataset_commitment.rs`, which proves a
dataset commitment demonstrates membership and nothing about weight.

Inclusion plus a contractually agreed rate is a complete commercial
product, and it is what the reported market is actually buying: a
per-fetch or per-use rate, not a Shapley value.

**Verdict: survives, by abandoning the thing that cannot be done.** The
negative research result is what makes the design defensible rather than
what threatens it.

---

## 7. "The largest buyers will route around any accounting layer" — UNRESOLVED

**The argument.** A handful of firms buy most of the AI training data.
OpenAI alone is reported to have roughly 24 publicly announced licensing
agreements, against single-digit counts for the largest publishers
[search-summary] ([Quartz](https://qz.com/ai-training-data-pricing-licensing-deals-market-052126)).
That is a monopsony. A monopsony does not adopt an accounting standard that
makes its usage auditable by counterparties; it declines, and the
counterparties accept, because the alternative is no deal. Any seller who
insists on DUAP is negotiating against a buyer who can simply buy elsewhere.

**Why it might fail.** Standardisation sometimes serves the concentrated
side: buyers integrating N publishers each with a bespoke reporting format
bear the integration cost, and a common schema reduces it. That is the
FOCUS pattern, where cloud providers — the concentrated side — adopted a
common billing schema. It is also the pattern behind the shift to
attribution and live-access deals: per-use payment requires a meter, and
the buyer needs one as much as the seller does.

**Why it might not.** FOCUS normalised a report the provider already had to
produce. DUAP asks a buyer to produce a report it currently does not, about
its own internal use, for the benefit of a counterparty. The incentive is
not symmetric, and the analogy may not hold.

**Verdict: UNRESOLVED, and the largest commercial risk.** No evidence in
this review settles it. It is not a technical question and no amount of
protocol work answers it. The only thing that answers it is one buyer
adopting, which is the falsification test in
`docs/market/go-to-market.md`.

---

## 8. "Legal rights are too unclear to license"

**The argument.** The consumer product assumes a person can license their
data. Frequently they cannot: privacy rights are generally inalienable
control rights rather than transferable property; copyright may sit with
whoever created the content; database rights sit with whoever compiled the
database; employment-generated and jointly-generated data have contested
ownership; sensor data may belong to the device maker or the operator.
Selling someone a licence to something they do not own is at best void and
at worst a consumer-protection problem.

**Why it succeeds for individuals.** It does. `docs/legal/regulatory-matrix.md`
sets out the analysis; the short version is that a data subject's GDPR
rights are not a licensable asset, and treating them as one is the single
most likely way this project causes harm rather than merely failing. This
compounds kill attempt 2: the consumer marketplace is not only
uneconomic, it rests on a rights model that does not exist.

**Why it fails for organisations.** An organisation licensing a dataset it
compiled, under a contract, with representations and warranties, is
ordinary commerce that happens at scale every day. The uncertainty is about
individuals, not about licensing.

**Verdict: fatal for individuals, manageable for organisations** — and the
protocol must never present a subject's consent as if it were a property
transfer. The wire format currently records a controller's *asserted*
lawful basis, which is the correct primitive, and no document may describe
it as a determination that the basis applies (project rule 04).

---

## 9. "Neither governance model is viable" — UNRESOLVED

**The argument.** A protocol controlled by one company is not neutral, and
no competitor will build their accounting on a competitor's standard. A
protocol governed by a foundation has no revenue, no roadmap authority and
no ability to force the reference implementation to keep up; it publishes
documents and waits. Gaia-X is the worked example of the second failure
mode and its own community treats adoption as the unresolved question
([Forrester](https://www.forrester.com/blogs/gaia-x-is-back-but-only-adoption-will-signal-success)).

**Why it might fail.** The split model — specification, schemas, vectors
and verifier under a foundation licence; hosted infrastructure, analytics
and compliance tooling commercial — has worked elsewhere, and FOCUS is a
current example of a neutral foundation convening competing vendors around
a schema. `GOVERNANCE.md` proposes that structure.

**Why it might not.** Proposing a structure is not executing one, and the
failure mode is slow: the specification drifts toward whatever the funded
implementation does, the neutrality claim becomes nominal, and the second
implementer notices. This repository already has a mild version of the
problem — the Go verifier and the Rust reference were written by the same
author, which is stated as a limit on the independence claim in
`docs/architecture/minimal-protocol.md` §10.

**Verdict: UNRESOLVED.** Along with kill attempt 7, this is where the
project most plausibly dies, and neither is a technical problem.

---

## 10. "Privacy law makes the ledger itself a liability"

**The argument.** An append-only log of who used whose data, when, is a
detailed behavioural record. Building it to prove compliance creates a
new high-value target and a new erasure problem: GDPR erasure rights sit
badly with an immutable log.

**Why it fails, at a cost already paid.** No payload data enters the log.
Log entries commit to digests; subjects appear as per-controller
pseudonyms; the log entry is 129 bytes. Erasure of source data is
represented as an obligation that is checkable by observing behaviour, not
as a claim that copies vanish, and the demonstration prints
`already_trained_model=NOT_UNLEARNED` rather than pretending otherwise.

**The residual, stated because rule 03 requires it.** Per-controller
pseudonyms give unlinkability *against identifiers* and nothing else. Two
controllers cannot join on the pseudonym; they can trivially join on
content, timing or any quasi-identifier, and the protocol never sees
payloads so it cannot help. Event metadata is itself disclosive: purpose,
data class and timing across a long enough window describes a person.

**Verdict: manageable, with a residual that must be disclosed in
`PRIVACY.md` and is.**

---

## 11. Kill attempts that failed too easily to keep

Recorded so the list above is not mistaken for the whole attack surface.

- *"It needs a blockchain and therefore inherits blockchain's problems."*
  It does not. ADR-0005 records the decision and the slice confirmed
  nothing required consensus among mutually distrusting parties.
- *"Post-quantum signatures make it impractical."* Measured: ML-DSA-44
  signing is 690.7 µs against Ed25519's 47.4 µs, and verification only 2.2×
  slower. The cost is concentrated in signing, which is the favourable
  direction for a protocol whose receipts are signed once and verified
  often. The 4.7× envelope-size penalty matters more than the CPU.
- *"The schema will never fit every industry."* Correct, and the reason for
  profiles. It is an argument about scope, not viability.

---

## 12. What survives

Stripping out everything the kill attempts removed:

**Removed outright:** the consumer data marketplace; individual
data-payout income as a product; the mass-market wallet as a first product;
attribution-based pricing; any claim that usage is cryptographically proven;
any framing in which a person's privacy rights are a licensable asset.

**Survives, conditionally:** a usage-accounting and evidence layer for
*organisations* licensing data to each other under metered, recurring
terms, where the parties are numerous enough or the terms dynamic enough
that bilateral spreadsheets are failing; with aggregation as a mandatory
cost constraint; with inclusion rather than influence as the AI claim; and
with the protocol's limits published as machine-readable data rather than
prose.

**Unresolved, and decisive:** whether concentrated buyers will adopt an
accounting layer that makes them auditable, and whether a governance
structure exists that is neutral enough to be adopted and funded enough to
be maintained.

**The honest summary:** the technical thesis survived its own vertical
slice and a second implementation. The consumer economic thesis did not
survive contact with arithmetic. The enterprise economic thesis is
unresolved and is not a question engineering can answer — which means the
next work is not more protocol.

## 13. Evidence that would stop the project

Stated in advance so the answer cannot be rationalised later.

1. An IDSA or FOCUS work item covering cross-organizational usage
   accounting with an evidence model. DUAP should then contribute to it and
   stop.
2. Twelve months of enterprise conversations in which no buyer will accept
   a metered, auditable licence at any price. Kill attempt 7 confirmed.
3. A demonstration that dispute rates under metered licences are low enough
   that the parties do not want shared evidence — meaning trust was never
   the binding constraint.
4. A reversal of the shift toward attribution and live-access deals back to
   one-time training licences, which would restore kill attempt 3 to fully
   fatal.
5. Failure to attract a second independent implementation within eighteen
   months of publishing the specification, which would falsify the kernel's
   central claim by demonstration rather than argument.
