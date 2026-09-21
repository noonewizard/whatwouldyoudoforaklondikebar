# 0004 — Can differential-privacy budgets be accounted across releases?

**Status:** open · 2026-09-21

## Problem

DUAP records an epsilon for a differentially private release and can
enforce a per-release maximum. It does not track a budget across releases,
does not compose epsilons, and does not verify that a release satisfies
the epsilon it claims.

A subject's protection depends on the *total* privacy loss across every
release that touched their data, across every controller. Nobody currently
accounts for that, in this protocol or elsewhere at scale.

## Known work

Dwork and Roth (*The Algorithmic Foundations of Differential Privacy*,
2014) for basic and advanced composition. Rényi differential privacy
(Mironov, CSF 2017) and the moments accountant (Abadi et al., CCS 2016) for
tighter composition in practice. Deployed systems — the US Census
Bureau's 2020 disclosure avoidance system, and Google's and Apple's local
DP deployments — each account for a budget within one controller's own
releases.

## Hypothesis

A cross-controller privacy-loss accountant is possible in principle — the
protocol already sees every declared release for a given subject
pseudonym — but is defeated in practice by three things: pseudonyms are
per-controller, so the accountant would need the linkage the design avoids;
composition bounds require knowing the mechanism, not just the epsilon; and
an undeclared release contributes loss that no accountant sees.

## Experiment

**Not yet run.** A first step that would be informative: implement a
per-controller accountant over the events a clearing node already holds,
using basic and advanced composition, and measure how quickly a realistic
release schedule exhausts a plausible budget. That is tractable and would
answer whether the question is urgent or academic.

## Result

None yet.

## Conclusion

None yet. The protocol records what is declared and enforces a per-release
ceiling, and `PRIVACY.md` says explicitly that it does neither composition
nor verification.

## Remaining uncertainty

Whether a budget that spans controllers is meaningful at all, given that
the privacy loss from a controller the subject never heard of is not
something the subject can consent to in advance.
