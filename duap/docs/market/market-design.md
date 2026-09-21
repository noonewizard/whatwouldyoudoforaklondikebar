# Market design

**Status:** REFERENCE · 2026-09-21
**Owner:** `market-design`, reviewed by `economics-engineer`

This document designs the market mechanisms DUAP supports, and — more
usefully — establishes which ones it should refuse to support. The
architectural rule the directive makes mandatory is that **measurement and
valuation are separate**, and it is enforced here by keeping every pricing
mechanism out of the protocol and in policy.

## 1. What is actually being sold

The directive asks whether the transactable object should be raw data,
access, a licence, a usage right, a query, computation, a derived-data
right, a training right, an inference right, attribution, a royalty, a
credit, a subscription or a pool share. The answer the evidence supports is
narrower than any of those lists suggests.

**What is sold is a *right to perform a defined use*, and what is priced is
*metered exercise of that right*.** The distinction matters because it
determines what must be measured:

| Candidate object | Verdict |
|---|---|
| Raw data | Sold today, and the transaction ends at delivery. No accounting layer is needed or wanted |
| Access | Same. A credential is not an accounting object |
| **A usage right** | **This is the object.** It is what a grant expresses and what an event exercises |
| A query | A *unit*, not an object — one way to meter the exercise of a usage right |
| Computation | Same: a unit |
| Training right / inference right | Purposes within a usage right, not separate objects. Modelling them separately duplicates the grant |
| Derived-data right | A constraint inside a usage right (derivation depth, downstream permissions), not a thing sold alone |
| Attribution | Not sellable. `research/ai-attribution/RESULTS.md` establishes it cannot be measured |
| Royalty / revenue share | A *pricing rule*, and one whose measurement depends on the buyer's revenue, not the seller's data. See §5 |
| Credits, futures, options, pool shares | Financial instruments. See §7 — mostly rejected |

Collapsing this to one object and a set of units is what keeps the protocol
small. The kernel has five objects; adding "training right" and "inference
right" as first-class types would have added two more and expressed nothing
new.

## 2. Pricing models, and which the protocol must support

The protocol does not choose a pricing model. It must be able to *carry*
the outcome of one, which is a narrower requirement and the reason
`duap-valuation` is a separate crate from `duap-meter`.

| Model | Supported | Mechanism |
|---|---|---|
| Fixed fee | Yes | A term with a flat rule; no metering needed, but events still record what happened |
| Per-record / per-query / per-byte / per-token / per-inference | Yes | `PricingRule::UnitTable`, keyed by metered unit (VS-3) |
| Per-minute / per-duration | Yes | A unit like any other |
| Subscription with included volume | Yes | Flat rule plus an overage rule; the counter decides which applies |
| Tiered / volume-discounted | Yes | A unit table with breakpoints |
| Auction | Yes, optionally | `duap-valuation::auction`, second-price sealed-bid with commit-reveal. Off the critical path |
| Negotiated | Yes | The negotiated number becomes a term. DUAP does not conduct the negotiation |
| Royalty on downstream revenue | **Conditionally** | See §5. Supported as an *obligation*, not as something DUAP can measure |
| Contribution-based | **No** | Refused. Cannot be measured; see `AI_ATTRIBUTION.md` |

The one refusal is the important row. A protocol that supported
contribution-based pricing would be asserting that contribution is
measurable, which this project's own research says it is not.

## 3. Price discovery, and why the formula in the brief is wrong

The directive proposes, and then asks to have attacked:

```
Price = Quantity × Base × Quality × Freshness × Exclusivity
              × Sensitivity × Purpose × Geography × Risk × Demand
```

**The attack.** Of the ten factors, `Quantity` is measured, and the rest
divide into two groups that should never have been multiplied together.

| Factor | Status |
|---|---|
| Quantity | **Measured.** The only one |
| Freshness | **Measurable.** Event timestamp minus resource timestamp |
| Exclusivity | **Contractual.** A fact about the grant, not an estimate |
| Geography, Purpose | **Categorical.** Facts about the event, usable as table keys |
| Quality | **Not measurable in general.** Domain-specific and contested |
| Sensitivity | **A policy judgement**, frequently a legal one |
| Risk | **A judgement**, and the buyer's and seller's differ |
| Demand | **Circular.** It is the price, not an input to it |
| Base rate | **The negotiated number.** Everything else is a modifier on it |

Multiplying a measurement by seven judgements produces a number with the
appearance of a calculation and the content of an opinion — and multiplying
by `Demand` makes it circular. The formula's real defect is that it hides
where the judgement entered.

