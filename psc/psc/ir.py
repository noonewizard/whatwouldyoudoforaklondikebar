"""Typed intermediate representation.

L0 specification (psc.spec) -> L1 abstract model (here) -> L2/L3 backend-native.
The Hamiltonian representation carries operator type, locality, coefficient,
units, uncertainty, symmetry, control dependence, and validity range, per
monograph section XI.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from enum import Enum
from typing import Callable, Dict, List, Optional, Sequence, Tuple

from .units import Q, Dim, ENERGY


class ModeClass(str, Enum):
    SPIN = "spin"
    FERMIONIC = "fermionic"
    BOSONIC = "bosonic"
    CLASSICAL_FIELD = "classical-field"
    SPECIES_COUNT = "species-count"      # reaction networks
    GEOMETRIC = "geometric"


@dataclass(frozen=True)
class OpTerm:
    """A single Hamiltonian term: coefficient * product of local operators."""
    operators: Tuple[str, ...]          # e.g. ("Z","Z")
    sites: Tuple[int, ...]
    coefficient: str                    # name of the control parameter
    dim: Dim = ENERGY
    uncertainty: float = 0.0
    symmetry: str = ""
    validity: str = ""

    @property
    def locality(self) -> int:
        return len(self.sites)


@dataclass(frozen=True)
class Dissipator:
    jump_operators: Tuple[str, ...]
    sites: Tuple[int, ...]
    rate_param: str
    uncertainty: float = 0.0
    calibration: str = ""


@dataclass
class L1Model:
    """Substrate-free abstract model."""
    mode_class: ModeClass
    n_sites: int
    terms: List[OpTerm] = field(default_factory=list)
    dissipators: List[Dissipator] = field(default_factory=list)
    symmetry_group: str = ""
    dimensionality: int = 1
    boundary: str = "open"
    filling: Optional[float] = None
    drive: Optional[Dict[str, float]] = None

    @property
    def max_locality(self) -> int:
        return max((t.locality for t in self.terms), default=0)

    def control_parameters(self) -> List[str]:
        seen: List[str] = []
        for t in self.terms:
            if t.coefficient not in seen:
                seen.append(t.coefficient)
        for d in self.dissipators:
            if d.rate_param not in seen:
                seen.append(d.rate_param)
        return seen

    def summary(self) -> str:
        return (f"L1[{self.mode_class.value}, N={self.n_sites}, "
                f"terms={len(self.terms)}, k-local={self.max_locality}, "
                f"params={self.control_parameters()}]")
