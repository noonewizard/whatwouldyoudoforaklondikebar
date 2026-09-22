# Part I — Foundations of Physical State Compilation

Sections 1–4: Executive Thesis · Definition · Mathematical Foundations · Physical
State Spaces

> **Revision note (Draft 3).** This Part was rewritten to remove a bias present in
> Drafts 1–2: the framework was developed as a generalization of *existing industrial
> manufacturing compilers*, with quantum systems entering as a special-purpose branch.
> That ordering was an artifact of where compilers happen to exist today, not of where
> the physics is deepest or where the abstraction is best posed. The hierarchy
> classical ≫ quantum is removed. The object is **Physical State Compilation**, and
> the classical manufacturing case is one back end among several.

---

## 1. Executive Thesis

### 1.1 The proposition under test

> **Can computation compile specifications into physically engineered states —
> including quantum states, engineered interactions, and collective phases of matter —
> and can those states then be prepared, stabilized, manipulated, measured, and
> verified?**

This is a research hypothesis, not a manufacturing roadmap. It is not answered by
extrapolating current fabrication capability, and it is not answered by assuming that
quantum matter must remain a specialist corner of materials science. It is also not
answered by noting that no law forbids it. Both failure modes are live, and §1.6
states the discipline required to avoid each.

### 1.2 The object

```
                    PHYSICAL STATE COMPILATION

  macro ─── meso ─── micro ─── atomic ─── electronic ─── quantum ─── collective
    │         │        │         │            │             │            │
    └─────────┴────────┴─────────┴────────────┴─────────────┴────────────┘
                                   │
                    Specification → Physical Dynamics → Verified State
```

No level is privileged. In particular:

- **The hardest targets are not at the classical level.** They are wherever the
  specification's acceptance region is smallest relative to what the substrate's
  dynamics reach unaided.
- **The most valuable targets are not at the classical level either.** A specified
  topological invariant, a designed many-body phase, or a deterministically placed
  spin defect can carry more economic and scientific value per gram than any bulk
  material.
- **The best-posed targets are, on current evidence, quantum.** §29.3 scores quantum
  matter against this framework's own compilability criteria and it wins on three of
  four. That finding is the reason for this revision.

### 1.3 The four load-bearing claims

**(A) Physical synthesis is compilation, and it is compilation *at every level where
it has been attempted*.**
Electronic design automation compiles behavioral specifications to mask sets.
Retrosynthesis compiles molecules to reaction sequences. DNA-origami compilers
compile shapes to staple sets. Adjoint methods compile optical specifications to
nanophotonic geometries. And — the case Drafts 1–2 under-weighted — **programmable
quantum simulators compile Hamiltonian specifications to lattice depths, detunings,
tweezer coordinates and pulse sequences, and the resulting phases are then verified
by order parameters, transport quantization, and anyon interferometry.** The
abstraction is not an extrapolation from manufacturing; it is the common structure of
five independent fields that arrived at it separately. **ESTABLISHED.**

**(B) The abstraction's reach is set by four substrate properties, and quantum matter
has them unusually well.**
A substrate admits a compiler when it offers a composable algebra of *conditions*
with characterized effects, a forward model accurate relative to the tolerance with a
known validity domain, a metrology channel of polynomial sample complexity, and
stationary yield statistics. Structural alloys satisfy these poorly and were made to
satisfy them over forty years of deliberate substrate discipline. Programmable
quantum systems satisfy three of them *natively*, because the action algebra maps
directly onto Hamiltonian terms, the forward model is the Schrödinger–Lindblad
equation rather than a fitted constitutive law, and every preparation begins from a
known state with no process history. **STRONGLY SUPPORTED** (§29.3).

**(C) The primitive is controlled emergence, not instruction execution.**
Explicit placement costs O(N_dof). Landscape compilation costs O(K(H*)) — the
description length of the conditions that select the target attractor. For targets
with a short generating description the ratio is astronomical, which is why
crystallization, chemistry, self-assembly, and Hamiltonian engineering are the
technologies that reached scale, and why instruction execution appears only in the
lowering of *boundary conditions*. **PLAUSIBLE**, argued from scaling, containment,
and the industrial record (§33), with a quantitative criterion (§31, §2.6).

