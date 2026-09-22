"""Dimensioned quantities with static-ish dimensional checking.

SI base dimension vector: (m, kg, s, A, K, mol, cd).
Affine units (degC, gauge pressure) are a distinct kind and cannot be multiplied.
"""
from __future__ import annotations
from dataclasses import dataclass
from typing import Tuple

DIMS = ("m", "kg", "s", "A", "K", "mol", "cd")
Dim = Tuple[int, int, int, int, int, int, int]

DIMENSIONLESS: Dim = (0,) * 7
LENGTH: Dim = (1, 0, 0, 0, 0, 0, 0)
MASS: Dim = (0, 1, 0, 0, 0, 0, 0)
TIME: Dim = (0, 0, 1, 0, 0, 0, 0)
TEMPERATURE: Dim = (0, 0, 0, 0, 1, 0, 0)
AMOUNT: Dim = (0, 0, 0, 0, 0, 1, 0)
ENERGY: Dim = (2, 1, -2, 0, 0, 0, 0)
FORCE: Dim = (1, 1, -2, 0, 0, 0, 0)
PRESSURE: Dim = (-1, 1, -2, 0, 0, 0, 0)
FREQUENCY: Dim = (0, 0, -1, 0, 0, 0, 0)


class DimensionError(TypeError):
    """Raised when an operation violates dimensional consistency."""


def _fmt(d: Dim) -> str:
    if d == DIMENSIONLESS:
        return "1"
    parts = [f"{n}^{p}" for n, p in zip(DIMS, d) if p]
    return "·".join(parts)


@dataclass(frozen=True)
class Q:
    """A dimensioned scalar. `affine` marks units with a non-zero origin."""
    value: float
    dim: Dim = DIMENSIONLESS
    affine: bool = False

    def _check(self, other: "Q", op: str) -> None:
        if self.dim != other.dim:
            raise DimensionError(
                f"cannot {op} {_fmt(self.dim)} and {_fmt(other.dim)}"
            )

    def __add__(self, other: "Q") -> "Q":
        self._check(other, "add")
        return Q(self.value + other.value, self.dim, self.affine or other.affine)

    def __sub__(self, other: "Q") -> "Q":
        self._check(other, "subtract")
        return Q(self.value - other.value, self.dim)

    def __mul__(self, other) -> "Q":
        if isinstance(other, (int, float)):
            return Q(self.value * other, self.dim, self.affine)
        if self.affine or other.affine:
            raise DimensionError("affine quantities cannot be multiplied")
        return Q(self.value * other.value, tuple(a + b for a, b in zip(self.dim, other.dim)))

    __rmul__ = __mul__

    def __truediv__(self, other) -> "Q":
        if isinstance(other, (int, float)):
            return Q(self.value / other, self.dim, self.affine)
        if self.affine or other.affine:
            raise DimensionError("affine quantities cannot be divided")
        return Q(self.value / other.value, tuple(a - b for a, b in zip(self.dim, other.dim)))

    def __lt__(self, other: "Q") -> bool:
        self._check(other, "compare")
        return self.value < other.value

    def __le__(self, other: "Q") -> bool:
        self._check(other, "compare")
        return self.value <= other.value

    def __repr__(self) -> str:
        return f"{self.value:g} [{_fmt(self.dim)}]"


def dimensionless(x: float) -> Q:
    return Q(x, DIMENSIONLESS)
