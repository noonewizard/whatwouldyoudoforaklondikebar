"""Model bundle with mandatory validity domains and error bounds.

Every model carries the metadata required by monograph section XV. A model whose
validity predicate fails on a candidate plan is not used, and any DERIVE
requirement depending on it is demoted to UNSUPPORTED.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from typing import Any, Callable, Dict, List, Optional


@dataclass
class Model:
    model_id: str
    version: str
    predict: Callable[[Dict[str, float]], Dict[str, float]]
    validity_domain: Callable[[Dict[str, float]], bool]
    prediction_error: Dict[str, float]          # per-observable absolute error bound
    training_domain: str = ""
    parameter_uncertainty: Dict[str, float] = field(default_factory=dict)
    known_failure_modes: List[str] = field(default_factory=list)
    calibration_status: str = "uncalibrated"
    source: str = ""

    def in_domain(self, params: Dict[str, float]) -> bool:
        return bool(self.validity_domain(params))

    def error_for(self, observable: str) -> float:
        return self.prediction_error.get(observable, float("inf"))


@dataclass
class ModelBundle:
    models: List[Model] = field(default_factory=list)

    def valid_at(self, params: Dict[str, float]) -> List[Model]:
        return [m for m in self.models if m.in_domain(params)]

    def predictions(self, params: Dict[str, float]) -> Dict[str, Dict[str, float]]:
        return {m.model_id: m.predict(params) for m in self.valid_at(params)}

    def disagreement(self, params: Dict[str, float], observable: str) -> float:
        """Spread of predictions across valid models, in the observable's units."""
        vals = []
        for m in self.valid_at(params):
            p = m.predict(params)
            if observable in p:
                vals.append(p[observable])
        if len(vals) < 2:
            return 0.0
        return max(vals) - min(vals)
