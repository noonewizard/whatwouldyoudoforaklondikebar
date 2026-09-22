"""Backend D -- a driven double-well landscape. Attractor compilation testbed.

V(x; a, barrier) = barrier * (x^2 - 1)^2  -  a * x

`a` tilts the landscape (which well is favoured); `barrier` sets the height.
Overdamped Langevin at temperature set by `noise`. Small enough to characterise
exactly, rich enough to exhibit attracting, metastable and unreachable targets.
"""
from __future__ import annotations
import math
import numpy as np
from typing import Dict, List, Optional, Tuple

from ..feasibility import Certificate
from ..ir import L1Model, ModeClass, OpTerm
from ..landscape import LandscapeResult, occupation
from ..spec import Spec
from ..units import DIMENSIONLESS as D
from ..verification import Witness
from .base import BackendResult


class DoubleWellBackend:
    name = "double_well_landscape"

    def __init__(self, noise: float = 0.35, t_op: float = 20.0,
                 x0: float = -1.0, n_traj: int = 300, dt: float = 2e-3):
        self.noise, self.t_op, self.x0 = noise, t_op, x0
        self.n_traj, self.dt = n_traj, dt

    def lower(self, spec: Spec) -> L1Model:
        return L1Model(ModeClass.CLASSICAL_FIELD, 1,
                       [OpTerm(("x^4",), (0,), "barrier", D),
                        OpTerm(("x",), (0,), "tilt", D)],
                       symmetry_group="Z2 (broken by tilt)", dimensionality=1)

    def param_space(self) -> Dict[str, Tuple[float, float]]:
        return {"tilt": (-1.5, 1.5), "barrier": (0.2, 4.0)}

    def control_channels(self) -> Tuple[int, float]:
        return 2, 10.0

    def reference_log_volume(self) -> float:
        return 8.0

    def _drift(self, tilt: float, barrier: float):
        # -dV/dx  with V = barrier*(x^2-1)^2 - tilt*x
        def f(x, t):
            return -(4.0 * barrier * x * (x ** 2 - 1.0)) + tilt
        return f

    def analyse(self, params: Dict[str, float]) -> LandscapeResult:
        tilt, barrier = params["tilt"], params["barrier"]
        return occupation(self._drift(tilt, barrier), self.noise,
                          np.array([self.x0]),
                          lambda x: (x[:, 0] > 0.5),
                          self.t_op, self.dt, self.n_traj, seed=11)

    def simulate(self, params: Dict[str, float]) -> BackendResult:
        r = self.analyse(params)
        return BackendResult(
            {"attractiveness": r.attractiveness,
             "capture_prob": r.capture_prob,
             "terminal_occupation": r.terminal_occupation,
             "escape_rate": r.escape_rate,
             "mean_first_entry": (r.mean_first_entry
                                  if math.isfinite(r.mean_first_entry) else 1e9)},
            aux={"n_traj": r.n_traj})

    def measure(self, params: Dict[str, float], observable: str,
                shots: int, seed: int) -> Tuple[float, float]:
        truth = self.simulate(params).observables[observable]
        rng = np.random.default_rng(seed)
        # occupancy is a Bernoulli mean: binomial standard error
        p = min(max(truth, 1e-6), 1 - 1e-6) if observable in (
            "attractiveness", "capture_prob", "terminal_occupation") else truth
        sigma = math.sqrt(p * (1 - p)) if observable in (
            "attractiveness", "capture_prob", "terminal_occupation") else 0.1 * abs(p)
        se = sigma / math.sqrt(max(1, shots))
        return float(truth + rng.normal(0.0, se)), float(se)

    @staticmethod
    def _shot_sigma(observable: str) -> float:
        return 0.5 if observable in ("attractiveness", "capture_prob",
                                     "terminal_occupation") else 0.1

    def witnesses(self) -> List[Witness]:
        def occ_sep(o):
            a = o.get("attractiveness", 0.0)
            return {"trapped_in_left_well": a, "no_barrier_diffusive": abs(a - 0.5)}

        def esc_sep(o):
            g = o.get("escape_rate", 0.0)
            return {"no_barrier_diffusive": min(1.0, g), "trapped_in_left_well": 0.2}

        return [Witness("occupancy_imaging", "attractiveness", 1.0, 0.5,
                        separation_fn=occ_sep),
                Witness("dwell_time_stats", "escape_rate", 4.0, 0.1,
                        separation_fn=esc_sep)]

    def static_obstruction(self, spec: Spec) -> Optional[Certificate]:
        return None
