# Part I — Foundations

Sections 1–4: Executive Thesis · Definition of Reality Compilation · Mathematical
Foundations · Physical State Spaces

---

## 1. Executive Thesis

### 1.1 The proposition under test

The proposal asserts that there exists a computational architecture

```
Compile(R_A, S, Ω) → (P*, R_B, Π)
```

taking an initial physical state, a formal specification, and an available
substrate, and returning an executable physical trajectory, a resulting state, and
a provenance record. The research question is whether transformation between
physically realizable states can be formalized as a general computational problem.

### 1.2 The verdict

**The answer is a qualified yes, but the qualification is the entire content.**

Four statements, in order of decreasing confidence:

**(A) Physical synthesis is already compilation, in several domains, today.**
**ESTABLISHED.** Electronic design automation compiles a behavioral specification
(Verilog/VHDL) through logic synthesis, place-and-route, and design-rule checking
into a mask set, which a fab executes to produce a physical object whose measured
properties are then verified against the specification, with cryptographic
provenance increasingly attached. CNC/CAM compiles a solid model into G-code.
Retrosynthesis planners compile a target molecule into a reaction sequence.
DNA-origami compilers (caDNAno, and successors) compile a target 3D shape into a
staple-strand set that self-assembles. These are real, industrial, and they are
*special cases of the proposed architecture*. The Reality Compiler is therefore
not a new idea in kind; it is a proposal to **generalize an existing and successful
pattern across substrates**.

**(B) The generalization is not free, and the obstruction is not computational
power.** **STRONGLY SUPPORTED.** Every existing physical compiler works because its
substrate supplies four things: a *discrete, composable action set* with
characterized effects; a *predictive forward model* accurate to within the
tolerance band; a *metrology channel* that can certify the specification with a
tractable number of measurements; and *yield statistics* stable enough to price
risk. Where a substrate lacks any one of these, no amount of compute produces a
compiler. The semiconductor industry did not get EDA by getting faster computers;
it got EDA by spending forty years making the substrate compilable — design rules,
standard cells, process corners, SPICE models, DFT/scan chains, SPC.
**This is the single most important strategic conclusion in this document: the
Reality Compiler program is primarily a program of substrate discipline, and only
secondarily a program of algorithms.**

**(C) There is no universal physical compiler.** **STRONGLY SUPPORTED**, argued in
§17. Universality fails separately at representation, optimization, control,
fabrication, and verification, and the failures are not of the same kind. What
*can* be universal is the **specification language and the attestation format** —
i.e., the front end and the ledger — with substrate-specific back ends. This is
exactly the LLVM architecture, and it is the correct architecture here.

**(D) The most defensible novel contribution of the framework is not the compiler.
It is the elevation of *verification sample complexity* to a first-class term in
the compilation objective.** **PLAUSIBLE**, and in the author's judgment the
highest-value idea in the proposal once corrected. Compiling *for verifiability* —
choosing among physically equivalent synthesis routes the one whose acceptance
predicate is cheapest to certify — is an underexploited degree of freedom. It has a
direct and successful precedent in design-for-testability in VLSI, where designs
are deliberately made suboptimal in area in exchange for polynomial rather than
exponential test-vector counts. No comparable discipline exists for materials,
chemistry, or biology. It should.

### 1.3 What is being rejected

| Original claim | Verdict | Replacement |
|---|---|---|
| "Software creating matter" | **CONTRADICTED** (conservation laws) | Computational synthesis of physically permitted state transformations of a supplied substrate |
| "Exact physical compilation is NP-hard" | **CONTRADICTED** as a characterization — too weak, and not a single class | A complexity *vector*; general reachability is undecidable (§16) |
| `R = (x,p,c,f,q)` as a state | **Mathematically incorrect** — mixes ontic, derived, and epistemic quantities | Ontic state σ ∈ Σ, derived property map Φ, epistemic belief 𝔟 over Σ (§4) |
| `R(T) ⊨ S` for a target state `R_B` | **Category error** | `Φ(σ(T)) ∈ 𝒦` with confidence 1−δ; the target is a *region*, not a state (§4.4) |
| Room-temperature macroscopic quantum coherence substrate | **UNSUPPORTED** as stated; strong form **CONTRADICTED** | A feasibility matrix of coherent resources with quantitative windows (§9) |
| "Cryptography protects matter" | **CONTRADICTED** | Cryptography binds *records*; physical binding requires an unclonability assumption (§12) |
| Quintillion-dollar valuation | **CONTRADICTED** (exceeds world GDP ×10⁴) | Bottom-up model, §22 |

