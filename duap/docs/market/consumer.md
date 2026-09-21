# Individuals: what is left after the marketplace is removed

**Status:** REFERENCE · 2026-09-21
**Owner:** `consumer-product`, reviewed by `privacy-engineer` and
`legal-research`

`docs/reviews/falsification.md` kills the consumer data marketplace twice
over — on unit economics (§2, §4) and on rights (§8). This document exists
so that "no marketplace" does not quietly become "no individuals", and so
that nobody rebuilds the wallet later without reading why it was removed.

## What was removed, and why it stays removed

| Removed | Reason |
|---|---|
| A marketplace where individuals sell data | Gross value per person is smaller than the cost of transacting it |
| Individual data income as a product promise | Same, plus it is a promise the system cannot keep |
| Per-use consent prompts | Consent fatigue at the frequency real collection occurs; the honest alternative is a standing policy, which is what cookie banners already are |
| Any framing of privacy rights as a licensable asset | Data-protection rights are control rights, not property. Licensing them is at best void |
| "Data assets: 42 / pending obligations: $183.42" dashboards | The numbers would be made up, and the framing implies an income stream that does not exist |

The last row deserves emphasis because it was in the original brief as a
mock interface. A dashboard showing a person a pending balance creates an
expectation of money. Where the money is a few dollars a year, the
interface is a misrepresentation regardless of the arithmetic behind it.

## What survives

### 1. Individuals as beneficiaries of collective arrangements

Collective licensing is the mechanism that works where individual
licensing does not, and it has a live instance: the RSL Collective, a
non-profit collective rights organisation founded in 2025 for web
publishers **[search-summary]**. Copyright markets reached the same answer
a century earlier.

In this model an individual joins a cooperative, the cooperative negotiates
once with buyers, and DUAP accounts for aggregate usage and distributes
proceeds. The individual never sees a per-event decision, because there is
nothing useful for them to decide at that granularity.

What DUAP contributes here is specific and small: `duap-valuation::distribute`
implements largest-remainder apportionment with a conservation invariant,
so a pool divides without losing or inventing money, and
`PayoutAccumulator` carries a residue rather than dropping dust. That is
the honest scope — cooperatives need correct distribution arithmetic and an
auditable record of it, not a marketplace.

**What remains unresolved:** cooperative governance, free-riding, member
bargaining power and whether a cooperative can negotiate better terms than
its members individually. `docs/research/` holds these as open questions.
The economic answer is not obvious: a cooperative's bargaining power
depends on whether its members' data is substitutable, and for most
consumer data it is.

### 2. Individuals as auditors of rights they already hold

This is the part with no revenue and possibly the most value.

Under GDPR, a data subject has a right of access (Regulation (EU) 2016/679
art. 15) and controllers must maintain records of processing activities
(art. 30). Those rights exist whether or not anyone is paid. A DUAP
receipt is a machine-readable, verifiable answer to "what did you do with
my data, under what basis, when" — and a subject can check a receipt
against the clearing node's public key without trusting the controller.

The product here is **transparency, not income**, and it must be described
that way. It is also the only individual-facing use that survives the
arithmetic, precisely because it does not require a transaction.

**Its limits, in the same breath:** a receipt proves what a controller
*reported*, not what it did (RISK-01); usage outside instrumentation is
invisible (RISK-02); per-controller pseudonyms prevent joining on the
identifier and do nothing against joining on content. A subject reading a
receipt is better informed than one reading a privacy policy, and is not
in possession of proof.

### 3. Individuals as revocation principals

Withdrawal of permission is a right, not a transaction, and DUAP
represents it correctly: revocation binds from an effective time, future
use is refused, and the demonstration prints
`already_trained_model=NOT_UNLEARNED` rather than implying the model
forgets. The value to an individual is that a withdrawal becomes a
checkable event with a timestamp rather than an email nobody can produce
later.

## Design rules for anything individual-facing

1. **Never show a projected or pending income figure.** Show settled
   amounts only, and only where they exist.
2. **Never present consent as a sale.** The lawful-basis field records what
   a controller asserts; no interface may present it as a determination or
   as a transfer of ownership.
3. **Never imply erasure of derived artefacts.** Say what was deleted and
   what was not.
4. **Never require the person to understand cryptography** — and never let
   the absence of that requirement become an implied guarantee. "Verified"
   in an interface must be a link to what was and was not established.
5. **No dark patterns, and the specific ones to refuse:** default-on
   sharing, pre-ticked purposes, a price shown without its duration and
   downstream rights, a "decline" path longer than the "allow" path, and
   any payout threshold not disclosed before the person opts in.

## The falsification test for this document

If a cooperative can be found that (a) has members, (b) has a buyer, and
(c) currently distributes proceeds using a spreadsheet it does not trust —
then the distribution-and-evidence role is real and worth building for. If
no such cooperative exists, or they are content with the spreadsheet, then
nothing individual-facing should be built at all and this document reduces
to a record of why.

Nobody has been asked. That is the honest state of this analysis.