**(D) Compilability is bounded by compressibility, and compressibility is what makes
a state interesting.**
Almost all quantum states of N qubits, and almost all arrangements of 10²³ atoms, have
no short description, no efficient preparation, and no feasible certification. The
unrestricted proposition is therefore **false by counting**. But the states with
names — crystals, phases, folded proteins, topological sectors, gapped ground states —
are precisely the compressible ones, and for gapped local Hamiltonians there is a
structural reason: **the area law**. This is the deepest finding in the framework and
it is developed as the theorem program in §39.

### 1.4 What is rejected

| Claim | Verdict | Replacement |
|---|---|---|
| "Software creating matter" | **CONTRADICTED** (conservation) | Compilation of physically permitted transformations of a supplied substrate |
| Classical manufacturing is the primary domain and quantum a special case | **Bias, now removed** | Physical State Compilation across the full level hierarchy; quantum matter is a peer domain and in several respects the best posed |
| "Exact physical compilation is NP-hard" | **CONTRADICTED** — far too weak, and not one class | A complexity vector (§16.7); general reachability is undecidable, and the hardness results bound *algorithms*, not matter (§35.2) |
| `R = (x,p,c,f,q)` as a state | **Mathematically incorrect** | (ontic state σ, property map Φ indexed by protocol, belief 𝔟) — §4.1 |
| Target `R_B` as a point | **Category error** | Acceptance region [S]; and for quantum matter this is *forced*, not merely convenient — phases do not exist at finite size (§39.2) |
| Room-temperature macroscopic quantum coherence | Splits four ways, DEMONSTRATED to CONTRADICTED | §29.8 |
| "Cryptography protects matter" | **CONTRADICTED** | Cryptography binds records; physical binding needs an unclonability assumption (§12) |
| Quintillion-dollar valuation | **CONTRADICTED** (≈476× the capitalized output of civilization) | §22 |

### 1.5 The revised strategic thesis

> **Reality Compilation is a general framework for compiling formal physical
> specifications into executable, measurable, and verifiable state transformations
> across multiple levels of physical description.**
>
> Classical manufacturing is one back end. Chemistry is another. Nanofabrication,
> biological synthesis, and quantum matter are others. The architecture is defined by
> the abstraction
>
> ```
>     Specification  →  Physical Dynamics  →  Verified State
> ```
>
> and the research program is the determination of **how far that abstraction
> extends** — which substrate classes admit it, with what guarantees, and where it
> provably fails.

The commercial reading of Drafts 1–2 (a compiler layer over manufacturing, priced off
the EDA capture ratio) remains valid and remains the near-term business. It is no
longer the thesis. It is one corollary of the thesis, at one level of the hierarchy.

### 1.6 The two symmetric errors

The epistemic rule governing every claim in this monograph, stated as a pair because
only one of the two is usually guarded against:

> **Error 1 (false prohibition).** "No current experiment demonstrates X" becomes
> "X cannot exist." Drafts 1–2 committed this three times (§36, errata E1, E2, E4).
>
> **Error 2 (false licence).** "No law forbids X" becomes "X is therefore feasible."
> This is the error a correction pass naturally over-corrects into, and it is equally
> destructive: it produces research programs with no critical path.

Both are prevented by the same discipline: **every boundary claim must name the
category that imposes it** — fundamental law, mathematical limitation,
information-theoretic limitation, controllability, stability, measurement,
fabrication, materials, current instrumentation, current computation, or economics.
§35 performs this audit across the whole document. A claim of the form
"ALLOWED / INACCESSIBLE" is not a prediction of success; it is a statement that the
obstruction is in the last six categories and therefore has a cost curve rather than
a proof.

---

## 2. Definition: Physical State Compilation

### 2.1 The first principle, unchanged

Three constraints are in the *type* of the problem, not merely acknowledged:

1. **Conservation.** Baryon number, charge, energy, momentum. The compiler sorts and
   rearranges a supplied inventory; a specification demanding absent elements fails to
   type-check (§5.6).
2. **Second law.** Local entropy reduction is paid for by export. Quantified in §15;
   the cost is small, which favors ambition rather than limiting it.
3. **Locality and causality.** Controls act through fields and contacts at finite
   speed and finite spatial resolution. Control authority is a property of the
   substrate, not a free parameter.

And one addition specific to quantum targets, which functions as a conservation law
and is developed in §3.5:

4. **Spectral invariance under closed-system control.** Unitary control cannot change
   the eigenvalue spectrum of ρ. Purification and cooling are therefore *not*
   control problems; they are dissipation or measurement problems.

