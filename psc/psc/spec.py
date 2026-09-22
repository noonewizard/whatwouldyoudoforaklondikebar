"""L0 specification layer: substrate-free.

A specification is an acceptance region over estimands, each of which is a
(property functional, measurement protocol) pair, with an evidence mode and a
declared alternative-hypothesis set.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from enum import Enum
from typing import Dict, List, Optional, Sequence

from .units import Q, Dim, DIMENSIONLESS, DimensionError


class Mode(str, Enum):
    """Evidence mode. See monograph section 5.1 Rule 4."""
    VERIFY = "VERIFY"   # will be measured on this article or a declared sample
    ASSUME = "ASSUME"   # inherited from an upstream attestation
    DERIVE = "DERIVE"   # predicted by a named model with a validity domain


class Cmp(str, Enum):
    GE = ">="
    LE = "<="
    IN = "IN"


class Statistic(str, Enum):
    MEAN = "MEAN"
    MIN = "MIN"
    MAX = "MAX"
    UCB = "UCB"          # upper confidence bound
    QUANTILE = "QUANTILE"


@dataclass(frozen=True)
class Estimand:
    """A property functional paired with its measurement protocol.

    Two estimands with the same name but different protocols do NOT unify.
    """
    name: str
    protocol: str
    dim: Dim = DIMENSIONLESS

    def key(self) -> str:
        return f"{self.name}@{self.protocol}"


@dataclass(frozen=True)
class Requirement:
    estimand: Estimand
    cmp: Cmp
    bound: Q
    mode: Mode
    delta: float = 0.05                 # false-accept bound for this requirement
    statistic: Statistic = Statistic.MEAN
    population: str = "UNIT"
    model: Optional[str] = None         # required iff mode is DERIVE
    upper: Optional[Q] = None           # for Cmp.IN

    def __post_init__(self) -> None:
        if self.estimand.dim != self.bound.dim:
            raise DimensionError(
                f"requirement on {self.estimand.key()} has dimension "
                f"{self.estimand.dim} but bound has {self.bound.dim}"
            )
        if self.mode is Mode.DERIVE and not self.model:
            raise ValueError(
                f"DERIVE requirement on {self.estimand.key()} names no model"
            )
        if not (0.0 < self.delta < 1.0):
            raise ValueError("delta must lie in (0,1)")

    def satisfied_by(self, x: float) -> bool:
        if self.cmp is Cmp.GE:
            return x >= self.bound.value
        if self.cmp is Cmp.LE:
            return x <= self.bound.value
        assert self.upper is not None
        return self.bound.value <= x <= self.upper.value

    def margin(self, x: float) -> float:
        """Signed distance to the acceptance boundary; positive means inside."""
        if self.cmp is Cmp.GE:
            return x - self.bound.value
        if self.cmp is Cmp.LE:
            return self.bound.value - x
        assert self.upper is not None
        return min(x - self.bound.value, self.upper.value - x)


@dataclass
class Spec:
    """S = <O, K, delta, beta, Alt>."""
    name: str
    requirements: List[Requirement] = field(default_factory=list)
    alternatives: List[str] = field(default_factory=list)   # Alt(S)
    beta: float = 0.05
    constraints: Dict[str, Q] = field(default_factory=dict)  # resource envelope
    notes: str = ""

    @property
    def delta_total(self) -> float:
        """Union bound over requirements."""
        return sum(r.delta for r in self.requirements)

    def coverage(self) -> Dict[str, float]:
        n = len(self.requirements) or 1
        c = {m.value: 0 for m in Mode}
        for r in self.requirements:
            c[r.mode.value] += 1
        return {k: v / n for k, v in c.items()}

    def estimands(self) -> Sequence[Estimand]:
        return [r.estimand for r in self.requirements]
