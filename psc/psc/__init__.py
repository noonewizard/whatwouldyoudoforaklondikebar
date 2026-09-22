"""Physical State Compilation -- prototype compiler.

Milestone 1: Specification -> Typed IR -> Feasibility -> Simulation -> Control
-> Measurement -> Verification -> Certificate, for one small quantum system,
with the same frontend semantics targeting a chemical and a geometric backend.
"""
__version__ = "0.1.0"

from .spec import Spec, Requirement, Estimand, Mode, Cmp, Statistic
from .units import Q, dimensionless
from .compiler import Compiler, Outcome, Result
from .models import Model, ModelBundle
from .provenance import Provenance

__all__ = ["Spec", "Requirement", "Estimand", "Mode", "Cmp", "Statistic",
           "Q", "dimensionless", "Compiler", "Outcome", "Result",
           "Model", "ModelBundle", "Provenance"]
