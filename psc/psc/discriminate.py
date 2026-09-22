"""Model Discrimination Engine.

Detects that models disagree, decides whether the disagreement matters to the
target, selects a discriminating measurement, and refuses compilation when model
uncertainty dominates the acceptance margin. Monograph section XV.
"""
from __future__ import annotations
import math
from dataclasses import dataclass
from typing import Dict, List, Optional, Sequence

from .models import ModelBundle
from .spec import Requirement


@dataclass
class Disagreement:
    observable: str
    spread: float
    margin: float
    matters: bool

    @property
    def dominance(self) -> float:
        return self.spread / self.margin if self.margin > 0 else float("inf")


@dataclass
class KnowledgeGap:
    observable: str
    experiment: str
    expected_info_bits: float
    cost: float
    resolves: str

    @property
    def value_per_cost(self) -> float:
        return self.expected_info_bits / self.cost if self.cost > 0 else float("inf")


def analyse(bundle: ModelBundle, params: Dict[str, float],
            requirements: Sequence[Requirement]) -> List[Disagreement]:
    out: List[Disagreement] = []
    valid = bundle.valid_at(params)
    if len(valid) < 2:
        return out
    for r in requirements:
        obs = r.estimand.name
        spread = bundle.disagreement(params, obs)
        if spread <= 0:
            continue
        preds = [m.predict(params).get(obs) for m in valid]
        preds = [p for p in preds if p is not None]
        margin = min(abs(r.margin(p)) for p in preds) if preds else 0.0
        out.append(Disagreement(obs, spread, margin, spread > margin))
    return out


def propose_experiments(disagreements: Sequence[Disagreement],
                        unit_cost: Dict[str, float]) -> List[KnowledgeGap]:
    """Rank candidate discriminating experiments by expected information per cost.

    Expected information is bounded above by log2(#models distinguishable), which
    for a two-model disagreement is 1 bit. We report the achievable fraction given
    the measurement's resolving power, not an idealised value.
    """
    gaps: List[KnowledgeGap] = []
    for d in disagreements:
        if not d.matters:
            continue
        c = unit_cost.get(d.observable, 1.0)
        # resolving power: how many standard deviations the spread represents
        frac = min(1.0, d.dominance / (1.0 + d.dominance))
        gaps.append(KnowledgeGap(
            observable=d.observable,
            experiment=f"measure {d.observable} to resolve {d.spread:.4g} spread",
            expected_info_bits=frac,
            cost=c,
            resolves=f"model disagreement on {d.observable} "
                     f"({d.dominance:.2f}x the acceptance margin)",
        ))
    gaps.sort(key=lambda g: -g.value_per_cost)
    return gaps
