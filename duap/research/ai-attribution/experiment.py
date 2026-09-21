#!/usr/bin/env python3
"""Do data-attribution methods agree with each other?

STATUS: EXPERIMENTAL.

The question this experiment exists to answer is narrow and practical:
if DUAP were to price a subject's contribution to a trained model using a
published attribution method, would the answer depend on which method was
chosen? If it does, the method is a policy choice dressed as a measurement,
and the protocol must not present it as the latter.

Four methods are compared on the same models and data:

  LOO      leave-one-out: the change in test loss from removing one point.
  Shapley  the exact Shapley value of each point, over all 2^n coalitions.
  MC       Monte-Carlo permutation approximation of the Shapley value.
  IF       the influence function approximation (Koh and Liang, ICML 2017),
           computed in closed form for ridge regression.

Everything is synthetic. Seeds are fixed. Running this script reproduces
`results.json` exactly.

References consulted for the definitions, not copied from:
  L. S. Shapley, "A value for n-person games", Contributions to the Theory
    of Games II, 1953.
  A. Ghorbani and J. Zou, "Data Shapley: Equitable Valuation of Data for
    Machine Learning", ICML 2019 (truncated Monte-Carlo estimator).
  P. W. Koh and P. Liang, "Understanding Black-box Predictions via
    Influence Functions", ICML 2017.
"""
from __future__ import annotations

import itertools
import json
import math
import pathlib
import time

import numpy as np

RIDGE = 1e-2
HERE = pathlib.Path(__file__).resolve().parent


# ---------------------------------------------------------------------------
# Model: ridge regression with a closed-form solution, so that a coalition's
# utility is exact and cheap. Using a model with a closed form is deliberate:
# it removes optimiser noise, so any disagreement between methods is a
# property of the methods rather than of the training run.
# ---------------------------------------------------------------------------

def fit(X: np.ndarray, y: np.ndarray, lam: float = RIDGE) -> np.ndarray:
    d = X.shape[1]
    if len(X) == 0:
        return np.zeros(d)
    A = X.T @ X + lam * len(X) * np.eye(d)
    return np.linalg.solve(A, X.T @ y)


def loss(w: np.ndarray, X: np.ndarray, y: np.ndarray) -> float:
    r = X @ w - y
    return float(r @ r / len(X))


def utility(idx, Xtr, ytr, Xte, yte) -> float:
    """Utility of a coalition: negative test MSE, floored at the
    empty-coalition value so utilities are comparable."""
    idx = list(idx)
    if not idx:
        return -loss(np.zeros(Xtr.shape[1]), Xte, yte)
    w = fit(Xtr[idx], ytr[idx])
    return -loss(w, Xte, yte)


# ---------------------------------------------------------------------------
# Attribution methods
# ---------------------------------------------------------------------------

def leave_one_out(Xtr, ytr, Xte, yte) -> np.ndarray:
    n = len(Xtr)
    full = utility(range(n), Xtr, ytr, Xte, yte)
    out = np.zeros(n)
    for i in range(n):
        rest = [j for j in range(n) if j != i]
        out[i] = full - utility(rest, Xtr, ytr, Xte, yte)
    return out


def exact_shapley(Xtr, ytr, Xte, yte) -> tuple[np.ndarray, int]:
    """Exact Shapley values by enumerating every coalition.

    phi_i = sum over S not containing i of
            |S|!(n-|S|-1)!/n! * (u(S + i) - u(S))
    """
    n = len(Xtr)
    members = list(range(n))
    cache: dict[frozenset, float] = {}
    evaluations = 0

    def u(s: frozenset) -> float:
        nonlocal evaluations
        if s not in cache:
            cache[s] = utility(sorted(s), Xtr, ytr, Xte, yte)
            evaluations += 1
        return cache[s]

    phi = np.zeros(n)
    fact = math.factorial
    for i in members:
        others = [j for j in members if j != i]
        for k in range(len(others) + 1):
            weight = fact(k) * fact(n - k - 1) / fact(n)
            for S in itertools.combinations(others, k):
                s = frozenset(S)
                phi[i] += weight * (u(s | {i}) - u(s))
    return phi, evaluations