### 2.2 Three distinct targets

These are not interchangeable and the framework must type them separately.

**A — Material synthesis.**
```
feedstock  →  material / structure
```
Creating a substance or a geometric object. The acceptance region is over
composition, geometry, microstructure, and bulk properties. Throughput and yield
dominate. This is the domain of Parts II, V, VIII–IX.

**B — Quantum-state preparation.**
```
ρ₀  →  ρ_T        within an existing, already-fabricated substrate
```
The substrate exists; the task is to drive it to a specified state. The acceptance
region is over expectation values, fidelities, and entanglement measures. Coherence,
control fidelity, and preparation depth dominate. This is quantum computing's
state-preparation problem and most of quantum control theory.

**C — Quantum-matter synthesis.**
```
substrate + interactions + environment + control  →  quantum phase
```
Creating or engineering a *system whose collective behavior constitutes* a desired
phase or emergent state. The acceptance region is over order parameters, invariants,
response functions, and excitation spectra. **The engineered object is the
Hamiltonian, not the state.**

> **C is the deepest form of physical compilation available, and it is the form for
> which the framework's central abstraction is most exactly true.** In A the compiler
> emits instructions and the substrate follows them. In B the compiler emits a
> trajectory through a fixed state space. In C the compiler emits *the law the system
> will obey*, and the system's own dynamics produce the target. That is
> Specification → Physical Dynamics → Verified State with nothing left over.

Note that C subsumes B operationally (preparing a state within an engineered
Hamiltonian) and that A is required to build the apparatus for both. The three are
layered, not competing.

### 2.3 Quantum Physical Synthesis

> **Definition 2.1 (Quantum Physical Synthesis).** The compilation of a declarative
> specification into a physically realizable preparation and control protocol for a
> quantum state, quantum phase, or engineered many-body system.

### 2.4 The generalized compilation object

```
        ( 𝔟₀ , S , Ω )   ──Compile──▶   ( P* , H* , L* , Ψ , Π )
```

| Output | Meaning | Primary for |
|---|---|---|
| **P\*** | preparation / control protocol: the time-ordered program of conditions | A, B, C |
| **H\*** | the engineered Hamiltonian or **effective** Hamiltonian H_eff(λ) | **C** |
| **L\*** | engineered dissipative and environmental channels {L_k(λ)} | **C**, and B where dissipative preparation is used |
| **Ψ** | measurement and verification protocol with a three-valued decision rule | all |
| **Π** | provenance and attestation record, exposing which models were used and with what validity witnesses | all |

**H\* is not merely discovered; it is designed.** The compiler's task in target class C
is the *inverse problem*: given a specification, determine the physical controls
λ ∈ Λ_Ω whose effective Hamiltonian H_eff(λ) has ground states, excited states,
steady states, or dynamical trajectories that satisfy the specification. The
realizable parameter space Λ_Ω is typically **low-dimensional** — an optical lattice
has of order ten independent knobs, a moiré stack three or four — which is the
structural reason this inverse problem is tractable where the classical composition ×
process search is not (§31.3).

### 2.5 Landscape Compilation

Promoted here from a technique to a **core architectural object**.

> **Definition 2.2 (Landscape Compilation).** The problem of determining physical
> parameters and control fields λ such that a specified acceptance region becomes
> *dynamically accessible, stable, attractive, or otherwise reliably preparable* under
> the system's own dynamics.

Formally: rather than specifying trajectories for {x₁,…,x_N}, find a landscape
𝓛(σ; λ) — an energy surface, free-energy surface, effective Hamiltonian, Liouvillian,
or Floquet quasi-energy structure — such that the target satisfies the dynamical
conditions

```
(a) ATTRACTION    the acceptance region [S] contains an attractor of the dynamics
                  generated by 𝓛(·;λ) — a gapped ground state, a deep free-energy
                  basin, a unique Liouvillian steady state, or a stable limit cycle
(b) ROBUSTNESS    the relevant gap (spectral Δ, Liouvillian Δ_L, or barrier height)
                  exceeds the noise scale by the required margin
(c) KINETICS      τ_relax(λ) ≤ T_max  — the system actually arrives
(d) SELECTIVITY   no competing attractor has comparable basin measure
(e) REALIZABILITY λ ∈ Λ_Ω
```