### 1.4 What is being kept, and strengthened

- The compile/execute/verify/attest pipeline as an **architecture**. Correct and
  buildable.
- The insistence that verification be a *layer*, not an afterthought. Correct, and
  the framework should go further than the proposal does: verification cost belongs
  inside the objective functional.
- The insistence on provenance. Correct, and it maps onto existing, deployed
  supply-chain integrity work (in-toto, SLSA, Sigstore, C2PA) rather than requiring
  new cryptography.
- Hamiltonian/landscape engineering as an alternative to trajectory computation.
  Correct in principle, with a precise and narrow statement of when it pays (§8.5).
- Self-assembly as computation. **ESTABLISHED** in the tile-assembly literature and
  a genuine special case of the framework (§18).

---

## 2. Definition of Reality Compilation

### 2.1 The corrected first principle

The proposal's first principle — replace "software creating matter" with
"computational synthesis and execution of physically permitted state
transformations" — is correct and should be tightened once more. Three
conservation-class constraints must be explicit in the type of the problem, not
merely acknowledged in prose:

1. **Conservation.** Baryon number, charge, energy, and momentum are conserved.
   The compiler *sorts and rearranges* a supplied inventory. Any specification
   requiring elements not in Ω is infeasible at compile time and must fail with a
   *typed* error, not a search timeout.
2. **Second law.** Any local entropy reduction in the workpiece must be paid for by
   at least as much entropy exported to the environment. The compiler's resource
   accounting must therefore include an environmental sink of finite capacity.
3. **Locality and causality.** Controls act through fields and contacts with finite
   propagation speed and finite spatial resolution. The control authority available
   is a property of Ω, not a free parameter.

### 2.2 Definition (informal)

> **Physical synthesis compilation** is the problem of producing, from a
> declarative specification of an acceptance predicate over measurable physical
> properties, an executable control program over a declared substrate's action
> algebra, together with a measurement plan sufficient to certify the predicate at
> a stated confidence, and a machine-checkable record binding the three.

Three things are deliberately *not* in this definition:

- **A target state.** See §4.4. The target is the acceptance region.
- **Any claim of optimality.** Optimality is a separate, harder problem (§16.4).
- **Any claim about substrate generality.** Generality is a property of a particular
  compiler, not of the definition.

### 2.3 Definition (formal)

Let a *substrate declaration* be

```
Ω = ⟨ Σ, 𝒜, 𝒯, ℳ, 𝒞, ℰ, 𝔐 ⟩
```

- **Σ** — the ontic state space at the declared level of description.
- **𝒜** — the *action algebra*: a set of primitive, physically available control
  actions, closed under a composition operator, with declared preconditions,
  durations, resource draws, and effect models. This is the analogue of an
  instruction set architecture.
- **𝒯** — the admissible control signal space (the analogue of "legal programs"):
  measurable functions u : [0,T] → 𝒰 with 𝒰 the instantaneous control set, plus
  bandwidth, amplitude, slew, and duty constraints.
- **ℳ** — the *measurement algebra*: available measurement operations, each with
  a declared estimator, precision, bias model, acquisition time, cost, and
  destructiveness.
- **𝒞** — consumables and inventory (a vector of counts/moles by species, plus
  energy and time budgets).
- **ℰ** — environment: temperature, pressure, atmosphere, radiation, vibration,
  and their tolerances and drifts.
- **𝔐** — the *model bundle*: a set of forward models with declared domains of
  validity and quantified error, plus a prior over which is right.

Let a *specification* be

```
S = ⟨ 𝒪, 𝒦, δ, ≺ ⟩
```

- **𝒪 = (O₁,…,O_k)** — a finite vector of *specified observables*, each a
  well-defined estimand with an operational measurement definition.
- **𝒦 ⊆ ℝ^k** — the *acceptance region*: a (typically product-of-intervals or
  semialgebraic) set of admissible observable values.
- **δ ∈ (0,1)** — the maximum tolerated probability of accepting an article that
  is outside 𝒦 (and, separately, a β for false rejection).
- **≺** — a preference order or scalarizing objective over feasible programs.

Then:

