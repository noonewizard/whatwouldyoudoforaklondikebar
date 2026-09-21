# Standards and prior art

**Status:** REFERENCE · 2026-09-21
**Owner:** `standards-liaison`, reviewed by `chief-architect`
**Purpose:** determine, against primary sources, whether DUAP is necessary
or whether an existing standard or combination already does the job.

The default position taken throughout is the one the directive sets:
**DUAP should bind existing standards rather than reinvent them.** A row in
these tables that says "DUAP reuses this" is a better outcome than a row
that says "DUAP replaces this", and the reuse rows outnumber the others.

## Evidence classification

Every factual claim below carries one of these labels. They are not
decoration: the conclusion of this document rests on which claims are
VERIFIED and which are not.

| Label | Meaning |
|---|---|
| **RETRIEVED** | The source document was fetched and read in full on the date given |
| **SEARCH-SUMMARY** | Surfaced by a web search on 2026-09-21; the underlying document could **not** be fetched from this environment, so the fact is reported at one remove |
| **STRONG** | Consistent across independent sources, or well-established in the author's knowledge and not contradicted by anything retrieved |
| **INFERENCE** | A conclusion the author draws, which could be wrong |
| **HYPOTHESIS** | A proposition this project intends to test |
| **UNKNOWN** | Not established; stated so it is not mistaken for a gap in the landscape |

### Retrieval limits — read this before relying on any citation

This review was conducted from a sandboxed environment whose network policy
permits outbound HTTPS only to a narrow set of hosts. In practice, on
2026-09-21:

- **One source was fetched and read in full:** the IDS Dataspace Protocol
  specification repository on `github.com`. Claims sourced from it are
  marked RETRIEVED.
- **Every other source was blocked**, including `w3.org`,
  `focus.finops.org`, `en.wikipedia.org`, `qz.com`, `digiday.com` and
  others. For those, a web-search tool returned titles, URLs and a
  generated summary, and that summary is all this document had. Such
  claims are marked SEARCH-SUMMARY and a URL next to one is **where the
  claim came from, not a document this author read**.

A SEARCH-SUMMARY figure can be wrong in ways this review could not detect:
the summary may misattribute a number between sources, or compress a
qualified statement into an unqualified one. Wherever a decision in this
repository rests on such a figure, the decision is written so that it does
not turn on the figure's precision — and where it would, the document says
so. Before any of this is relied on commercially, the primary sources must
be read.

Second limit: standards move. Every version number below is a snapshot of
2026-09-21.

---

## 1. The question this document answers

DUAP claims a gap. The gap is only real if the following chain has no
existing standard covering its middle:

```
catalogue -> negotiate -> authorize -> USE -> measure -> prove -> account -> clear -> settle
\______________________________/        \_________________________/  \_________________/
        well covered                        the claimed gap            well covered
```

So the test is narrow and falsifiable:

> **Is there a published, cross-organizational standard that represents a
> unit of data *usage* as an evidence-bearing object carrying an economic
> obligation?**

If yes, DUAP is redundant and this project should stop. The rest of this
document is the search for that standard.

---

## 2. Rights and policy expression

### W3C ODRL Information Model 2.2