The landscape classes the compiler searches over:

| Class | Target type | Mechanism |
|---|---|---|
| Energy landscapes | A, C | ground states, metastable basins |
| Free-energy landscapes | A | self-assembly, crystallization, phase selection |
| Effective Hamiltonians | C | Schrieffer–Wolff, perturbative gadgets, Floquet averaging |
| Dissipative landscapes (Liouvillian) | B, C | engineered reservoirs; the target as unique steady state |
| Floquet-engineered landscapes | C | periodic driving; artificial gauge fields; topological bands |
| Quasiparticle landscapes | C | band structure, dispersions, excitation spectra |
| Topological sectors | C | superselection sectors; degeneracy on non-trivial topology |
| Metastable basins | A, C | kinetically trapped phases — most engineering materials live here |
| Symmetry-breaking manifolds | C | order-parameter spaces and their topology (defects) |

**The compiler should preferentially exploit natural dynamics rather than micromanage
them**, and §32 gives the criterion for when this pays and by how much: the advantage
class is determined by the relaxation time alone, and ranges from *qualitative*
(trajectory control does not exist as an option) through *exponential* (gapped
many-body preparation) and *polynomial* (adaptive constant-depth preparation) to
*none* (glassy landscapes).

### 2.6 Physical computational leverage

The stronger question — *can the system's own dynamics perform most of the
synthesis?* — needs a measure. Define

> **Definition 2.3 (Physical computational leverage).**
> ```
>              I_verified(S)
>   ℒ(S,Ω) = ──────────────────────
>             K(P*) + C_ver(Ψ)
> ```
> where **I_verified(S)** is the information content of the *certified* claim —
> log₂ of the reduction in accessible state-space measure that the verification
> actually establishes — **K(P\*)** is the description length of the control program,
> and **C_ver** is the verification cost.

Read off its behavior:

- **Crystal growth.** ~10 control parameters certify, via one diffraction pattern, a
  universally quantified claim about 10²³ atomic positions. ℒ ~ 10²².
- **Compiled quantum phase.** A handful of λ parameters plus an interferometric
  measurement certify a topological invariant over the whole system. ℒ enormous, and
  *further* enhanced because the estimand is an integer (§29.7, Prop. 29.1).
- **CNC machining of a bespoke part.** Control program length ≈ feature count ≈
  certified claim size. **ℒ ≈ 1.** No leverage — correctly, because there is nothing
  to compress.

> **ℒ ≫ 1 is exactly the condition under which physical dynamics are doing the
> computational work.** It is the operational form of the compression ratio κ (§31),
> with the verification term included — and including it matters, because leverage
> that cannot be certified is not leverage.

Maximizing ℒ is a different objective from minimizing cost, and it is the right
objective for a *research* program: it selects for substrates and targets where the
physics carries the load. §42 develops both objectives and their relationship.

### 2.7 Formal definition

Let a **substrate declaration** be

```
Ω = ⟨ Σ, 𝒜, Λ_Ω, 𝒯, ℳ, 𝒞, ℰ, 𝔐 ⟩
```

- **Σ** — ontic state space at the declared level ℓ.
- **𝒜** — the **algebra of conditions**: primitive physically available controls —
  impose this boundary, set this field, hold this drive, couple to this reservoir,
  place this tweezer — with declared preconditions, durations, and resource draws.
  Explicit placement is the degenerate element in which one condition affects one
  degree of freedom.
- **Λ_Ω** — the realizable control-parameter space: {λ : H(λ), L_k(λ) physically
  available}. The search space for target class C.
- **𝒯** — admissible control signals with bandwidth, amplitude, and slew limits.
- **ℳ** — measurement algebra with estimators, precision, bias, cost, destructiveness.
- **𝒞** — inventory and budgets. **ℰ** — environment envelope, treated as a *costed
  free variable*, not a constraint (§34). **𝔐** — model bundle with validity domains
  and error bounds.

Let a **specification** be `S = ⟨𝒪, 𝒦, δ, β, ≺⟩`: a finite vector of estimands (each a
property functional paired with a measurement protocol), a measurable acceptance
region, false-accept and false-reject bounds, and a preference order.

