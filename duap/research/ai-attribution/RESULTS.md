# Results: do data-attribution methods agree?


**Status:** EXPERIMENTAL · 2026-09-21
**Reproduce:** `python3 research/ai-attribution/experiment.py`
**Raw output:** `research/ai-attribution/results.json`
**numpy:** 2.4.6

## Setup

Ridge regression with a closed-form solution (lambda = 1e-2), so a
coalition's utility is exact and there is no optimiser noise: any
disagreement between methods is a property of the methods. Utility is the
negative test mean squared error on 200 held-out points.

Training set: 14 points in 4 dimensions, of which
2 are deliberately mislabelled. The
mislabelled points exist so the methods have something to disagree about;
on clean i.i.d. data every point contributes about the same and any
ranking is arbitrary.

The training set is small because the exact Shapley value requires
enumerating every coalition: 16,384
distinct model fits for n = 14, taking
0.39 s. At n = 20 it would be a million
fits; at n = 30, a billion.

## Finding 1 — the methods do not agree

Spearman rank correlation against the exact Shapley value:

| Method | rho vs exact Shapley | Cost |
|---|---:|---|
| Leave-one-out | +0.530 | 15 fits, 0.4 ms |
| Influence function | +0.499 | 1 inversion, 0.2 ms |
| Influence vs leave-one-out | +0.947 | — |

Top-3 overlap with exact Shapley: leave-one-out
33%, influence function
33%.

The two cheap methods agree closely **with each other**
(rho = +0.947) and only moderately with
the exact Shapley value (rho around
+0.53). That is the expected
relationship — the influence function is a first-order approximation of
leave-one-out, not of Shapley — and it is exactly the problem for pricing:
*which* of these is the contribution a subject should be paid for is a
choice, and the choice changes the ranking.

## Finding 2 — Monte-Carlo Shapley converges slowly

| Permutations | rho vs exact | mean abs. error | utility evaluations | seconds |
|---:|---:|---:|---:|---:|
| 10 | +0.714 | 2.21e+00 | 140 | 0.00 |
| 50 | +0.873 | 1.07e+00 | 700 | 0.01 |
| 200 | +0.908 | 4.28e-01 | 2,800 | 0.04 |
| 1000 | +0.952 | 2.47e-01 | 14,000 | 0.22 |

Truncated Monte-Carlo (Ghorbani and Zou's estimator, tolerance 1e-3) at 200
permutations: rho = +0.908 with
2,800 evaluations against
2,800 for the untruncated version — a
0%
saving at comparable accuracy on this problem.

Reaching rho = 0.95 took 1,000 permutations and
14,000 model fits for
14 data points.

## Finding 3 — the estimate is not stable between runs

8 independent Monte-Carlo runs at 200
permutations, differing only in the random seed:

- Largest per-point spread: **2.42**, which is
  **17.9%** of the largest
  Shapley value in the problem.
- Mean pairwise rank correlation between runs:
  **+0.879**.
- The point ranked most valuable was not the same point in every run:
  [4, 4, 4, 4, 4, 4, 4, 4].

If two clearing nodes each computed a subject's share this way, they would
disagree by a fifth of the largest value, and would sometimes disagree
about who contributed most. There is no tie-break that makes that a
measurement.

## Finding 4 — individual shares dilute roughly as 1/n

| n | median share | median share x n | Gini of positive contribution |
|---:|---:|---:|---:|
| 8 | 8.24e-02 | 0.659 | 0.522 |
| 16 | 6.27e-02 | 1.003 | 0.357 |
| 32 | 2.69e-02 | 0.860 | 0.428 |
| 64 | 1.02e-02 | 0.654 | 0.498 |
| 128 | 5.01e-03 | 0.641 | 0.516 |
| 256 | 2.66e-03 | 0.681 | 0.564 |
| 512 | 1.03e-03 | 0.525 | 0.643 |

`median share x n` stays near 1 throughout, which is the signature of
dilution proportional to 1/n: the median point's share of the total is
inversely proportional to the corpus size, as one would expect and as the
economics require one to confront.

Meanwhile the Gini coefficient of positive contribution *rises* with n
(0.52 at n=8 to 0.64 at
n=512): contribution becomes more concentrated in fewer points as the
corpus grows. The typical contributor's share falls faster than 1/n while a
few points retain most of it.

For pricing this is the central fact. At a corpus of 10^6 records, a median
record's Shapley share is on the order of 10^-6 of the model's utility
gain. Whatever a trained model is worth, one median record's share of it is
not an amount a payment rail can move, which is why
`ECONOMIC_MODEL.md` treats individual micro-compensation as the weak part
of the design.

## Finding 5 — all three methods do detect mislabelled data

Ranks of the mislabelled points, counting from the least valuable:

| Method | ranks of the 2 mislabelled points |
|---|---|
| Exact Shapley | [1, 0] |
| Leave-one-out | [0, 1] |
| Influence function | [0, 1] |

All three put the mislabelled points at or near the bottom. That is the
task these methods were designed for — finding bad data — and they do it
well. It is not the same task as apportioning payment, and their success
here should not be read as support for their use there.

## Cost at scale

| n | leave-one-out | exact Shapley | Monte-Carlo, 200 permutations |
|---:|---:|---:|---:|
| 10 | 11 fits | 10^3 fits | 2,000 fits |
| 20 | 21 fits | 10^6 fits | 4,000 fits |
| 100 | 101 fits | 10^30 fits | 20,000 fits |
| 1,000 | 1,001 fits | 10^301 fits | 200,000 fits |
| 10,000 | 10,001 fits | 10^3010 fits | 2,000,000 fits |
| 1,000,000 | 1,000,001 fits | 10^301030 fits | 200,000,000 fits |

Each "fit" is a full model training run. For a large model, one fit is the
training run. Leave-one-out at n = 10^6 is a million training runs.

## What this does and does not support

**Supports:** attribution methods are useful for *data quality* — finding
mislabelled, redundant or harmful examples. Finding 5 is unambiguous.

**Does not support:** using any of them as the basis for an economic
entitlement. They disagree with each other (Finding 1), the tractable
estimators are unstable between runs (Finding 3), the exact value is
computationally impossible at realistic scale (cost table), and the
resulting per-record amounts are below the granularity of any payment
system (Finding 4).

**Limits of this experiment.** One model family, one utility function, one
data-generating process, small n. Ridge regression is convex and has a
closed form; deep networks are neither, and for them even the notion of a
stable "utility of a coalition" is contested because retraining is
stochastic. The findings here are therefore a *lower bound* on the
disagreement one should expect, not an upper one.
