# Valuation

**Status:** SPECIFIED (draft) · 2026-09-21

`ECONOMIC_MODEL.md` explains why there is no universal price. This document
specifies the machinery that produces *a* price, and the procedure for
calibrating it.

## 1. The form

```
charge = base(unit) x quantity x PI(m_i)
```

Multiplicative rather than additive because the factors are close to
independent proportional effects, and because a multiplicative form cannot
drive a price negative. Every factor is an exact rational, so the product
is exact and reproducible, and order of multiplication does not matter.

## 2. The four attribute families

Following the design brief's decomposition, and keeping them separate
because they are calibrated differently:

### Intrinsic — properties of the data itself

| Attribute | In the protocol | Source |
|---|---|---|
| sensitivity | tier t0–t4, per class, may be raised per event | ontology, event |
| re-identification risk | prior 0–100 per class | ontology |
| special-category status | per-regime tags | ontology |
| freshness | age at time of use | computed |
| scarcity, uniqueness, accuracy, provenance quality | **not modelled** | — |

The last row matters: four of the brief's intrinsic attributes are not in
the protocol, because none is measurable from an event. Scarcity requires a
market; uniqueness requires the corpus; accuracy requires ground truth;
provenance quality requires a judgement. Recorded as a gap rather than
filled with a guess.

### Usage — what is being done

purpose (and whether it is commercial), operation, retention basis and
period, and whether the use is exclusive. All present in the event.

### Market — supply, demand, substitution

**Not modelled.** There is no market to observe. The `auction` pricing rule
exists so that a deployment that *does* have a market can feed its clearing
price in, and the protocol will then reproduce the arithmetic.

### Risk — privacy, security, regulatory, re-identification

Partially modelled through the sensitivity tier, the re-identification
prior and the cross-border multiplier. Security and regulatory risk are
not, because they are properties of the *holder*, not of the data, and the
protocol has no basis to assess a holder.

## 3. The default coefficients

```
sensitivity   t0 1/2   t1 1     t2 2     t3 4     t4 8
reid          x1.05 per 10 points of prior above 50
special       x1.5
exclusivity   x3
retention     indefinite x2.5, over one year x1.5
cross-border  x1.25
freshness     half-life 180 days, floored at 1/8
```

**These are parameters. They are not findings, measurements, or
recommendations.** They were chosen so that the ordering is defensible —
more sensitive costs more, exclusive costs more, stale costs less — and for
no stronger reason. Any deployment that uses them unchanged in a real
transaction is using our guess as its price.

### How much the guess matters, measured

That paragraph used to end there, as an assertion.
`research/valuation-sensitivity/` now measures it: 61 data classes crossed
with four usage profiles, 244 keys, priced by the real engine under seven
defensible coefficient policies plus a flat null.

| | Result |
|---|---|
| Ratio of highest to lowest charge for the same key, **median** | **11.3×** |
| Same, 90th percentile / maximum | 37.9× / 45.2× |
| Spearman rank correlation between policies | **+0.686 to +0.987**, mean +0.879 |
| Agreement on the ten most expensive keys | **unanimous, every pair** |
| Strict comparisons that flip, worst pair | 21.5% |
| Largest movement in one key's share of a fixed portfolio | 2.89 pp — **7.1× the mean share** |

So the claim this section makes is now supported rather than merely
modest. The multiplier set is a defensible **ordinal** instrument and an
indefensible **cardinal** one, and the gap between those is about an order
of magnitude at the median.

Three consequences, each binding:

1. **No artefact may present a multiplier product as a price.** Two
   parties applying defensible coefficients to the same usage differ 11×
   at the median. Showing one of those numbers as "the value" of data
   would be showing one arbitrary point from a wide range.
2. **Comparative statements may be relied on, with a stated failure
   rate.** "This usage costs more than that one" survives across
   policies, unanimously at the top of the distribution, and flips about
   one time in five at the weakest pair.
3. **Both parties to a distribution must pin the policy in advance.** A
   7×-mean-share swing is not a rounding difference. Use
   `PricingRule::Schedule` and pin the schedule by digest in the grant;
   choosing a policy at settlement time means choosing who gets paid
   after seeing the data.