> **Definition 2.1 (Physical Synthesis Compilation, PSC).**
> Given (𝔟₀, S, Ω) where 𝔟₀ is a belief (a probability measure) over the initial
> ontic state, produce a tuple (P*, Ψ, Π) such that
>
> - **P\*** is an executable program over 𝒜 ∪ ℳ, feasible under 𝒯, 𝒞, ℰ;
> - **Ψ** is a *measurement and decision plan*: a sequence of operations from ℳ and
>   an acceptance rule A : (measurement records) → {PASS, FAIL, INCONCLUSIVE};
> - **Π** is a verifiable record binding S, Ω, P*, Ψ, the executed trace, and the
>   measurement outcomes;
>
> and such that, under the model bundle 𝔐,
>
> ```
> Pr[ A(records) = PASS  ∧  Φ(σ(T)) ∉ 𝒦 ]  ≤  δ
> ```
>
> i.e. **the compiler's contract is about the joint behavior of the process and the
> test, not about the state.**

This is the reformulation that makes the whole framework well-typed. It says: *the
compiler is not responsible for producing a state; it is responsible for producing
a process-plus-test pair whose false-accept rate is bounded.* That is exactly what
a fab, a pharmaceutical plant, and an aerospace supplier are each contractually
responsible for today, which is evidence that the formulation is the right one.

### 2.4 On the name

**Recommendation: do not publish under "Reality Compiler."** The term asserts that
the object of compilation is reality, which is false — the object of compilation is
a bounded substrate under declared conditions. The name will cost credibility in
exactly the venues (Nature Materials, PRX, POPL/PLDI, QIP) where the work needs to
land. Proposed alternatives, in order of preference:

1. **Physical Synthesis Compiler (PSC)** — parallel to "logic synthesis," immediately
   legible to the EDA and formal-methods communities.
2. **Substrate-Targeted Synthesis Compiler**.
3. **Specification-to-Substrate (S2S) compilation**.

Keep "Reality Compiler" as the program/product name if desired; the distinction
between a product name and a technical term is normal and costs nothing.

---

## 3. Mathematical Foundations

### 3.1 The four problems, restated correctly

The proposal's RC-0…RC-4 are close to right but conflate decision problems with
search problems and omit the epistemic layer. Restated:

**RC-0 — Feasibility (decision).**
Given (𝔟₀, S, Ω), does there exist P ∈ Programs(Ω) with
Pr[Φ(σ(T)) ∈ 𝒦] ≥ 1 − δ ?
*This is the wrong primitive to lead with*, because for most substrates it is
undecidable (§16.2). The useful primitive is bounded feasibility:

**RC-0′ — Bounded feasibility.**
…with |P| ≤ L, T ≤ T_max, resources ≤ 𝒞, over a *finite* action algebra 𝒜 and a
*declared* model class. This is decidable, and typically NP-hard to
PSPACE-hard. **This restriction is not a technicality; it is the move that makes
the theory exist.**

**RC-1 — Reachability.**
Characterize Reach(𝔟₀, Ω, T) ⊆ Δ(Σ), the set of belief states attainable. Note the
target: reachability is on *beliefs*, not states, because the controller never
knows σ. This is the correct formulation and it is the one used in robust and
stochastic control. Under full-state feedback it degenerates to ordinary
reachability; in real fabrication it never does.

**RC-2 — Optimization.**
P* = argmin_{P feasible} 𝔍[P] with 𝔍 defined in §7.4. Note that under model
uncertainty the correct problem is distributionally robust:
P* = argmin_P sup_{m ∈ 𝔐} 𝔼_m[𝔍]. Plain expected-cost minimization over a single
model is the standard failure mode of simulation-driven process design.

**RC-3 — Lowering / execution.**
A *compilation* map from the abstract program to machine-specific control signals,
including calibration binding. The correct formal object is a refinement relation
in the sense of program refinement: each lowering step must preserve the acceptance
guarantee, and this obligation should be discharged by proof or by measurement, not
assumed.

**RC-4 — Verification.**
A statistical decision procedure with a declared operating characteristic
(false-accept ≤ δ, false-reject ≤ β) under a stated measurement model. The output
must be three-valued; two-valued verification silently converts "we did not measure
enough" into "PASS," which is the dominant real-world failure mode.

### 3.2 Why "physically permitted" needs a definition

The phrase "physically permitted state transformation" has an exact meaning in one
setting and is vague everywhere else.

- **Closed quantum systems.** Permitted maps are unitary; the constraints are
  conservation of the spectrum of ρ and of all symmetry charges. **ESTABLISHED.**
