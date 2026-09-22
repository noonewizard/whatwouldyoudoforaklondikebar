# Results: does the choice of multiplier policy change who pays?

**Status:** EXPERIMENTAL · 2026-09-22
**Harness:** `crates/duap-valuation/src/bin/sensitivity.rs`
**Reproduce:** `cargo run -p duap-valuation --bin sensitivity`
**Raw output:** `results.json`

## Problem

`VALUATION.md` §3 says the default coefficients are "ordinally defensible
and nothing stronger". `MultiplierPolicy`'s own documentation says nothing
in it claims to know what data is worth. Both were **asserted**. Neither
had been measured.

`research/ai-attribution/` asked the equivalent question about attribution
and got a negative answer: defensible estimators disagree on rank at
Spearman ρ ≈ +0.53, so no two of them would pay the same people. Pricing
has the same shape — several defensible parameterisations, no ground truth
— and nobody had checked whether it fails the same way.

Two sub-questions, which can come apart:

- **Cardinal.** How far does the *amount* move between policies? If far,
  no artefact may present a multiplier product as a price.
- **Ordinal.** Does the *ranking* survive? If two parties using different
  defensible policies still agree who owes more than whom, the multiplier
  set is usable as a comparative instrument even though it is useless as
  an absolute one. That is exactly the claim `VALUATION.md` makes.

## Known work

Data valuation has no settled method; `research/ai-attribution/RESULTS.md`
reviews the attribution side. This experiment is narrower and does not
attempt a valuation method. It measures the **sensitivity of an existing
mechanism to its own parameters**, which is a question about this
implementation rather than about the field.

## Hypothesis

That the multiplier set is cardinally unstable and ordinally stable — the
first because arbitrary coefficients multiply, the second because every
defensible policy points the same direction on every factor.

## Experiment

All 61 data classes from the ontology, crossed with four usage profiles
that vary what the multipliers key on: `routine`, `long-retention`,
`exclusive-indefinite`, `stale-cross-border`. **244 usage keys**, each
priced by the real `PriceEngine` — not a reimplementation — under eight
policies.

Base price and quantity are held constant across policies deliberately.
They are negotiated inputs; letting them vary would measure the
negotiation instead of the coefficients.

| Policy | Position it represents |
|---|---|
| `shipped` | The defaults |
| `flat` | **Null hypothesis**: all multipliers 1. Do the modifiers do anything? |
| `linear-sensitivity` | Sensitivity 1..5 rather than doubling — same direction, different curvature |
| `steep-sensitivity` | Quadrupling: severe data priced punitively |
| `risk-led` | Price re-identification risk, not category |
| `retention-led` | Price how long they keep it, not what it is |
| `regulator-flavoured` | Heavy special-category and cross-border |
| `no-decay` | Freshness decay off: history as valuable as fresh |

`flat` produces identical charges for every key, so rank correlation
against it is *undefined* rather than low, and it is excluded from every
rank statistic. It is retained because it answers the prior question of
whether the modifiers matter at all. Ties are excluded from the inversion
count for the same reason — counting a tie against a strict order as a
disagreement would report `flat` as disagreeing with everything, which is
the opposite of what a constant ranking means.

## Result

### The modifiers do change the price — the null is rejected

Top decile's share of total charge: **0.102** under `flat` (a uniform
distribution over deciles, as a control should give) against **0.44–0.66**
under every other policy. The multipliers concentrate charge substantially.

### Cardinal: hopeless, as expected

Ratio of highest to lowest charge for the same key, across the seven
non-null policies:

| | Ratio |
|---|---:|
| Median | **11.3×** |
| 90th percentile | 37.9× |
| Maximum | 45.2× (`protected.immigration` / stale-cross-border) |

Including the null policy, the median is 25.6× and the maximum 858×, but
those are inflated by construction since `flat` is the floor.

**A multiplier product is not a price.** Two parties applying defensible
coefficients to the same usage differ by an order of magnitude at the
median. Any interface presenting a multiplier output as "the value" of
data would be presenting one arbitrary point from a wide range.

### Ordinal: strong, and this is the useful finding

| Measure | Value |
|---|---|
| Spearman ρ, range | **+0.686 to +0.987** |
| Spearman ρ, mean | **+0.879** |
| Weakest pair | `retention-led` vs `no-decay` (ρ = +0.686) |
| Strict inversion rate, worst pair | 21.5% |
| **Top-10 overlap, worst pair** | **1.0 — unanimous** |

Every pair of rival policies picks out **exactly the same ten most
expensive keys**. Compare the attribution experiment, where top-3 overlap
was 33%.

So `VALUATION.md`'s claim is now measured rather than asserted, and it
holds: the multiplier set is a defensible *ordinal* instrument and an
indefensible *cardinal* one.

### The qualification the documentation did not have

Ordinal agreement is strong but **not unconditional**. At the weakest
pair, 21.5% of strictly-ordered comparisons flip — roughly one in five.
Two parties could both be reasonable and still disagree about which of two
specific usages costs more, one time in five.

And distribution is materially policy-dependent. Holding a portfolio
fixed, the largest movement in a single key's *share of the total* is
**2.89 percentage points**, which against a mean share of 1/244 is
**7.1× the mean share** (`identity.government` / exclusive-indefinite).
A pool distributed under `retention-led` pays that key seven mean-shares
more than one distributed under `no-decay`. `duap-valuation::distribute`
apportions exactly and conserves value; it cannot make the *inputs*
policy-independent, and nothing in the protocol claims it can.

## Conclusion

1. **Keep the separation.** Measurement and valuation being separate
   crates is vindicated by a 11.3× median spread: the measured counter is
   stable, the price computed from it is not.
2. **Never present a multiplier product as a price.** Already the
   documented position; now supported by measurement rather than
   modesty.
3. **The ordinal claim may be relied on, with a stated failure rate.**
   Comparative statements — "this usage costs more than that one" — hold
   across defensible policies, with unanimous agreement at the top of the
   distribution and about one in five comparisons flipping at the weakest
   pair.
4. **Both parties to a distribution must agree the policy in advance.**
   A 7×-mean-share swing is not a rounding difference. The policy belongs
   in the grant or the schedule, pinned by digest, not chosen at
   settlement. `PricingRule::Schedule` already allows this and nothing
   required it; that is now a recommendation in `VALUATION.md`.

## Remaining uncertainty

- **Only coefficients vary, not functional form.** All eight policies are
  the same multiplicative structure. A genuinely different approach —
  additive, threshold-based, or learned — is not represented, and the
  strong ordinal agreement is partly an artefact of shared structure. This
  is the most important limitation and it is not small.
- **The corpus is synthetic and uniform.** Every class is weighted
  equally. A real portfolio is concentrated in a few classes, and
  concentration would change the share-shift figure in either direction.
- **Base price and quantity are held constant.** Real charge variation
  includes negotiated base prices, which plausibly swamp coefficient
  variation entirely. This experiment cannot see that.
- **No ground truth.** As with attribution, there is no correct answer to
  correlate against — only agreement between policies. Strong agreement
  among wrong methods is still wrong.
