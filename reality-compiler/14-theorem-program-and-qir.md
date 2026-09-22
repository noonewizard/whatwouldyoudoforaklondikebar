# Part XIV — The Compilability Theorem Program and the Quantum Physical IR

Sections 39–43

---

## 39. Is Quantum Matter Compilable? A Theorem Program

### 39.1 The proposition

> **Proposition Q (to be proved, disproved, or restricted).**
> For a sufficiently characterized physical substrate, a desired class of quantum
> many-body states can be represented as an acceptance region and compiled into
> experimentally executable Hamiltonian, dissipative, geometric, and environmental
> controls, with bounded preparation and verification error.

The answer is not assumed. It is derived below, and it is: **false as stated, true on
a characterizable class, and the class has a structural reason for existing.**

Proposition Q is a conjunction of four independent claims, each of which can fail
alone:

| | Claim | Name |
|---|---|---|
| **Q1** | The target class is representable as an acceptance region over finitely many measurable functionals | *Representability* |
| **Q2** | There exists λ ∈ Λ_Ω whose H(λ), L(λ) has the target among its ground/steady/dynamical states | *Realizability* |
| **Q3** | A protocol reaches it in bounded time with bounded error | *Preparability* |
| **Q4** | Bounded measurement certifies membership at the required confidence | *Verifiability* |

### 39.2 Q1 — Representability: **fails strictly, holds operationally**

A gapped quantum phase is an equivalence class of states under finite-depth local
unitary circuits (equivalently, under adiabatic paths that do not close the gap).
This is a statement about **connectivity in state space in the thermodynamic limit**.

Two consequences:

1. **Phase membership is not a function of expectation values at finite size.**
   No finite set of measurements on a finite system certifies it. This is not an
   instrumentation limit; it is what the definition says.
2. **Therefore Q1 is false as literally stated** — a phase is not an acceptance region
   over finitely many functionals of a finite system.

But the operational version holds, and it is what physics actually does:

> **Q1′ (operational representability).** A phase-target is representable as a
> *nested family* of acceptance regions indexed by system size, together with a
> finite-size-scaling hypothesis. The certified claim is "the observables take values
> in 𝒦(N) for N ∈ {N₁,…,N_j}, and the scaling is consistent with phase X against
> stated alternatives," never "this system is in phase X."

**This is the single most important structural finding of the theorem program**, and
it validates the framework's central reformulation from an unexpected direction: the
acceptance-region formalism is not a convenience adopted to make verification
tractable. For quantum matter **it is the only available formulation**, and condensed
matter physics has been implicitly using it for fifty years. Every experimental claim
of a new phase is exactly this: a region, a set of sizes, a scaling collapse, and a
list of excluded alternatives.

Corollary for the DSL: a phase specification must carry its **alternative hypothesis
set** 𝒜. "This is a spin liquid" is not a specification. "These observables lie in
these regions, with this scaling, against these named alternatives (valence-bond
solid, spin glass, magnetically ordered, disorder-dominated)" is. The compiler should
refuse the former.

**Classes where Q1′ holds cleanly:** symmetry-broken phases (local order parameters);
topological phases (Wilson loops, topological entanglement entropy, braiding phases,
quantized responses — all finite-size computable with known scaling); free-fermion and
Gaussian states (full covariance-matrix description).
**Classes where it is hard:** quantum-critical regimes (exponents require several
sizes and careful extrapolation); phases distinguished only by long-range
entanglement structure with no quasi-local witness at accessible sizes.

### 39.3 Q2 — Realizability: **holds constructively on a characterized class**

Three positive results, of increasing generality:

**(i) Parent-Hamiltonian construction.** For matrix-product states and, under
conditions, projected-entangled-pair states, there is a *constructive* procedure
producing a local, frustration-free Hamiltonian having that state as its (often
unique) ground state. So for any target admitting an efficient tensor-network
description, **Q2 is answered by construction, not by search.** **ESTABLISHED.**

**(ii) Universal Hamiltonian families.** Certain simple spin-lattice models are
universal simulators: they reproduce the full physics — spectrum, partition function,
dynamics — of any other spin Hamiltonian with polynomial overhead. So for the spin
class, **Q2 holds for *any* target up to encoding.** **ESTABLISHED.** The honest
caveats: the overhead's constants can be severe, and it is simulation within an
encoded subspace rather than native realization.

