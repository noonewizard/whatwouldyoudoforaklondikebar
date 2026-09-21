# Economic model

**Status:** SPECIFIED (draft) · 2026-09-21

This document sets out the economic design: who pays whom, for what, how
the numbers are produced, and which of the resulting incentives the
protocol handles and which it does not.

No number here is a forecast. Where a figure appears it is either measured
(and cited) or an illustration (and labelled).

## 1. The flow

```
  controller ──charge──▶ clearing node ──share──▶ subject
       │                      │
       │                      └──fee──▶ clearing node revenue
       │                      └──tax──▶ tax authority (hook only)
       └──────────── data stays here ─────────────
```

A controller pays for authorised use of a subject's data. The clearing node
splits the charge into a subject share, a clearing fee and any tax, and
posts all of it to a double-entry ledger before any money moves.

The subject's share is a **liability from the moment it accrues**, not from
the moment it is paid. That is the point of running an accounting layer
rather than a payment system: what is owed exists on the books whether or
not it has crossed a rail.

## 2. Separation of measurement, valuation and settlement

This separation is architectural and load-bearing:

| Layer | Question | Crate | Can it see the others? |
|---|---|---|---|
| Measurement | how much happened? | `duap-meter` | no pricing |
| Valuation | what does that cost? | `duap-valuation` | no metering |
| Accounting | who owes whom? | `duap-ledger` | takes priced quantities |
| Settlement | how does money move? | rails | takes balances |

The reason is adoption, not tidiness. Two parties who disagree completely
about what data is worth can still agree on what happened. A protocol that
fuses measurement and valuation forces agreement on the harder question
first, and gets neither.

## 3. What is charged for

Only what can be counted without judgement:

| Unit | Counted when |
|---|---|
| record | a record is collected, read, transformed or transferred |
| query | a query touches the subject's data |
| inference | an inference is produced using it |
| subject-day | a subject's data is retained for a day |
| token | a training token is attributable to it |
| impression | an advertising impression is informed by it |
| byte | data is at rest or in transit |
| retrieval | the record enters a context window |

`share` is a dimensionless attribution weight and is explicitly
**non-additive**: summing shares across scopes is meaningless, and the
aggregator refuses rather than producing a plausible wrong number.

## 4. Price formation

```
charge = base_rate(unit) x quantity x PI(multipliers)
```

Multiplicative, exact, and itemised: every price carries the list of
factors that produced it, and recomputing the product from the list must
reproduce the combined factor (tested).

The multipliers are functions of properties the protocol already records:
sensitivity tier, re-identification prior, special-category status,
retention basis, exclusivity, cross-border transfer, and freshness decay.

**The coefficients are parameters, not findings.** They are chosen to be
*ordinally* defensible — more sensitive costs more, exclusive costs more,
stale costs less — and nothing stronger. `VALUATION.md` gives the
calibration procedure a deployment should run against its own realised
transactions, and records that we have not run it, because there is no
market to calibrate against.

Six pricing rules are supported: free, per-unit, a unit table, marginal
tiers, revenue share, and auction. A grant carries the rule; a schedule can
be referenced by pinned digest so it cannot be edited behind the grant.

## 5. Why there is no universal price

Three reasons, each sufficient:

1. **Value is combinatorial.** The marginal contribution of one subject to
   a corpus of ten million is close to zero even when the corpus is
   valuable. Pricing per subject at the corpus's average value would
   overstate by orders of magnitude; pricing at the true marginal value
   would produce amounts below any payment rail's floor.
2. **Value is contextual.** The same location record is worth a different
   amount to a logistics optimiser, an advertiser and an insurer, and the
   difference is not a multiplier on a common base.
3. **There is no market.** Prices are discovered by trade. No liquid market
   in individual data authorisations exists, so any "correct" price today
   is an assertion.

What the protocol contributes is not the price but the *machinery*: a
signed, comparable, reproducible record of what was used and what was
agreed. Prices can then be discovered by the parties, by a market, or by a
regulator, and the accounting layer works the same way under all three.

## 6. The micropayment problem

At realistic rates a subject earns fractions of a cent per period.
Measured in the demonstration: one subject, one day of moderate service
usage, 0.19 EUR — and that is with deliberately generous illustrative
rates.

No payment rail moves 0.19 EUR economically. Card and bank transfers cost
cents to tens of cents each.

DUAP's answer, in order:

1. **Never round to zero.** Amounts accrue at nano-minor-unit precision.
2. **Carry the balance as a liability.** It is owed from accrual.
3. **Release on a threshold** the deployment sets from its own rail costs.
4. **Say what happens to dust.** Balances that never cross the threshold
   are a real problem with no clean answer. The options — escheat to the
   state, assignment to a nominated charity, expiry after notice,
   aggregation into a collective fund — differ in their legal treatment by
   jurisdiction, and the protocol takes no position. It exposes
   `PayoutAccumulator::dust` so a deployment cannot avoid noticing.

