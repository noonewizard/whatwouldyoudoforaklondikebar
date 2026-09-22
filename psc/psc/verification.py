"""Verification as a compiler backend.

Witness selection (coverage formulation -- provably submodular), sample
complexity, and the three-valued acceptance decision. See monograph 9.8.
"""
from __future__ import annotations
import math
from dataclasses import dataclass, field
from enum import Enum
from typing import Callable, Dict, List, Optional, Sequence, Tuple


class Verdict(str, Enum):
    PASS = "PASS"
    FAIL = "FAIL"
    UNVERIFIED = "UNVERIFIED"     # measured, inconclusive at the declared confidence
    UNKNOWN = "UNKNOWN"           # not measured / no admissible plan


@dataclass
class Witness:
    """A measurement primitive from the measurement algebra M.

    `separates` gives the baseline |gap| to each named alternative. `separation_fn`,
    when supplied, makes that gap a function of the operating point -- which is the
    physically correct behaviour: a correlator separates an ordered state from a
    paramagnet far better deep in the phase than near criticality. Without this
    dependence the verification-aware objective has nothing to optimise over.
    """
    name: str
    observable: str
    cost_per_shot: float
    sigma: float                  # per-shot standard deviation
    destructive: bool = False
    separates: Dict[str, float] = field(default_factory=dict)  # alternative -> |gap|
    separation_fn: Optional[Callable[[Dict[str, float]], Dict[str, float]]] = None

    def gaps_at(self, obs: Optional[Dict[str, float]]) -> Dict[str, float]:
        if self.separation_fn is not None and obs is not None:
            return self.separation_fn(obs)
        return self.separates


@dataclass
class MeasurementPlan:
    allocation: Dict[str, int]                # witness name -> shots
    total_cost: float
    covered: List[str]
    uncovered: List[str]

    @property
    def total_shots(self) -> int:
        return sum(self.allocation.values())


def shots_for_margin(sigma: float, margin: float, delta: float) -> int:
    """Two-sided normal bound: n >= (z * sigma / margin)^2.

    z is the (1-delta) standard normal quantile, computed by bisection on erf so
    that no scipy dependency is needed here.
    """
    if margin <= 0:
        return 10 ** 9
    target = 1.0 - 2.0 * delta
    lo, hi = 0.0, 10.0
    for _ in range(200):
        mid = 0.5 * (lo + hi)
        if math.erf(mid / math.sqrt(2.0)) < target:
            lo = mid
        else:
            hi = mid
    z = 0.5 * (lo + hi)
    return max(1, int(math.ceil((z * sigma / margin) ** 2)))


def select_witnesses(witnesses: Sequence[Witness],
                     alternatives: Sequence[str],
                     margins: Dict[str, float],
                     delta: float,
                     budget: float,
                     operating_point: Optional[Dict[str, float]] = None
                     ) -> MeasurementPlan:
    """Greedy min-cost cover over the alternative set.

    COVERAGE formulation only. The coverage function is monotone and submodular,
    so greedy carries the standard ln|Alt| approximation guarantee. The mutual
    information formulation is NOT submodular in general and is not used here.
    See monograph 9.8.3.
    """
    remaining = set(alternatives)
    chosen: Dict[str, int] = {}
    cost = 0.0
    pool = list(witnesses)
    while remaining and pool:
        best, best_ratio, best_n, best_new = None, float("inf"), 0, set()
        for w in pool:
            sep = w.gaps_at(operating_point)
            new = {a for a in remaining if sep.get(a, 0.0) > 0.0}
            if not new:
                continue
            gaps = [sep[a] for a in new]
            m = min(min(gaps), margins.get(w.observable, float("inf")))
            n = shots_for_margin(w.sigma, m, delta / max(1, len(alternatives)))
            c = n * w.cost_per_shot
            ratio = c / len(new)
            if ratio < best_ratio:
                best, best_ratio, best_n, best_new = w, ratio, n, new
        if best is None:
            break
        if cost + best_n * best.cost_per_shot > budget:
            break
        chosen[best.name] = best_n
        cost += best_n * best.cost_per_shot
        remaining -= best_new
        pool.remove(best)
    return MeasurementPlan(chosen, cost, sorted(set(alternatives) - remaining),
                           sorted(remaining))


def decide(estimate: float, sigma_eff: float, requirement, delta: float,
           beta: float) -> Tuple[Verdict, float]:
    """Three-valued decision. Never collapses UNVERIFIED into PASS."""
    m = requirement.margin(estimate)
    if sigma_eff <= 0:
        return (Verdict.PASS if m >= 0 else Verdict.FAIL), float("inf")
    z = m / sigma_eff
    # one-sided normal tail
    p_out = 0.5 * (1.0 - math.erf(z / math.sqrt(2.0)))     # Pr[truly outside | data]
    p_in = 1.0 - p_out
    if p_in >= 1.0 - delta:
        return Verdict.PASS, z
    if p_out >= 1.0 - beta:
        return Verdict.FAIL, z
    return Verdict.UNVERIFIED, z
