"""Adversarial test suite (monograph section XXI).

Every failure mode must produce a TYPED diagnostic. A silent COMPILE on an
ill-formed or physically impossible specification is a soundness bug.
"""
import math
import sys, os
import pytest

sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from psc import Spec, Requirement, Estimand, Mode, Cmp, Q, Compiler, Model, ModelBundle
from psc.units import DIMENSIONLESS, ENERGY, LENGTH, MASS, DimensionError
from psc.feasibility import ObstructionClass
from psc.compiler import Outcome
from psc.ptypes import (ControlMode, TargetKind, check_control_mode, spectral_screen)
from psc.verification import Verdict, decide, shots_for_margin, select_witnesses, Witness
from psc.backends.quantum_spin import QuantumSpinBackend
from psc.backends.crn import CRNBackend
from psc.backends.geometry import GeometryBackend

D = DIMENSIONLESS


def req(name, cmp_, val, mode=Mode.VERIFY, **kw):
    return Requirement(Estimand(name, "proto", D), cmp_, Q(val, D), mode, **kw)


# ============================================================ TYPE ERRORS
class TestTypeErrors:
    def test_dimension_mismatch_rejected(self):
        with pytest.raises(DimensionError):
            Requirement(Estimand("gap", "spec", ENERGY), Cmp.GE,
                        Q(1.0, LENGTH), Mode.VERIFY)

    def test_affine_multiplication_rejected(self):
        from psc.units import Q as QQ
        degC = QQ(25.0, (0, 0, 0, 0, 1, 0, 0), affine=True)
        with pytest.raises(DimensionError):
            degC * degC

    def test_derive_without_model_rejected(self):
        with pytest.raises(ValueError, match="names no model"):
            Requirement(Estimand("x", "p", D), Cmp.GE, Q(1.0, D), Mode.DERIVE)

    def test_delta_out_of_range_rejected(self):
        with pytest.raises(ValueError, match="delta"):
            req("x", Cmp.GE, 1.0, delta=1.5)

    def test_missing_alternative_set_diagnosed(self):
        s = Spec("no_alt", [req("zz_end", Cmp.GE, 0.3)], alternatives=[])
        r = Compiler(QuantumSpinBackend(8), grid=9).compile(s)
        assert any("alternative hypothesis set" in d for d in r.diagnostics)

    def test_vacuous_union_bound_diagnosed(self):
        s = Spec("vacuous",
                 [req(f"zz_end", Cmp.GE, 0.0, delta=0.4) for _ in range(3)],
                 alternatives=["paramagnetic"])
        r = Compiler(QuantumSpinBackend(8), grid=9).compile(s)
        assert any("vacuous" in d for d in r.diagnostics)

    def test_unknown_observable_obstructs(self):
        s = Spec("bad_obs", [req("chirality", Cmp.GE, 1.0)],
                 alternatives=["paramagnetic"])
        r = Compiler(QuantumSpinBackend(8), grid=9).compile(s)
        assert r.outcome is Outcome.OBSTRUCT
        assert r.certificate.cls is ObstructionClass.SUBSTRATE_EXPRESSIVITY

    def test_unknown_model_diagnosed(self):
        s = Spec("ghost_model",
                 [req("zz_end", Cmp.GE, 0.3, mode=Mode.DERIVE, model="nonexistent")],
                 alternatives=["paramagnetic"])
        r = Compiler(QuantumSpinBackend(8), grid=9).compile(s)
        assert any("unknown model" in d for d in r.diagnostics)