- **Open quantum systems.** Permitted maps are CPTP channels. With a thermal bath
  and energy-conserving global unitaries, the permitted maps are the *thermal
  operations*, and reachability is governed by **thermomajorization**, a partial
  order — not a total order. For states diagonal in the energy eigenbasis, ρ → ρ′
  is achievable by thermal operations iff ρ thermomajorizes ρ′. This is a genuine,
  rigorous, and underused answer to "what transformations are physically permitted,"
  and it is the closest existing thing to a reachability theory for the framework.
  **ESTABLISHED** (resource theory of thermodynamics; Horodecki–Oppenheim,
  Brandão–Horodecki–Oppenheim–Renes–Spekkens; see `CITATIONS.md`).
  *Critical caveat:* it assumes exact energy conservation and arbitrary bath access,
  and in the single-shot regime it produces a whole family of "second laws" rather
  than one. It also does not model kinetics, which is where the real obstruction
  lives.
- **Classical macroscopic systems.** There is no comparably clean theory. Permitted
  = reachable under the actual control authority, which is an empirical question
  about 𝒜.

**Conclusion.** "Physically permitted" is not a single predicate. The framework
should carry a *layered* permission stack:

```
CONSERVATION  (hard, exact, checkable symbolically)
   ⊃ THERMODYNAMIC  (thermomajorization / free-energy, exact for the idealized model)
      ⊃ KINETIC     (barrier heights, rates, timescales — where feasibility is actually decided)
         ⊃ CONTROL  (what 𝒜 can actually do — where feasibility is actually decided)
            ⊃ ECONOMIC (what 𝒞 can pay for)
```

with the honest note that **for nearly every real target, the binding layer is
KINETIC or CONTROL, not the two above it.** Diamond is thermodynamically forbidden
at room temperature and pressure and is manufactured at scale anyway, because the
kinetic barrier to graphitization is ~730 kJ/mol. Conversely, thousands of
thermodynamically favorable targets are unreachable because no route exists. Any
framework that leads with thermodynamics is optimizing the wrong layer.
**STRONGLY SUPPORTED.**

### 3.3 Dynamics

The governing object is a controlled, noisy, hybrid dynamical system. In the most
general useful form:

```
dσ = f(σ, u(t), θ) dt + g(σ, θ) dW      (continuous evolution, θ = model parameters)
σ⁺ = J_e(σ⁻, u)                         (discrete jumps: tool changes, phase transitions,
                                          material addition, reaction events)
y_j = h_j(σ(t_j)) + η_j                 (measurements, from ℳ)
```

with a guard condition determining when jumps fire. This is precisely a **stochastic
hybrid system**, and that identification is load-bearing, because the decidability
results for hybrid systems apply directly (§16.2) and they are severe.

For quantum substrates the corresponding object is a controlled Lindblad equation:

```
ρ̇ = −(i/ħ)[H₀ + Σ_j u_j(t) H_j, ρ] + Σ_k ( L_k ρ L_k† − ½{L_k†L_k, ρ} )
```

with the controls entering the Hamiltonian (coherent control) and, in the
dissipative-engineering case, the L_k as well (§8.4).

### 3.4 The core structural theorem the framework needs (and does not yet have)

The framework's most important missing mathematical object is a
**composition theorem**:

> **Desideratum 3.1 (Compositionality).** If program P₁ certifies that the article
> lies in 𝒦₁ at confidence 1−δ₁, and P₂ is a program that, for every input in 𝒦₁,
> certifies output in 𝒦₂ at confidence 1−δ₂, then the composite P₂∘P₁ certifies 𝒦₂
> at confidence ≥ 1 − (δ₁+δ₂+ε) for a bounded interaction term ε.

Without this, multi-step synthesis plans cannot be reasoned about compositionally
and every route must be validated end-to-end empirically — which is exactly the
status quo in chemical process development and exactly the thing the framework
exists to fix.

The interaction term ε is not generally small. It is bounded when:
- the intermediate acceptance region 𝒦₁ is a *sufficient* statistic for the
  downstream process (i.e., downstream behavior depends on the article only through
  Φ₁), and
- the downstream process is *robust* on 𝒦₁ (Lipschitz in the relevant metric with a
  constant small enough that 𝒦₁'s diameter maps inside 𝒦₂'s).

**This gives the framework its first genuinely actionable design rule:**
*specify intermediates by sufficient statistics and design steps to be
contractive on them.* This is the physical-synthesis analogue of designing digital
logic with noise margins — the reason digital abstraction composes at all is that
each stage is contractive on the specified variable (voltage) and the specified
variable is sufficient. **PLAUSIBLE as a general design rule; ESTABLISHED as the
explanation for why digital electronics composes.**

