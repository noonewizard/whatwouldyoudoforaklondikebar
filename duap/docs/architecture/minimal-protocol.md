# The minimal protocol

**Status:** SPECIFIED · 2026-09-21 · supersedes nothing

This document answers the question the whole project turns on: what is the
*smallest* thing that has to be standardised for a data usage accounting
layer to exist, and what must deliberately stay out of it.

The test it is written against: **an independent organisation must be able
to implement the protocol kernel from this document and the specification it
points to, without adopting any of this repository's software.** If the
kernel cannot be independently implemented, it is not a protocol; it is a
product with a specification attached.

---

## 1. What is the irreducible primitive?

**A signed, canonically encoded assertion that a specific operation was
performed on a specific class of data about a specific pseudonymous subject,
under a specific authorization.**

Everything else in DUAP is derived from that object:

- metering is counting them;
- provenance is linking them;
- valuation is pricing them;
- accounting is posting them;
- clearing is aggregating, receipting and invoicing them;
- settlement is discharging what the accounting says is owed.

The primitive is an *assertion*, not an observation. This is the single most
important framing decision in the project, and the rest of the design
follows from taking it seriously: no software can verify that a claim about
the world is true. What a protocol can do is make the assertion **specific,
attributable, non-repudiable, time-bounded and comparable** — so that a
false assertion is a provable falsehood rather than a difference of opinion.

### Why not "the data itself"

A protocol whose primitive is data has to move data, which means it has to
solve confidentiality, residency and minimisation before it can account for
anything. A protocol whose primitive is an *assertion about* data moves only
digests, and can be deployed into an existing system without changing where
that system's data lives. See `docs/adr/0001-scope.md`.

### Why not "the consent record"

Consent-record protocols describe what was permitted. They cannot describe
what was done, so they cannot support accounting, pricing or dispute. DUAP
carries the authorization too, but the *primitive* is the use, not the
permission.

---

## 2. What is a Data Event?

A **Data Usage Event** is the canonical encoding of the primitive.
Normatively specified in `specs/protocol-v0.1.md` §4; the model is
`crates/duap-model/src/event.rs`.

It carries, and carries nothing else:

| Group | Fields | Why it is irreducible |
|---|---|---|
| Identity of the record | schema version, event id, occurrence and record times, agent, sequence | Without a sequence, suppression is undetectable |
| Parties | controller, optional processor | Accounting needs an accountable party |
| Subject | subject scope (pseudonym, cohort, or non-personal) | Attribution needs a subject; privacy forbids a global one |
| Classification | data class, effective sensitivity, collection method | Pricing and policy are functions of these |
| Action | operation, purpose, optional lawful-basis assertion | The operation is what is being accounted for |
| Authority | authorization reference: grant, epoch, grant digest | A use with no cited authority is not accountable |
| Measurement | quantity (unit and amount) | Accounting needs a number |
| Lifecycle | retention policy | Retention is a priced and policed dimension |
| Lineage | provenance inputs, output, transform | Derived-rights propagation needs an edge |
| Economics | optional counterparty, declared revenue, exclusivity | Only present where the use has a commercial character |
| Evidence | optional salted commitment to the value | Disputes need something to open |
| Extension | namespaced extension map | Evolution without a version bump |

**What it deliberately does not carry:** the data. Not the location, not the
message, not the identifier. The strongest statement the protocol makes
about content is a salted hash commitment.

---

## 3. What is a Data Usage Right?

A **right** in DUAP is not an object. It is the *conjunction* of:

1. a **grant** — a signed authorization document from the subject (or the
   rights-holder) to a controller, at a specific epoch; and
2. a **decision** — the deterministic evaluation of that grant against a
   specific event.

Making the right a derived thing rather than a stored object is deliberate.
A stored "right" would need to be revoked, expired, transferred and
reconciled as a separate lifecycle, and the divergence between the stored
right and the grant it came from would be the protocol's main bug source.
Evaluating on demand means there is exactly one source of truth and it is
signed.

A grant answers ten questions, and the language is built so none can be left
implicit: **who, to whom, over what data, for what purpose, where, from
when, until when, with what onward and derivative rights, at what price, and
how it can be withdrawn.** `specs/protocol-v0.1.md` §5.

---

## 4. What is a Data Usage Receipt?

A signed statement by a clearing node that, over a stated period, a stated
controller performed a stated quantity of a stated operation on a stated
class of data about a stated subject, under a stated authorization, at a
stated charge — with the supporting events committed by a Merkle root that
is anchored in a transparency log.

A receipt establishes exactly five things and explicitly denies five others.
The list is in `specs/protocol-v0.1.md` §6 and is returned programmatically
by `Receipt::claims()`, so a user interface shows the same list the
specification states.

The distinction that matters: an **acknowledgement** is per-event, immediate
and says "I hold this"; a **receipt** is per-period, priced and is an
accounting document. Conflating them is a category error.

---

## 5. What must be cryptographically verifiable?

Six things, and nothing else needs to be:

1. **Authorship** of every protocol object — digital signature.
2. **Integrity** of every protocol object — the signature covers a
   domain-separated digest of canonical bytes.
3. **Binding between objects** — an event cites a grant's digest, a receipt
   cites an event-set root, an invoice line cites the same root. No object
   can be swapped for a similar one.
4. **Existence by a time** — inclusion in a transparency log whose signed
   tree heads are dated, plus consistency proofs against history rewriting.
5. **Coverage** — that a receipt's charge is computed over exactly a stated
   set of events, checkable by anyone who obtains the set.
6. **Arithmetic** — that the charge follows from the quantity, the rule and
   the multipliers, recomputable by both parties.

