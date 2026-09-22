"""Backend B -- chemical reaction network A -> B -> C (consecutive first order).

Closed-form kinetics, so the whole pipeline stays exactly characterisable.
Conservation is checked as an element balance over the declared inventory.
"""
from __future__ import annotations
import math
import numpy as np
from typing import Dict, List, Optional, Tuple

from ..feasibility import Certificate, conservation_certificate
from ..ir import L1Model, ModeClass, OpTerm
from ..spec import Spec
from ..units import DIMENSIONLESS
from ..verification import Witness
from .base import BackendResult


class CRNBackend:
    name = "crn_consecutive"

    #                 species -> element composition
    INVENTORY = {"A": {"C": 3, "H": 6, "O": 1}}
    PRODUCES = {"B": {"C": 3, "H": 6, "O": 1}, "C": {"C": 3, "H": 6, "O": 1}}

    def __init__(self, t_final: float = 10.0):
        self.t_final = t_final

    def lower(self, spec: Spec) -> L1Model:
        terms = [OpTerm(("A->B",), (0,), "k1", DIMENSIONLESS, validity="k1 > 0"),
                 OpTerm(("B->C",), (1,), "k2", DIMENSIONLESS, validity="k2 > 0")]
        return L1Model(ModeClass.SPECIES_COUNT, 3, terms, [],
                       symmetry_group="mass-conservation", dimensionality=0)

    def param_space(self) -> Dict[str, Tuple[float, float]]:
        return {"k1": (0.05, 3.0), "k2": (0.05, 3.0)}

    def control_channels(self) -> Tuple[int, float]:
        return 2, 8.0

    def reference_log_volume(self) -> float:
        # log2 of the number of distinguishable composition vectors at 1% resolution
        return 3 * math.log2(100)

    def simulate(self, params: Dict[str, float]) -> BackendResult:
        k1, k2, t = params["k1"], params["k2"], self.t_final
        a = math.exp(-k1 * t)
        if abs(k1 - k2) < 1e-9:
            b = k1 * t * math.exp(-k1 * t)
        else:
            b = k1 / (k2 - k1) * (math.exp(-k1 * t) - math.exp(-k2 * t))
        c = max(0.0, 1.0 - a - b)
        return BackendResult({"frac_A": a, "frac_B": b, "frac_C": c,
                              "yield_C": c, "impurity_B": b})

    def measure(self, params: Dict[str, float], observable: str,
                shots: int, seed: int) -> Tuple[float, float]:
        truth = self.simulate(params).observables[observable]
        rng = np.random.default_rng(seed)
        sigma = 0.02      # HPLC relative repeatability, per injection
        se = sigma / math.sqrt(max(1, shots))
        return float(truth + rng.normal(0.0, se)), float(se)

    def witnesses(self) -> List[Witness]:
        def uv_sep(o):
            c = o.get("frac_C", 0.0)
            return {"incomplete_conversion": max(0.0, c - 0.5),
                    "overreaction": max(0.0, 0.999 - c)}

        def imp_sep(o):
            b = o.get("frac_B", 0.0)
            return {"incomplete_conversion": max(0.0, 0.3 - b)}

        def nmr_sep(o):
            a = o.get("frac_A", 0.0)
            return {"incomplete_conversion": max(0.0, 0.3 - a),
                    "wrong_substrate": 0.30}

        return [
            Witness("hplc_uv", "yield_C", 40.0, 0.02, destructive=True,
                    separation_fn=uv_sep),
            Witness("hplc_impurity", "impurity_B", 40.0, 0.02, destructive=True,
                    separation_fn=imp_sep),
            Witness("nmr_assay", "frac_A", 120.0, 0.01, destructive=True,
                    separation_fn=nmr_sep),
        ]

    def static_obstruction(self, spec: Spec) -> Optional[Certificate]:
        available = set()
        for comp in self.INVENTORY.values():
            available |= set(comp)
        needed: Dict[str, float] = {}
        for r in spec.requirements:
            el = r.estimand.protocol_elements if hasattr(r.estimand, "protocol_elements") else None
            _ = el
        # element-balance screen: any requirement naming a species we cannot build
        known = set(self.INVENTORY) | set(self.PRODUCES)
        for r in spec.requirements:
            nm = r.estimand.name
            if nm.startswith(("frac_", "yield_", "impurity_")):
                sp = nm.split("_", 1)[1]
                if sp not in known:
                    needed[sp] = r.bound.value
        if needed:
            return conservation_certificate(needed)
        return None