**(iii) Where Q2 fails.** A generic state of N qubits has **no local parent
Hamiltonian**. The counting argument is decisive: the space of local Hamiltonians on
N sites has poly(N) parameters; the space of states has dimension 2^N. The set of
states arising as ground states of local Hamiltonians is therefore measure-zero in
Hilbert space.

> **Q2 holds exactly on the states that *are* ground/steady states of realizable local
> Hamiltonians — which is a measure-zero but physically exhaustive set.**

### 39.4 Q3 — Preparability: **holds on three identified sub-classes, fails in the worst case**

**Obstructions (all theorem-grade, all cited in §29.5):** Lieb–Robinson depth floor
ξ/v for local unitary preparation; gap closure and Kibble–Zurek defect scaling on
adiabatic paths through criticality; QMA-hardness of ground-state energy estimation,
which implies preparing ground states of *arbitrary* local Hamiltonians is at least as
hard.

**Sub-classes where Q3 holds:**

| Sub-class | Mechanism | Depth / time |
|---|---|---|
| States with a gapped adiabatic path from an efficiently preparable state | adiabatic ramp | O(1/Δ²) — poly when the gap is poly |
| Long-range-entangled states in the measurement-preparable class | constant-depth adaptive circuit with feedforward | **O(1)** — beats the Lieb–Robinson floor because classical communication is not velocity-limited |
| Unique steady states of rapidly mixing engineered Liouvillians | dissipative preparation | O(1/Δ_L) — and *self-correcting*, since the target is an attractor |
| Gaussian / free-fermion states | quadratic circuits | poly |
| Stabilizer and topological states | Clifford circuits, or adaptive constant depth | poly / O(1) |

**Where Q3 fails:** arbitrary local-Hamiltonian ground states (QMA-hard); states
requiring exponential-depth circuits; glassy landscapes where τ_relax is exponential
(§32.3, bottom row).

### 39.5 The sharpest open question bounding Q3

**The quantum PCP conjecture.** If it holds, estimating the ground energy of a local
Hamiltonian to *constant relative error* remains QMA-hard. The implication for this
framework is direct and severe: **approximate compilation would be no easier than
exact compilation in the worst case**, and the usual escape — "we only need
tolerance" — would not apply to worst-case instances.

It does not follow that the framework fails, because worst-case instances are not the
instances anyone wants (§39.7). But it does mean that **no general approximation
guarantee for quantum-matter compilation can be expected**, and that all guarantees
must be class-relative. **OPEN**, and it should be tracked as the single external
result most likely to change this program's theoretical foundations.

### 39.6 Q4 — Verifiability: **holds on structured targets, with a strong positive result**

Full state tomography costs Θ(d²/ε²) copies — exponential in qubit number, and
irrelevant, since nobody specifies a state. The certification tools that matter:

- **Classical shadows**: O(log m / ε²) copies for m local observables, independent of
  Hilbert-space dimension.
- **Direct fidelity estimation** and efficient verification protocols for
  stabilizer/graph-state targets.
- **Quantized responses**: integer-valued estimands. **Proposition 29.1** — when the
  specification names a topological invariant, the acceptance margin is O(1) rather
  than O(tolerance), and sample complexity is set by readout noise alone. Quantized
  Hall conductance is measured to parts in 10⁹ and defines the SI ohm.
- **Symmetry indicators, entanglement witnesses, response functions, spectroscopy,
  transport** — all polynomial.

**Where Q4 fails:** generic states; and — genuinely open — verifying a quantum
simulator's output precisely in the regime where classical simulation fails, which is
the regime of interest. Partial answers exist (cross-platform comparison, random
circuit benchmarking, self-testing, invariant-based checks) and none is complete.

### 39.7 The synthesis: the class CQM(Ω)