Everything else — that the operation happened, that nothing was omitted,
that the price is fair, that a deletion promise will be honoured — is
**outside** what cryptography can deliver, and the protocol says so in the
receipt itself rather than in a footnote.

---

## 6. What must remain off-ledger?

- **All personal data**, without exception. The log commits to digests.
- **Event bodies.** They stay with the controller; the log carries a batch
  root.
- **Grant contents.** A grant is a subject's policy; publishing it publishes
  the subject's preferences. The log carries its digest.
- **Prices in a bilateral agreement.** A schedule may be published; a
  negotiated price is referenced by digest.
- **Identity bindings.** The mapping from pseudonym to person is never in
  the protocol, on a ledger or otherwise.

The rule: *the log is a commitment device, not a database.* If removing a
value from the log would break a verification, it belongs there; otherwise it
does not.

---

## 7. What must be economically measurable?

Only what can be counted without judgement:

| Measurable now | Unit |
|---|---|
| Records collected, read, transformed, transferred | record |
| Queries that touched the subject's data | query |
| Inferences produced | inference |
| Subject-days of retention | subject-day |
| Tokens attributable to a training run | token |
| Impressions informed by the data | impression |
| Bytes at rest or in transit | byte |

These are measurable because the instrumented system already knows them: it
is counting things it does anyway. The protocol's contribution is to make
the count signed, non-duplicated and comparable.

**Valuation is strictly separate from measurement**, and the separation is
architectural: `duap-meter` cannot price and `duap-valuation` cannot meter.
That boundary is the reason the accounting layer can be adopted by parties
who disagree completely about what data is worth.

---

## 8. What cannot currently be measured reliably?

Stated plainly, because a protocol that pretends otherwise will produce
confident wrong numbers:

1. **The contribution of one datum to a trained model.** Shapley values,
   influence functions and leave-one-out estimates all exist, all disagree,
   all cost more than the model's training run at realistic scale, and none
   of them is a measurement of economic contribution. DUAP can prove
   *inclusion*. See `AI_ATTRIBUTION.md`.
2. **Usage outside instrumentation.** The protocol sees what it is told. A
   copy taken outside it is invisible; sequence gaps narrow this and do not
   close it. See §41 of the design brief and `THREAT_MODEL.md` T-20.
3. **The market value of a specific person's data.** There is no liquid
   market, most value is combinatorial, and the marginal value of one
   subject in a large corpus is usually near zero even where the corpus is
   valuable. `VALUATION.md`.
4. **Whether an anonymisation claim holds.** Re-identification risk depends
   on auxiliary data that nobody enumerates. The protocol records the claim
   and the method, not a verdict.
5. **Whether a deletion happened.** Deletion is attested, not observed.
6. **Whether a purpose assertion is true.** "For fraud detection" is a
   controller's characterisation of its own intent.

Each of these is a research question with an entry in `docs/research/`,
not a gap to be papered over.

---

## 9. What should explicitly NOT be part of the core protocol?

The kernel is small on purpose. These are excluded, with the reason:

| Excluded | Why |
|---|---|
| A consensus mechanism | Nothing in the protocol requires agreement between mutually distrusting validators. A signed append-only log with consistency proofs provides detection of equivocation at a fraction of the cost. `docs/adr/0005`. |
| A currency or token | Settlement is a rail interface. Requiring a token would couple the accounting layer to a monetary policy and to a jurisdictionally fraught asset class. |
| A pricing model | The protocol carries *a* price and the arithmetic to reproduce it. Standardising *the* price would be standardising an answer that varies by sector, jurisdiction and party. |
| AI influence attribution | Not measurable to a standard that could bear economic weight. Kept EXPERIMENTAL and outside every core dependency. |
| Identity binding | Binding a pseudonym to a legal person is a registry function with its own regulation. The protocol needs identifiers to be stable and auditable, not real. |
| A tax engine | Determination depends on facts the protocol does not have. It provides hooks and records what a determination would need. |
| A dispute adjudicator | Questions of fact are settled by people. The protocol makes the evidence portable and checkable. |
| Enforcement against non-participants | Software cannot compel an organisation to report activity that happens entirely outside it. §41 of the brief; `docs/research/0003-uninstrumented-participants.md`. |
| Data transport | DUAP accounts for data movement; it does not move data. |
| A schema for the data itself | The taxonomy classifies data; it does not model it. |

---

## 10. The kernel

Putting the answers together, the kernel is four documents and five
objects.

**Documents:** the canonical encoding rules, the object schemas, the
signature construction, and the evaluation semantics.

**Objects:** Event, Grant, Revocation, Receipt, Log entry.

**Conformance levels** (`spec/vectors/`): L1 encoding and digests, L2
signatures, L3 proofs and receipts, L4 authorization evaluation, L5 pricing
arithmetic. An implementation states its level; a browser extension that
checks receipts needs L3, a clearing node needs L5.

Everything else in this repository — the clearing node, the ledger, the
gateway, the CLI, the SDKs, the dashboards — is *a* reference
implementation of the parts above the kernel. A conforming implementation
may share none of it.

### Evidence that the kernel is independently implementable

`gateway/` contains an implementation of L1-L3 written in Go against this
specification rather than translated from the Rust. It passes all 77
vectors in its range. Writing it found one defect in the data model (an
unnecessary arbitrary-precision requirement in the JSON view, now removed),
which is the kind of thing only a second implementation finds.

That is evidence, not proof: the same author wrote both, and the second
implementation covers three of five levels. The claim this document makes is
therefore bounded — *the kernel has been implemented twice, independently
enough to find a specification defect* — and the stronger claim waits for an
implementation by someone else.