> **Definition 2.4 (Physical State Compilation).**
> Given (𝔟₀, S, Ω), produce (P*, H*, L*, Ψ, Π) such that P* is executable over 𝒜 and
> feasible under 𝒯, 𝒞, ℰ; H* and L* lie in Λ_Ω; Ψ is a measurement plan with a
> three-valued acceptance rule A; Π binds all of the above with model-validity
> witnesses; and for every model m ∈ 𝔐,
>
> ```
>   Pr_m[ A(Y) = PASS ∧ Φ(σ_T) ∉ 𝒦 ] ≤ δ ,
>   Pr_m[ A(Y) = FAIL ∧ Φ(σ_T) ∈ 𝒦 ] ≤ β .
> ```
>
> The compiler's contract is over the joint behavior of **process and test**, never
> over the state.

A compiler is **sound** if every emitted tuple satisfies this under the declared 𝔐;
**complete relative to Ω and horizon L** if it emits a valid tuple or an infeasibility
certificate whenever one of length ≤ L exists; and **honest** if it emits UNKNOWN
otherwise. Soundness is relative to 𝔐, and Π must expose that relativity.

### 2.8 On the name

For publication: **Physical State Compilation** as the field, **Physical Synthesis
Compiler** as the artifact, **Quantum Physical Synthesis** for target class C.
"Reality Compiler" is a program name. The technical vocabulary should not assert that
the object of compilation is reality; it is a declared substrate under declared
conditions, and the honesty of that restriction is what makes the claims defensible.

---

## 3. Mathematical Foundations

### 3.1 The classical problem family

**RC-0 — Feasibility.** ∃P with Pr[Φ(σ_T) ∈ 𝒦] ≥ 1−δ? Undecidable in general for
hybrid dynamics; the useful primitive is bounded:

**RC-0′ — Bounded feasibility.** …with |P| ≤ L, T ≤ T_max, finite 𝒜, declared model
class. Decidable; NP-hard to PSPACE-hard. **This restriction is what makes the theory
exist.**

**RC-1 — Reachability.** Characterize Reach(𝔟₀, Ω, T) ⊆ Δ(Σ) — on *beliefs*, because
the controller never knows σ.

**RC-2 — Optimization.** Distributionally robust: argmin_P sup_{m∈𝔐} 𝔼_m[𝔍].

**RC-3 — Lowering.** A refinement relation; each step carries a proof or a measured
calibration witness.

**RC-4 — Verification.** A three-valued statistical decision procedure with a declared
operating characteristic.

### 3.2 The quantum problem family

The quantum analogues are **not** the classical problems with different notation.
They have different invariants, different reachable-set structure, and different
verification theory. They are stated as a parallel family.

---

**QR-0 — Quantum Feasibility.**
> Given (ρ₀, S, Ω): does there exist a physically permitted control program —
> λ(t) ∈ Λ_Ω, admissible dissipative couplings, admissible measurements — such that
> ρ(T) ∈ [S] within confidence 1−δ and resource bounds 𝒞?

*Complexity.* Contains ground-state preparation for local Hamiltonians as a special
case, hence **QMA-hard** in general. Decidable for bounded Λ_Ω and bounded T.
If the quantum PCP conjecture holds, even *constant-relative-error* approximation
remains QMA-hard — which would mean approximate feasibility is no easier than exact
in the worst case. **Open, and the sharpest single open question bounding this
problem** (§39.5).

---

**QR-1 — Quantum Reachability.**
> Characterize Reach(ρ₀, Ω, T) under each control mode separately: Hamiltonian
> control, dissipative control, measurement, environmental coupling, and their
> combinations, subject to resource restrictions.

The structure here is sharp and is the quantum analogue of a conservation law
(§3.5): the three modes have *qualitatively different* reachable sets, and knowing
which mode is required is the first thing the compiler must determine.

---

**QR-2 — Quantum Landscape Synthesis.**
> Find H*, L*, u* such that the specified state or phase becomes reliably accessible —
> i.e. satisfies conditions (a)–(e) of Definition 2.2.

This is the inverse Hamiltonian design problem. Positive structural results exist
(parent-Hamiltonian constructions for tensor-network states; universal Hamiltonian
families) and are the backbone of §39.3.

---

**QR-3 — Robust Quantum Preparation.**
> Optimize preparation under uncertainty in Hamiltonian parameters, control
> calibration, temperature, decoherence rates, defects, disorder, and environmental
> coupling.

