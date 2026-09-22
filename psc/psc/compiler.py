"""The compiler pipeline.

    S -> type check -> static obstruction -> representation/model selection
      -> landscape synthesis -> robust selection -> measurement-plan synthesis
      -> execution -> measurement -> verification -> Pi

Returns exactly one of COMPILE / OBSTRUCT / MEASURE (Corollary 17.11: the
three-valued outcome is forced, not chosen).
"""
from __future__ import annotations
import itertools
import math
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Dict, List, Optional, Sequence, Tuple

import numpy as np

from .discriminate import Disagreement, KnowledgeGap, analyse, propose_experiments
from .feasibility import Certificate, grid_exhaustion_certificate
from .models import ModelBundle
from .provenance import NodeKind, Provenance
from .spec import Cmp, Mode, Requirement, Spec
from .verification import (MeasurementPlan, Verdict, decide, select_witnesses,
                           shots_for_margin)


class Outcome(str, Enum):
    COMPILE = "COMPILE"
    OBSTRUCT = "OBSTRUCT"
    MEASURE = "MEASURE"


@dataclass
class Plan:
    params: Dict[str, float]
    predicted: Dict[str, float]
    margins: Dict[str, float]
    worst_margin: float
    measurement_plan: Optional[MeasurementPlan] = None
    verify_cost: float = 0.0


@dataclass
class Result:
    outcome: Outcome
    spec: str
    backend: str
    plans: List[Plan] = field(default_factory=list)
    certificate: Optional[Certificate] = None
    gaps: List[KnowledgeGap] = field(default_factory=list)
    verdicts: Dict[str, str] = field(default_factory=dict)
    coverage: Dict[str, float] = field(default_factory=dict)
    leverage: float = 0.0
    provenance: Optional[Provenance] = None
    diagnostics: List[str] = field(default_factory=list)
    measure_rate_note: str = ""

    def summary(self) -> str:
        lines = [f"  outcome      : {self.outcome.value}",
                 f"  backend      : {self.backend}",
                 f"  coverage     : " + ", ".join(
                     f"{k} {v:.0%}" for k, v in sorted(self.coverage.items()) if v)]
        if self.certificate:
            lines.append(f"  obstruction  : [{self.certificate.cls.value}] "
                         f"{self.certificate.statement}")
            lines.append(f"  re-checkable : {self.certificate.check()}")
            if self.certificate.caveat:
                lines.append(f"  caveat       : {self.certificate.caveat}")
        for i, p in enumerate(self.plans[:3]):
            ps = ", ".join(f"{k}={v:.4g}" for k, v in p.params.items())
            lines.append(f"  plan #{i+1}      : {ps}  worst margin {p.worst_margin:+.4g}")
            if p.measurement_plan:
                lines.append(f"      verify   : {p.measurement_plan.total_shots} shots, "
                             f"cost {p.measurement_plan.total_cost:.1f}, "
                             f"covers {p.measurement_plan.covered}")
                if p.measurement_plan.uncovered:
                    lines.append(f"      UNCOVERED alternatives: "
                                 f"{p.measurement_plan.uncovered}")
        if self.verdicts:
            lines.append("  verdicts     : " + ", ".join(
                f"{k}={v}" for k, v in self.verdicts.items()))
        if self.gaps:
            g = self.gaps[0]
            lines.append(f"  next measure : {g.experiment}")
            lines.append(f"      resolves : {g.resolves}  "
                         f"({g.expected_info_bits:.2f} bits / {g.cost:.1f} cost)")
        if self.leverage:
            lines.append(f"  leverage Λ   : {self.leverage:.3g} "
                         f"(certified bits per control bit)")
        for d in self.diagnostics:
            lines.append(f"  ! {d}")
        return "\n".join(lines)


