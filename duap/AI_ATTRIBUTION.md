# AI attribution

**Status:** mixed — dataset commitments are PRODUCTION; influence
estimation is EXPERIMENTAL and is a dependency of nothing · 2026-09-21

## Three claims that are routinely run together

Almost every discussion of paying people for training data conflates these.
DUAP keeps them apart in code, in the object model, and in this document.

| Claim | Can it be established? | By what |
|---|---|---|
| **Inclusion** — datum *d* was in the dataset model *M* was trained on | **Yes, cryptographically** | A dataset commitment plus a Merkle inclusion proof |
| **Influence** — *M* would be materially different without *d* | **Estimable, badly** | Leave-one-out, influence functions, Shapley; they disagree |
| **Economic contribution** — how much of *M*'s value is owed to *d* | **No** | A matter of agreement or law, informed by the above, determined by neither |

The rest of this document is the evidence for the entries in the second
column.

## 1. Inclusion: what the protocol proves

A dataset is a content-addressed object whose identifier is the Merkle root
over its members' content identifiers, in a declared order. Publishing that
root before or at training time fixes the membership.

What follows, and is tested in
`crates/duap-provenance/tests/dataset_commitment.rs`:

- A holder of record *d* can prove it was a member, with an O(log n) path.
- A non-member cannot be shown to be one.
- The trainer cannot extend the dataset behind a published root: adding a
  record changes it.
- The provenance graph records the lineage
  `source → dataset → model → output`, with the attribution weight on each
  edge and a `WeightBasis` recording whether it was `Uniform` (assumed),
  `Volumetric`, `Contractual`, `Measured` or `Adjudicated`.

What does **not** follow:

- That the dataset published is the dataset used. A trainer can commit to
  one corpus and train on another. Closing that gap requires a training
  attestation from a trusted execution environment or a reproducible
  training pipeline, neither of which is implemented here and both of which
  substitute a hardware or reproducibility trust assumption for the
  cryptographic one.
- That the model's weights depend on the record in any detectable way.

## 2. Influence: the experiment

`research/ai-attribution/` runs four published methods against each other on
the same model and data, with an exact ground truth. Full results:
`research/ai-attribution/RESULTS.md`. Reproduce with
`python3 research/ai-attribution/experiment.py`.

The model is ridge regression with a closed-form solution, chosen so that a
coalition's utility is exact and there is no optimiser noise: any
disagreement is a property of the methods, not of the training run.

### The findings

**They do not agree.** Against the exact Shapley value, Spearman rank
correlation was **+0.53** for leave-one-out and **+0.50** for the influence
function — while the two cheap methods agreed with each other at **+0.95**.
That relationship is expected (the influence function approximates
leave-one-out, not Shapley) and it is exactly the problem: *which* notion
of contribution a subject should be paid for is a choice, and the choice
changes the ranking.

**The tractable estimator is unstable.** Eight independent Monte-Carlo runs
at 200 permutations, differing only in the seed, produced a maximum
per-point spread of **17.9% of the largest Shapley value**, with a mean
pairwise rank correlation of **+0.88**. They did not always agree on which
point contributed most. Two clearing nodes computing a subject's share this
way would disagree by a fifth of the largest value.

**The exact value is computationally out of reach.** 16,384 model fits for
14 data points. At 10^6 records the exact computation needs 10^301,030 fits;
leave-one-out needs a million training runs; Monte-Carlo at 200 permutations
needs 200 million.

**Individual shares dilute as 1/n, and concentrate.** The median point's
share of total utility falls proportionally to 1/n, while the Gini
coefficient of positive contribution *rises* from 0.52 at n=8 to 0.64 at
n=512. The typical contributor's share falls faster than 1/n while a few
points keep most of it. At a corpus of 10^6 the median record's share is on
the order of 10^-6 of the model's utility gain, which is not an amount any
payment rail can move.

**They do work for data quality.** All three methods ranked the
deliberately mislabelled points at or near the bottom. That is the task
these methods were built for, and they do it well. It is not the same task
as apportioning payment.

### Limits of the experiment

One model family, one utility function, one data-generating process, and
small *n*. Ridge regression is convex with a closed form; deep networks are
neither, and for them even a stable "utility of a coalition" is contested
because retraining is stochastic. These findings are therefore a **lower
bound** on the disagreement to expect, not an upper one.

## 3. What DUAP therefore does

**In the protocol, as production:**

- Dataset commitments, as content-addressed Merkle roots.
- A provenance graph recording `source → dataset → model → output` with
  exact-rational weights.
- `WeightBasis` on every edge, so a reader can always tell whether a share
  was measured, agreed, adjudicated or merely assumed uniform.
- Metering of AI operations in their natural units: tokens for training,
  records for embedding and evaluation, comparisons for preference data,
  episodes for reinforcement learning, retrievals for RAG.
- Termination rules so attribution stops somewhere: depth, negligible
  share, terminal node kinds, and severing edges.
- An authorization vocabulary that distinguishes the AI uses: a subject can
  permit evaluation and inference while denying training, or permit
  fine-tuning and deny distillation.

**Outside the protocol, as research:**

- Every influence estimate. No crate outside `research/` depends on one,
  and `.claude/agents/ai-attribution-researcher.md` forbids introducing
  such a dependency.

**Not attempted:**

- Training attestations from trusted execution environments. They would
  close the "committed dataset is the used dataset" gap and substitute a
  hardware trust assumption; the trade is real and unexamined here.
- Proof of training. Zero-knowledge proofs of a training run exist in the
  literature for small models and are many orders of magnitude from
  practical for large ones.
- Machine unlearning as a remedy. The ontology has an `ai.unlearn`
  operation, and it records an *attempt* with an evidence reference.
  Efficacy is not assumed, and the demonstration states plainly that a
  revocation after training leaves the model unchanged
  (`already_trained_model=NOT_UNLEARNED`).

## 4. The honest position on paying for training data

The protocol can prove a record was in a committed corpus, and can meter
and price its use on terms the parties agreed in advance. That supports a
**contractual** model: a per-token or per-record rate, agreed before
training, metered and receipted.

It cannot support a **contribution** model — paying each record in
proportion to what it added — because the contribution is not measurable to
a standard that would survive a dispute, and because the resulting amounts
are below any payment granularity.

Anyone proposing to pay individuals in proportion to their measured
contribution to a large model should be asked three questions this
experiment makes concrete: which method, why that one rather than the one
that ranks differently, and what the answer's spread is between two honest
computations of it.