Identifying, for each substrate, whether such a *restoring, sufficient, contractive*
variable exists is, in this author's view, the correct central research question of
the whole program. It is restated as Open Problem 1 in §27.

---

## 4. Physical State Spaces

### 4.1 Why `R = (x, p, c, f, q)` fails

The proposed tuple contains four different kinds of object:

- `x, p` — ontic dynamical variables. Legitimate members of a state.
- `c` (composition) — **redundant**: composition is a function of the microstate
  (which nuclei are where). Including it separately permits inconsistent states.
- `f` (functional properties) — **not a state component at all**. Tensile strength,
  conductivity, and catalytic activity are *functionals of the state* and of the
  test protocol. Yield strength is not a property of an atomic configuration; it is
  a property of a configuration *plus a loading protocol, strain rate, temperature,
  and specimen geometry*. Putting `f` in the state licenses the fiction that one can
  specify a property independently of how it is measured, which is the most common
  error in inverse-design problem statements.
- `q` (epistemic/verification/provenance) — **belongs to the observer, not the
  system**. A sample does not carry its own certificate; a *record* does.

Including all five in one tuple means the compiler can produce states that are
internally contradictory (a composition inconsistent with the configuration, a
strength inconsistent with the microstructure) and cannot detect this.
**Mathematically incorrect; replace.**

### 4.2 The replacement

Three objects, kept separate and typed:

**(i) Ontic state.** σ ∈ Σ_ℓ at level of description ℓ. Nothing else.

**(ii) Property map.** Φ_ℓ : Σ_ℓ × 𝒫 → ℝ^k, where 𝒫 is the *protocol*: the test
conditions under which the property is defined. Every specified property is a pair
(estimand, protocol). Writing `tensile_strength ≥ 400 MPa` without a protocol is an
ill-formed program, and the DSL in §5 rejects it at type-check time. This single
rule eliminates a large class of real industrial disputes.

**(iii) Epistemic state.** 𝔟 ∈ Δ(Σ_ℓ) — a probability measure, or in the
set-membership formulation a compact uncertainty set. The compiler plans in 𝔟-space
(this is a belief-MDP / POMDP formulation, §7.2). Provenance Π is a separate
artifact attached to the *record*, never to σ.

So: **R := (σ, ℓ)** for the physics, **𝔟** for what the compiler knows, **Φ** for
what the specification talks about, **Π** for what can be proven to a third party.

### 4.3 The representation hierarchy, and the honest statement about it

The proposal asks for R_macro ↔ R_meso ↔ R_micro ↔ R_quantum with the compiler
moving between them. The hierarchy is real; the bidirectional arrows are not.

| Level | State object | Typical span | Downward map | Upward map |
|---|---|---|---|---|
| **Quantum** | ρ on a Hilbert space; MPS/PEPS/tensor network; second-quantized wavefunction | 10⁰–10² atoms (exact); 10²–10⁴ (tensor networks, with entanglement-structure caveats) | — | coarse-grain to force field |
| **Micro (atomistic)** | positions/momenta of nuclei; interatomic potential or MLIP | 10³–10⁹ atoms, ns–µs | parameterize from quantum | coarse-grain to phase field / CG beads |
| **Meso** | phase fields, dislocation densities, grain structure, CRN markings, polymer CG beads | µm–mm, µs–s | calibrate from atomistic | homogenize |
| **Macro** | continuum fields (stress, temperature, concentration), constitutive laws | mm–m, s–hours | homogenize | — |

**The load-bearing honest statement:** coarse-graining downward→upward is
*information-destroying and generally not invertible*, and this is not a solvable
engineering problem. The upward maps are **many-to-one**; a macroscopic
specification does not determine a microstructure, which is precisely why inverse
design is underdetermined and why the compiler must return a *set* of candidates
plus a discriminating measurement plan, not a unique answer. **ESTABLISHED.**

Furthermore, the coarse-graining maps carry *errors that are not uniformly bounded*.
Machine-learned interatomic potentials are accurate inside their training
distribution and silently wrong outside it; CALPHAD assessments interpolate well and
extrapolate badly; continuum constitutive models fail at free surfaces, cracks, and
phase boundaries — i.e., exactly where interesting things happen. A compiler that
treats model error as a small additive term is building on sand. **STRONGLY
SUPPORTED.** The framework's response must be architectural: every model in 𝔐
carries a *validity domain predicate* and an *error bound*, the compiler refuses to
use a model outside its declared domain, and the plan includes measurements chosen
to discriminate between models in 𝔐 whenever their predictions diverge inside the
tolerance band. This is the *active model-discrimination* requirement (§7.5) and it
is a genuine architectural contribution.