def mc_shapley(Xtr, ytr, Xte, yte, permutations: int, seed: int,
               truncate_tol: float | None = None) -> tuple[np.ndarray, int]:
    """Permutation-sampling estimator; with truncate_tol it is the
    truncated Monte-Carlo variant of Ghorbani and Zou."""
    rng = np.random.default_rng(seed)
    n = len(Xtr)
    phi = np.zeros(n)
    evaluations = 0
    full = utility(range(n), Xtr, ytr, Xte, yte)
    empty = utility([], Xtr, ytr, Xte, yte)
    for _ in range(permutations):
        perm = rng.permutation(n)
        prev = empty
        coalition: list[int] = []
        for i in perm:
            coalition.append(int(i))
            if truncate_tol is not None and abs(full - prev) < truncate_tol:
                # The remaining marginals are assumed negligible.
                cur = prev
            else:
                cur = utility(coalition, Xtr, ytr, Xte, yte)
                evaluations += 1
            phi[i] += cur - prev
            prev = cur
    return phi / permutations, evaluations


def influence(Xtr, ytr, Xte, yte, lam: float = RIDGE) -> np.ndarray:
    """Closed-form influence of each training point on the test loss.

    For a quadratic objective the Hessian is constant, so the influence of
    upweighting z_i on the test loss is

        I(z_i) = - grad_te^T H^{-1} grad_i

    and removing the point is approximately +I/n. Sign is aligned with LOO:
    positive means removing the point hurts.
    """
    n, d = Xtr.shape
    w = fit(Xtr, ytr, lam)
    H = 2 * (Xtr.T @ Xtr) / n + 2 * lam * np.eye(d)
    Hinv = np.linalg.inv(H)
    grad_te = 2 * Xte.T @ (Xte @ w - yte) / len(Xte)
    out = np.zeros(n)
    for i in range(n):
        gi = 2 * Xtr[i:i + 1].T @ (Xtr[i:i + 1] @ w - ytr[i:i + 1]) / 1.0
        out[i] = float(grad_te @ (Hinv @ gi.ravel())) / n
    return out


# ---------------------------------------------------------------------------
# Statistics
# ---------------------------------------------------------------------------

def rank(a: np.ndarray) -> np.ndarray:
    order = np.argsort(np.argsort(-a))
    return order.astype(float)


def spearman(a: np.ndarray, b: np.ndarray) -> float:
    ra, rb = rank(a), rank(b)
    ra = ra - ra.mean()
    rb = rb - rb.mean()
    denom = math.sqrt(float(ra @ ra) * float(rb @ rb))
    return float(ra @ rb / denom) if denom else float("nan")


def top_k_overlap(a: np.ndarray, b: np.ndarray, k: int) -> float:
    ta = set(np.argsort(-a)[:k].tolist())
    tb = set(np.argsort(-b)[:k].tolist())
    return len(ta & tb) / k


def gini(a: np.ndarray) -> float:
    """Concentration of positive contribution. 0 = uniform, 1 = one point
    holds everything."""
    x = np.clip(a, 0, None)
    if x.sum() == 0:
        return float("nan")
    x = np.sort(x)
    n = len(x)
    idx = np.arange(1, n + 1)
    return float((2 * (idx * x).sum()) / (n * x.sum()) - (n + 1) / n)


# ---------------------------------------------------------------------------
# Data
# ---------------------------------------------------------------------------

