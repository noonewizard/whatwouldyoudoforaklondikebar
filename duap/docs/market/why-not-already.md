# Why has this not already happened?

**Status:** REFERENCE · 2026-09-21
**Owner:** `economics-red-team`

The directive treats this as one of the most important questions in the
investigation, and it is right to, because the default reading of an empty
market is the wrong one. An absent market is evidence about economics
before it is evidence about opportunity. This document works through the
candidate explanations and separates the ones DUAP addresses from the ones
it does not — and the second list is longer.

**Retrieval limits** are as stated in `docs/market/competitive-landscape.md`:
market facts below arrived as web-search summaries and are marked **[ss]**.

## The candidate explanations, scored

For each: is this a real cause of the missing market, and does DUAP address
it?

| # | Candidate cause | Real? | DUAP addresses? |
|---|---|---|---|
| 1 | Nobody built the technology | **No** | — |
| 2 | Per-participant value is too low to transact | **Yes, decisive** | **No** |
| 3 | Legal rights are unclear for individuals | **Yes, decisive** | **No** |
| 4 | Usage cannot be observed once data leaves | **Yes** | **Partially** |
| 5 | No standard unit of account | **Yes** | **Yes** |
| 6 | Verification costs exceed transaction value | **Yes** | **Partially**, via aggregation |
| 7 | Attribution of value is impossible | **Yes** | **No** — it declines the question |
| 8 | Transaction costs of negotiation | **Yes** | **No** — RSL and collectives address it |
| 9 | Incumbents prefer opacity | **Yes** | **No** |
| 10 | Chicken-and-egg between buyers and sellers | **Yes** | **No** |
| 11 | Privacy law prohibits the transactions | **Mostly no** | — |
| 12 | Demand did not exist | **No longer true** | — |

Five rows say "no" or "no longer true", and three of the "yes" rows are
ones DUAP does not address. That ratio is the honest summary of this
document.

## 1. "Nobody built the technology" — false, and the most dangerous belief available

Signatures, Merkle logs, policy languages, provenance models, metering and
double-entry accounting are all decades old. Every component of DUAP is
assembled from published, standardised primitives; `docs/standards/prior-art.md`
finds nine existing standards it should bind and three objects it needs to
define.

Any plan that implicitly assumes technical novelty was the blocker is
already wrong. The blockers below are economic, legal and institutional,
and a protocol addresses at most one of them.

## 2. Per-participant value is too low — decisive, and DUAP does not fix it

Worked in full in `docs/reviews/falsification.md` §2. Reported estimates of
personal data value cluster around $10 per person per year to a platform,
with whole-industry figures reaching a few hundred dollars for the highest-value
users **[ss]**. ARPU is an upper bound that overstates the data component
substantially, because it includes the platform's product and distribution.

A market cannot exist where the gross value of a participant's position is
smaller than the cost of establishing, metering, verifying, disputing and
settling it. This is arithmetic and no protocol improves it. It is the
primary reason the individual data market does not exist, and it explains
Datacoup's reported 2019 shutdown **[ss]** without needing a second cause.

**What it does not explain** is the absence of an *enterprise* usage
market, where positions are large. For that the causes are 4, 5, 8, 9 and
10.

## 3. Legal rights are unclear for individuals — decisive, and DUAP does not fix it

A person's data-protection rights are control rights, not property.
Copyright may sit with a creator; database rights with a compiler;
sensor-generated data with a device maker or an operator; employment-generated
data with an employer. `docs/legal/regulatory-matrix.md` works this
through. The result is that for most data associated with a person, that
person has no transferable economic right to license, and a market cannot
form around an asset that does not exist.

This is a legal fact, not a technical gap, and a protocol that recorded
such licences would be recording void ones.

## 4. Usage cannot be observed once data leaves — real, and where DUAP has something

This is the classic explanation and it remains true. Once a copy exists
outside an observed boundary, no cryptography detects what happens to it;
`security/findings.md` RISK-02 accepts this explicitly.

The reason it is worth attacking anyway is that the market has partially
routed around it in one place and not others. At the crawl boundary,
Cloudflare put an intermediary in the path and metered it — over a billion
HTTP 402 responses per day is reported **[ss]** — which works precisely
because a gate exists. Inside the buyer's infrastructure no gate exists and
no meter has appeared.

DUAP's answer is weaker than a gate and stronger than nothing: a signed,
sequenced self-report, whose gaps are detectable and whose double-counts
across reporters are detectable, bound to the licence it exercises. This
does not make a liar honest. It makes a liar leave evidence.

## 5. No standard unit of account — real, and the one DUAP squarely addresses

Every metered data licence today invents its own unit, its own reporting
format and its own reconciliation process. "Per 1,000 pages scraped" is
TollBit's **[ss]**; per successful retrieval is Cloudflare's **[ss]**; a
per-token or per-record basis is whatever a bilateral contract says.