> **Definition 39.1 (Compilable Quantum Matter).** A target class 𝒞 is in **CQM(Ω)**
> if
>
> **(i)** 𝒞 admits a finite witness set — finitely many bounded-sensitivity
> observables whose values, with a declared finite-size-scaling hypothesis, separate
> 𝒞 from a declared alternative set 𝒜 at the working sizes;
> **(ii)** 𝒞 contains a state with an efficient parent Hamiltonian realizable in
> Λ_Ω, natively or under universal-simulation encoding;
> **(iii)** that state is reachable in poly(N) time from an efficiently preparable
> initial state — by gapped adiabatic path, constant-depth adaptive circuit, or
> rapidly mixing engineered Liouvillian;
> **(iv)** the witness set of (i) is estimable within poly(N) copies at the required
> confidence.

**Contained in CQM:** tensor-network states with efficient parent Hamiltonians;
stabilizer and topological states (Abelian, and the measurement-preparable
non-Abelian classes); symmetry-broken phases with local order parameters;
free-fermion/Gaussian states; steady states of rapidly mixing engineered
Liouvillians; the lattice models realizable natively on each back end (§40.3).

**Not contained:** generic states of the Hilbert space; targets whose parent
Hamiltonian is non-local; targets requiring exponential preparation depth; phases with
no finite-size witness against their alternatives.

### 39.8 The verdict on Proposition Q, and why the restriction is not a defect

> **Proposition Q is FALSE in general and TRUE on CQM(Ω).**
>
> The disproof of the general case is a counting argument: states with poly-length
> descriptions are measure-zero in Hilbert space, so almost every quantum many-body
> state is not describable, not preparable, and not verifiable. No improvement in
> technology touches this — it is a statement about the size of Hilbert space.

**And this is exactly as true of classical matter.** Almost every arrangement of 10²³
atoms is not specifiable either; there is not enough paper in the universe. Drafts 1–2
never said this, and it matters, because it shows the quantum restriction is not a
special weakness of quantum matter. It is the same restriction, made visible by an
exponential.

The states with *names* — crystals, phases, folded proteins, topological sectors,
gapped ground states, engineered lattice models — are precisely the compressible ones.
This is the κ argument of §31 restated: **compilability is compressibility**, and
physics is interesting because the compressible sector is where structure lives.

### 39.9 Why the compressible sector is large: the area law

The remaining question is whether CQM is a thin curiosity or a fat, physically
central class. There is a structural answer.

> **Area law.** Ground states of gapped local Hamiltonians have entanglement entropy
> across a region boundary scaling with the boundary *area* rather than the volume.
> **Proved in one dimension** (Hastings, 2007), where it implies efficient
> matrix-product-state approximability with polynomial bond dimension.
> **Conjectured in higher dimensions**, with substantial numerical and partial
> analytical support. **OPEN in general.**

The chain this creates is the theorem program's central result:

```
   gapped local Hamiltonian
        ⇒  area law                          (proved in 1D; conjectured generally)
        ⇒  efficient tensor-network description
        ⇒  κ ≪ 1  (short generating description)
        ⇒  efficient parent Hamiltonian available      [Q2]
        ⇒  efficient preparation plausible             [Q3]
        ⇒  efficient witness estimation                [Q4]
        ⇒  target ∈ CQM(Ω)
```

> **Therefore: the compilability of quantum matter rests on the area law.**
> If it holds broadly, the physically relevant sector — gapped ground states of local
> Hamiltonians, which is most of condensed matter physics — is compilable. Where it
> fails (critical systems with logarithmic violations, volume-law states, generic
> excited states, chaotic dynamics), compilation fails with it, and *for the same
> reason*.

This is a falsifiable structural criterion rather than a slogan, and it converts
"is quantum matter compilable?" into a question with a known research frontier:
**how broadly does the area law hold?** That question is already a major open problem
in mathematical physics, pursued for independent reasons. This framework does not need
to solve it; it needs to be honest that its scope is co-extensive with the answer.

### 39.10 Restricted-class theorems worth attempting

Concrete, provable-looking statements that would advance the program:

- **T1.** For any MPS target with bond dimension D on N sites and a gapped parent
  Hamiltonian, the compilation problem (Q2–Q4 jointly) is solvable in time poly(N, D)
  with preparation error ε in poly(1/ε). *Assessment: likely provable; the
  constituent pieces exist.*
