# Business model

**Status:** REFERENCE (analysis, not a plan of record) · 2026-09-21
**Owner:** `venture-strategy`

The question is not which model produces the most revenue in a model. It
is which model produces revenue **without giving anyone a reason to stop
using the protocol**, because a protocol with a defection incentive built
into its funding does not reach the scale at which any model pays.

## 1. The disqualifying test

Apply first, to every candidate: *does this fee grow with the value the
customer gets from the protocol, in a way that makes routing around it
rational?*

`economics/results.md` finding 5 computes the take-rate case: in the
aggressive scenario a fee on cleared value yields about $20k per customer
per month against $25k of subscription revenue. The fee is not small. It
is disqualified anyway, because a fee proportional to the value flowing
through an open protocol is the strongest available incentive to
implement the open protocol and not the service — and the specification,
vectors and verifier are deliberately public precisely so that customers
*can* do that.

This is the same reasoning by which `docs/market/market-design.md` §8
forbids a value-proportional fee as a market-design rule. It is a
constraint, not a preference.

## 2. Candidates, assessed

| Model | Aligned with adoption? | Verdict |
|---|---|---|
| Take rate on cleared value | **No** — §1 | **Rejected** |
| Marketplace / transaction fees | No — same reasoning, plus the reference implementation does not match parties | **Rejected** |
| Clearing fees per netted obligation | Partly. Small and volume-linked, but still a toll on protocol use | **Rejected for v1**, revisit only if netting becomes a distinct service someone asks for |
| Hosted infrastructure subscription | **Yes.** Priced on operating a service, not on the value passing through | **Primary** |
| Enterprise support and integration | Yes | **Secondary** |
| Compliance and audit tooling | Yes. Value is in the reporting, not the metering | **Secondary** |
| Analytics over a customer's own usage | Yes, with a hard constraint: a customer's data is theirs, and cross-customer analytics is a trust violation regardless of aggregation | **Secondary, constrained** |
| Certification fees | Neutral. Must fund administration only, never gate interoperability (`GOVERNANCE.md` §5) | **To the foundation, not the company** |
| Attribution as a service | — | **Rejected.** `research/ai-attribution/RESULTS.md` says it cannot be delivered |
| Selling data or insights derived from customers' events | — | **Rejected absolutely.** An accounting intermediary that monetises what it accounts for has destroyed the neutrality that is its only asset, and would raise DGA structural questions (`docs/legal/regulatory-matrix.md` §1.2) |

## 3. Why subscription works here, quantitatively

`economics/results.md` finding 1: at base assumptions the infrastructure
cost of running a customer is under a dollar a month against thousands of
dollars of plausible subscription revenue, and break-even is in the
trillions of events per customer per month. Gross margin on the
infrastructure component is not the problem.

Finding 3 says where the cost actually is: human dispute handling swings
total cost by more than every infrastructure assumption combined. So the
cost structure of this business is **people, not servers**, and the model
must price accordingly — which subscription with a support tier does
naturally and a per-event fee does not.

The pricing consequence: price on organisational scale, counterparty count
and support level. Do not price on event volume, because event volume is
nearly free to serve, and pricing on it would both overcharge for
something cheap and create the defection incentive §1 rules out.

## 4. What is given away, and why that is the product strategy

Specification, schemas, conformance vectors, verifier, reference SDK — all
free and independently implementable. This is not generosity. An
accounting protocol whose evidence only one vendor can check is not
evidence; it is a vendor's assertion, which is what the market already has
in Cloudflare's meter and TollBit's counter
(`docs/market/competitive-landscape.md` §2).

**The entire differentiator is that the evidence does not require trusting
the party that produced it.** That property is destroyed by any
proprietary component on the verification path. So the free tier is not a
funnel — it is the product's central claim, and the commercial offering
has to survive being unnecessary.

## 5. Moat analysis

The directive says not to claim being first is a moat. It is not.

| Candidate moat | Real? |
|---|---|
| Being first | **No.** RSL and Cloudflare were faster into the adjacent market |
| Network effects on liquidity | **Weak.** `docs/market/market-design.md` §6: DUAP is useful to two parties, so there is little liquidity effect to accumulate. Good for adoption, bad for defensibility — and these are the same fact |
| Switching costs from accumulated evidence | **Real but modest.** Historical receipts stay verifiable after a switch, by design. Making them unverifiable would be a moat and would break the protocol |
| Clearing and settlement relationships | **Potentially real.** Netting across many counterparties requires being the party many counterparties use. This is the only structural moat identified |
| Standards leadership | **Real if achieved, and it has not been.** No standards body has been approached |
| Certification and reputation | Real, slow, and belongs to the foundation |
| Accumulated conformance and interoperability testing | Modest. Vectors are public |
| Proprietary attribution | **Non-existent.** The research says it cannot be done |

**The honest conclusion:** this is a weakly defensible business by design.
The properties that make the protocol credible — public vectors, an
independent verifier, portable evidence, no proprietary verification path
— are precisely the properties that prevent lock-in. A version with a
strong moat would be a version nobody should trust.

That is a legitimate strategic position and it must be stated to investors
in exactly these terms rather than discovered by them. The realistic
outcome is a good infrastructure business in a standard the company does
not own, not a platform monopoly.

## 6. What would change this assessment

- If dispute rates turn out high (`economics/results.md` finding 4, the
  number nobody knows), dispute resolution becomes the product and the
  model shifts toward a service business.
- If netting across many counterparties becomes valuable, the clearing
  relationship becomes the moat and a per-netting fee becomes defensible —
  because at that point the fee buys a service rather than taxing protocol
  use.
- If no buyer will meter its own use (falsification kill attempt 7), there
  is no business at all and the specification should be donated to whoever
  will maintain it.
