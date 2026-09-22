"""The physical type system.

Classifies a compilation target by what it demands of the control mode, so that
impossible control modes are rejected before any expensive optimisation.
Implements the compile-time screens of monograph sections 3.5 / 9.4.1.
"""
from __future__ import annotations
from dataclasses import dataclass
from enum import Enum
from typing import Dict, List, Optional


class TargetKind(str, Enum):
    ENTROPY_PRESERVING = "entropy-preserving"
    ENTROPY_CHANGING = "entropy-changing"
    ENERGY_CHANGING = "energy-changing"
    PARTICLE_NUMBER_CHANGING = "particle-number-changing"
    SYMMETRY_CHANGING = "symmetry-changing"
    TOPOLOGY_CHANGING = "topology-changing"
    MEASUREMENT_DEPENDENT = "measurement-dependent"


class ControlMode(str, Enum):
    CLOSED_UNITARY = "closed-unitary"
    MARKOVIAN_OPEN = "markovian-open"
    MEASUREMENT = "measurement"
    FEEDBACK = "feedback"
    THERMAL_OPERATIONS = "thermal-operations"
    RESERVOIR_ENGINEERING = "reservoir-engineering"
    CLASSICAL_MACROSCOPIC = "classical-macroscopic"


# What each control mode can change. Derived from the scope-stated theorems in
# monograph section 9.4: unitary control preserves spec(rho) exactly.
_CAPABILITY: Dict[ControlMode, set] = {
    ControlMode.CLOSED_UNITARY: {
        TargetKind.ENTROPY_PRESERVING,
        TargetKind.ENERGY_CHANGING,       # work may be done on a driven system
        TargetKind.SYMMETRY_CHANGING,
    },
    ControlMode.MARKOVIAN_OPEN: set(TargetKind) - {TargetKind.MEASUREMENT_DEPENDENT},
    ControlMode.MEASUREMENT: set(TargetKind),
    ControlMode.FEEDBACK: set(TargetKind),
    ControlMode.THERMAL_OPERATIONS: {
        TargetKind.ENTROPY_CHANGING,
        TargetKind.ENTROPY_PRESERVING,
    },
    ControlMode.RESERVOIR_ENGINEERING: set(TargetKind) - {TargetKind.MEASUREMENT_DEPENDENT},
    ControlMode.CLASSICAL_MACROSCOPIC: set(TargetKind),
}


@dataclass
class TypeVerdict:
    ok: bool
    reason: str = ""
    required_resource: Optional[str] = None


def check_control_mode(kinds: List[TargetKind], mode: ControlMode) -> TypeVerdict:
    """Reject impossible control modes before optimisation."""
    allowed = _CAPABILITY[mode]
    bad = [k for k in kinds if k not in allowed]
    if not bad:
        return TypeVerdict(True)
    if mode is ControlMode.CLOSED_UNITARY and TargetKind.ENTROPY_CHANGING in bad:
        return TypeVerdict(
            False,
            "spectral invariance: closed unitary control preserves spec(rho), so it "
            "cannot change von Neumann entropy or purity",
            required_resource="entropy sink (cold reservoir, engineered dissipation, "
                              "or measurement with erasure)",
        )
    return TypeVerdict(
        False,
        f"control mode {mode.value} cannot realise target kinds "
        f"{[k.value for k in bad]}",
    )


def spectral_screen(spec_purity_initial: float, spec_purity_target: float,
                    mode: ControlMode, tol: float = 1e-9) -> TypeVerdict:
    """Theorem 9.5 as an executable compile-time screen."""
    if mode is not ControlMode.CLOSED_UNITARY:
        return TypeVerdict(True)
    if abs(spec_purity_initial - spec_purity_target) <= tol:
        return TypeVerdict(True)
    return TypeVerdict(
        False,
        f"spectral certificate: purity {spec_purity_initial:.6f} -> "
        f"{spec_purity_target:.6f} under closed unitary control; spec(rho) is invariant",
        required_resource="entropy sink",
    )