- **T2.** For stabilizer targets, CQM membership is decidable in poly time and the
  minimum sufficient measurement set (§41) is computable exactly.
  *Assessment: likely provable.*
- **T3.** For any target with a rapidly mixing engineered Liouvillian, preparation
  error is bounded independently of the Hamiltonian gap, and the protocol is robust
  to parameter uncertainty of size ≤ Δ_L/c. *Assessment: plausible; would formalize
  why dissipative engineering is strategically preferable.*
- **T4.** (Negative.) There exist families in CQM for which the *compiler's search*
  for λ is NP-hard even though the compiled protocol is efficient. *Assessment: almost
  certainly true; worth stating because it separates compile-time from run-time
  hardness, which the complexity vector (§16.7) already anticipates.*
- **T5.** The measure of the spec cell μ([S]) lower-bounds required samples and
  upper-bounds achievable ℒ. *Assessment: this is Open Problem 2; the quantum case may
  be the tractable one, because Hilbert-space measure is better behaved than measure
  on configuration space.*

---

## 40. The Quantum Physical IR

### 40.1 The design question

The ambition is
```
   Quantum Physical IR  →  { cold atoms, ions, superconducting, photonic,
                             solid-state, molecular, … }
```
the quantum analogue of `LLVM IR → {CPU, GPU}`. Do not ask for a universal
quantum-matter compiler; ask which substrate classes share a common intermediate
representation.

### 40.2 The layered IR

```
L0  SPECIFICATION        observables, invariants, acceptance regions, alternative
                         hypothesis set, confidences.                  SUBSTRATE-FREE

L1  ABSTRACT MODEL       canonical algebraic form:
                         mode type (spin-S / fermionic / bosonic), interaction graph,
                         term coefficients {J_ij, U, V, t, Δ, B, E, Ω_R, γ, …},
                         symmetry group, dissipators {L_k}.            SUBSTRATE-FREE

        ── the quantum-specific compiler passes live here ──

L2  ENCODED MODEL        after algebra-changing transformations:
                         fermion→qubit mappings, locality reduction, Trotterization,
                         universal-Hamiltonian encoding.        SUBSTRATE-CLASS-SPECIFIC

L3  SUBSTRATE-NATIVE     lattice depth, detunings, scattering length, tweezer
                         coordinates, twist angle, circuit layout, cavity parameters.
                                                                     BACKEND-SPECIFIC

L4  CONTROL SIGNALS      pulse shapes, ramps, holograms, AWG waveforms, gate schedules.
                                                                      DEVICE-SPECIFIC
```

### 40.3 The L1→L2 passes are real, named, and have quantified overhead

This is what makes the IR a genuine compiler design rather than an analogy. Each pass
below is an established transformation with a known cost:

| Pass | Transformation | Overhead |
|---|---|---|
| **Jordan–Wigner** | fermionic modes → qubits | Pauli weight O(N) |
| **Bravyi–Kitaev** | fermionic modes → qubits | Pauli weight O(log N) |
| **Ternary-tree mappings** | fermionic modes → qubits | ~log₃ N weight; near-optimal |
| **Fermionic swap networks** | non-local → local under a fixed connectivity | O(N) depth per Trotter step |
| **Perturbative gadgets** | k-local → 2-local, preserving the low-energy spectrum to ε | **Large coupling-ratio blowup** — the dominant practical cost, and the honest weak point of the whole encoding story |
| **Schrieffer–Wolff / Floquet averaging** | microscopic H → effective H_eff in a subspace or rotating frame | validity window in the small parameter; heating in the Floquet case |
| **Trotter–Suzuki decomposition** | continuous evolution → discrete gate sequence | error bounds now tight for many structured Hamiltonians |
| **Universal-Hamiltonian encoding** | arbitrary spin H → a fixed universal family | polynomial, with severe constants |
| **Bosonic truncation** | infinite-dimensional mode → finite cutoff | controlled by occupation bound; fails for squeezed/displaced regimes |

