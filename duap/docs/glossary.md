# Glossary

**Status:** REFERENCE · 2026-09-21

The directive requires these terms be used precisely and never collapsed
into one another. Most of the confusion in this field comes from treating
four of them as synonyms, so each entry says what the term is **not**.

## The ten-stage abstraction

The protocol's spine, and the order matters: each stage depends on the one
before and none may be skipped.

```
Collect → Authorize → Identify → Classify → Measure →
Process → Derive → Transfer → Monetize → Settle
```

---

## Core objects

**Data** — the information itself. DUAP never carries it. Events carry a
*commitment* to data, never the data, and the demonstration prints
`raw_value_transmitted=false` for that reason.

**Data Resource** — a defined information resource, identified so that a
grant can name it. Not the data; a handle to it.

**Data Right** — a legally or contractually defined right concerning data.
A term of art from law, not a DUAP object.

**Data Usage Right** — a permitted action over a data resource, under
stated conditions. The economic primitive. Expressed as a *term* inside a
**Grant**. Not ownership, not access, not a licence document.

**Grant** — the DUAP object expressing one or more Data Usage Rights,
signed by the subject or their agent, versioned by **epoch**. **It is not
the contract.** The contract is the contract; the grant is a
machine-readable record of terms a party asserts it granted.

**Data Usage Event** — a recorded instance of data use. A *signed
assertion by the reporting party*. Not proof the processing occurred
(RISK-01), and silent about processing nobody reported (RISK-02).

**Data Usage Receipt** — a clearing node's signed statement about a set of
events. Establishes that the node made this statement and cannot alter it
undetected. Does **not** establish that the operations happened;
`Receipt::claims()` returns five established and five not-established
claims as data, so tooling surfaces the limits without anyone reading a
document.

**Data Usage Obligation** — an economic liability arising from usage. What
is owed. Distinct from an **obligation** in the authorization sense below,
and the collision is unfortunate; where ambiguity is possible this
repository writes "economic obligation" or "authorization obligation".

**Log entry** — a transparency-log commitment timestamping a receipt. 129
bytes. Commits to a digest, never to content.

---

## Process terms, in the order they are conflated

These five are the ones most often used interchangeably, and doing so is
how a system ends up claiming more than it can support.

**Measurement** — counting what happened. Objective, mechanical, and the
only one of the five DUAP performs with confidence. `duap-meter`.

**Valuation** — assigning economic value to what was measured. A
*judgement*, parameterised by policy. Kept in a separate crate from
measurement because the directive makes that separation mandatory and
because fusing them would imply prices are measurements. `duap-valuation`.

**Accounting** — recording economic obligations in a double-entry ledger.
Mechanical, given a valuation. `duap-ledger`.

**Clearing** — reconciling obligations between parties, including
multilateral netting. Produces a net position. DUAP computes it and does
not execute it.

**Settlement** — discharging obligations by actually moving money. DUAP
does **not** do this. It hands a cleared obligation to a payment system
that already exists, which is a scope decision reinforced by
money-transmission exposure (`docs/legal/regulatory-matrix.md` §5).

---

## Evidence terms

**Provenance** — the relationship between entities, activities and
derivations. That this model came from that dataset. A W3C PROV concept
DUAP binds rather than reinvents.

**Attribution** — an evidence-supported relationship between data and an
outcome. **Provenance ends where attribution begins**, and DUAP stops at
that line. `research/ai-attribution/RESULTS.md` measures why: defensible
estimators disagree at Spearman ρ ≈ +0.53 with 33% top-3 overlap, so
choosing among them changes who gets paid.

**Inclusion** — that specific data was present in a dataset.
**Cryptographic**, via a dataset commitment. The only one of these three
DUAP claims, and — usefully — the one EU AI Act art. 53(1)(d)'s training
content summary asks for.

**Influence** — whether data materially affected a model. Estimable,
badly. Not claimed.

**Economic contribution** — what a dataset was worth to an outcome. Not
establishable by any method in this repository. Not claimed.

**Non-repudiation** — that a party made a specific statement at a specific
time and cannot later produce a different version undetected. This *is*
claimed, and it is the protocol's real evidentiary product. It is not
proof the statement was true.

---

## Terms this repository refuses

**Data ownership** — avoided, because it is usually wrong. A person's
data-protection rights are inalienable control rights, not property; a
dataset's database rights sit with whoever compiled it. Say *control*,
*access*, *authorization*, *licensing* or *economic rights*, whichever is
meant.

**Verified usage** — avoided. A signed self-report is attributable and
non-repudiable, not verified. Signing telemetry does not make it true.

**Immutable** — avoided for the transparency log. It is *append-only*, and
equivocation is **detected** given auditor gossip, never prevented.

**Trustless** — avoided. Every deployment trusts a clearing node not to
fabricate (RISK-01) and every participant to instrument honestly
(RISK-02).

**Anonymous** — avoided for pseudonyms. Per-controller pseudonyms give
unlinkability *against identifiers* and nothing against content, timing or
quasi-identifiers.

**Compliant** — forbidden without a citation to an authority's own
published decision (rule 04). No such decision exists for anything here.

---

## Status vocabulary

Two overlapping sets, aligned by ADR-0017's predecessor ADR-0015 after
they differed by one word and silently disabled a check.

**Module markers** (rule 01): `PRODUCTION`, `PRODUCTION-CANDIDATE`,
`REFERENCE`, `PROTOTYPE`, `EXPERIMENTAL`, `STUB`, `UNIMPLEMENTED`.

**Subsystem statuses** (rule 10, in `docs/STATUS.md`): the above plus
`CONCEPT`, `RESEARCH`, `SPECIFIED`, `DEPRECATED`, `REJECTED`.

`docs/STATUS.md` is the single authority for a subsystem's status; a
module marker describes the module.

---

## Evidence labels

Used in the research and market documents, where most sources could not be
retrieved from this environment:

| Label | Meaning |
|---|---|
| **RETRIEVED** | The source was fetched and read |
| **SEARCH-SUMMARY** | A web search returned this; the underlying page could not be opened |
| **STRONG** | Consistent across sources, or well-established and uncontradicted |
| **INFERENCE** | A conclusion drawn here, which could be wrong |
| **HYPOTHESIS** | Something this project intends to test |
| **UNKNOWN** | Not established, stated so it is not mistaken for a gap |
