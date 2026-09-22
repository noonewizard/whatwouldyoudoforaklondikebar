"""Backend A -- transverse-field Ising chain, exact diagonalisation.

H = -J sum_<ij> Z_i Z_j - h sum_i X_i     (open boundary conditions)

Chosen because the whole pipeline is mathematically characterisable at this size:
the spectrum is exact, the phase diagram is known, and the finite-size behaviour
of every observable can be checked against theory.
"""
from __future__ import annotations
import numpy as np
from dataclasses import dataclass
from functools import lru_cache
from typing import Dict, List, Optional, Tuple

from ..feasibility import Certificate, metrological_certificate
from ..ir import Dissipator, L1Model, ModeClass, OpTerm
from ..spec import Spec
from ..units import ENERGY, DIMENSIONLESS
from ..verification import Witness
from .base import BackendResult

_I = np.eye(2)
_X = np.array([[0.0, 1.0], [1.0, 0.0]])
_Z = np.array([[1.0, 0.0], [0.0, -1.0]])


def _op_at(op: np.ndarray, site: int, n: int) -> np.ndarray:
    m = np.array([[1.0]])
    for k in range(n):
        m = np.kron(m, op if k == site else _I)
    return m


@lru_cache(maxsize=64)
def _ops(n: int):
    xs = tuple(_op_at(_X, i, n) for i in range(n))
    zs = tuple(_op_at(_Z, i, n) for i in range(n))
    return xs, zs


class QuantumSpinBackend:
    name = "quantum_spin_tfim"

    def __init__(self, n_sites: int = 10, n_eigs: int = 3):
        if n_sites > 13:
            raise ValueError("dense diagonalisation is capped at 13 sites")
        self.n = n_sites
        self.n_eigs = n_eigs

    # ---- IR -------------------------------------------------------------
    def lower(self, spec: Spec) -> L1Model:
        terms: List[OpTerm] = []
        for i in range(self.n - 1):
            terms.append(OpTerm(("Z", "Z"), (i, i + 1), "J", ENERGY,
                                uncertainty=0.0, symmetry="Z2",
                                validity="|J| <= J_max"))
        for i in range(self.n):
            terms.append(OpTerm(("X",), (i,), "h", ENERGY,
                                symmetry="Z2", validity="|h| <= h_max"))
        return L1Model(ModeClass.SPIN, self.n, terms, [],
                       symmetry_group="Z2 x translation(broken by OBC)",
                       dimensionality=1, boundary="open")

    def param_space(self) -> Dict[str, Tuple[float, float]]:
        return {"J": (0.0, 2.0), "h": (0.0, 2.0)}

    def control_channels(self) -> Tuple[int, float]:
        # two global knobs, ~12 effective bits each (limited by source stability)
        return 2, 12.0

    def reference_log_volume(self) -> float:
        """log2 of the accessible state-space measure used for I_verified."""
        return float(self.n)          # 2^n basis states

    # ---- physics --------------------------------------------------------
    def hamiltonian(self, J: float, h: float) -> np.ndarray:
        xs, zs = _ops(self.n)
        H = np.zeros((2 ** self.n, 2 ** self.n))
        for i in range(self.n - 1):
            H -= J * (zs[i] @ zs[i + 1])
        for i in range(self.n):
            H -= h * xs[i]
        return H

    def simulate(self, params: Dict[str, float]) -> BackendResult:
        J, h = params["J"], params["h"]
        H = self.hamiltonian(J, h)
        evals, evecs = np.linalg.eigh(H)
        g = evecs[:, 0]
        xs, zs = _ops(self.n)
        zz_end = float(g @ (zs[0] @ zs[self.n - 1]) @ g)
        x_mag = float(sum(g @ xs[i] @ g for i in range(self.n)) / self.n)
        obs = {
            "energy_density": float(evals[0] / self.n),
            "gap_01": float(evals[1] - evals[0]),
            "gap_02": float(evals[2] - evals[0]),
            "zz_end": zz_end,
            "x_mag": x_mag,
        }
        return BackendResult(obs, {}, {"spectrum": evals[:self.n_eigs].tolist()})

    def measure(self, params: Dict[str, float], observable: str,
                shots: int, seed: int) -> Tuple[float, float]:
        """Sample the observable with projective shot noise."""
        truth = self.simulate(params).observables[observable]
        rng = np.random.default_rng(seed)
        sigma = self._shot_sigma(observable)
        est = truth + rng.normal(0.0, sigma / np.sqrt(max(1, shots)))
        return float(est), float(sigma / np.sqrt(max(1, shots)))

    @staticmethod
    def _shot_sigma(observable: str) -> float:
        # per-shot standard deviation; bounded observables have sigma <= 1
        return {"zz_end": 1.0, "x_mag": 1.0,
                "energy_density": 0.5, "gap_01": 0.3, "gap_02": 0.3}.get(observable, 1.0)

    # ---- verification ---------------------------------------------------
    def witnesses(self) -> List[Witness]:
        # Separation is a FUNCTION of the operating point. A correlator separates an
        # ordered state from a paramagnet in proportion to the order itself; a
        # spectroscopic gap measurement separates from the critical point in
        # proportion to the gap. Static separation values would make the
        # verification-aware objective vacuous.
        def zz_sep(o):
            z = max(0.0, o.get("zz_end", 0.0))
            return {"paramagnetic": z, "disordered": z}

        def xm_sep(o):
            x = max(0.0, o.get("x_mag", 0.0))
            return {"ferromagnetic_classical": x, "paramagnetic": max(0.0, 1.0 - x)}

        def gap_sep(o):
            g = max(0.0, o.get("gap_02", 0.0))
            return {"gapless_critical": g, "paramagnetic": 0.5 * g,
                    "disordered": 0.3 * g}

        def e_sep(o):
            e = abs(o.get("energy_density", 0.0))
            return {"gapless_critical": 0.25 * e, "disordered": 0.2 * e}

        return [
            Witness("zz_correlator", "zz_end", 1.0, 1.0, separation_fn=zz_sep),
            Witness("transverse_mag", "x_mag", 1.0, 1.0, separation_fn=xm_sep),
            Witness("spectroscopy", "gap_02", 6.0, 0.3, separation_fn=gap_sep),
            Witness("calorimetry", "energy_density", 3.0, 0.5,
                    separation_fn=e_sep),
        ]

    def static_obstruction(self, spec: Spec) -> Optional[Certificate]:
        names = {r.estimand.name for r in spec.requirements}
        known = {"energy_density", "gap_01", "gap_02", "zz_end", "x_mag"}
        unknown = names - known
        if unknown:
            return Certificate(
                cls=__import__("psc.feasibility", fromlist=["ObstructionClass"]
                               ).ObstructionClass.SUBSTRATE_EXPRESSIVITY,
                statement=f"observables not expressible on this substrate: "
                          f"{sorted(unknown)}",
                witness={"unknown": sorted(unknown), "known": sorted(known)},
            )
        return None