**Honest assessment of portability.** The IR *works* — an L1 model can be lowered to
several back ends. But portability is genuinely cheap only *within* a substrate class
whose native Hamiltonian family matches the target's algebra, and genuinely expensive
across classes, chiefly because of gadget coupling-ratio blowup. The analogy to LLVM
holds at the level of architecture and fails at the level of cost uniformity: compiling
a GPU kernel to a CPU is slow; compiling a strongly-correlated fermionic model to a
Rydberg array can be *infeasible*, not merely slow.

### 40.4 Back-end native families

| Back end | Native Hamiltonian family | Native dissipation | Best-matched targets |
|---|---|---|---|
| **Optical lattices (cold atoms)** | Bose/Fermi–Hubbard; tunable t, U, filling, dimension | weak, engineerable | Hubbard physics, Mott/superfluid, magnetism |
| **Rydberg tweezer arrays** | Ising and XY with programmable *geometry*; blockade constraints | spontaneous emission | spin models, frustrated geometries, topological spin liquids, arbitrary interaction graphs |
| **Trapped ions** | long-range spin-spin (tunable power law) via Mølmer–Sørensen | engineerable, high-fidelity | long-range spin models, adaptive circuits, non-Abelian order |
| **Superconducting circuits** | transmon lattices, tunable couplers, bosonic modes | strongly engineerable (a key strength) | digital-analog simulation, topological order, cat/bosonic codes |
| **Photonic** | Gaussian/bosonic; linear optics + nonlinearity | loss (native), engineerable | bosonic sampling, continuous-variable states, topological photonics |
| **Quantum dots / donors in silicon** | Hubbard-like with real material disorder | phonons | small-scale fermionic simulation, spin qubits, single-dopant devices |
| **Moiré / solid-state** | *not programmable in software* — the Hamiltonian is set at fabrication by twist, strain, stacking, gating | material-intrinsic | correlated and topological electronic phases at scale and, uniquely, at ambient conditions |
| **Molecular systems** | chemically designed spin and vibronic structure | solvent/vibrational | chemically tunable spin systems |

**The structurally important row is moiré/solid-state**, and it is a different kind of
back end from all the others: its λ is set *once, at fabrication*, not continuously in
software. It is therefore the bridge between target class C and target class A —
compiling a quantum phase there is a *materials* fabrication problem, which is why
twist angle is such a consequential parameter and why this back end is the one that
could deliver engineered quantum matter outside a cryostat.

### 40.5 What is genuinely universal, and what is not

| Layer | Universal? |
|---|---|
| L0 specification language and evidence calculus | **Yes.** Observables, protocols, tolerances, and confidences are substrate-free by construction |
| L1 canonical algebraic form | **Yes for spin/fermion/boson lattice models.** This is the real IR and it covers a large fraction of many-body physics |
| L1→L2 passes | **Yes as transformations; no as costs.** Same pass, wildly different overhead per target |
| L2→L3 lowering | **No.** Backend-specific by construction |
| Verification protocol synthesis | **Partially** — witness *classes* transfer (order parameters, invariants, shadows); instruments do not |

> **Revised universality verdict.** There is no universal compiler for matter. There
> *is* a universal front end (L0–L1), a set of universal *passes* with non-universal
> costs, and — by the universal-Hamiltonian theorem — a genuine universal compiler for
> **spin-Hamiltonian physics** with polynomial overhead. That is a much stronger
> positive result than Drafts 1–2 allowed, and it is bounded: polynomial overhead with
> severe constants, on an encoded subspace, for one algebra class.

---

## 41. The Minimum Sufficient Measurement Set

The formal core of QR-4, and the point where "compile for verifiability" becomes an
algorithm rather than a principle.

### 41.1 The problem

```
Given:  target class 𝒞, alternative hypothesis set 𝒜, measurement algebra ℳ
        with per-element cost, precision, destructiveness, and copy consumption

Find:   W ⊆ ℳ  minimizing  Σ_{w ∈ W} cost(w) · n_w

s.t.    for every alternative a ∈ 𝒜, the predicted values of W under 𝒞 and under a
        differ by ≥ margin m_w relative to the estimator's standard error,
        at joint confidence 1−δ over |W| tests
```

**Structure.** This is a *test cover* / discriminating-set problem. It is NP-hard. Its
objective is submodular in W (each added measurement has diminishing marginal
discriminating power), so **greedy selection achieves a (1−1/e) approximation** — a
genuine guarantee, and one of the few in the whole framework.