This is the FOCUS problem exactly: N providers times M consumers means N×M
adapters until someone publishes a schema. FOCUS demonstrates the pattern
resolving in a comparable market **[ss]**.

It is also the smallest of the real causes. A shared unit of account is
necessary for the market and nowhere near sufficient, and a project that
mistakes the one problem it solves for the reason the market is missing
will build the right thing for the wrong market.

## 6. Verification costs exceed transaction value — real, and constrains the design

Measured in this repository: 127.7 µs to ingest an event, 209.1 µs to verify
a receipt, 665 bytes per Ed25519 envelope
(`benchmarks/results/2026-09-21-ci-runner.md`). Against a fraction-of-a-cent
transaction this is absurd, and it is why per-event evidence for
individuals cannot work.

It stops being decisive when evidence aggregates: a counter covering 20,000
events closes into one priced, receipted, anchored invoice in 7.44 ms. The
design consequence is a hard constraint rather than an optimisation — DUAP
is viable only where evidence aggregates, and any product framing implying
per-event receipts for individuals is selling something the cost model does
not support.

## 7. Attribution of value is impossible — real, and DUAP declines the question

`research/ai-attribution/RESULTS.md` measures it: defensible estimators
disagree at Spearman ρ ≈ +0.53 against exact Shapley, with 33% top-3
overlap. Any market that must first agree what a contribution was worth
will not clear.

DUAP's response is to refuse the question and price *metered usage under
agreed terms* instead, which is what the deployed market does too —
Cloudflare prices retrievals, TollBit prices pages, RSL declares
per-crawl and per-inference rates **[ss]**. None of them attempts to
measure contribution. That convergence is the strongest evidence that
declining the question is correct rather than evasive.

## 8. Transaction costs of negotiation — real, and being solved by others

Negotiating a licence per rights-holder per buyer does not scale. The
market's answer is the one copyright markets reached a century ago:
collective licensing. The RSL Collective is the current instance **[ss]**,
and it is an institutional answer, not a protocol one.

DUAP does not reduce negotiation cost and should stop claiming adjacency
to the problem. It accounts for what a licence — however negotiated —
produces.

## 9. Incumbents prefer opacity — real, and unaddressed

An organisation whose data practices are currently unexamined has a clear
reason not to adopt an accounting layer that makes them examinable. This is
kill attempt 7 in `docs/reviews/falsification.md` and it is unresolved.
Brookings' characterisation of the emerging licensing market as "same
gatekeepers, new tollbooths" **[ss]** is the same observation from the
political side: the parties with the leverage have limited interest in
transparency infrastructure.

No protocol property changes an incentive. Only regulation, procurement
pressure or a counterparty with leverage does.

## 10. Chicken-and-egg — real, and unaddressed by protocol work

Sellers will not instrument without buyers who require it; buyers will not
require it without sellers who support it. `docs/market/go-to-market.md`
proposes the standard resolution — find the case where a single party
benefits before the network exists — and that is a commercial exercise,
not an engineering one.

## 11. Privacy law prohibits the transactions — mostly false

GDPR and comparable regimes constrain *how* personal data is processed;
they do not prohibit organisations from licensing data to each other under
a lawful basis. The EU Data Act (Regulation (EU) 2023/2854), applicable
from 12 September 2025 **[ss]**, moves in the opposite direction by
requiring connected-product data to be made available, with compensation
terms for third parties. `docs/legal/regulatory-matrix.md` covers what is
and is not constrained.

Privacy law is a real constraint on the *consumer* product, via cause 3.
It is not the reason the enterprise market is missing.

## 12. Demand did not exist — no longer true

For most of the period during which these markets failed, there was no
buyer with large, urgent, budgeted demand for third-party data under
metered terms. Reported AI licensing deal values and the emergence of
usage-metered structures **[ss]** indicate that changed.

This is the single genuine change in the environment, and it is why the
question is worth reopening at all. It is also the most fragile premise in
the whole analysis: it rests on trade-press reporting this author could not
open, and on a trend that could reverse.

## Conclusion

The market is missing for several reasons at once. Of the eight real ones,
DUAP squarely addresses **one** (no standard unit of account), partially
addresses **two** (unobservable usage, verification cost), and does not
address **five** (per-participant value, unclear individual rights,
attribution, negotiation cost, incumbent incentives, chicken-and-egg —
six, counting the last as separate).

That is not a reason to stop. It is a reason to be precise about what is
being claimed, and to stop describing DUAP as the thing that unlocks the
data economy. The defensible claim is that *if* metered cross-boundary data
licensing grows, it will need a shared unit of account and a shared
evidence object, and neither exists for usage that happens inside a
counterparty. Everything else on the list will be solved by someone else or
not at all.