# ========================================================== PHYSICS ERRORS
class TestPhysicsErrors:
    def test_spectral_screen_blocks_entropy_change_under_unitary(self):
        v = spectral_screen(0.5, 1.0, ControlMode.CLOSED_UNITARY)
        assert not v.ok and "spec(rho) is invariant" in v.reason
        assert v.required_resource and "entropy sink" in v.required_resource

    def test_spectral_screen_passes_for_open_system(self):
        assert spectral_screen(0.5, 1.0, ControlMode.MARKOVIAN_OPEN).ok

    def test_control_mode_typing_rejects_entropy_changing_unitary(self):
        v = check_control_mode([TargetKind.ENTROPY_CHANGING],
                               ControlMode.CLOSED_UNITARY)
        assert not v.ok

    def test_unreachable_target_obstructs_with_certificate(self):
        # long-range order AND a large E1-E0 gap: the ordered phase has an
        # exponentially small doublet splitting, so this is physically impossible
        s = Spec("order_plus_doublet_gap",
                 [req("zz_end", Cmp.GE, 0.5), req("gap_01", Cmp.GE, 0.5)],
                 alternatives=["paramagnetic"])
        r = Compiler(QuantumSpinBackend(10), grid=15).compile(s)
        assert r.outcome is Outcome.OBSTRUCT
        assert r.certificate.cls is ObstructionClass.GRID_EXHAUSTION
        assert r.certificate.check()
        assert r.certificate.caveat, "search-based certificate must state its scope"

    def test_resource_obstruction_mass_budget(self):
        s = Spec("too_heavy", [req("mass", Cmp.LE, 50.0)],
                 alternatives=["undersized_section"])
        r = Compiler(GeometryBackend(mass_budget_kg=2.0), grid=9).compile(s)
        assert r.outcome is Outcome.OBSTRUCT
        assert r.certificate.cls is ObstructionClass.RESOURCE
        assert r.certificate.check()

    def test_conservation_obstruction_missing_species(self):
        s = Spec("alchemy", [req("yield_Xenon", Cmp.GE, 0.9)],
                 alternatives=["incomplete_conversion"])
        r = Compiler(CRNBackend(), grid=9).compile(s)
        assert r.outcome is Outcome.OBSTRUCT
        assert r.certificate.cls is ObstructionClass.CONSERVATION
        assert r.certificate.check()


# ============================================================ MODEL ERRORS
class TestModelErrors:
    def _two_models(self, spread):
        a = Model("model_a", "1", lambda p: {"zz_end": 0.55},
                  lambda p: True, {"zz_end": 0.02})
        b = Model("model_b", "1", lambda p: {"zz_end": 0.55 + spread},
                  lambda p: True, {"zz_end": 0.02})
        return ModelBundle([a, b])

    def test_model_disagreement_forces_MEASURE(self):
        s = Spec("disputed",
                 [req("zz_end", Cmp.GE, 0.50, mode=Mode.DERIVE, model="model_a")],
                 alternatives=["paramagnetic"])
        r = Compiler(QuantumSpinBackend(8), self._two_models(0.60), grid=9).compile(s)
        assert r.outcome is Outcome.MEASURE
        assert r.gaps and r.gaps[0].observable == "zz_end"
        assert any("model spread exceeds" in d for d in r.diagnostics)

    def test_small_disagreement_does_not_block(self):
        s = Spec("agreed",
                 [req("zz_end", Cmp.GE, 0.20, mode=Mode.DERIVE, model="model_a")],
                 alternatives=["paramagnetic"])
        r = Compiler(QuantumSpinBackend(8), self._two_models(0.001), grid=9).compile(s)
        assert r.outcome is not Outcome.MEASURE

    def test_out_of_domain_model_is_not_used(self):
        m = Model("narrow", "1", lambda p: {"zz_end": 9.9},
                  lambda p: p["J"] > 99.0, {"zz_end": 0.01})
        b = ModelBundle([m])
        assert b.valid_at({"J": 1.0, "h": 0.5}) == []


# ===================================================== VERIFICATION ERRORS
class TestVerificationErrors:
    def test_three_valued_decision_never_collapses_unverified(self):
        r = req("x", Cmp.GE, 1.0)
        v, _ = decide(1.0001, 0.5, r, 0.05, 0.05)
        assert v is Verdict.UNVERIFIED

    def test_clear_pass_and_fail(self):
        r = req("x", Cmp.GE, 1.0)
        assert decide(3.0, 0.1, r, 0.05, 0.05)[0] is Verdict.PASS
        assert decide(-1.0, 0.1, r, 0.05, 0.05)[0] is Verdict.FAIL

    def test_zero_margin_needs_unbounded_samples(self):
        assert shots_for_margin(1.0, 0.0, 0.05) >= 10 ** 9

    def test_shot_count_scales_as_inverse_square_margin(self):
        n1 = shots_for_margin(1.0, 0.10, 0.05)
        n2 = shots_for_margin(1.0, 0.05, 0.05)
        assert 3.5 <= n2 / n1 <= 4.5

    def test_uncovered_alternative_is_reported(self):
        ws = [Witness("w1", "o1", 1.0, 1.0, separates={"alt_a": 0.5})]
        mp = select_witnesses(ws, ["alt_a", "alt_b"], {"o1": 0.5}, 0.05, 1e6)
        assert mp.uncovered == ["alt_b"]

    def test_indistinguishable_target_leaves_everything_uncovered(self):
        ws = [Witness("blind", "o1", 1.0, 1.0, separates={})]
        mp = select_witnesses(ws, ["alt_a"], {"o1": 1.0}, 0.05, 1e6)
        assert mp.covered == [] and mp.uncovered == ["alt_a"]

    def test_budget_exhaustion_leaves_alternatives_uncovered(self):
        ws = [Witness("costly", "o1", 1e6, 1.0, separates={"alt_a": 0.01})]
        mp = select_witnesses(ws, ["alt_a"], {"o1": 0.01}, 0.05, 10.0)
        assert mp.uncovered == ["alt_a"]