`crates/duap-valuation/tests/ordinal_stability.rs` turns the ordinal
finding into a property the build enforces, so a change to these
coefficients cannot quietly invalidate consequence 2. Its floors are
looser than the measured values — ρ ≥ 0.60 and 7 of 10 top keys — so
re-tuning is allowed and destroying the ordinal claim is not.

**The limitation that matters most:** all seven policies share the same
multiplicative functional form, so this measures sensitivity to
*coefficients*, not to *approach*. A additive, threshold-based or learned
valuation is not represented, and the strong ordinal agreement is partly
an artefact of shared structure.

## 4. Freshness: an exact approximation of decay

Exponential decay is irrational and cannot be represented exactly, and an
accounting system must not depend on a floating-point approximation two
implementations might round differently.

The protocol uses exact halving per half-life with linear interpolation
within the period. It is within 6% of `2^(-t/h)` everywhere, exact in
rational arithmetic, and trivially reproducible. Tested to be monotone
non-increasing and floored.

## 5. Calibration procedure

The procedure a deployment should run before using any coefficient in
anger. We have not run it: there is no market to calibrate against, and
saying so is more useful than producing a number.

1. **Collect realised transactions.** At least several hundred bilateral
   agreements with known prices, classes, purposes, volumes and durations.
2. **Normalise to a per-unit basis** using the same units the protocol
   meters in.
3. **Fit the multipliers by regression on the log of the unit price**,
   with the attribute set as regressors. The multiplicative form becomes
   additive under the logarithm, so ordinary least squares applies and each
   coefficient is directly interpretable as a multiplier.
4. **Check ordinal consistency.** A fitted coefficient that inverts the
   expected ordering (stale data priced above fresh) is a signal that the
   attribute is proxying for something else, not a discovery.
5. **Hold out a test set** and report the out-of-sample error on the log
   price. A model that cannot predict within a factor of two is not a
   pricing model.
6. **Re-fit on a schedule** and publish the coefficients as a pinned
   schedule so that a grant citing it is priced by the version it cited.

Two warnings. Realised prices reflect bargaining power as much as value, so
a fitted model reproduces the existing distribution rather than a fair one.
And selection bias is severe: the transactions you can observe are the ones
that happened, which excludes every refusal.

## 6. Minimum prices

A subject can attach a `min_price` obligation. It is a floor on the
**final** amount after multipliers: a subject who insisted on a penny a
record gets a penny a record even if every multiplier points down. Tested.

## 7. Auctions

Second-price sealed-bid with a reserve and commit-reveal. The mechanism and
its limits are documented in `crates/duap-valuation/src/auction.rs`:
commit-reveal stops the auctioneer's last look; it does not stop shill
bidding or a bidding ring, and Vickrey's truthful-bidding result assumes
independent private values and no budget constraints, neither of which
holds for data lots.

## 8. Distribution

A pool is apportioned by largest-remainder (Hamilton): floor every share,
then hand remaining units to the largest remainders, ties broken
deterministically. The parts sum to the whole exactly, and every recipient
is within one minor unit of their exact share.

Largest-remainder admits the Alabama paradox — adding to the pool can
reduce someone's allocation. For a monthly reapportionment of money that is
a curiosity; the divisor methods that avoid it violate quota instead, which
is worse here because a recipient who gets less than their floor will
dispute it.

Unallocated share stays with the payer. Attribution legitimately
terminates, and the remainder belongs to the controller, not to the
recipients.

## 9. What this is not

- Not a valuation of anyone's data.
- Not a recommendation of any rate.
- Not calibrated against any market.
- Not a claim that the attribute set is complete; four intrinsic
  attributes and the whole market family are missing and named as missing.
- Not a cardinal instrument. Measured: 11.3× median spread between
  defensible policies (§3).

## 10. What it is

One thing, stated as narrowly as the evidence supports:

> A reproducible, explainable, **ordinal** adjustment to a negotiated base
> price, whose ranking is stable across defensible parameterisations and
> whose absolute output is not.

Reproducible because every factor is an exact rational and the product
does not depend on order. Explainable because `AppliedMultiplier` retains
each factor and its reason, so a price can be read line by line — an
unexplainable price is a dispute waiting to happen. Ordinal because
§3 measures it to be, and cardinal-only-if-calibrated because nobody has
calibrated it.