```
min_{λ(·)}  sup_{θ ∈ Θ}  𝔼_θ [ 𝔍 ]    s.t.  Pr_θ[ ρ_T ∈ [S] ] ≥ 1−δ  ∀θ ∈ Θ
```

This is where most experimental effort actually goes and where most compiled
protocols fail. Robustness, not peak fidelity, is the correct objective: a protocol
achieving 0.999 at nominal parameters and 0.6 at the calibration drift observed over
an hour is worse than one achieving 0.98 everywhere.

---

**QR-4 — Quantum Verification.**
> Construct statistical tests certifying the specified state or phase **without
> assuming full state tomography is practical**, and find the *minimum sufficient
> measurement set*.

Formally (§41):
```
find  W ⊆ ℳ  minimizing  Σ_{w∈W} cost(w)
s.t.  the values of W separate [S] from the relevant alternative hypotheses 𝒜
      with margin m, at confidence 1−δ, using n(W) copies
```
A test-cover problem: NP-hard, submodular, greedily (1−1/e)-approximable.

---

### 3.3 The compiler optimizes four properties jointly, not one

The standard objective in quantum control is preparation fidelity. That is the wrong
objective for a compiler, and stating the right one is one of this framework's
substantive contributions:

```
              preparability  +  stability  +  measurability  +  verifiability
```

- **Preparability** — can the protocol reach it, in available time, at available
  fidelity?
- **Stability** — does it stay? (Gap, Liouvillian gap, barrier height, lifetime
  against the actual noise spectrum.)
- **Measurability** — is there *any* accessible observable sensitive to the
  specification?
- **Verifiability** — can that observable certify the acceptance region within a
  feasible number of copies?

A target scoring perfectly on the first and poorly on the fourth is not compilable in
any useful sense, because nothing distinguishes success from failure. **This is the
"compile for verifiability" thesis extended into quantum matter, and quantum matter
is where it bites hardest** — because the gap between "prepared" and "known to be
prepared" is larger there than anywhere else in physics.

### 3.4 Permitted transformations: the layered permission stack

"Physically permitted" is not a single predicate.

```
CONSERVATION      hard, exact, checkable symbolically
 ⊃ SPECTRAL       for closed quantum control: the spectrum of ρ is invariant (§3.5)
   ⊃ THERMODYNAMIC  thermomajorization / free energy — exact for the idealized model
     ⊃ KINETIC      barrier heights, gaps, relaxation times
       ⊃ CONTROL    what 𝒜 and Λ_Ω actually reach
         ⊃ ECONOMIC what 𝒞 can pay for
```

For nearly every real target — classical or quantum — **the binding layer is KINETIC
or CONTROL.** Diamond is thermodynamically forbidden at room conditions and
manufactured at scale because the graphitization barrier is ~730 kJ/mol. A Hubbard
antiferromagnet is thermodynamically permitted and blocked by entropy removal
kinetics. A framework that leads with thermodynamics optimizes the wrong layer.
**STRONGLY SUPPORTED.**

### 3.5 The quantum conservation law

> **Proposition 3.1 (Spectral invariance).** Under closed-system control — any
> H(λ(t)), however elaborate — the evolution of ρ is unitary, hence the eigenvalue
> spectrum of ρ is invariant. No coherent control protocol can change the purity,
> the von Neumann entropy, or any spectral function of the state.

Consequences, which are load-bearing for the compiler:

1. **ρ₀ ↛ ρ_T by unitary control unless spec(ρ₀) = spec(ρ_T).** A feasibility check as
   cheap and as absolute as the classical element balance, and it should run in the
   same compile-time screen (§6.4).
2. **Cooling, purification, and entropy removal are not control problems.** They
   require dissipation (coupling to a colder or engineered reservoir) or measurement
   (which changes the spectrum conditionally, at the cost of a classical record and
   its erasure). The compiler must *type* the target: does reaching it require an
   entropy-changing operation? If so, unitary control alone cannot be the answer
   regardless of how good the pulses are.
3. **This explains the empirical pattern** that entropy — not laser power, not
   control fidelity — is the binding constraint in cold-atom many-body preparation
   (§29.9 Target A). It is not an engineering accident. It is Proposition 3.1.
4. **Under open-system control** the reachable set is much larger: with a suitable
   engineered reservoir, any state can in principle be prepared as a steady state,
   which is why dissipative engineering is the strategically important mode (§8.4,
   §29.5).

