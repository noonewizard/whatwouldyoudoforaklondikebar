# Compliance mapping

**This document is not legal advice.** It is an engineering record of which
artefacts DUAP produces and which regulatory requirements those artefacts
are mechanically relevant to. Whether any artefact satisfies any
requirement is a legal judgement that this document does not make, and
that qualification is not softened anywhere else in this repository.

**Status:** REFERENCE · 2026-09-21
**Owner:** `compliance-researcher`
**Governed by:** project rule 04, which forbids "compliant", "certified",
"meets the requirements of" and "satisfies article N" without a citation to
an authority's own published decision. No such decision exists for anything
here.

## What DUAP does and does not do

DUAP records assertions and produces evidence. **It does not make anyone
compliant with anything.** An organisation with bad data practices and a
DUAP deployment has bad data practices and a detailed record of them —
which is arguably worse for it than having no record, and that is a
feature of an honest accounting system rather than a defect.

## Classification

Every row is marked:

- **technical** — DUAP produces an artefact mechanically relevant to the
  requirement;
- **interpretation-required** — whether that artefact satisfies the
  requirement is a legal judgement.

No row is marked "satisfied", because no row is.

## GDPR — Regulation (EU) 2016/679

| Provision | Requirement, in brief | DUAP artefact | Class |
|---|---|---|---|
| art. 5(2) | Accountability: demonstrate compliance with the principles | Signed events with asserted lawful basis, anchored receipts | interpretation-required |
| art. 6 | A lawful basis is required for processing | The event's lawful-basis field records **what the controller asserts**. It is not a determination that the basis applies | technical |
| art. 7(3) | Withdrawal of consent must be as easy as giving it | `duap-auth::revocation`, with an effective time and deterministic prospective semantics | technical |
| art. 15 | Right of access | A subject can enumerate receipts naming their pseudonym and verify each from the node's public key alone | technical |
| art. 17 | Right to erasure | **Partially, and honestly.** A revocation with a `DeleteSourceAndDerived` policy records an obligation to delete. It does not establish deletion happened, and the demonstration prints `already_trained_model=NOT_UNLEARNED` rather than implying otherwise | interpretation-required |
| art. 20 | Data portability | Receipts and grants are canonically encoded and portable by construction | technical |
| art. 28(3) | Processor instructions must be documented | A grant is a machine-readable record of the terms a controller asserts it granted. **It is not the contract** | interpretation-required |
| art. 30(1) | Records of processing activities | Events carry controller, processor, purpose, category, recipients and jurisdiction. Whether the set satisfies art. 30's enumeration is a legal judgement | interpretation-required |
| art. 32 | Security of processing | Signed events, a transparency log, per-controller pseudonyms. **No independent audit has been performed** | interpretation-required |
| arts. 44–49 | Transfers to third countries | Jurisdiction is recorded per event. Whether a clearing node outside the EEA processing pseudonymous metadata effects a transfer is an open question for counsel | interpretation-required |

**The unresolved question underneath this entire table**, recorded as
question 2 in `docs/legal/regulatory-matrix.md` §7: whether pseudonymous
event records are personal data under art. 4(1) in the hands of a clearing
node that cannot re-identify them. The answer determines the privacy
posture of the whole system and has not been obtained.

## EU AI Act — Regulation (EU) 2024/1689

| Provision | Requirement, in brief | DUAP artefact | Class |
|---|---|---|---|
| art. 53(1)(d) | Providers of general-purpose AI models must draw up and make publicly available a sufficiently detailed summary of training content, per an AI Office template | A dataset commitment establishes **inclusion**: that specific data was in a training set. `crates/duap-provenance/tests/dataset_commitment.rs` proves it establishes membership and nothing about weight | technical |

**Why this row matters more than the others.** The obligation is to
describe *what was included*, which is the one claim `AI_ATTRIBUTION.md`
establishes as cryptographic. DUAP can defend it precisely because it
refuses the adjacent claims — influence and economic contribution — that
its own research shows are not establishable.

**Retrieval note:** the template's adoption on 24 July 2025 and
applicability from 2 August 2025 arrived through a web-search summary of
pages this environment could not open. Verify against the Official Journal
before relying on it.

## EU Data Act — Regulation (EU) 2023/2854

| Provision area | Requirement, in brief | DUAP artefact | Class |
|---|---|---|---|
| Connected-product data access | Users must be able to access data generated by their use of connected products | Events record what was collected and under what authorization | interpretation-required |
| Making data available to third parties | Data holders make data available on agreed terms, with compensation provisions | Grants express terms; obligations account for what is owed | interpretation-required |

## EU Data Governance Act — Regulation (EU) 2022/868

**This one applies to a DUAP operator, not to its users**, and it is the
reason this section exists at all. Providers of data intermediation
services must notify a competent authority and observe structural
requirements including constraints on using intermediated data for their
own purposes.

| Question | Status |
|---|---|
| Is a clearing node that processes only digests and counters a data intermediation service under art. 2(11)? | **UNKNOWN.** First question for counsel; plausibly determines whether a hosted EU product can exist |
| Would the commercial prohibition on monetising intermediated data bind an operator? | Moot by design — `docs/business/business-model.md` rejects selling anything derived from customers' events absolutely |

## United States

| Regime | Relevance | Class |
|---|---|---|
| CCPA as amended by CPRA | Consumer rights to know, delete and opt out of sale/sharing; "sale" is defined broadly | interpretation-required |
| State data-broker registration regimes | Whether an accounting intermediary that never holds data is in scope | **UNKNOWN** |
| HIPAA, GLBA, FCRA, COPPA | Sectoral regimes reached only if the accounted data is in scope | **UNKNOWN** |

**These rows are deliberately thin.** This author did not verify the
current text or status of any US instrument from a primary source, and a
detailed-looking mapping built on that would be worse than an admission.
`docs/legal/regulatory-matrix.md` lists them as analysis to commission.

## What an auditor should not conclude from a DUAP deployment

Stated as plainly as the table above, because this is the section most
likely to be skipped.

1. **That the recorded processing occurred as described.** The reporter
   signed; the reporter could have lied. RISK-01.
2. **That all processing was recorded.** Usage outside instrumentation is
   invisible. RISK-02.
3. **That a lawful basis applies.** The field records an assertion.
4. **That data was erased.** A deletion obligation is not a deletion.
5. **That the system is secure.** No independent audit has been performed
   on any part of this repository, and the cryptographic subsystem's own
   status line says so.

An auditor who treats a receipt as proof rather than as an attributable,
non-repudiable, tamper-evident statement has drawn a stronger conclusion
than the artefact supports — and `Receipt::claims()` returns the
not-established claims as data specifically so that tooling can surface
them rather than relying on anyone reading this file.
