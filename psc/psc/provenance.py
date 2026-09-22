"""Typed provenance graph.

Pi is not a log file. It is a DAG whose replay reproduces the acceptance
decision. Monograph section XXII; tested by F-30.
"""
from __future__ import annotations
import hashlib
import json
import time
from dataclasses import dataclass, field, asdict
from enum import Enum
from typing import Any, Dict, List, Optional


class NodeKind(str, Enum):
    SPECIFICATION = "specification"
    MODEL = "model"
    ABSTRACTION = "abstraction"
    OPTIMIZATION = "optimization"
    COMPILATION = "compilation"
    LOWERING = "lowering"
    EXECUTION = "execution"
    MEASUREMENT = "measurement"
    VERIFICATION = "verification"
    OBSTRUCTION = "obstruction"


def _digest(payload: Any) -> str:
    return hashlib.sha3_256(
        json.dumps(payload, sort_keys=True, default=str).encode()
    ).hexdigest()[:32]


@dataclass
class Node:
    node_id: str
    kind: NodeKind
    parents: List[str]
    payload: Dict[str, Any]
    assumptions: List[str] = field(default_factory=list)
    uncertainty: Dict[str, float] = field(default_factory=dict)
    algorithm: str = ""
    version: str = ""
    calibration: str = ""
    status: str = ""
    timestamp: float = field(default_factory=time.time)

    @property
    def digest(self) -> str:
        return _digest({
            "kind": self.kind.value, "parents": sorted(self.parents),
            "payload": self.payload, "assumptions": self.assumptions,
            "algorithm": self.algorithm, "version": self.version,
        })


@dataclass
class Provenance:
    nodes: Dict[str, Node] = field(default_factory=dict)
    order: List[str] = field(default_factory=list)

    def add(self, kind: NodeKind, payload: Dict[str, Any],
            parents: Optional[List[str]] = None, **kw) -> str:
        parents = parents or ([self.order[-1]] if self.order else [])
        nid = f"{kind.value}:{len(self.order)}"
        self.nodes[nid] = Node(nid, kind, parents, payload, **kw)
        self.order.append(nid)
        return nid

    def root_digest(self) -> str:
        """Merkle-style root over the DAG in topological (insertion) order."""
        h = ""
        for nid in self.order:
            h = _digest({"prev": h, "node": self.nodes[nid].digest})
        return h

    def assumptions(self) -> List[str]:
        out: List[str] = []
        for nid in self.order:
            for a in self.nodes[nid].assumptions:
                if a not in out:
                    out.append(a)
        return out

    def replay(self) -> Dict[str, Any]:
        """Recompute the acceptance decision from the record alone.

        Returns the verdict recorded by the verification node together with the
        recomputed digest, so an independent party can confirm both.
        """
        verdicts = [self.nodes[n] for n in self.order
                    if self.nodes[n].kind is NodeKind.VERIFICATION]
        return {
            "root_digest": self.root_digest(),
            "n_nodes": len(self.order),
            "verdict": verdicts[-1].payload.get("verdict") if verdicts else None,
            "assumptions": self.assumptions(),
        }

    def to_dict(self) -> Dict[str, Any]:
        return {"nodes": {k: asdict(v) for k, v in self.nodes.items()},
                "order": self.order, "root": self.root_digest()}
