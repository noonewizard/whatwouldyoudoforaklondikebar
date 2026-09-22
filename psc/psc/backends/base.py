"""Backend protocol. Every substrate implements exactly this surface."""
from __future__ import annotations
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, Protocol, Sequence, Tuple

from ..feasibility import Certificate
from ..ir import L1Model
from ..spec import Spec
from ..verification import Witness


@dataclass
class BackendResult:
    observables: Dict[str, float]
    uncertainty: Dict[str, float] = field(default_factory=dict)
    aux: Dict[str, Any] = field(default_factory=dict)


class Backend(Protocol):
    name: str

    def lower(self, spec: Spec) -> L1Model: ...
    def param_space(self) -> Dict[str, Tuple[float, float]]: ...
    def simulate(self, params: Dict[str, float]) -> BackendResult: ...
    def measure(self, params: Dict[str, float], observable: str,
                shots: int, seed: int) -> Tuple[float, float]: ...
    def witnesses(self) -> List[Witness]: ...
    def static_obstruction(self, spec: Spec) -> Optional[Certificate]: ...
    def control_channels(self) -> Tuple[int, float]: ...
    def reference_log_volume(self) -> float: ...
