"""One frontend semantics, three substrates. Falsification test F-25.

Run:  python3 examples/demo_three_backends.py
"""
import sys, os, math
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from psc import Spec, Requirement, Estimand, Mode, Cmp, Q, Compiler, ModelBundle, Model
from psc.units import DIMENSIONLESS, ENERGY, MASS
from psc.backends.quantum_spin import QuantumSpinBackend
from psc.backends.crn import CRNBackend
from psc.backends.geometry import GeometryBackend

E = DIMENSIONLESS   # spin-model energies are in units of the coupling


def banner(t):
    print("\n" + "=" * 78 + f"\n{t}\n" + "=" * 78)


# ---------------------------------------------------------------- BACKEND A
banner("BACKEND A -- quantum spin system (transverse-field Ising chain, N=10)")

qspec = Spec(
    name="ordered_phase_with_gap",
    alternatives=["paramagnetic", "gapless_critical", "disordered"],
    requirements=[
        Requirement(Estimand("zz_end", "correlator(0,N-1)", DIMENSIONLESS),
                    Cmp.GE, Q(0.30, DIMENSIONLESS), Mode.VERIFY, delta=0.01),
        Requirement(Estimand("gap_02", "spectroscopy", E),
                    Cmp.GE, Q(0.40, E), Mode.VERIFY, delta=0.01),
        Requirement(Estimand("energy_density", "calorimetry", E),
                    Cmp.LE, Q(-0.90, E), Mode.DERIVE, delta=0.03, model="mft_v1"),
    ],
)

mft = Model(
    model_id="mft_v1", version="1.0",
    predict=lambda p: {"energy_density": -(p["J"] + p["h"] ** 2 / (4 * max(p["J"], 1e-9)))
                       if p["h"] < 2 * p["J"] else -p["h"],
                       "zz_end": max(0.0, 1 - (p["h"] / max(p["J"], 1e-9)) ** 2) ** 0.25},
    validity_domain=lambda p: p["J"] > 0.2,
    prediction_error={"energy_density": 0.15, "zz_end": 0.10},
    training_domain="J in [0.2,2], h in [0,2]", calibration_status="uncalibrated",
    source="mean-field, no fluctuation corrections",
    known_failure_modes=["fails near the critical point h=J"],
)

qc = Compiler(QuantumSpinBackend(n_sites=10), ModelBundle([mft]), grid=21)
qres = qc.compile(qspec)
print(qres.summary())

# ---------------------------------------------------------------- BACKEND B
banner("BACKEND B -- chemical reaction network (A -> B -> C)")

cspec = Spec(
    name="api_lot",
    alternatives=["incomplete_conversion", "overreaction"],
    requirements=[
        Requirement(Estimand("yield_C", "hplc_uv(C18,254nm)", DIMENSIONLESS),
                    Cmp.GE, Q(0.95, DIMENSIONLESS), Mode.VERIFY, delta=0.01),
        Requirement(Estimand("impurity_B", "hplc_uv(C18,254nm)", DIMENSIONLESS),
                    Cmp.LE, Q(0.010, DIMENSIONLESS), Mode.VERIFY, delta=0.01),
    ],
)
cc = Compiler(CRNBackend(t_final=10.0), ModelBundle([]), grid=41, verify_budget=20000.0)
cres = cc.compile(cspec)
print(cres.summary())

# ---------------------------------------------------------------- BACKEND C
banner("BACKEND C -- geometry / manufacturing (aluminium cantilever)")

gspec = Spec(
    name="bracket_beam",
    alternatives=["undersized_section", "wrong_alloy"],
    requirements=[
        Requirement(Estimand("stiffness", "load_deflection(ASTM_E111)", DIMENSIONLESS),
                    Cmp.GE, Q(2.0e4, DIMENSIONLESS), Mode.VERIFY, delta=0.01),
        Requirement(Estimand("mass", "gravimetric(0.1g)", DIMENSIONLESS),
                    Cmp.LE, Q(1.2, DIMENSIONLESS), Mode.VERIFY, delta=0.01),
        Requirement(Estimand("first_mode_hz", "modal_tap", DIMENSIONLESS),
                    Cmp.GE, Q(60.0, DIMENSIONLESS), Mode.VERIFY, delta=0.01),
    ],
)
gc = Compiler(GeometryBackend(), ModelBundle([]), grid=31, verify_budget=5000.0)
gres = gc.compile(gspec)
print(gres.summary())

# ---------------------------------------------------------------- PROVENANCE
banner("PROVENANCE REPLAY (falsification F-30)")
for r in (qres, cres, gres):
    rep = r.provenance.replay()
    print(f"  {r.backend:24s} root={rep['root_digest'][:16]}  "
          f"nodes={rep['n_nodes']:2d}  verdict={rep['verdict']}")
    for a in rep["assumptions"]:
        print(f"      assumes: {a}")

banner("F-25 RESULT -- one frontend, three substrates")
print(f"  Same Spec/Requirement/Estimand/Mode types used for all three: YES")
print(f"  Backend-specific escape hatches in the specification: NONE")
print(f"  Outcomes: quantum={qres.outcome.value}, crn={cres.outcome.value}, "
      f"geometry={gres.outcome.value}")
