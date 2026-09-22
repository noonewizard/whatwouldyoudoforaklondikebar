"""Backend C -- a rectangular cantilever beam. Deliberately trivial physics.

Its purpose is to test whether the SAME frontend semantics can target a
non-quantum, non-chemical substrate without escape hatches (falsification F-25).
"""
from __future__ import annotations
import math
import numpy as np
from typing import Dict, List, Optional, Tuple

from ..feasibility import Certificate, resource_certificate
from ..ir import L1Model, ModeClass, OpTerm
from ..spec import Spec
from ..units import DIMENSIONLESS
from ..verification import Witness
from .base import BackendResult


class GeometryBackend:
    name = "geometry_beam"

    def __init__(self, length_m: float = 0.5, E_pa: float = 70e9,
                 rho: float = 2700.0, mass_budget_kg: float = 2.0):
        self.L, self.E, self.rho, self.mass_budget = length_m, E_pa, rho, mass_budget_kg

    def lower(self, spec: Spec) -> L1Model:
        terms = [OpTerm(("section",), (0,), "width", DIMENSIONLESS),
                 OpTerm(("section",), (1,), "height", DIMENSIONLESS)]
        return L1Model(ModeClass.GEOMETRIC, 2, terms, [], dimensionality=3)

    def param_space(self) -> Dict[str, Tuple[float, float]]:
        return {"width": (0.005, 0.10), "height": (0.005, 0.10)}

    def control_channels(self) -> Tuple[int, float]:
        # two dimensions, machined to 10 um over a 95 mm range -> ~13.2 bits
        return 2, math.log2(0.095 / 10e-6)

    def reference_log_volume(self) -> float:
        return 2 * math.log2(0.095 / 10e-6)

    def simulate(self, params: Dict[str, float]) -> BackendResult:
        w, h = params["width"], params["height"]
        I = w * h ** 3 / 12.0
        k = 3.0 * self.E * I / self.L ** 3          # tip stiffness, N/m
        mass = self.rho * w * h * self.L
        f1 = (1.875 ** 2) / (2 * math.pi) * math.sqrt(
            self.E * I / (self.rho * w * h * self.L ** 4))
        return BackendResult({"stiffness": k, "mass": mass, "first_mode_hz": f1})

    def measure(self, params: Dict[str, float], observable: str,
                shots: int, seed: int) -> Tuple[float, float]:
        truth = self.simulate(params).observables[observable]
        rng = np.random.default_rng(seed)
        rel = {"stiffness": 0.03, "mass": 0.002, "first_mode_hz": 0.01}[observable]
        se = rel * truth / math.sqrt(max(1, shots))
        return float(truth + rng.normal(0.0, se)), float(se)

    def witnesses(self) -> List[Witness]:
        return [
            Witness("load_deflection", "stiffness", cost_per_shot=25.0, sigma=0.03 * 1e4,
                    separates={"undersized_section": 2e3, "wrong_alloy": 1e3}),
            Witness("gravimetric", "mass", cost_per_shot=2.0, sigma=0.004,
                    separates={"undersized_section": 0.2, "wrong_alloy": 0.3}),
            Witness("modal_tap", "first_mode_hz", cost_per_shot=15.0, sigma=1.0,
                    separates={"wrong_alloy": 5.0}),
        ]

    def static_obstruction(self, spec: Spec) -> Optional[Certificate]:
        for r in spec.requirements:
            if r.estimand.name == "mass" and r.cmp.value == "<=":
                if r.bound.value > self.mass_budget:
                    return resource_certificate("mass budget", r.bound.value,
                                                self.mass_budget)
        return None