**SEARCH-SUMMARY:** ODRL 2.2 became a W3C Recommendation on 15 February 2018
([W3C news](https://www.w3.org/news/2018/odrl-2-2-is-now-a-w3c-recommendation/);
specification at <https://www.w3.org/TR/odrl-model/>).

**STRONG (author knowledge, not contradicted by anything retrieved):** ODRL models `Policy`, `Permission`, `Prohibition`, `Duty`,
`Asset`, `Party`, `Action` and `Constraint`. It is a policy *expression*
language: it describes what is permitted, forbidden and required. It does
not define how a policy is enforced, how compliance is monitored, or how
actual usage is reported back. (`w3.org` was blocked, so the specification text was not read; this is
the author's knowledge of ODRL, stated as such rather than dressed as a
quotation.)

| Question | Answer |
|---|---|
| Defines rights and obligations? | Yes — this is what it is for |
| Represents an actual usage occurrence? | No |
| Produces evidence? | No |
| Carries an economic obligation with a computed amount? | No — `Duty` can reference payment, but no measurement produces the amount |
| Crosses organizational boundaries? | Yes, as a shared vocabulary |

**Relationship to DUAP: reuse, do not replace.** A DUAP Grant expresses
the same shape of thing as an ODRL Policy and should be expressible as one.
The correct engineering output is a bidirectional profile, not a competing
rights language. What DUAP adds is the object ODRL has no room for: the
record that the permission was *exercised*, how much, and what is now owed.

**Known limit of that claim (INFERENCE):** a sufficiently determined ODRL
profile could carry usage reports in extension properties. Nothing prevents
it. The argument for a separate object is that a usage record has a
different lifecycle, a different signer, a different retention requirement
and a different privacy exposure from the policy it exercises, and
collapsing them produces an object that is hard to share safely.

### IDS / IDSA Dataspace Protocol

This is the strongest candidate for making DUAP unnecessary, and it is
treated at length in `docs/reviews/falsification.md`.

**SEARCH-SUMMARY:** the Dataspace Protocol reached a final 1.0.0 release as
version 2025-1 in July 2025 and is on track for ISO submission
([IDSA](https://internationaldataspaces.org/dataspace-protocol-2025-1-on-track-for-iso-submission/)).
It has an official test compatibility kit (TCK) for verifying
implementations.

**RETRIEVED:** its specification comprises a dataspace model and
terminology, common functionalities, a Catalog Protocol (over DCAT), a
Contract Negotiation Protocol, and a Transfer Process Protocol, each with
an HTTPS binding
([specification repository](https://github.com/International-Data-Spaces-Association/ids-specification),
retrieved 2026-09-21).

**RETRIEVED:** the specification as documented in that repository **does not
address usage metering, billing, accounting, clearing or settlement**, and
contains no cryptographically verifiable receipt of data usage. It states
that it "does not cover the data transfer process as such", governing the
initiation and decommissioning of transfers rather than what happens to the
data afterwards (retrieved 2026-09-21).

| Question | Answer |
|---|---|
| Catalogue, negotiate, authorize, initiate transfer? | Yes, and well |
| Measure what was then done with the data? | No |
| Produce a verifiable receipt? | No |
| Compute an obligation? | No |
| Clear or settle? | No |

**Relationship to DUAP: complementary, and the most important integration
target in this document.** The Dataspace Protocol owns everything to the
left of "USE" in the chain in §1. DUAP owns what is to the right. A
DUAP Grant should be derivable from a Dataspace Protocol Agreement, and a
DUAP Receipt should be a thing a dataspace connector can emit. If DUAP is
ever adopted, the most likely path is as a usage-accounting profile
alongside the Dataspace Protocol rather than in competition with it.

**HYPOTHESIS to test:** that the IDSA community perceives the accounting
gap as a gap worth filling, rather than as deliberately out of scope
because it belongs to bilateral contracts. This has not been tested with
anyone from that community and is a significant unknown.

### Gaia-X

**SEARCH-SUMMARY:** Gaia-X maintains a Trust Framework, with a 3.0 "Danube"
release presented, and describes a transition to a "Season 2.0" focused on
economically sustainable data spaces and market adoption, with
standardisation pursued through CEN/CENELEC and ISO/IEC
([Gaia-X general assembly summary, 2026](https://gaia-x.eu/gaia-x-ordinary-general-assembly-2026-key-highlights/)).

**SEARCH-SUMMARY:** independent commentary questions whether adoption has followed
the specification work — Forrester's assessment is titled
["Gaia-X Is Back, But Only Adoption Will Signal Success"](https://www.forrester.com/blogs/gaia-x-is-back-but-only-adoption-will-signal-success)
and notes it is not clear that what has been built meets a business need
customers see value in.

**Relationship to DUAP: a cautionary precedent more than a competitor.**
Gaia-X is the closest thing to an attempt at the institutional layer DUAP
would need, and its difficulty is not technical. This is evidence for a
claim made in `docs/market/why-not-already.md`: in this problem domain,
specification quality has repeatedly failed to predict adoption, and any
plan whose critical path is "publish a good specification" is already
known to be insufficient.

---

## 3. Provenance and lineage

### W3C PROV

**SEARCH-SUMMARY:** the W3C PROV family, including PROV-O: The PROV Ontology,
was published as W3C Recommendations on 30 April 2013
([PROV-O](https://www.w3.org/TR/prov-o/);
[W3C blog](https://www.w3.org/blog/2013/prov-a-framework-for-provenance-interchange/)).

PROV models `Entity`, `Activity` and `Agent`, and relations among them —
`wasGeneratedBy`, `used`, `wasDerivedFrom`, `wasAttributedTo`.

| Question | Answer |
|---|---|
| Represents derivation relationships? | Yes, and this is exactly DUAP's provenance need |
| Represents an authorization? | No |
| Measures quantity? | No |
| Carries an obligation? | No |
| Integrity-protected? | Not intrinsically; PROV is a model, not a cryptographic construction |

**Relationship to DUAP: reuse.** DUAP's derivation graph is a PROV graph
with two additions — the edges are integrity-protected by the event digests
they connect, and each node can carry a metered quantity. `duap-provenance`
should publish a PROV-O serialisation, and does not yet; that is recorded
as a gap below.

**The line this document draws, because the directive requires it:**
provenance ends where attribution begins. PROV can state that a model was
derived from a dataset. It cannot state how much of the model's value came
from that dataset, and neither can DUAP. See `AI_ATTRIBUTION.md`.

### SPDX and CycloneDX

**STRONG:** both are software bill-of-materials formats, with mature
tooling and real adoption, and both have extended toward AI/ML and dataset
description.

**Relationship to DUAP: adjacent and instructive, not overlapping.** An
SBOM describes composition at a point in time. DUAP describes events over
time. The instructive part is adoption mechanics: SBOMs became widespread
substantially because of procurement and regulatory pressure rather than
because engineers wanted them — a pattern `docs/market/go-to-market.md`
takes seriously.

---

## 4. Telemetry and metering

### OpenTelemetry

**STRONG:** OpenTelemetry is a CNCF project defining traces, metrics and
logs with a shared data model (OTLP), and is very widely deployed in
production systems.

| Question | Answer |
|---|---|
| Measures what a system did? | Yes, and at enormous scale |
| Crosses organizational boundaries? | Rarely — telemetry is overwhelmingly intra-organizational |
| Signed or non-repudiable? | No |
| Authorization-linked? | No |
| Carries an economic obligation? | No |

**Relationship to DUAP: reuse as an instrumentation substrate, and the
single most valuable adoption lever in this document.** If a DUAP event can
be produced from an OpenTelemetry span by a collector processor, the
integration cost for an enterprise drops from "instrument your pipeline" to
"add an exporter", which is the difference between a project and a
configuration change. `docs/market/go-to-market.md` treats this as the
primary technical adoption path.

**The limit (INFERENCE, and important):** a span is a self-report from the
system being measured. Exporting it as a signed DUAP event does not make it
more true; it makes it attributable and non-repudiable. That is a real
improvement and a smaller one than it sounds. RISK-02 in
`security/findings.md` is the honest statement of what remains.

### FinOps FOCUS

**SEARCH-SUMMARY:** FOCUS is an open specification defining a common schema for
technology cost and usage data, maintained by the FinOps Foundation under
the Linux Foundation. Release 1.3 was ratified by the FOCUS Steering
Committee on 4 December 2025, and later versions are published
([FOCUS specification](https://focus.finops.org/focus-specification/)).
As of v1.2 it defines 65 standardized columns
([v1.2](https://focus.finops.org/focus-specification/v1-2/)).

**This is the closest existing analogue to what DUAP is trying to be**, and
the comparison is worth stating precisely because it is the most useful one
in this document:

| | FOCUS | DUAP |
|---|---|---|
| Normalises usage data across providers | Yes | Yes |
| Direction of the relationship | A provider bills a customer | Either party may owe either party |
| Who generates the record | The seller | The party that used the data, or a gateway observing it |
| Evidence model | Trust the provider's billing system | Signed events, transparency log, receipt with stated limits |
| Rights linkage | None — you bought compute, the terms are the cloud contract | Every event references the grant that authorized it |
| Disputes | Out of scope | A first-class object |

**Relationship to DUAP: the design precedent to follow, and proof the
pattern works.** FOCUS demonstrates that competing commercial parties will
adopt a common usage schema when the alternative is every customer building
N adapters. That is the strongest available evidence for DUAP's core
adoption hypothesis. It is also evidence about *how*: FOCUS succeeded as a
schema convened by a neutral foundation with practitioner demand behind it,
not as a protocol with a company behind it.

**INFERENCE, and the uncomfortable half:** FOCUS works because the billing
relationship already exists and only the format was missing. DUAP is
proposing a schema for a transaction that mostly does not happen yet. The
analogy therefore supports DUAP's *design* and not its *demand*.

---

## 5. Financial and accounting standards

### ISO 20022

**STRONG:** ISO 20022 is the financial-messaging standard used by major
payment infrastructures, providing structured messages with rich remittance
data.

**Relationship to DUAP: settlement binding, and a boundary DUAP must not
cross.** DUAP computes what is owed and stops. A cleared DUAP obligation
should map to an ISO 20022 payment initiation with the receipt reference in
the remittance information. DUAP does not move money, does not hold funds,
and does not become a payment system — a design constraint driven as much
by `docs/legal/regulatory-matrix.md` (money transmission) as by taste.

### XBRL

**STRONG:** XBRL is the standard for business reporting used in regulatory
filings in many jurisdictions.

**Relationship to DUAP: a downstream consumer, not a dependency.** If data
usage obligations ever become material enough to disclose, they would be
disclosed in XBRL. Nothing in DUAP v0.1 depends on this and nothing should.

---

## 6. Identity, credentials and authorization

### OAuth 2.0, OpenID Connect, W3C Verifiable Credentials, W3C DIDs

**STRONG:** OAuth 2.0 (RFC 6749) and OpenID Connect are the dominant
authorization and authentication standards on the web. W3C Verifiable
Credentials and Decentralized Identifiers are W3C Recommendations providing
cryptographically verifiable claims and self-sovereign identifier schemes.

**Relationship to DUAP: reuse, entirely.** DUAP defines no identity system.
It defines an identifier *format* for organisations and self-certifying key
identifiers for signing keys, and expects those to be bound to whatever
identity infrastructure a deployment already uses. ADR-0006 records that
decision and its cost: DUAP inherits whatever weaknesses the deployment's
identity layer has, and says so rather than pretending to solve identity.

**An honest note on Verifiable Credentials (INFERENCE):** a DUAP Receipt
is close in spirit to a VC and could plausibly be profiled as one. It is
not, currently, because the canonical-encoding requirements DUAP places on
digests are stricter than JSON-LD canonicalisation makes comfortable. That
is a defensible engineering reason and also a real interoperability cost,
and it is recorded here rather than buried.

---

## 7. Privacy and personal-data semantics

### W3C DPV

**SEARCH-SUMMARY:** the Data Privacy Vocabulary is developed by the W3C Data
Privacy Vocabularies and Controls Community Group; v1 was released in
December 2022 ([DPVCG](https://www.w3.org/community/dpvcg/2022/12/05/dpv-v1-release/)),
and versions 2.0 and 2.1 have since been published, with 2.1 extending
scope to data and AI technologies and adding draft extensions for
jurisdictions, laws and sectors
([DPV 2.1](https://w3c-cg.github.io/dpv/2.1/dpv/)).

**UNKNOWN:** the precise release date of DPV 2.1 was not established from
the sources retrieved. Stated here rather than approximated.

**Relationship to DUAP: reuse for purpose and lawful-basis vocabulary.**
DUAP's purpose taxonomy should map to DPV purposes, and its lawful-basis
codes to DPV's. Inventing a parallel privacy vocabulary would be exactly
the reinvention the directive forbids. `duap-model`'s taxonomy is
generated from `ontology/duap-ontology-v1.json` specifically so that such a
mapping can be produced mechanically; the mapping is not yet written, and
that is recorded as a gap below.

### DCAT

**STRONG:** DCAT is a W3C Recommendation for describing data catalogues,
and is the catalogue vocabulary the IDS Dataspace Protocol builds on.

**Relationship to DUAP: reuse.** A DUAP data resource identifier should
resolve to a DCAT description where one exists.

---

## 8. The summary matrix

Reading: does the standard cover this stage of the chain in §1, across
organizational boundaries?

| Standard | Catalogue | Authorize | Measure use | Prove use | Attribute | Account | Clear | Settle |
|---|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| ODRL 2.2 | — | ● | — | — | — | — | — | — |
| IDS Dataspace Protocol 2025-1 | ● | ● | — | — | — | — | — | — |
| Gaia-X Trust Framework | ◐ | ◐ | — | — | — | — | — | — |
| W3C PROV | — | — | — | ◐ | ◐ | — | — | — |
| SPDX / CycloneDX | ◐ | — | — | — | — | — | — | — |
| OpenTelemetry | — | — | ● | — | — | — | — | — |
| FinOps FOCUS | — | — | ● | — | — | ● | — | — |
| ISO 20022 | — | — | — | — | — | — | ◐ | ● |
| XBRL | — | — | — | — | — | ◐ | — | — |
| OAuth 2.0 / OIDC | — | ● | — | — | — | — | — | — |
| W3C VC / DID | — | ◐ | — | ◐ | — | — | — | — |
| W3C DPV | — | ◐ | — | — | — | — | — | — |
| DCAT | ● | — | — | — | — | — | — | — |

● covers it · ◐ partially or adjacently · — does not

**The empty region is real and it is specific.** No standard in this table
represents a unit of data usage as an evidence-bearing object carrying an
economic obligation across an organizational boundary. Two come close from
opposite sides: FOCUS meters and accounts but only within an existing
seller-to-buyer billing relationship and with no rights linkage or evidence
model; the Dataspace Protocol establishes rights across boundaries and
stops at the moment of transfer.

**INFERENCE:** DUAP's defensible claim is therefore narrow. It is not that
nobody has built rights expression, provenance, metering or settlement —
all four are mature. It is that nothing binds them into a single object
that survives crossing an organizational boundary. That claim is falsifiable
and `docs/reviews/falsification.md` attempts to falsify it.

---

## 9. What DUAP reuses, binds, and defines

| Layer | Decision |
|---|---|
| Identity | **Reuse.** OAuth/OIDC/VC/DID as deployed. DUAP defines identifier format only. |
| Catalogue | **Reuse.** DCAT. |
| Rights expression | **Bind.** ODRL profile, bidirectional with DUAP Grant. |
| Negotiation and transfer setup | **Bind.** IDS Dataspace Protocol. DUAP does not define negotiation. |
| Privacy semantics | **Bind.** DPV for purposes and lawful bases. |
| Provenance | **Bind.** PROV-O serialisation of the derivation graph. |
| Instrumentation | **Bind.** OpenTelemetry collector processor producing DUAP events. |
| Cost/usage reporting | **Bind.** FOCUS mapping for infrastructure-side costs. |
| Settlement | **Bind.** ISO 20022 for payment initiation. |
| **Usage event** | **Define.** Nothing above represents it. |
| **Usage receipt** | **Define.** Nothing above represents it. |
| **Obligation linked to metered evidence** | **Define.** Nothing above represents it. |

Three definitions, nine bindings. If that ratio ever inverts, the project
has lost its argument.

---

## 10. Gaps in this repository, as of this review

These are bindings the analysis above says DUAP needs and which do not
exist in the code. They are listed so that the table in §9 is not read as a
description of what is built.

| Binding | Status | Owner |
|---|---|---|
| ODRL profile and Grant↔Policy mapping | UNIMPLEMENTED | `standards-liaison` |
| PROV-O serialisation of the derivation graph | UNIMPLEMENTED | `provenance-engineer` |
| DPV purpose and lawful-basis mapping | UNIMPLEMENTED | `standards-liaison` |
| OpenTelemetry collector processor | UNIMPLEMENTED | `integration-engineer` |
| FOCUS column mapping | UNIMPLEMENTED | `economics-engineer` |
| ISO 20022 remittance binding | UNIMPLEMENTED | `settlement-engineer` |
| DCAT resolution for resource identifiers | UNIMPLEMENTED | `standards-liaison` |

`INTEROPERABILITY.md` holds the design for each; none is written in code,
and `docs/STATUS.md` records the subsystems that would host them as CONCEPT.

## 11. What would change this document's conclusion

- A published usage-accounting profile for the Dataspace Protocol, or an
  IDSA work item covering metering and settlement. This is the most likely
  way DUAP becomes redundant, and it would be a good outcome.
- FOCUS extending beyond provider-to-customer billing into
  cross-organizational data-rights accounting.
- An ODRL profile that carries signed usage reports and obligations, with
  implementations.
- Evidence that bilateral contracts and audit rights are sufficient in
  practice, and that the parties to data licensing deals do not want a
  shared accounting object. This is the economic rather than technical
  falsification, and `docs/market/why-not-already.md` takes it seriously.

Any of these should be treated as grounds to narrow or stop the project,
not as a competitive threat to be argued away.