**ESTABLISHED** (elementary), and, to this author's knowledge, not usually stated as
a compile-time feasibility screen — which is where it belongs.

### 3.6 The composition requirement

> **Desideratum 3.2 (Compositionality).** If P₁ certifies membership in 𝒦₁ at
> confidence 1−δ₁, and P₂ certifies 𝒦₂ given any input in 𝒦₁ at 1−δ₂, then P₂∘P₁
> certifies 𝒦₂ at ≥ 1 − (δ₁+δ₂+ε) for a bounded interaction term ε.

ε is bounded when 𝒦₁ is a *sufficient statistic* for the downstream process and the
downstream process is *contractive* on it. Digital electronics composes because
voltage-with-noise-margins is both. Chemistry and metallurgy largely lack such a
variable; **quantum error correction manufactures one deliberately** — the logical
subspace, with the code distance as the margin, restored every cycle by syndrome
extraction.

That observation deserves emphasis because it closes a loop:

> **Quantum error correction is the only known constructed instance of the digital
> abstraction outside classical electronics.** It is a *restoring, sufficient,
> contractive* variable engineered into a physical substrate on purpose. Whether an
> analogous construction exists for chemical or structural substrates is Open
> Problem 1 (§27), and QEC is the existence proof that the construction is possible at
> all.

---

## 4. Physical State Spaces

### 4.1 Why `R = (x, p, c, f, q)` fails

The proposed tuple mixes four kinds of object. `x, p` are ontic. `c` (composition) is
determined by the microstate and is redundant — including it permits inconsistent
states. `f` (functional properties) are **functionals of the state and of a test
protocol**: yield strength is not a property of an atomic configuration but of a
configuration plus a loading protocol, strain rate, temperature, and specimen
geometry. `q` (epistemic/provenance) belongs to the observer; a sample does not carry
its certificate, a *record* does.

**Replacement — three typed objects:**

- **Ontic state** σ ∈ Σ_ℓ at declared level ℓ. Nothing else.
- **Property map** Φ_ℓ : Σ_ℓ × 𝒫 → ℝᵏ, where 𝒫 is the protocol. Every specified
  property is a pair (estimand, protocol); writing a property without one is a type
  error (§5.1 Rule 1).
- **Epistemic state** 𝔟 ∈ Δ(Σ_ℓ). The compiler plans in belief space. Π attaches to
  records, never to σ.

For quantum targets the same typing applies with σ → ρ (or a tensor-network
description), Φ → expectation values, invariants, and response functions, and 𝔟 → a
distribution over Hamiltonian parameters and calibration state. **The type discipline
is level-independent, which is the first concrete evidence that a level-independent
front end is achievable** (§40).

### 4.2 The level hierarchy, without a privileged level

| Level | State object | Span | Upward map | What is specified here |
|---|---|---|---|---|
| **Collective / phase** | order parameters, invariants, response functions, excitation spectra | thermodynamic limit (approached by finite-size scaling) | — | topological invariants, Tc, σ_xy, spin-liquid character |
| **Quantum** | ρ; MPS/PEPS/tensor network; second-quantized wavefunction | 10⁰–10² exact; 10²–10⁴ with tensor networks under entanglement-structure assumptions | coarse-grain to effective model | fidelity, entanglement, coherence, gate error |
| **Electronic** | band structure, correlation functions, density | 10⁰–10³ atoms | parameterize force fields | gaps, masses, topology of bands |
| **Atomic** | nuclear positions/momenta; interatomic potential | 10³–10⁹ atoms, ns–µs | coarse-grain | composition, defects, interfaces, dopant sites |
| **Micro / meso** | phase fields, dislocation densities, grain structure, CRN markings | µm–mm | homogenize | microstructure, texture, porosity |
| **Macro** | continuum fields, constitutive laws | mm–m | — | geometry, bulk properties |

**Two structural facts about the hierarchy, and they are the reason compilation is
hard at every level equally:**

1. **The stack is deterministic upward and degenerate downward.** Nuclear positions
   determine electronic structure (Born–Oppenheimer); electronic structure does not
   determine nuclear positions. A macroscopic specification does not determine a
   microstructure. **Every layer boundary is therefore a one-to-many inverse problem,
   and the compiler's output is always a set** (§30).