### 41.2 The witness library

The compiler searches over these, costed:

| Witness | Discriminates | Cost character |
|---|---|---|
| Local order parameters | symmetry-broken vs disordered | cheap, many copies |
| Correlators / structure factors | ordering wavevector, correlation length | cheap |
| Response functions, susceptibilities | criticality, soft modes | moderate |
| Spectroscopy (gap, dispersion) | gapped vs gapless; excitation content | moderate |
| Transport, quantized conductance | topological invariants | **cheapest per bit of certainty — integer estimand** |
| Symmetry indicators | band topology from symmetry eigenvalues at high-symmetry points | very cheap when applicable |
| Entanglement witnesses | entanglement presence/depth | moderate |
| Topological entanglement entropy | long-range entanglement | expensive (Renyi-2 swap tests) |
| Randomized measurements / classical shadows | m observables at once | **O(log m/ε²)** |
| Compressed sensing tomography | low-rank states | poly in rank |
| Anyon interferometry | braiding statistics, Abelian vs non-Abelian | demonstrated; moderate |
| Defect signatures | disorder, impurity phases | cheap |
| Dynamical response / quench spectroscopy | integrability, thermalization, phase boundaries | moderate |

### 41.3 The design consequence

Because the objective is a *cost-weighted* cover, the compiler will systematically
prefer targets and routes whose discriminating witnesses are cheap. In quantum matter
this favors, in order: **quantized/topological observables** (integer estimands,
O(1) margin), then **symmetry indicators**, then **local order parameters**, and
against **entanglement-entropy-based witnesses**.

> This is compile-for-verifiability with a concrete, computable preference ordering —
> and it makes a testable prediction (F-13): among physically equivalent routes to a
> quantum target, those specified by quantized observables should cost an order of
> magnitude less to certify.

### 41.4 The honest limit

The minimum sufficient set is minimal *against the declared alternative set 𝒜*. It
says nothing about alternatives nobody listed. This is the quantum instance of §11.3's
unspecified-property problem, and it is the mechanism behind essentially every
retracted claim of a new phase: the witness set was sufficient against the
alternatives the authors considered, and an unconsidered alternative explained the
data. **𝒜 is therefore a first-class, adversarially reviewed artifact of the
specification**, and the compiler should demand it explicitly.

---

## 42. Objective Functionals

### 42.1 The engineering objective

```
𝔍[P] = sup_{m ∈ 𝔐} {  w_E·E  +  w_T·T  +  w_R·R  +  w_C·C
                     +  w_V·C_verify  +  w_U·U  +  w_S·S_stability  }
```

| Term | Meaning | Note |
|---|---|---|
| E | energy and resource consumption | rarely binding (§15) |
| T | synthesis/preparation time | binding for coherence-limited targets |
| R | failure/risk | use CVaR, not expectation — tails dominate |
| C | economic cost | |
| C_verify | verification cost | **the framework's distinctive term** (§41) |
| U | uncertainty / model risk | epistemic, must be reported separately from aleatoric |
| **S_stability** | instability and sensitivity penalty | **new, and essential for quantum targets** |

**On S_stability**, which Drafts 1–2 omitted and which is not optional for target
classes B and C: a protocol reaching fidelity 0.999 at nominal parameters and 0.6
under the calibration drift observed over one hour is worse than one reaching 0.98
everywhere. The correct penalty is the *sensitivity* of the outcome to the parameters
that actually drift:

```
S_stability = Σ_j  s_j · | ∂(outcome) / ∂θ_j |²  ·  Var(θ_j)
```

with θ ranging over Hamiltonian parameters, calibration offsets, temperature,
decoherence rates, disorder realization, and environmental coupling — i.e. exactly the
uncertainty set of QR-3. This term is what makes the compiler produce protocols that
survive contact with an apparatus, and its absence is the most common reason
theoretically optimal control sequences fail in the laboratory.

The specification remains a **constraint** (Pr[Φ(σ_T) ∈ 𝒦] ≥ 1−δ), never a cost term.
You meet spec at confidence or you do not ship.

