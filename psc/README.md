# psc — Physical State Compilation (prototype)

A working prototype of the Physical Synthesis Compiler described in
[`../reality-compiler/`](../reality-compiler/). **Milestone 1** from the research
directive:

```
Specification → Typed IR → Feasibility → Simulation → Control
              → Measurement → Verification → Certificate
```

for one small quantum system, with **the same frontend semantics** targeting a
chemical and a geometric backend.

## Run it

```bash
pip install numpy pytest
python3 examples/demo_three_backends.py       # one frontend, three substrates
python3 examples/benchmark_verifiability.py   # F-28 A/B benchmark
python3 -m pytest tests/ -q                   # 30 adversarial tests
```

## What it does

The compiler returns exactly one of **COMPILE**, **OBSTRUCT**, or **MEASURE**.
The three-valued outcome is not a design choice — it is forced by undecidability
plus certificate hardness (monograph Corollary 17.11).

- **COMPILE** — a route and a measurement plan, with false-accept bounded at δ
  *relative to the declared model bundle*.
- **OBSTRUCT** — an independently re-checkable certificate from one of eight
  classes (conservation, spectral, thermodynamic, controllability, resource,
  safety, metrological, substrate-expressivity), plus the weaker search-based
  grid-exhaustion class which states its own scope.
- **MEASURE** — model uncertainty dominates the acceptance margin; the compiler
  refuses to compile and emits the discriminating experiment instead.

## Architecture

| Module | Role |
|---|---|
| `psc/units.py` | dimensioned quantities; affine units are a distinct kind |
| `psc/spec.py` | L0 specification: estimands, acceptance regions, evidence modes, Alt(S) |
| `psc/ptypes.py` | physical type system; spectral and control-mode screens |
| `psc/ir.py` | L1 abstract model: operator terms with locality, units, uncertainty |
| `psc/models.py` | model bundle with mandatory validity domains |
| `psc/feasibility.py` | typed, re-checkable obstruction certificates |
| `psc/landscape.py` | *(not yet implemented — see Known gaps)* |
| `psc/verification.py` | witness selection (coverage formulation), sample complexity, three-valued decision |
| `psc/discriminate.py` | Model Discrimination Engine |
| `psc/provenance.py` | typed provenance DAG with tamper-evident digests |
| `psc/compiler.py` | the pipeline |
| `psc/backends/` | quantum spin (TFIM, exact diagonalisation), CRN, geometry |

## Known gaps

This is a prototype and its limits are load-bearing, not incidental:

1. **Search is exhaustive grid.** No continuous optimisation, no branch-and-bound.
   Consequently `OBSTRUCT` from search carries only the weak `GRID_EXHAUSTION`
   certificate, whose caveat field says exactly what it does and does not prove.
2. **No landscape/attractor synthesis module.** The compiler optimises parameters,
   not landscapes. Conditions A1–A6 and the figure of merit 𝔄 are specified in the
   monograph and not implemented.
3. **Λ (leverage) is a placeholder.** Its value is dominated by the backend's
   declared `reference_log_volume`, which is currently a modelling choice rather
   than a measurement. Do not read the numbers.
4. **No refinement/lowering stage.** IR levels L2–L4 are not implemented, so
   condition **Ref** is untested.
5. **No compositionality.** Single-step only; condition **Cmp** is untested.
6. **Verification uses the normal approximation** and ignores correlated errors
   between requirements. The union bound is applied without checking independence.
