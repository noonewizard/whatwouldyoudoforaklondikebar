# Legal and regulatory issue map

**Status:** REFERENCE · 2026-09-21
**Owner:** `legal-research`
**This is not legal advice.** It is an engineering document that identifies
questions requiring qualified counsel in each relevant jurisdiction. Nothing
here is a legal conclusion, and no statement in this repository may be read
as one. Project rule 04 governs what may be claimed and forbids the words
"compliant", "certified" and "satisfies article N" without a citation to an
authority's own published decision. No such decision exists for anything in
this repository.

**Retrieval limits:** on 2026-09-21 this environment could reach only
`github.com`; every citation below arrived through a web-search tool's
summary of pages that could not be opened. Provisions are cited by
instrument and article so that counsel can check them directly. Marked
**[ss]** where the date or content came from a search summary.

---

## 1. The two questions that matter most

Most of this document is a map. Two entries on it change the architecture,
and they are stated first.

### 1.1 The EU AI Act creates demand for exactly the claim DUAP can make

Regulation (EU) 2024/1689 art. 53(1)(d) requires providers of
general-purpose AI models to draw up and make publicly available a
sufficiently detailed summary of the content used for training, according
to a template provided by the AI Office. The Commission adopted the
explanatory notice and template on 24 July 2025, applicable from
2 August 2025, with information blocks covering model and provider
metadata, an organised listing of main data source categories — public
datasets, licensed datasets, crawled or scraped content, user data,
synthetic data — and processing and governance aspects including
copyright **[ss]**
([Article 53](https://artificialintelligenceact.eu/article/53/);
[Regulation](https://eur-lex.europa.eu/eli/reg/2024/1689/oj/eng)).

**Why this matters to the architecture.** The obligation is to describe
*what was included*. That is precisely the claim
`AI_ATTRIBUTION.md` establishes as cryptographic and the only one
`crates/duap-provenance/tests/dataset_commitment.rs` permits: a dataset
commitment proves membership and nothing about weight or contribution.

A DUAP dataset commitment is therefore a mechanically relevant artefact
for a training-content summary — **technical**, in rule 04's taxonomy.
Whether it satisfies the obligation is **interpretation-required** and is
a question for counsel, and this document does not answer it. But it means
the one claim DUAP can defend is the one a regulation now asks for, which
is a better alignment than this project expected to find.

### 1.2 The Data Governance Act may regulate DUAP itself

Regulation (EU) 2022/868 entered into force 23 June 2022 and has applied
since 24 September 2023; providers of data intermediation services must
notify a competent authority, with transitional compliance for those
already operating by 24 September 2025 **[ss]**
([Regulation](https://eur-lex.europa.eu/legal-content/EN/ALL/?uri=CELEX:32022R0868);
[DGA art. 13](https://www.cms-digitallaws.com/en/dga/article-13/)).

**Why this matters more than the rest of the map.** Every other entry
here concerns DUAP's *users*. This one concerns DUAP. An entity that
operates hosted clearing infrastructure between data holders and data
users in the EU has to ask whether it is a data intermediation service,
and if it is, the DGA's notification and structural requirements —
including constraints on using the intermediated data for its own purposes
— apply to it.

**The architectural consequence, which is already in the design and now
has a second reason:** the reference clearing node computes obligations
and never touches payloads or money. Commitments, not content; receipts,
not funds; ISO 20022 handoff, not settlement. That was chosen to avoid
money-transmission exposure (§5) and to keep the trust surface small. It
also happens to be the posture that makes the intermediation question
easiest to answer.

**Open question for counsel, recorded rather than guessed:** whether a
clearing node that never holds data or funds, and only processes digests
and counters, is a data intermediation service within the meaning of DGA
art. 2(11). This author does not know, and the answer plausibly determines
whether a hosted product can exist in the EU in its current form.

---

## 2. Personal data: what an individual can and cannot license

The single most consequential legal finding for the product, and the basis
of kill attempt 8 in `docs/reviews/falsification.md`.

| Right | Instrument | Transferable? | Consequence for DUAP |
|---|---|---|---|
| Data protection rights | Regulation (EU) 2016/679 (GDPR) arts. 15–22 | **No.** Control rights exercisable against a controller, not property | A person cannot sell them. Any product implying they can is misrepresenting the law |
| Consent | GDPR art. 6(1)(a), art. 7(3) | Withdrawable at any time | Consent is not a contract term and cannot be made irrevocable by paying for it |
| Copyright in content a person created | National implementations of international copyright law | **Yes**, licensable | This is a real licensable asset, and it is the one the AI content market is transacting |
| Database rights (EU) | Directive 96/9/EC | Yes, held by the maker of the database | Belongs to whoever compiled it — usually not the individual |
| Trade secrets | Directive (EU) 2016/943 | Protectable, not licensable as such | Relevant to enterprise data |
| Sensor / connected-product data | Regulation (EU) 2023/2854 (Data Act) | Access right for the user; see §3 | An access right, not ownership |

**The conclusion the design must carry:** DUAP's lawful-basis field records
what a *controller asserts*. It is not a determination that the basis
applies, no document may describe it as one (rule 04, requirement 4), and
no interface may present consent as a sale (`docs/market/consumer.md`).

## 3. The EU Data Act

Regulation (EU) 2023/2854 entered into force 11 January 2024 and applies
from 12 September 2025, with design obligations for products placed on the
market from 12 September 2026 **[ss]**
([Regulation](https://eur-lex.europa.eu/eli/reg/2023/2854/oj/eng);
[Commission](https://digital-strategy.ec.europa.eu/en/policies/data-act)).

It establishes rules on access to connected-product and related-service
data for users, making data available to third parties, switching between
data processing services, and interoperability **[ss]**.

**Relevance:** it creates an obligation to make data available on defined
terms — the closest thing to a regulatory driver for usage accounting in
industrial and IoT contexts, and the reason `docs/market/go-to-market.md`
keeps industrial telemetry as the strongest second beachhead. Whether
compensation arrangements under the Data Act would benefit from shared
usage evidence is **interpretation-required** and untested.

## 4. Jurisdictional map

Each row is a question for counsel, not an answer.

| Jurisdiction | Instruments to analyse | Principal question |
|---|---|---|
| EU | GDPR 2016/679; Data Act 2023/2854; DGA 2022/868; AI Act 2024/1689; Database Directive 96/9/EC; Trade Secrets Directive 2016/943 | Is a clearing node a data intermediation service? Do receipts assist art. 30 records? |
| EU (transfers) | GDPR arts. 44–49 | Do event metadata and pseudonyms constitute a transfer when a clearing node is outside the EEA? |
| US federal | Sectoral: HIPAA, GLBA, FCRA, COPPA | Does accounting for health, financial, credit or children's data pull the operator into a sectoral regime? |
| US states | CCPA as amended by CPRA, and comparable state laws | Do data-broker registration obligations attach to an accounting intermediary that never holds data? |
| UK | UK GDPR; Data Protection Act 2018 | As EU, separately |
| Asia-Pacific | PIPL (China); APPI (Japan); PDPA (Singapore); India's DPDP Act | Cross-border transfer and localisation constraints on a clearing node |

**UNKNOWN, and stated rather than filled in:** this author has not
verified the current text or status of the non-EU instruments in this
table from primary sources, and lists them as the analysis to commission
rather than as analysis performed.

## 5. Regulatory exposure created by DUAP's own design choices

The directive asks which parts of the system could trigger financial
regulation and to minimise unnecessary exposure. This is the analysis, and
each row's mitigation is already in the architecture.

| Activity | Risk | Design response |
|---|---|---|
| Holding customer funds pending settlement | Money transmission / payment services licensing | **Not done.** `duap-ledger` records what is owed; settlement is handed to ISO 20022 or an existing payment system. No funds are held |
| Netting obligations between parties | Possible clearing-house characterisation | Netting is computed and *proposed*; parties settle bilaterally. DUAP is not a central counterparty and takes no position |
| Issuing tradable credits or tokens | Securities / e-money | **Not done.** Prepaid credits are modelled in `docs/market/market-design.md` §7 as a customer prepayment, and derivatives are rejected |
| Publishing a price index | Benchmark regulation; manipulation | **Rejected** in market design §7 |
| Operating a marketplace matching buyers and sellers | Data brokerage registration | The reference implementation does not match parties. DUAP accounts for licences negotiated elsewhere |
| Intermediating data between holders and users | DGA data intermediation | §1.2. Open question for counsel |

**The pattern:** every high-exposure activity has been kept out of the
protocol, and in each case the same choice also kept the trust surface
small. Where regulatory caution and good architecture agreed, that is luck
worth noting rather than a principle to rely on.

## 6. What a DUAP artefact is, and is not, in a legal sense

| Artefact | What it is | What it is not |
|---|---|---|
| Grant | A machine-readable record of terms a party asserts it granted | A contract. The contract is the contract |
| Event | A signed assertion by a reporting party | Proof the described processing occurred |
| Receipt | A clearing node's signed statement about events it received | Proof those events were true or complete (RISK-01) |
| Lawful-basis code | The controller's assertion of a basis | A determination that the basis applies (rule 04) |
| Revocation | A record that withdrawal was communicated, with an effective time | Erasure of anything already copied or derived |
| Log inclusion proof | Evidence of when a receipt was committed | Evidence about the receipt's content being accurate |

**The one legal property worth claiming:** non-repudiation. A signed
event, anchored in a transparency log, is evidence that a specific party
made a specific statement at a specific time and cannot later produce a
different version without detection. In a dispute that is worth
something. It is emphatically not evidence that the statement was true,
and every artefact in this repository says so — including
`Receipt::claims()`, which returns the non-established claims as data so a
verifier cannot read the receipt without reading its limits.

## 7. Questions requiring counsel, in priority order

1. Is a clearing node that processes only digests and counters a data
   intermediation service under DGA art. 2(11)? **Blocks any hosted EU
   product.**
2. Do pseudonymous event records constitute personal data under GDPR
   art. 4(1) in the hands of a clearing node that cannot re-identify them?
   **Determines the entire privacy posture.**
3. Does a dataset commitment assist an AI Act art. 53(1)(d) training
   content summary, and under what conditions? **Determines the beachhead's
   regulatory pull.**
4. Do receipts constitute or assist art. 30 records of processing
   activities?
5. Does operating a clearing node outside the EEA effect a transfer under
   GDPR arts. 44–49?
6. Do US state data-broker registration regimes reach an accounting
   intermediary that never holds data?
7. Is a multilateral netting proposal, not executed by the operator,
   regulated activity in any target jurisdiction?

None of these has been answered. Each is a gate on a specific product
decision, and they are ordered by which decision they block.