def make_dataset(n: int, d: int, seed: int, noise: float = 0.1,
                 outliers: int = 2):
    """Synthetic linear data with a few mislabelled points.

    The mislabelled points exist so that the methods have something to
    disagree about: on clean, i.i.d. data every point contributes about the
    same and any method ranks them arbitrarily.
    """
    rng = np.random.default_rng(seed)
    w_true = rng.normal(size=d)
    X = rng.normal(size=(n, d))
    y = X @ w_true + noise * rng.normal(size=n)
    for i in range(outliers):
        y[i] += 5.0 * rng.choice([-1.0, 1.0])
    Xte = rng.normal(size=(200, d))
    yte = Xte @ w_true + noise * rng.normal(size=200)
    return X, y, Xte, yte, list(range(outliers))


# ---------------------------------------------------------------------------
# Experiments
# ---------------------------------------------------------------------------

def experiment_agreement(n=14, d=4, seed=7):
    X, y, Xte, yte, outliers = make_dataset(n, d, seed)

    t0 = time.perf_counter()
    phi, exact_evals = exact_shapley(X, y, Xte, yte)
    t_exact = time.perf_counter() - t0

    t0 = time.perf_counter()
    loo = leave_one_out(X, y, Xte, yte)
    t_loo = time.perf_counter() - t0

    t0 = time.perf_counter()
    inf = influence(X, y, Xte, yte)
    t_inf = time.perf_counter() - t0

    mc_results = {}
    for perms in (10, 50, 200, 1000):
        t0 = time.perf_counter()
        est, evals = mc_shapley(X, y, Xte, yte, perms, seed=1234)
        mc_results[perms] = {
            "spearman_vs_exact": spearman(est, phi),
            "top3_overlap_vs_exact": top_k_overlap(est, phi, 3),
            "max_abs_error": float(np.max(np.abs(est - phi))),
            "mean_abs_error": float(np.mean(np.abs(est - phi))),
            "utility_evaluations": evals,
            "seconds": time.perf_counter() - t0,
        }

    t0 = time.perf_counter()
    tmc, tmc_evals = mc_shapley(X, y, Xte, yte, 200, seed=1234,
                                truncate_tol=1e-3)
    t_tmc = time.perf_counter() - t0

    return {
        "n_train": n,
        "d": d,
        "seed": seed,
        "mislabelled_indices": outliers,
        "exact_shapley": {
            "values": phi.tolist(),
            "utility_evaluations": exact_evals,
            "seconds": t_exact,
        },
        "loo": {"values": loo.tolist(), "seconds": t_loo},
        "influence": {"values": inf.tolist(), "seconds": t_inf},
        "tmc_200": {
            "spearman_vs_exact": spearman(tmc, phi),
            "utility_evaluations": tmc_evals,
            "seconds": t_tmc,
        },
        "mc": mc_results,
        "agreement": {
            "spearman_loo_vs_shapley": spearman(loo, phi),
            "spearman_if_vs_shapley": spearman(inf, phi),
            "spearman_if_vs_loo": spearman(inf, loo),
            "top3_loo_vs_shapley": top_k_overlap(loo, phi, 3),
            "top3_if_vs_shapley": top_k_overlap(inf, phi, 3),
        },
        "detection": {
            "shapley_ranks_mislabelled_last": [
                int(np.argsort(phi).tolist().index(i)) for i in outliers
            ],
            "loo_ranks_mislabelled_last": [
                int(np.argsort(loo).tolist().index(i)) for i in outliers
            ],
            "if_ranks_mislabelled_last": [
                int(np.argsort(inf).tolist().index(i)) for i in outliers
            ],
        },
    }


def experiment_stability(n=14, d=4, seed=7, perms=200, trials=8):
    """How much does a Monte-Carlo estimate move between runs?

    If two clearing nodes compute a subject's share independently, this is
    the disagreement they would have to reconcile.
    """
    X, y, Xte, yte, _ = make_dataset(n, d, seed)
    phi, _ = exact_shapley(X, y, Xte, yte)
    ests = []
    for t in range(trials):
        est, _ = mc_shapley(X, y, Xte, yte, perms, seed=1000 + t)
        ests.append(est)
    E = np.array(ests)
    spread = E.max(axis=0) - E.min(axis=0)
    scale = float(np.max(np.abs(phi)))
    return {
        "permutations": perms,
        "trials": trials,
        "per_point_spread": spread.tolist(),
        "max_spread": float(spread.max()),
        "max_spread_relative_to_largest_value": float(spread.max() / scale),
        "mean_pairwise_spearman": float(
            np.mean([spearman(E[i], E[j])
                     for i in range(trials) for j in range(i + 1, trials)])
        ),
        "rank_of_top_point_across_trials": [
            int(np.argmax(E[t])) for t in range(trials)
        ],
    }


