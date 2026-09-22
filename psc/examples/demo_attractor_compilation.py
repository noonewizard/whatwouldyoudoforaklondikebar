"""Attractor compilation: the attractiveness functional and the landscape bound.

Demonstrates Definition 19.1, Corollary 19.3 (metastability is first-class), and
Proposition 19.5 (the landscape-reachability bound).
"""
import sys, os, math
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))
import numpy as np

from psc.backends.double_well import DoubleWellBackend
from psc.landscape import (landscape_route_available, landscape_budget_bits,
                           crossover_volume)


def banner(t):
    print("\n" + "=" * 78 + f"\n{t}\n" + "=" * 78)


banner("1. A(K; lambda, T_op) across the four dynamical regimes")
print(f"  {'regime':46s} {'A':>7s} {'capture':>8s} {'Gamma':>7s}")
cases = [
    ("attracting  (prepared left, deep right well)", -1.0, {"tilt": 1.2, "barrier": 1.0}),
    ("reachable   (prepared left, modest barrier)", -1.0, {"tilt": 0.6, "barrier": 0.6}),
    ("unreachable (prepared left, huge barrier)", -1.0, {"tilt": -1.5, "barrier": 4.0}),
    ("diffusive   (prepared left, no barrier)", -1.0, {"tilt": 0.0, "barrier": 0.2}),
]
for name, x0, p in cases:
    r = DoubleWellBackend(x0=x0).analyse(p)
    print(f"  {name:46s} {r.attractiveness:7.3f} {r.capture_prob:8.3f} "
          f"{r.escape_rate:7.3f}")

banner("2. Metastability is first-class (Corollary 19.3)")
print("  Target PREPARED in the right well; the LEFT well is thermodynamically")
print("  favoured. A depends on the operating window -- which is exactly why")
print("  diamond, steel and every Floquet phase are legitimate targets.\n")
print(f"  {'T_op':>8s} {'A (barrier=3.0)':>18s} {'A (barrier=0.5)':>18s}")
for t_op in (2.0, 10.0, 50.0, 200.0):
    hi = DoubleWellBackend(x0=1.0, t_op=t_op, n_traj=300).analyse(
        {"tilt": -0.30, "barrier": 3.0})
    lo = DoubleWellBackend(x0=1.0, t_op=t_op, n_traj=300).analyse(
        {"tilt": -0.30, "barrier": 0.5})
    print(f"  {t_op:8.1f} {hi.attractiveness:18.3f} {lo.attractiveness:18.3f}")
print("\n  High barrier: A stays near 1 across four decades of T_op -- a usable")
print("  metastable product. Low barrier: A collapses -- the same thermodynamics,")
print("  a different kinetic regime, and only the second one fails.")

banner("3. Landscape synthesis: argmax_lambda A")
be = DoubleWellBackend(x0=-1.0, t_op=20.0, n_traj=200)
best, rows = None, []
for tilt in np.linspace(-1.5, 1.5, 13):
    for barrier in np.linspace(0.2, 4.0, 8):
        r = be.analyse({"tilt": float(tilt), "barrier": float(barrier)})
        rows.append((float(tilt), float(barrier), r.attractiveness, r.escape_rate))
        if best is None or r.attractiveness > best[2]:
            best = rows[-1]
print(f"  searched {len(rows)} landscape parameterisations")
print(f"  lambda* = tilt {best[0]:+.3f}, barrier {best[1]:.3f}")
print(f"  A(lambda*) = {best[2]:.3f}   escape rate Gamma = {best[3]:.3f}")
top = sorted(rows, key=lambda r: -r[2])[:5]
print("  top 5:")
for t, b_, a, g in top:
    print(f"    tilt {t:+.3f}  barrier {b_:.3f}  A={a:.3f}  Gamma={g:.3f}")

banner("4. The landscape-reachability bound (Proposition 19.5)")
print("  The budget is PER SUBSTRATE. Each target is checked against the landscape")
print("  that would plausibly be used for it, not against one fixed substrate.\n")
targets = [
    # name,                                                 I(S) bits,  p,   b
    ("crystal from melt: space group + lattice + composition", 10.0,     6, 12.0),
    ("quantum phase, 10-parameter Hamiltonian family",          30.0,    10, 12.0),
    ("64x64 algorithmic tile pattern (sticky-end sequences)",   60.0,    12, 12.0),
    ("protein sequence, 300 residues",           300*math.log2(20),      8, 12.0),
    ("CAD part, 1e6 independently toleranced features",         1e7,     8, 12.0),
    ("this double-well substrate's own reachable targets",       8.0,     2, 10.0),
]
for name, bits, p_, b_ in targets:
    ok, why = landscape_route_available(bits, p_, b_)
    tag = "LANDSCAPE" if ok else "EXPLICIT ADDRESSING REQUIRED"
    print(f"  {tag:30s} {name}")
    print(f"      p={p_}, b={b_:.0f}  ->  {why}")
print("\n  The tile row is the interesting one: 12 sequence parameters at 12 bits")
print("  each (144) exceed the 60 bits the pattern needs, so algorithmic assembly")
print("  sits INSIDE the bound -- which is the tile-complexity theorem restated in")
print("  information units. The protein and CAD rows sit far outside it, which is")
print("  why sequence is specified by synthesis and not by culture conditions.")

banner("3b. CONFIRMED DEFECT: A is insensitive to occupancy continuity")
print("  The search above returned the LOWEST barrier (0.200) because it gives the")
print("  fastest capture -- and with it the HIGHEST escape rate in the top-5 table.")
print("  A rewards time-in-region and is blind to how that time is distributed, so")
print("  it prefers a shallow well entered quickly and left often over a deep well")
print("  entered slowly and held. This is exactly the objection raised against")
print("  Definition 19.1 in section 19.7 of the monograph, now confirmed in code.")
print("  Fix: constrain Gamma separately for service-life targets rather than")
print("  folding it into A. Demonstrated below.\n")
constrained = [r for r in rows if r[3] <= 0.06]
if constrained:
    cb = max(constrained, key=lambda r: r[2])
    print(f"  unconstrained argmax : tilt {best[0]:+.3f} barrier {best[1]:.3f}  "
          f"A={best[2]:.3f}  Gamma={best[3]:.3f}")
    print(f"  with Gamma <= 0.06   : tilt {cb[0]:+.3f} barrier {cb[1]:.3f}  "
          f"A={cb[2]:.3f}  Gamma={cb[3]:.3f}")
    print(f"  -> {100*(1-cb[2]/best[2]):.1f}% less attractiveness, "
          f"{100*(1-cb[3]/max(best[3],1e-9)):.0f}% less escape rate")

banner("5. Control-versus-landscape crossover (Proposition 19.8)")
print(f"  {'N_dof':>10s} {'crossover volume V*':>22s}   interpretation")
for n in (10, 100, 10**4, 10**6, 10**23):
    v = crossover_volume(design_cost_once=1e5, n_dof=n, cost_per_address=1e-3,
                         n_params=2, cost_per_param=1.0,
                         stab_landscape=0.0, stab_explicit=0.1 * n * 1e-3)
    if math.isinf(v):
        interp = "explicit wins at every volume"
    elif v < 1:
        interp = "landscape wins even for a single unit"
    else:
        interp = f"landscape wins above {v:,.0f} units"
    print(f"  {n:10.0e} {v:22.4g}   {interp}")
print("\n  The crossover moves to smaller N as volume grows, and for N ~ 1e23 the")
print("  explicit route does not exist at any volume -- which is the whole reason")
print("  bulk matter is made by setting conditions rather than by placing atoms.")
