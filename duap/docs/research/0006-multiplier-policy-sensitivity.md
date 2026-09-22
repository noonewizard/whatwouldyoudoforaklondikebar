# 0006 — How much does the multiplier policy change who pays?

**Status:** ANSWERED · 2026-09-22
**Experiment:** `crates/duap-valuation/src/bin/sensitivity.rs`
**Results:** `research/valuation-sensitivity/RESULTS.md`

## Problem

`VALUATION.md` claimed the default coefficients were "ordinally
defensible and nothing stronger". That was an assertion. If it was wrong
in the cardinal direction, every price the system produces is arbitrary;
if it was wrong in the ordinal direction, every comparative statement is
too.

## Known work

`research/ai-attribution/` asked the same shape of question about
attribution and answered it negatively — defensible estimators disagree
on rank at ρ ≈ +0.53, so none can ground a payment. Nobody had checked
whether pricing fails the same way.

## Hypothesis

Cardinally unstable (arbitrary coefficients multiply), ordinally stable
(every defensible policy points the same direction on every factor).

## Experiment

244 usage keys — all 61 data classes × four usage profiles — priced by
the real `PriceEngine` under seven rival policies plus a flat null. Base
price and quantity held constant so the measurement is of coefficients
rather than negotiation.

## Result

Both halves of the hypothesis confirmed, with a sharper split than
expected.

- **Cardinal:** median 11.3×, p90 37.9×, max 45.2× spread on the same key.
- **Ordinal:** ρ +0.686 to +0.987, mean +0.879, and **unanimous agreement
  on the ten most expensive keys for every pair**.
- The null is rejected: top-decile share 0.102 under flat against
  0.44–0.66 under every rival, so the modifiers do real work.
- Not unconditional: 21.5% of strict comparisons flip at the weakest
  pair, and one key's share of a fixed portfolio moves by 7.1× the mean
  share.

## Conclusion

The documented position was right and is now measured. Three things
changed as a result:

1. `VALUATION.md` §3 carries the numbers, and §10 states what the
   instrument *is* rather than only what it is not.
2. Pinning the policy by digest in the grant moved from "possible" to
   "recommended", because a 7×-mean-share swing chosen at settlement time
   is choosing who gets paid after seeing the data.
3. `crates/duap-valuation/tests/ordinal_stability.rs` enforces the
   ordinal property, negative-controlled by inverting the sensitivity
   ordering — which drops ρ to −0.110 and top-10 overlap to zero, and
   fails the build.

## Remaining uncertainty

All seven policies share one multiplicative functional form, so this
measures sensitivity to coefficients and not to approach. The corpus is
synthetic and weights every class equally, where a real portfolio is
concentrated. Base price and quantity are fixed, and real negotiation
plausibly swamps coefficient variation. And as with attribution there is
no ground truth: strong agreement among wrong methods is still wrong.