def experiment_dilution(d=4, seed=11):
    """How does one point's share behave as the corpus grows?

    This is the economic question: if a model is trained on a million
    records, is any individual's Shapley value distinguishable from zero?
    """
    rows = []
    for n in (8, 16, 32, 64, 128, 256, 512):
        X, y, Xte, yte, _ = make_dataset(n, d, seed, outliers=1)
        est, evals = mc_shapley(X, y, Xte, yte, 200, seed=99)
        full = utility(range(n), X, y, Xte, yte)
        empty = utility([], X, y, Xte, yte)
        total = full - empty
        share = np.clip(est, 0, None)
        share = share / share.sum() if share.sum() > 0 else share
        rows.append({
            "n": n,
            "total_utility_gain": total,
            "median_share": float(np.median(share)),
            "max_share": float(share.max()),
            "gini_of_positive_contribution": gini(est),
            "median_share_times_n": float(np.median(share) * n),
            "utility_evaluations": evals,
        })
    return rows


def experiment_cost_model():
    """Cost of each method in utility evaluations, as a function of n."""
    # 2^n is reported as a base-10 exponent: at n = 10^6 the integer has
    # over 300,000 digits, which is itself the point being made.
    return [
        {"n": n,
         "loo_utility_evaluations": n + 1,
         "exact_shapley_log10_evaluations": round(n * math.log10(2), 1),
         "mc_200_permutations_evaluations": 200 * n}
        for n in (10, 20, 100, 1000, 10_000, 1_000_000)
    ]


def main() -> None:
    results = {
        "status": "EXPERIMENTAL",
        "model": "ridge regression, closed form, lambda=1e-2",
        "utility": "negative test mean squared error",
        "agreement": experiment_agreement(),
        "stability": experiment_stability(),
        "dilution": experiment_dilution(),
        "cost_model": experiment_cost_model(),
        "numpy_version": np.__version__,
    }
    out = HERE / "results.json"
    out.write_text(json.dumps(results, indent=2) + "\n")
    print(f"wrote {out}")

    a = results["agreement"]
    print("\nAgreement with exact Shapley (Spearman rank correlation):")
    print(f"  leave-one-out      {a['agreement']['spearman_loo_vs_shapley']:+.3f}")
    print(f"  influence function {a['agreement']['spearman_if_vs_shapley']:+.3f}")
    print(f"  IF vs LOO          {a['agreement']['spearman_if_vs_loo']:+.3f}")
    print("\nMonte-Carlo convergence:")
    for perms, r in a["mc"].items():
        print(f"  {perms:>5} perms: rho={r['spearman_vs_exact']:+.3f} "
              f"mae={r['mean_abs_error']:.2e} evals={r['utility_evaluations']}")
    s = results["stability"]
    print(f"\nStability across {s['trials']} independent 200-permutation runs:")
    print(f"  max spread {s['max_spread']:.3e} "
          f"({100 * s['max_spread_relative_to_largest_value']:.1f}% of the largest value)")
    print(f"  mean pairwise rank correlation {s['mean_pairwise_spearman']:+.3f}")
    print("\nDilution as the corpus grows:")
    for r in results["dilution"]:
        print(f"  n={r['n']:>4}  median share {r['median_share']:.2e}  "
              f"gini {r['gini_of_positive_contribution']:.3f}")


if __name__ == "__main__":
    main()