**What DUAP does instead.** `duap-valuation::multiplier` implements a
multiplier policy whose coefficients are labelled, in the crate
documentation, as *parameters chosen to be ordinally defensible, not
measurements of what data is worth*. The crate's status line says no claim
is made that any number in it is the right price for anything. The
multipliers exist so that a party who wants to express "precise location is
worth more than coarse location for advertising" can do so consistently and
reproducibly; they do not discover a price.

**Where price actually comes from:** negotiation, or an auction, or a
posted rate. The market discovers it. The protocol records it and applies
it reproducibly, which is a smaller and achievable job.

## 4. Do not claim data has one value

The directive requires this and the design honours it by construction: the
same event can be priced differently by different parties, because pricing
is a function of the *grant term*, and two grants over the same resource
can carry different terms. Cost, market price, utility, strategic value,
replacement cost, marginal value, option value and revenue contribution are
distinct quantities, and DUAP represents exactly one of them — the
contractually agreed price for a metered exercise — and is silent on the
other eight.

A concrete consequence worth stating: DUAP cannot tell a subject whether
they were paid fairly. It can tell them what they were paid, under which
term, for what measured quantity, with evidence. Conflating those two would
be the most damaging claim this project could make.

## 5. Royalties on downstream revenue

The directive asks whether data royalties are viable — $100,000 upfront
plus 0.05% of defined downstream revenue.

**The mechanism works; the measurement does not.** DUAP can represent the
obligation, attach it to a grant, and account for a royalty payment once an
amount is known. What it cannot do is establish the amount, because the
input is the *buyer's revenue*, which is inside the buyer's accounting
system and outside any boundary DUAP observes.

This is classified honestly in the obligation taxonomy: a royalty is an
**Economic** obligation — priced at settlement, not enforced by the
protocol. Enforcement is an audit clause in a contract, exactly as it is in
music and publishing royalties, and those industries' long history of
royalty disputes is the realistic expectation, not an edge case.

**Verdict:** viable as a contract, supported as an accounting object, and
never to be described as verifiable. A protocol that implied it could check
a licensee's revenue would be lying.

## 6. Liquidity and the cold start

A two-sided market needs both sides, and DUAP's position is unusual:
**it does not need liquidity to be useful.** A single pair of
counterparties under one metered licence gets value from a shared,
verifiable accounting of that licence — the reconciliation problem is
bilateral before it is ever a market.

This is the most important structural advantage available and it should
shape everything: DUAP is a *reconciliation* protocol that can become a
market protocol, not a market that must be bootstrapped. `docs/market/go-to-market.md`
builds on it.

The minimum viable network is therefore **two**, and the interesting
threshold is the one at which netting starts to pay — roughly when a
participant has obligations with enough counterparties that multilateral
netting beats bilateral settlement. `duap-ledger::net` implements it; at
what participant count it matters commercially is UNKNOWN.

## 7. Financial instruments: mostly rejected

The directive asks about data futures, options, credits, indexes and
insurance, and asks to be careful about securities law.

| Instrument | Verdict |
|---|---|
| Prepaid usage credits | **Acceptable.** A prepayment against future metered use is ordinary commerce. Note that holding customer funds may raise money-transmission questions in some jurisdictions; see `docs/legal/regulatory-matrix.md` |
| Receivables from cleared obligations | **Acceptable, with care.** An invoice is a receivable; factoring receivables is an existing market and not DUAP's business |
| Data futures / options | **Rejected for now.** A derivative on a data price requires a reliable reference price, which §3 says does not exist. Building derivatives on a price nobody can verify is how a market becomes manipulable, and it attracts securities and commodities regulation for no corresponding benefit |
| Data price indexes | **Rejected as a product, retained as a research question.** An index over unverifiable, heterogeneous, mostly-private prices would be a number with a decimal point and no content. If DUAP ever carries enough comparable cleared transactions, the question can be revisited with data |
| Data insurance | **Not DUAP's business.** Insurers may find receipts useful as evidence, which is an integration, not a product |

The rejections share one reason: each requires a trustworthy price, and
§3 establishes that DUAP does not produce one.

## 8. What the protocol must never do

- Choose a price. It records one.
- Assert that a price is fair, correct or market-rate.
- Support contribution-based pricing.
- Present a royalty obligation as verifiable.
- Publish an index or reference price before the underlying transactions
  exist and are comparable.
- Take a fee proportional to the value flowing through it. This is a
  business-model constraint with a market-design reason:
  `economics/results.md` finding 5 shows the fee would be large, and a
  large fee proportional to value is the strongest possible incentive to
  route around the protocol.