# ========================================================= COMPILER ERRORS
class TestCompilerIntegrity:
    def test_every_obstruction_carries_a_recheckable_certificate(self):
        cases = [
            (Spec("m", [req("mass", Cmp.LE, 50.0)], alternatives=["x"]),
             GeometryBackend(mass_budget_kg=2.0)),
            (Spec("c", [req("yield_Xenon", Cmp.GE, 0.9)], alternatives=["x"]),
             CRNBackend()),
            (Spec("o", [req("chirality", Cmp.GE, 1.0)], alternatives=["x"]),
             QuantumSpinBackend(8)),
        ]
        for s, be in cases:
            r = Compiler(be, grid=9).compile(s)
            assert r.outcome is Outcome.OBSTRUCT
            assert r.certificate is not None
            assert r.certificate.check(), f"certificate not re-checkable: {r.certificate}"

    def test_provenance_replay_reproduces_verdict(self):
        s = Spec("replay", [req("zz_end", Cmp.GE, 0.2)],
                 alternatives=["paramagnetic"])
        r = Compiler(QuantumSpinBackend(8), grid=11).compile(s)
        rep = r.provenance.replay()
        assert rep["root_digest"] and rep["n_nodes"] > 0
        assert rep["verdict"] in ("PASS", "MIXED")
        assert rep["assumptions"], "provenance must expose its assumptions"

    def test_provenance_digest_is_tamper_evident(self):
        s = Spec("tamper", [req("zz_end", Cmp.GE, 0.2)],
                 alternatives=["paramagnetic"])
        r = Compiler(QuantumSpinBackend(8), grid=11).compile(s)
        before = r.provenance.root_digest()
        nid = r.provenance.order[0]
        r.provenance.nodes[nid].payload["name"] = "forged"
        assert r.provenance.root_digest() != before

    def test_never_silently_compiles_an_impossible_spec(self):
        impossible = [
            Spec("i1", [req("zz_end", Cmp.GE, 1.5)], alternatives=["x"]),
            Spec("i2", [req("gap_02", Cmp.GE, 1e6)], alternatives=["x"]),
            Spec("i3", [req("x_mag", Cmp.GE, 2.0)], alternatives=["x"]),
        ]
        for s in impossible:
            r = Compiler(QuantumSpinBackend(8), grid=11).compile(s)
            assert r.outcome is not Outcome.COMPILE, f"{s.name} silently compiled"

    def test_coverage_reports_evidence_modes(self):
        s = Spec("cov", [req("zz_end", Cmp.GE, 0.2),
                         req("gap_02", Cmp.GE, 0.1, mode=Mode.DERIVE, model="m")],
                 alternatives=["paramagnetic"])
        m = Model("m", "1", lambda p: {"gap_02": 1.0}, lambda p: True, {"gap_02": 0.1})
        r = Compiler(QuantumSpinBackend(8), ModelBundle([m]), grid=9).compile(s)
        assert abs(r.coverage["VERIFY"] - 0.5) < 1e-9
        assert abs(r.coverage["DERIVE"] - 0.5) < 1e-9


# ================================================= CROSS-BACKEND SEMANTICS
class TestFrontendUniversality:
    def test_same_spec_types_target_all_three_backends(self):
        """Falsification F-25: one frontend semantics, three substrates."""
        cases = [
            (QuantumSpinBackend(8),
             Spec("q", [req("zz_end", Cmp.GE, 0.2)], alternatives=["paramagnetic"])),
            (CRNBackend(),
             Spec("c", [req("yield_C", Cmp.GE, 0.5)],
                  alternatives=["incomplete_conversion"])),
            (GeometryBackend(),
             Spec("g", [req("mass", Cmp.LE, 1.5)],
                  alternatives=["undersized_section"])),
        ]
        for be, s in cases:
            r = Compiler(be, grid=9).compile(s)
            assert r.outcome in (Outcome.COMPILE, Outcome.MEASURE, Outcome.OBSTRUCT)
            assert r.coverage and r.provenance is not None
