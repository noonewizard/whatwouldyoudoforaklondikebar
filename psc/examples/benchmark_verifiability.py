"""Falsification F-28 (prototype scale): does compile-for-verifiability pay?

Sequential: pick the route with the best physics margin, then verify it.
Joint:      pick the route minimising (physics penalty + verification cost).

Reports cost-to-certify at EQUAL attained confidence.
"""
import sys, os, itertools
import numpy as np
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from psc import Spec, Requirement, Estimand, Mode, Cmp, Q
from psc.units import DIMENSIONLESS as D
from psc.verification import select_witnesses, shots_for_margin
from psc.backends.quantum_spin import QuantumSpinBackend
from psc.backends.crn import CRNBackend
from psc.backends.geometry import GeometryBackend


def route_costs(backend, spec, grid=21):
    space = backend.param_space()
    names = sorted(space)
    axes = [np.linspace(space[n][0], space[n][1], grid) for n in names]
    rows = []
    for combo in itertools.product(*axes):
        params = dict(zip(names, map(float, combo)))
        obs = backend.simulate(params).observables
        margins = {}
        ok = True
        for r in spec.requirements:
            v = obs.get(r.estimand.name)
            if v is None:
                ok = False
                break
            m = r.margin(v)
            if m < 0:
                ok = False
                break
            margins[r.estimand.name] = m
        if not ok:
            continue
        mp = select_witnesses(backend.witnesses(), spec.alternatives, margins,
                              spec.requirements[0].delta, 1e9,
                              operating_point=obs)
        if mp.uncovered:
            continue
        rows.append((params, min(margins.values()), mp.total_cost, mp.total_shots))
    return rows


def run(label, backend, spec, grid=21):
    rows = route_costs(backend, spec, grid)
    if not rows:
        print(f"{label:22s}  no feasible route found")
        return None
    seq = max(rows, key=lambda r: r[1])                    # best physics margin
    # joint: among routes within 80% of the best margin, cheapest to certify
    thresh = 0.8 * seq[1]
    pool = [r for r in rows if r[1] >= thresh] or rows
    joint = min(pool, key=lambda r: r[2])
    red = 100.0 * (1 - joint[2] / seq[2]) if seq[2] > 0 else 0.0
    marg_pen = 100.0 * (1 - joint[1] / seq[1]) if seq[1] > 0 else 0.0
    print(f"{label:22s}  routes={len(rows):5d}")
    print(f"    sequential : margin {seq[1]:+.4g}  cost-to-certify {seq[2]:9.1f}  "
          f"shots {seq[3]}")
    print(f"    joint      : margin {joint[1]:+.4g}  cost-to-certify {joint[2]:9.1f}  "
          f"shots {joint[3]}")
    print(f"    reduction  : {red:+.1f}% cost   ({marg_pen:+.1f}% margin penalty)")
    return red


def req(name, cmp_, val, **kw):
    return Requirement(Estimand(name, "p", D), cmp_, Q(val, D), Mode.VERIFY, **kw)


print("=" * 78)
print("F-28 (prototype scale) -- compile-for-verifiability A/B")
print("=" * 78)

results = []
results.append(run("quantum_spin", QuantumSpinBackend(9),
    Spec("q", [req("zz_end", Cmp.GE, 0.25, delta=0.01),
               req("gap_02", Cmp.GE, 0.30, delta=0.01)],
         alternatives=["paramagnetic", "gapless_critical", "disordered"]), grid=17))

results.append(run("crn", CRNBackend(10.0),
    Spec("c", [req("yield_C", Cmp.GE, 0.90, delta=0.01),
               req("impurity_B", Cmp.LE, 0.05, delta=0.01)],
         alternatives=["incomplete_conversion", "overreaction"]), grid=31))

results.append(run("geometry", GeometryBackend(),
    Spec("g", [req("stiffness", Cmp.GE, 1.5e4, delta=0.01),
               req("mass", Cmp.LE, 1.5, delta=0.01)],
         alternatives=["undersized_section", "wrong_alloy"]), grid=25))

good = [r for r in results if r is not None]
print("-" * 78)
if good:
    print(f"median cost reduction: {float(np.median(good)):+.1f}%")
    print(f"F-28 threshold: PASS requires >=30%, FAIL below 10%")
    med = float(np.median(good))
    print(f"prototype verdict: "
          f"{'PASS' if med >= 30 else ('MARGINAL' if med >= 10 else 'FAIL')}")