class Compiler:
    def __init__(self, backend, bundle: Optional[ModelBundle] = None,
                 grid: int = 21, verify_budget: float = 5000.0, seed: int = 7):
        self.backend = backend
        self.bundle = bundle or ModelBundle([])
        self.grid = grid
        self.verify_budget = verify_budget
        self.seed = seed

    # ---- stages ---------------------------------------------------------
    def _consistency_check(self, spec: Spec) -> List[str]:
        """The witness's declared sigma must match the backend's measurement model.

        A mismatch silently corrupts every sample-size computation, so it is a
        typed error rather than a warning.
        """
        diags: List[str] = []
        if not hasattr(self.backend, "_shot_sigma"):
            return diags
        for w in self.backend.witnesses():
            declared = w.sigma
            try:
                actual = self.backend._shot_sigma(w.observable)
            except Exception:
                continue
            if actual <= 0:
                continue
            ratio = declared / actual
            if not (0.5 <= ratio <= 2.0):
                diags.append(
                    f"INCONSISTENT-MEASUREMENT-MODEL: witness '{w.name}' declares "
                    f"sigma={declared:g} for '{w.observable}' but the backend's "
                    f"sampling model uses sigma={actual:g} (ratio {ratio:.3g}); "
                    f"sample-size computations for this witness are unsound")
        return diags

    def _boundary_check(self, plan: Plan) -> List[str]:
        diags: List[str] = []
        space = self.backend.param_space()
        hit = [k for k, v in plan.params.items()
               if abs(v - space[k][0]) < 1e-9 or abs(v - space[k][1]) < 1e-9]
        if hit:
            diags.append(
                f"BOUNDARY-LIMITED: optimum sits on the edge of the declared "
                f"parameter box in {hit}; the true optimum may lie outside Lambda_Omega "
                f"and the margin reported is a lower bound only")
        return diags

    def _typecheck(self, spec: Spec, pi: Provenance) -> List[str]:
        diags: List[str] = self._consistency_check(spec)
        for r in spec.requirements:
            if r.mode is Mode.DERIVE and r.model:
                ids = {m.model_id for m in self.bundle.models}
                if r.model not in ids:
                    diags.append(f"DERIVE requirement on {r.estimand.key()} names "
                                 f"unknown model '{r.model}'")
        if not spec.alternatives:
            diags.append("specification carries no alternative hypothesis set Alt(S); "
                         "witness minimality is undefined (monograph 9.15)")
        if spec.delta_total >= 1.0:
            diags.append(f"union bound over requirements gives delta_total="
                         f"{spec.delta_total:.2f} >= 1; specification is vacuous")
        pi.add(NodeKind.SPECIFICATION,
               {"name": spec.name, "n_requirements": len(spec.requirements),
                "delta_total": spec.delta_total, "alternatives": spec.alternatives},
               algorithm="typecheck", version="0.1",
               assumptions=["requirements independent (union bound)"],
               status="ok" if not diags else "warnings")
        return diags

    def _search(self, spec: Spec, pi: Provenance) -> Tuple[List[Plan], float, float]:
        space = self.backend.param_space()
        names = sorted(space)
        axes = [np.linspace(space[n][0], space[n][1], self.grid) for n in names]
        best: List[Plan] = []
        all_margins: List[float] = []
        obs_grid: Dict[str, List[float]] = {}
        for combo in itertools.product(*axes):
            params = dict(zip(names, map(float, combo)))
            res = self.backend.simulate(params)
            margins: Dict[str, float] = {}
            for r in spec.requirements:
                v = res.observables.get(r.estimand.name)
                if v is None:
                    margins[r.estimand.key()] = -float("inf")
                    continue
                margins[r.estimand.key()] = r.margin(v)
                obs_grid.setdefault(r.estimand.name, []).append(v)
            worst = min(margins.values())
            all_margins.append(worst)
            best.append(Plan(params, dict(res.observables), margins, worst))
        best.sort(key=lambda p: -p.worst_margin)
        # empirical Lipschitz constant of the worst-margin surface over the grid
        lip = 0.0
        for n, ax in zip(names, axes):
            step = float(ax[1] - ax[0]) if len(ax) > 1 else 1.0
            vals = [p.worst_margin for p in best]
            lip = max(lip, (max(vals) - min(vals)) / max(step, 1e-12) / len(names))
        resolution = min(float(ax[1] - ax[0]) for ax in axes if len(ax) > 1)
        pi.add(NodeKind.OPTIMIZATION,
               {"n_points": self.grid ** len(names), "best_margin": best[0].worst_margin,
                "resolution": resolution},
               algorithm="exhaustive-grid", version="0.1",
               assumptions=[f"grid resolution {resolution:g} in each of "
                            f"{len(names)} parameters"])
        return best, lip, resolution

    def _plan_measurements(self, spec: Spec, plan: Plan, pi: Provenance) -> None:
        margins = {}
        for r in spec.requirements:
            if r.mode is Mode.VERIFY:
                margins[r.estimand.name] = max(1e-9, plan.margins[r.estimand.key()])
        mp = select_witnesses(self.backend.witnesses(), spec.alternatives,
                              margins, spec.requirements[0].delta, self.verify_budget,
                              operating_point=plan.predicted)
        plan.measurement_plan = mp
        plan.verify_cost = mp.total_cost
        pi.add(NodeKind.MEASUREMENT,
               {"allocation": mp.allocation, "cost": mp.total_cost,
                "covered": mp.covered, "uncovered": mp.uncovered},
               algorithm="greedy-min-cost-cover", version="0.1",
               assumptions=["COVERAGE formulation: monotone submodular, ln|Alt| "
                            "approximation. NOT the mutual-information formulation, "
                            "which is not submodular in general."])

    def _execute_and_verify(self, spec: Spec, plan: Plan,
                            pi: Provenance) -> Dict[str, str]:
        verdicts: Dict[str, str] = {}
        pi.add(NodeKind.EXECUTION, {"params": plan.params},
               algorithm=self.backend.name, version="0.1")
        for i, r in enumerate(spec.requirements):
            if r.mode is not Mode.VERIFY:
                verdicts[r.estimand.key()] = f"{r.mode.value}(not measured)"
                continue
            shots = shots_for_margin(
                self.backend._shot_sigma(r.estimand.name)
                if hasattr(self.backend, "_shot_sigma") else 1.0,
                max(1e-9, plan.margins[r.estimand.key()]), r.delta)
            shots = min(shots, 200000)
            est, se = self.backend.measure(plan.params, r.estimand.name,
                                           shots, self.seed + i)
            v, z = decide(est, se, r, r.delta, spec.beta)
            verdicts[r.estimand.key()] = f"{v.value}(n={shots}, z={z:.1f})"
        pi.add(NodeKind.VERIFICATION,
               {"verdict": "PASS" if all(x.startswith("PASS") or "not measured" in x
                                         for x in verdicts.values()) else "MIXED",
                "per_requirement": verdicts},
               algorithm="three-valued-decision", version="0.1",
               assumptions=["normal approximation to the estimator's sampling "
                            "distribution", "requirements independent"])
        return verdicts

    def _leverage(self, spec: Spec, plan: Plan) -> float:
        """Operational leverage Lambda (monograph Definition 16.2), in bits/bits."""
        n_ctrl, b_ctrl = self.backend.control_channels()
        denom = n_ctrl * b_ctrl
        # I_verified: certified reduction in accessible state-space measure,
        # counted only over requirements actually measured.
        verified = [r for r in spec.requirements if r.mode is Mode.VERIFY]
        if not verified or denom <= 0:
            return 0.0
        frac = len(verified) / max(1, len(spec.requirements))
        return float(self.backend.reference_log_volume() * frac / denom)

    # ---- driver ---------------------------------------------------------
    def compile(self, spec: Spec) -> Result:
        pi = Provenance()
        diags = self._typecheck(spec, pi)

        cert = self.backend.static_obstruction(spec)
        if cert is not None:
            pi.add(NodeKind.OBSTRUCTION,
                   {"class": cert.cls.value, "statement": cert.statement},
                   algorithm="static-screen", version="0.1")
            return Result(Outcome.OBSTRUCT, spec.name, self.backend.name,
                          certificate=cert, coverage=spec.coverage(),
                          provenance=pi, diagnostics=diags)

        pi.add(NodeKind.MODEL,
               {"models": [m.model_id for m in self.bundle.models]},
               algorithm="model-selection", version="0.1")

        plans, lip, resolution = self._search(spec, pi)
        best = plans[0]

        if best.worst_margin < 0:
            cert = grid_exhaustion_certificate(
                self.grid ** len(self.backend.param_space()), resolution,
                best.worst_margin, lip)
            pi.add(NodeKind.OBSTRUCTION,
                   {"class": cert.cls.value, "statement": cert.statement},
                   algorithm="grid-exhaustion", version="0.1")
            return Result(Outcome.OBSTRUCT, spec.name, self.backend.name,
                          plans=plans[:3], certificate=cert,
                          coverage=spec.coverage(), provenance=pi,
                          diagnostics=diags)

        dis = analyse(self.bundle, best.params, spec.requirements)
        dominating = [d for d in dis if d.matters]
        if dominating:
            gaps = propose_experiments(dominating, {d.observable: 50.0 for d in dis})
            pi.add(NodeKind.ABSTRACTION,
                   {"disagreements": [(d.observable, d.spread, d.margin)
                                      for d in dominating]},
                   algorithm="model-discrimination", version="0.1",
                   status="model uncertainty dominates acceptance margin")
            return Result(Outcome.MEASURE, spec.name, self.backend.name,
                          plans=plans[:3], gaps=gaps, coverage=spec.coverage(),
                          provenance=pi,
                          diagnostics=diags + [
                              "refusing to compile: model spread exceeds the "
                              "acceptance margin on " +
                              ", ".join(d.observable for d in dominating)])

        for p in plans[:3]:
            self._plan_measurements(spec, p, pi)

        if best.measurement_plan and best.measurement_plan.uncovered:
            diags.append(
                "verification plan does not separate the target from "
                f"{best.measurement_plan.uncovered}; acceptance is UNVERIFIED "
                "against those alternatives")

        diags += self._boundary_check(best)
        verdicts = self._execute_and_verify(spec, best, pi)
        lam = self._leverage(spec, best)
        pi.add(NodeKind.COMPILATION,
               {"params": best.params, "leverage": lam},
               algorithm="psc-compiler", version="0.1")

        failed = [k for k, v in verdicts.items() if v.startswith("FAIL")]
        unver = [k for k, v in verdicts.items() if v.startswith("UNVERIFIED")]
        outcome = Outcome.COMPILE
        if failed or unver:
            outcome = Outcome.MEASURE
            diags.append(f"not all requirements certified: FAIL={failed}, "
                         f"UNVERIFIED={unver}")
        return Result(outcome, spec.name, self.backend.name, plans=plans[:3],
                      verdicts=verdicts, coverage=spec.coverage(), leverage=lam,
                      provenance=pi, diagnostics=diags)
