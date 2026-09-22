"""Landscape / attractor compilation.

Implements Definition 19.1: attractiveness A(K; lambda, T_op) is the expected
occupation fraction of the acceptance region over the operating window. It is
the unique scalar that handles attracting sets, metastable sets, stationary
distributions and unreachable sets without special cases (Proposition 19.2).
"""
from __future__ import annotations
import math
from dataclasses import dataclass, field
from typing import Callable, Dict, List, Optional, Sequence, Tuple

import numpy as np


@dataclass
class LandscapeResult:
    attractiveness: float          # A in [0,1]
    capture_prob: float            # fraction of trajectories that ever enter K
    mean_first_entry: float        # inf if never
    escape_rate: float             # Gamma, per unit time, estimated from exits
    terminal_occupation: float     # A restricted to the final time slice
    n_traj: int


def occupation(drift: Callable[[np.ndarray, float], np.ndarray],
               noise: float,
               x0: np.ndarray,
               in_region: Callable[[np.ndarray], np.ndarray],
               t_op: float,
               dt: float = 1e-3,
               n_traj: int = 400,
               seed: int = 0) -> LandscapeResult:
    """Estimate A by Euler-Maruyama sampling of an overdamped Langevin flow.

    drift(x, t) -> dx/dt ;  noise is sqrt(2D).
    """
    rng = np.random.default_rng(seed)
    n_steps = max(1, int(round(t_op / dt)))
    x = np.repeat(np.asarray(x0, dtype=float)[None, :], n_traj, axis=0)
    occ = np.zeros(n_traj)
    ever = np.zeros(n_traj, dtype=bool)
    first = np.full(n_traj, np.inf)
    exits = 0
    prev_in = in_region(x)
    sq = noise * math.sqrt(dt)
    for k in range(n_steps):
        t = k * dt
        x = x + drift(x, t) * dt + sq * rng.normal(size=x.shape)
        cur = in_region(x)
        occ += cur.astype(float)
        newly = cur & ~ever
        first[newly] = t
        ever |= cur
        exits += int(np.sum(prev_in & ~cur))
        prev_in = cur
    A = float(np.mean(occ) / n_steps)
    total_in_time = float(np.sum(occ) * dt)
    gamma = (exits / total_in_time) if total_in_time > 0 else 0.0
    return LandscapeResult(
        attractiveness=A,
        capture_prob=float(np.mean(ever)),
        mean_first_entry=float(np.mean(first[np.isfinite(first)]))
                          if np.any(np.isfinite(first)) else float("inf"),
        escape_rate=gamma,
        terminal_occupation=float(np.mean(prev_in.astype(float))),
        n_traj=n_traj,
    )


# --------------------------------------------------------------------------
# Proposition 19.5: the landscape-reachability bound.
# --------------------------------------------------------------------------
def landscape_budget_bits(n_params: int, bits_per_param: float) -> float:
    """p * b -- the maximum specification information a landscape can carry."""
    return n_params * bits_per_param


def landscape_route_available(spec_info_bits: float, n_params: int,
                              bits_per_param: float) -> Tuple[bool, str]:
    """Corollary 19.6. Returns (available, explanation).

    This is a HARD CEILING from a counting argument, not an estimate of what is
    achievable: the map lambda -> outcome is many-to-one and noisy in practice, so
    realisable I(S) is far below the bound. A 'True' here means 'not excluded by
    counting', never 'achievable'.
    """
    budget = landscape_budget_bits(n_params, bits_per_param)
    if spec_info_bits <= budget:
        return True, (f"I(S)={spec_info_bits:.1f} bits <= p*b={budget:.1f} bits; "
                      f"landscape route not excluded by counting")
    return False, (f"I(S)={spec_info_bits:.1f} bits > p*b={budget:.1f} bits; "
                   f"NO landscape parameterisation can address this target family. "
                   f"Explicit addressing of ~{spec_info_bits:.0f} bits is required "
                   f"(Proposition 19.5)")


def crossover_volume(design_cost_once: float, n_dof: int, cost_per_address: float,
                     n_params: int, cost_per_param: float,
                     stab_landscape: float = 0.0,
                     stab_explicit: float = 0.0) -> float:
    """Proposition 19.8: production volume above which the landscape route wins."""
    lhs_per_unit = n_params * cost_per_param + stab_landscape
    rhs_per_unit = n_dof * cost_per_address + stab_explicit
    if rhs_per_unit <= lhs_per_unit:
        return float("inf")          # explicit is cheaper at any volume
    return design_cost_once / (rhs_per_unit - lhs_per_unit)
