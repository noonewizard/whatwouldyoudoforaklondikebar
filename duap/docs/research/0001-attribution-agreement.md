# 0001 — Do data-attribution methods agree enough to price with?

**Status:** answered, negatively · 2026-09-21

## Problem

If DUAP priced a subject's contribution to a trained model using a
published attribution method, would the answer depend on which method was
chosen, and how stable would it be?

## Known work

Shapley (1953) for the value; Ghorbani and Zou (ICML 2019) for Data
Shapley and the truncated Monte-Carlo estimator; Koh and Liang (ICML 2017)
for influence functions. Each paper reports that its method identifies
low-quality data well. None of them, so far as we found, reports the
pairwise agreement between methods as a *ranking* problem, which is what
matters for apportioning payment.

## Hypothesis

The methods would agree on ranking closely enough (rank correlation above
0.9) that the choice of method would not materially change a payment.

## Experiment

`research/ai-attribution/experiment.py`. Ridge regression with a
closed-form solution so utilities are exact and optimiser noise is
excluded. 14 training points in 4 dimensions with 2 deliberately
mislabelled, exact Shapley over all 16,384 coalitions as ground truth,
compared against leave-one-out, influence functions and Monte-Carlo
Shapley at 10/50/200/1000 permutations, plus a stability study over 8
seeds and a dilution study to n=512.

## Result

The hypothesis is false. Rank correlation against exact Shapley was +0.53
(leave-one-out) and +0.50 (influence functions), while those two agreed
with each other at +0.95. Monte-Carlo needed 1,000 permutations to reach
+0.95 against exact. Across 8 seeds at 200 permutations the spread was
17.9% of the largest value and the top-ranked point was not always the
same. Median share dilutes as 1/n while concentration (Gini) rises.

Full numbers: `research/ai-attribution/RESULTS.md`.

## Conclusion

Influence estimation must not be a dependency of any pricing path in the
protocol. Dataset commitments (inclusion) are cryptographic and are
production; influence is EXPERIMENTAL and is a dependency of nothing.
Enforced by `.claude/agents/ai-attribution-researcher.md` and by the crate
dependency graph.

## Remaining uncertainty

The experiment uses one convex model family with a closed form. For deep
networks the disagreement is likely larger, not smaller, because retraining
is stochastic and the coalition utility is itself noisy — but that is an
expectation, not a measurement. Extending the experiment to a small neural
network with fixed initialisation would test it and has not been done.
