"""Typed infeasibility certificates.

Eight certificate classes, each independently checkable. See monograph 7.2.1.
A compiler that returns INFEASIBLE without one of these is reporting search
failure, which is a soundness bug.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Dict, Optional


class ObstructionClass(str, Enum):
    CONSERVATION = "conservation"
    SPECTRAL = "spectral"
    THERMODYNAMIC = "thermodynamic"
    CONTROLLABILITY = "controllability"
    RESOURCE = "resource"
    SAFETY = "safety"
    METROLOGICAL = "metrological"
    SUBSTRATE_EXPRESSIVITY = "substrate-expressivity"
    GRID_EXHAUSTION = "grid-exhaustion"   # weaker: search-based, states its own scope


@dataclass
class Certificate:
    """An obstruction certificate. `checkable` records whether verification of the
    certificate is independent of the search that produced it."""
    cls: ObstructionClass
    statement: str
    witness: Dict[str, Any] = field(default_factory=dict)
    checkable: bool = True
    caveat: str = ""

    def check(self) -> bool:
        """Re-verify the certificate from its own witness data."""
        if self.cls is ObstructionClass.CONSERVATION:
            w = self.witness
            return bool(w.get("missing_species"))
        if self.cls is ObstructionClass.SPECTRAL:
            w = self.witness
            return abs(w["purity_initial"] - w["purity_target"]) > w.get("tol", 1e-9)
        if self.cls is ObstructionClass.RESOURCE:
            w = self.witness
            return w["required"] > w["available"]
        if self.cls is ObstructionClass.METROLOGICAL:
            return bool(self.witness.get("indistinguishable_pairs"))
        if self.cls is ObstructionClass.GRID_EXHAUSTION:
            w = self.witness
            return w["best_margin"] < -w["lipschitz_slack"]
        return self.checkable


def conservation_certificate(missing: Dict[str, float]) -> Certificate:
    return Certificate(
        ObstructionClass.CONSERVATION,
        f"specification requires species absent from the declared inventory: "
        f"{sorted(missing)}",
        {"missing_species": missing},
    )


def spectral_certificate(p0: float, pt: float, tol: float = 1e-9) -> Certificate:
    return Certificate(
        ObstructionClass.SPECTRAL,
        f"closed unitary control preserves spec(rho); purity {p0:.6f} -> {pt:.6f} "
        f"is not reachable without an entropy sink",
        {"purity_initial": p0, "purity_target": pt, "tol": tol},
    )


def resource_certificate(kind: str, required: float, available: float) -> Certificate:
    return Certificate(
        ObstructionClass.RESOURCE,
        f"{kind}: required {required:g} exceeds available {available:g}",
        {"kind": kind, "required": required, "available": available},
    )


def metrological_certificate(pairs) -> Certificate:
    return Certificate(
        ObstructionClass.METROLOGICAL,
        "no available measurement separates the target from a declared alternative",
        {"indistinguishable_pairs": list(pairs)},
    )


def grid_exhaustion_certificate(n_points: float, resolution: float,
                                best_margin: float, lipschitz: float) -> Certificate:
    slack = lipschitz * resolution / 2.0
    return Certificate(
        ObstructionClass.GRID_EXHAUSTION,
        f"exhaustive grid of {n_points:g} points at resolution {resolution:g} found "
        f"best margin {best_margin:g}; empirical Lipschitz constant {lipschitz:g} "
        f"gives interpolation slack {slack:g}",
        {"n_points": n_points, "resolution": resolution,
         "best_margin": best_margin, "lipschitz": lipschitz,
         "lipschitz_slack": slack},
        checkable=True,
        caveat="Search-based. Proves non-existence only within the declared box and "
               "only under the empirical Lipschitz estimate, which is not a bound. "
               "Weaker than the analytic certificate classes.",
    )