### 4.4 The central correction: targets are regions, not states

Three independent arguments, each sufficient:

**(1) Measure-theoretic.** For any continuous Σ, every single state has measure
zero. `Pr[σ(T) = σ_B] = 0` for any stochastic dynamics. A specification naming a
point is unsatisfiable with probability 1.

**(2) Metrological.** No measurement returns a point. Every measurement returns an
interval or a posterior. `V(R̂_T, S)` as written in the proposal presupposes
comparing two states; what actually happens is comparing an *estimate with
uncertainty* against a *region*. The three-valued output the proposal correctly
demands (PASS/FAIL/UNCERTAIN) is only definable if the target is a region — against
a point target the answer is always UNCERTAIN.

**(3) Engineering.** No specification anyone has ever written names a state. They
name tolerances. ASME Y14.5 geometric dimensioning and tolerancing, ASTM material
standards, ICH pharmaceutical specifications, and semiconductor process windows are
all acceptance regions over functionals. The industry converged on regions because
regions are the only thing that can be met and checked. **ESTABLISHED.**

So the framework's central relation becomes:

```
σ(T) ⊨ S    ⟺    Φ(σ(T), 𝒫) ∈ 𝒦
```

and the compiler's guarantee is about Pr[·], not about identity. Formally the target
is the **spec cell**

```
[S] := Φ(·, 𝒫)⁻¹(𝒦) ⊆ Σ
```

which is typically an enormous, highly degenerate subset of Σ — and the degeneracy
is a *resource*, not a nuisance: it is the slack the optimizer spends on
manufacturability, cost, and verifiability. **The measure of [S] relative to Σ is
the single best predictor of whether a compilation problem is easy or hard**, and
formalizing that intuition is Open Problem 2 (§27).

### 4.5 When is the quantum level actually needed?

A practical triage, because "quantum" appears throughout the proposal and is usually
not required:

| Need quantum-level state | Do not |
|---|---|
| Bond making/breaking, reaction barriers, excited states, catalysis | Elastic/plastic deformation of bulk metals |
| Magnetism, superconductivity, strongly correlated electrons | Heat transfer, fluid flow, most diffusion |
| Spectroscopic signatures, color, photochemistry | Grain growth, phase-field evolution |
| Qubit/spin-based devices, NV centers, molecular magnets | Most polymer processing, most ceramics sintering |
| Isotope- and spin-selective chemistry | Machining, joining, additive deposition |

The honest consequence: **for most of the near-term economic surface of the
framework (§20–22), the quantum layer enters only through parameterizing
interatomic potentials and thermochemistry at compile time, and never through
coherent control of the workpiece.** Treating the quantum substrate as central
inverts the actual dependency structure.

### 4.6 Representations that should be first-class in the implementation

Not a survey; a shortlist of what the compiler's IR actually needs.

- **Molecular graphs** with stereochemistry (SMILES/SMARTS/InChI, or better, a
  typed graph IR) — for chemical routes.
- **Crystal structures** with symmetry (space group + Wyckoff positions + occupancy)
  — the canonical form matters for deduplication and for provenance hashing.
- **Reaction networks as Petri nets / vector addition systems** — this identification
  is important because it imports a complete decidability and complexity theory
  (§16.3).
- **Phase-field / microstructure tensors** with explicit representative-volume-element
  semantics.
- **Geometry as boundary representation plus a tolerance schema** (GD&T), not as a
  mesh; meshes are a lowering artifact.
- **Tensor networks (MPS/PEPS) with declared bond dimension** for the quantum layer —
  and a declared entanglement-structure assumption, because the representation is
  only faithful when that assumption holds.
- **Belief representations:** Gaussian-process posteriors over property maps, and
  particle/ensemble representations over model parameters θ. The compiler's native
  currency is a posterior, not a value.

A design note with real consequences: **the IR must be lossy in a declared,
auditable way.** Every abstraction step should emit into Π the statement "this step
assumed X and discarded Y." That is what makes the resulting attestation meaningful
(§12) and is, again, standard practice in EDA (abstraction with formally recorded
assumptions) and absent everywhere else.