2. **Independent control exists only at the nonequilibrium handles.** Temperature,
   driving, preparation history, and dissipation do not follow from the atomic
   configuration. They are where the extra design freedom lives, and consequently
   **joint compilation across layers is possible precisely where a nonequilibrium
   handle spans a boundary** — twist angle spans geometry and electronic structure;
   strain spans atomic and electronic; drive frequency spans electronic and
   collective; dissipation spans quantum and collective. These are exactly the
   parameters that produced the last decade's surprises, and that is not a
   coincidence — it is a search heuristic (§30).

### 4.3 Coarse-graining is lossy, and the loss is worst where it matters

Upward maps are many-to-one and information-destroying; this is not a solvable
engineering problem, and it is why inverse design is underdetermined. Worse, the
errors are **not uniformly bounded**: machine-learned potentials are accurate in
distribution and silently wrong outside it; CALPHAD interpolates well and extrapolates
badly; continuum constitutive models fail at free surfaces, cracks, and phase
boundaries — precisely where interesting behavior lives. Tensor-network
representations are faithful only under their declared entanglement-structure
assumption.

**Architectural response, required at every level:** every model in 𝔐 carries a
validity-domain predicate and an error bound; the compiler refuses to use a model
outside its declared domain; and the plan includes measurements chosen to
discriminate between models whenever their predictions diverge inside the tolerance
band (§6.9). **This is the single requirement that makes `DERIVE` mode meaningful, and
F-1 tests whether it can actually be met.**

### 4.4 Targets are regions — and for quantum matter this is forced

Three arguments establish it generally:

1. **Measure-theoretic.** Every single state has measure zero in a continuous Σ. A
   specification naming a point is unsatisfiable with probability 1.
2. **Metrological.** No measurement returns a point. The three-valued decision the
   framework requires is only definable against a region.
3. **Engineering.** No specification ever written names a state. GD&T, ASTM material
   standards, ICH pharmaceutical specifications, and semiconductor process windows are
   all acceptance regions over functionals.

**A fourth argument applies specifically to quantum matter and is stronger than the
other three:**

4. **Phases do not exist at finite size.** A quantum phase is an equivalence class
   under finite-depth local unitary circuits (or adiabatic paths without gap closure)
   — a statement about *connectivity in state space in the thermodynamic limit*, not
   about the values of observables on any finite system. No finite measurement on any
   finite system can certify phase membership. What can be certified is that the
   observables take values consistent with the phase and that the finite-size scaling
   is consistent with it.

So the acceptance-region reformulation is not a modeling convenience for quantum
matter. **It is the only formulation available**, and the framework's general
epistemic structure — certify a region at a confidence, never an identity — turns out
to be exactly what condensed-matter physics already does when it claims to have
observed a phase. That convergence is evidence the formalism is right.

The spec cell is
```
[S] := Φ(·, 𝒫)⁻¹(𝒦) ⊆ Σ
```
and its degeneracy is a **resource**: it is the slack the optimizer spends on
manufacturability, robustness, and verifiability. Its measure relative to Σ is
conjectured to be the best single predictor of compilation difficulty (Open Problem 2;
tested by F-14 through the related quantity κ).

### 4.5 Representations the implementation needs

Not a survey — what the IR actually requires, now spanning all levels:

- **Molecular graphs** with stereochemistry; **crystal structures** in canonical form
  (space group + Wyckoff + occupancy) for deduplication and provenance hashing.
- **Reaction networks as vector addition systems** — importing a complete decidability
  theory (§16.3).
- **Phase fields / microstructure tensors** with explicit RVE semantics.
- **Geometry as boundary representation plus a tolerance schema**; meshes are a
  lowering artifact.
- **Second-quantized Hamiltonians** in a canonical algebraic form: mode type
  (spin-S / fermionic / bosonic), interaction graph, term coefficients, symmetries,
  and dissipators. **This is the quantum IR's Level 1** (§40).
- **Tensor networks (MPS/PEPS)** with declared bond dimension *and* declared
  entanglement-structure assumption.
- **Stabilizer / topological data**: stabilizer generators, anyon models, fusion rules.
- **Belief representations**: posteriors over property maps and over model parameters.
  The compiler's native currency is a posterior, not a value.

**The IR must be lossy in a declared, auditable way.** Every abstraction step emits
into Π the statement "this step assumed X and discarded Y." That is what makes the
resulting attestation meaningful, it is standard practice in EDA, and it is absent
almost everywhere else.