The honest summary: **individual micro-compensation is a weak part of this
design.** It works arithmetically and it may not work economically. Two
alternatives are more likely to matter in practice and both are compatible
with the protocol: business-to-business licensing, where the amounts are
large and the accounting layer's value is the audit trail; and collective
bargaining, where a union or data trust negotiates on behalf of many
subjects and the per-subject split is internal to it.

## 7. Incentive analysis

For each participant: what the protocol makes rational.

### The controller

Pays per reported use. Its incentive is to **under-report**.

Controls: per-stream sequence numbers make partial suppression visible;
double-count detection stops a processor's report being discarded silently;
audit sampling against the controller's own systems is the only real
check, and it is an engagement rather than a protocol feature.

**This is the weakest incentive in the design and it is structural.** The
party that pays is the party that reports. Every accounting system with
that shape — tax self-assessment, emissions reporting, royalty statements —
relies on audit and penalty rather than on the reporting mechanism.

### The subject

Paid per reported use. Its incentive is to **over-report**, and it cannot:
events are signed by the controller. It can refuse authorisation, which is
its actual lever.

Sybil subjects (T-17) do not pay, because compensation follows usage a
controller reports, and a controller has no reason to report usage about a
person it has no relationship with. Had the design paid per enrolment, this
would be fatal — which is why it does not.

### The clearing node

Earns a fee on cleared volume. Its incentive is to **maximise cleared
volume**, which aligns with accepting events and misaligns with rejecting
fraudulent ones.

Controls: receipts commit to event-set roots a subject can check; the log
is public; a node that clears fabricated volume is detectable by any
subject who checks a receipt against their own records.

**Residual:** a fee proportional to volume is a bad incentive. A flat
subscription, or a fee on *disputed* volume rebated on resolution, would be
better. Left as a deployment choice with the analysis recorded, because the
protocol does not mandate a fee structure.

### The processor

Reports on behalf of a controller. Its incentive is to **over-report** if
paid per report, which is why the default double-count policy prefers the
controller's report and why the collision is recorded rather than silently
resolved.

### The auditor

Paid to check. Its incentive depends entirely on who pays it — the failure
mode of every audit market. `GOVERNANCE.md` makes auditor independence a
governance requirement, which is a statement of intent, not a mechanism.

## 8. Clearing and netting

Gross obligations are netted multilaterally: each participant's net
position is computed and debtors are matched to creditors greedily,
producing at most *n*−1 transfers instead of up to *n*² .

The matching is not guaranteed to minimise the transfer *count* — that is
NP-hard — and an approximation whose output participants must audit is not
worth the saving. What is guaranteed and tested: positions settle exactly,
and gross positions remain reconstructible from the ledger, so a
liquidator can unwind.

Whether netting is legally effective against an insolvent participant
depends on the netting agreement and the applicable insolvency law. The
protocol records; it does not determine.

## 9. Economic attacks and what stops them

| Attack | Control | Disposition |
|---|---|---|
| Inflate counters | The inflater pays | deterred |
| Suppress events | Sequence gaps | partially detected |
| Double-report | Operation fingerprint | detected |
| Edit a schedule after the fact | Pinned digest | prevented |
| Skim by repeated rounding | One rounding per line, residue posted | prevented |
| Split transactions across tiers | Aggregation windows; schedule design | not prevented |
| Shill bidding in an auction | Commit-reveal, logged commitments, enrolled bidders | deterred |
| Exploit FX rates | Cross-currency movements are two entries through an FX account, never one | prevented |
| Settle and not pay | Ledger liability persists | out of scope, bounded |

## 10. Where attribution stops

Attribution through the derivation graph must terminate or every statistic
traces to everyone. Four termination rules, all parameters:

1. **Depth.** A maximum number of derivation hops.
2. **Negligible share.** Shares below a floor are dropped *and counted*, so
   the shortfall is visible rather than silent.
3. **Terminal node kinds.** A deployment may declare, for example, that a
   differentially private release terminates.
4. **Severing edges.** An edge marked severed carries lineage for audit but
   no attribution; used where a transformation is agreed or adjudicated to
   extinguish the upstream claim.

Note what rule 3 does *not* say: differential privacy bounds what can be
learned about an individual, and the protocol does not assert that this
extinguishes an economic claim. Whether it should is a policy question, and
the protocol makes it one.

## 11. Open questions

Recorded rather than resolved:

- Is individual micro-compensation viable at all, or is the realistic unit
  of compensation a collective?
- What fee structure for a clearing node avoids rewarding volume?
- How should dust be treated, given that the answer differs by
  jurisdiction?
- Does a market in data authorisations have enough participants on both
  sides to discover a price, or is it structurally thin?
- Does making data usage priced *increase* collection, by making it a
  budgeted input rather than a free one?

The last is the most uncomfortable and the least studied. A protocol that
prices data usage could plausibly increase it, by converting an
unquantified reputational risk into a quantified, budgetable cost. Nothing
in this repository resolves that, and it is recorded in
`docs/research/0005-does-pricing-increase-collection.md`.