### 42.2 The research objective

```
                        I_verified(S)
   maximize   ℒ(S,Ω) = ───────────────────
                        K(P*) + C_ver(Ψ)
```

— physical computational leverage (Definition 2.3). This is a *different* objective
from 𝔍 and is the right one for a research program rather than a production line,
because it selects for the regime where the physics carries the load.

**The relationship between them.** 𝔍 asks "what is the cheapest way to make this
thing?" ℒ asks "how much physical capability did we obtain per unit of control we had
to exert and certify?" They can disagree sharply: a CNC route may minimize 𝔍 for a
one-off part while having ℒ ≈ 1, and a self-assembly route may have ℒ ~ 10²⁰ while
being economically worse at low volume. **A program optimizing only 𝔍 will
systematically under-invest in exactly the substrates where the paradigm's leverage
lives**, because those substrates are early and therefore expensive.

Practical recommendation: **use ℒ to select research directions and 𝔍 to select
production routes**, and report both. Reporting ℒ also gives the program a single
number that measures whether it is actually making progress on its own thesis, which
𝔍 does not.

### 42.3 The four-way joint objective, restated

Neither functional replaces the requirement of §3.3 that the compiler optimize
**preparability + stability + measurability + verifiability** jointly. ℒ's
denominator captures the last two; S_stability captures the second; the chance
constraint captures the first. A compiler that maximizes fidelity alone is optimizing
one of four.

---

## 43. The Ultimate Research Question

> **Can physical reality itself become a compilation target?**
>
> Not all reality. Not unconstrained reality. Not matter from nothing. Rather: *can
> sufficiently characterized physical substrates be transformed, through compiled
> control programs, into specified, measurable, and verifiable physical states —
> including quantum states and emergent quantum phases?*

### 43.1 The answer, as derived

**Yes, on the compressible sector; no, on the generic sector; and the compressible
sector is where all structured physics lives.**

Stated with its scope and its reason:

| | |
|---|---|
| **For quantum matter** | Yes on **CQM(Ω)** (Definition 39.1). The class is characterized by four conditions and its existence rests structurally on the **area law** — proved in 1D, conjectured generally. No on the generic sector, by a counting argument that no technology can touch |
| **For classical matter** | The same statement, with κ (§31) in place of the area law: compilable where a short generating description exists (crystals, phases, self-assembled and rule-generated structures), and reducible to explicit instruction execution — with its throughput arithmetic — where κ ≈ 1 |
| **The scaling** | Set by how broadly the area law and, classically, low-κ structure hold. This is a question already open in mathematical physics for independent reasons; the framework's scope is co-extensive with its answer and should say so |
| **Where it fails** | Volume-law states, chaotic dynamics, generic excited states, glassy landscapes, and bespoke targets with no compressible generating rule. In each case compilation fails **for the same reason** — the description does not compress — which is why one criterion covers both halves of the framework |

### 43.2 What would change the answer

**Toward yes (broader scope):** proof of the area law in d ≥ 2; refutation of the
quantum PCP conjecture; discovery of a restoring-contractive intermediate variable for
a non-electronic substrate (Open Problem 1) — quantum error correction being the
existence proof that such variables can be *constructed*; practical constant-depth
adaptive preparation at scale (F-12).

**Toward no (narrower scope):** proof of quantum PCP, eliminating general
approximation guarantees; demonstration that model calibration fails
out-of-distribution (F-1) — which would make every `DERIVE` claim in the framework
worthless regardless of the physics; saturation of coherence improvements across
platforms (F-16), which would convert decoherence from a cost curve into a ceiling.

### 43.3 Why this is a scientific question rather than a manifesto

It has a characterized positive class, a counting-argument disproof of the general
case, a structural conjecture (the area law) whose status is independently tracked, a
set of restricted-class theorems worth attempting (§39.10), and sixteen falsification
experiments with quantitative thresholds (§25, §38) — three of which, run first for
under $6M, determine whether the foundation exists at all.

That is the difference between "reality is programmable" as a slogan and as a research
program. **The slogan cannot be wrong. The program can be, in at least sixteen
identified ways, and the fastest of them reports in nine months.**
