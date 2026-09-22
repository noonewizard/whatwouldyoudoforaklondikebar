# Part IV — The Quantum Substrate

Section 9. **Rewritten in Draft 4** under adversarial review; supersedes the Draft 1
text entirely and incorporates the Draft 2 corrections (errata E2, E4).

---

## 9. Quantum Substrate

### 9.1 Quantum resources and scope

Three roles, with wholly different maturity. Conflating them inflates the framework's
apparent quantum dependence and has been the single most common error in this area.

#### Use 1 — Quantum computation at compile time
```
   quantum computer  →  better physical model  →  better compiled plan
```
The workpiece is classical; the quantum machine is in the data centre, computing
electronic structure, reaction barriers, or correlated-electron properties that feed
𝔐.

**Status.** The case for *exponential* advantage in generic ground-state chemistry has
been substantially weakened: the bottleneck is preparing an initial state with
non-vanishing overlap with the true ground state, and classical heuristics (DMRG,
coupled cluster, selected CI, QMC) scale better in practice than naive comparison
suggests. Polynomial advantage for Hamiltonian *dynamics* simulation remains solid.
Resource estimates for canonical hard targets have fallen by several orders of
magnitude through better algorithms but still imply ~10⁵–10⁷ physical qubits.
**Classification: CONTESTED for generic exponential advantage (tending CONTRADICTED);
STRONGLY SUPPORTED for polynomial advantage in dynamics.** *Not on the near-term
critical path.*

#### Use 2 — Quantum state engineering at run time
```
   quantum workpiece  →  controlled quantum state / phase
```
The artifact itself has quantum degrees of freedom that are deliberately prepared:
spin defects, single dopants, engineered arrays, superconducting circuits.
**Classification: DEMONSTRATED at the scales in §9.10.** *This is where the framework's
thesis is most directly testable.*

#### Use 3 — Quantum-enhanced metrology
```
   quantum sensor  →  better verification
```
NV magnetometry and scanning-NV imaging; squeezed light for sub-shot-noise
displacement sensing, deployed in gravitational-wave detection; NV-based NMR at
nanoscale volumes.
**Classification: STRONGLY SUPPORTED and deployed.** *This is the near-term quantum
contribution to the framework, and it is ahead of quantum computing on the relevant
timeline. Drafts 1–2 ordered these backwards.*

---

### 9.2 Quantum matter as a compilation target

> **DEFINITION 9.1 (Quantum Matter Synthesis).**
> ```
>   substrate + interactions + environment + control  →  quantum phase / function
> ```
> The engineered object is the Hamiltonian and its dissipative environment, not the
> instruction sequence and not the trajectory.

#### 9.2.1 Why this is a peer target, not an exotic extension

Scored against the framework's own four compilability criteria (§10.3):

| Criterion | Structural alloy | Programmable quantum matter |
|---|---|---|
| Composable conditions with characterized, local effects | Poor — path-dependent microstructure, non-local effects | **Excellent** — lattice depth, detuning, tweezer coordinates, drive frequency each map to a declared term in H |
| Forward model accurate to ≪ tolerance, with known validity domain | Poor — constitutive models fail at interfaces and defects | **Excellent in form, hard in solution** — the model *is* the Schrödinger–Lindblad equation. The difficulty is solving it, not knowing it |
| Metrology with polynomial sample complexity | Moderate | **Mixed, and excellent for quantized targets** (§9.8) |
| Stationary, characterized yield statistics | Good for mature processes | **Excellent** — each shot prepared fresh from a known initial state; no history dependence, no ageing, no batch drift |

> **PROPOSITION 9.2.** On three of the four criteria, programmable quantum matter is a
> *better*-posed compilation target than structural materials. The apparent difficulty
> — cryostats, lasers, vacuum — is infrastructure cost, not a compilability deficit.
> **Classification: PLAUSIBLE**, argued from the criteria; the criteria themselves are
> a proposal (§10.3), so this inherits their status.

> **PROPOSITION 9.3 (Quantum matter escapes the Avogadro constraint).** A phase is a
> pattern of correlations over a length scale, not a quantity of substance. The
> required system size is set by the correlation length, the topological sector, and
> the excitations of interest — typically 10²–10⁴ sites — not by Avogadro's number.
> Serial atom-by-atom assembly, which cannot produce bulk matter, is therefore a
> *working manufacturing technology for this target class*. **Classification:
> ESTABLISHED** (from the definition of a phase plus §9.10's demonstrated array sizes).

#### 9.2.2 Target maturity classification

Maturity is stated for *engineered/programmable* realization, with the materials route
noted separately where they differ. This distinction is load-bearing and was blurred
in Draft 1.

| Target | Engineered/programmable route | Materials route | Notes |
|---|---|---|---|
| **Lattice models** (Hubbard, Ising, XY, Heisenberg, *t*–*J*) | **DEMONSTRATED** — Bose/Fermi–Hubbard in optical lattices; programmable Ising/XY geometry in Rydberg arrays; long-range spin models in ions | n/a | Entropy, not control, is the binding constraint |
| **Superconducting phases** | n/a | **DEMONSTRATED** to ~203 K (H₃S, 155 GPa) and ~250 K (LaH₁₀, 170 GPa), with active disputes over some magnetization analyses. Ambient-pressure room-T: **UNSUPPORTED** — three retractions | **No known theorem bounds T_c.** Ambient room-T superconductivity is ALLOWED and undiscovered, not forbidden |
| **Correlated-electron phases** (Mott, heavy fermion, strange metal) | **DEMONSTRATED** in cold atoms and moiré | **DEMONSTRATED** in oxides, moiré | Predictive *design* remains **OPEN** |
| **Topological order, Abelian** | **DEMONSTRATED** — toric-code ground states prepared and topological entanglement entropy measured on superconducting hardware; spin-liquid signatures on Rydberg arrays | candidate materials only | |
| **Topological order, non-Abelian** | **DEMONSTRATED (2023–2024)** — D₄ order on 27 trapped-ion qubits via adaptive circuit, fidelity per site >98.4%, non-Abelian braiding detected by anyon interferometry; Fibonacci string-net braiding on superconducting hardware | **SPECULATIVE** as an intrinsic material phase | **Sharply distinct from Majorana hardware qubits**, which remain SPECULATIVE with a poor replication record |
| **Topological invariants / band topology** | **DEMONSTRATED** — invariants measured directly in cold atoms, photonic lattices, moiré | **DEMONSTRATED** | |
| **Fractionalized phases (FQAH / fractional Chern)** | — | **DEMONSTRATED (2023)** — fractional quantum anomalous Hall states at ν = −2/3, −3/5 at *zero* magnetic field in twisted MoTe₂ | Twist angle as a compilable parameter |
| **Magnetic phases, spin textures** | **DEMONSTRATED** | **DEMONSTRATED** — skyrmions via interfacial DMI engineering | |
| **Quantum spin liquids** | **DEMONSTRATED at the level of signatures** on Rydberg arrays | **CONTESTED** — no confirmed material | Alternative-hypothesis discipline (§9.8.4) is essential here |
| **Moiré systems** | **DEMONSTRATED** — twist angle, filling, and displacement field select among correlated, superconducting, and topological phases | same | The bridge between target classes C and A: λ is fixed at fabrication, not in software |
| **Programmable atomic arrays** | **DEMONSTRATED at 6,100 sites** with ~13 s coherence, 99.98% single-qubit control, ~23 min trap lifetime (2025) | n/a | |
| **Quantum metamaterials** | **DEMONSTRATED** | **DEMONSTRATED** | |
| **Photonic matter / topological photonics** | **DEMONSTRATED** | **DEMONSTRATED** | The most *manufacturable* topological matter — it is lithography |
| **Polaritonic systems** | **DEMONSTRATED**, including room-temperature condensation in organic microcavities | **DEMONSTRATED** | Genuine room-temperature macroscopic coherence |
| **Dissipatively stabilized many-body states** | **DEMONSTRATED** for small systems (entangled steady states in ions and circuits; bit-flip-suppressed cat qubits) | n/a | Many-body dissipative *phases* at scale: **PLAUSIBLE, not demonstrated** |
| **Nonequilibrium / time-crystalline phases** | **DEMONSTRATED (2021)** on superconducting, trapped-ion, and NV platforms | n/a | A phase with no equilibrium counterpart, compiled by driving |
| **Excitonic insulator** | — | **CONTESTED** — signatures reported, interpretation disputed | |

---

### 9.3 The Quantum-Matter Intermediate Representation

#### 9.3.1 Audit of the proposed field list

The proposed `QuantumMatterSpec` mixes four categories that must be separated, and
one field that should not be an input at all.

| Proposed field | Verdict | Where it belongs |
|---|---|---|
| `symmetry` | **Keep** — a group (point, space, internal, time-reversal, particle-hole), well-defined and substrate-free | **L1 model** |
| `topology` | **Split — ambiguous.** Conflates (a) the topology of the lattice/manifold, (b) topological *order*, (c) *band* topology. These are different objects with different witnesses | (a) → L1 model; (b),(c) → **L0 acceptance region** as invariants |
| `dimensionality` | **Keep**, with care: spatial dimension *d*, plus effective dimensionality when anisotropy or confinement makes them differ | **L1 model** |
| `filling` | **Keep, conditionally** — meaningful for lattice fermion/spin models, undefined for continuum bosonic systems. Must be typed on mode class | **L1 model**, guarded |
| `interaction_graph` | **Keep** — core of the IR | **L1 model** |
| `coupling_tensor` | **Keep**, with a declared operator basis and sign/normalization convention. Without the convention the tensor is meaningless | **L1 model** |
| `band_structure` | **Remove as an input.** Band structure is a *consequence* of H, not an independent degree of freedom. Including it as a field permits specifications inconsistent with their own Hamiltonian | **DERIVE**, or re-express as a constraint on H |
| `order_parameter` | **Move** — this is a target property, not a model component. Placing it in the model layer repeats Draft 1's `f` category error | **L0 acceptance region**, with a measurement protocol |
| `correlation_length` | **Move** — a property | **L0 acceptance region** |
| `gap_requirement` | **Keep, but disambiguate**: Hamiltonian spectral gap, Liouvillian gap, single-particle vs many-body gap, and **at what system size**. An unqualified "gap" is UNDEFINED | **L0 acceptance region**, typed |
| `temperature` | **Move** — an environment parameter and a resource, not a specification | **Environment ℰ** |
| `field` | **Move** — a control/environment parameter | **Environment ℰ / control λ** |
| `dissipation` | **Keep** — the jump operators {L_k}, with their coupling rates | **L1 model** |
| `boundary_conditions` | **Keep, and emphasize** — topological sectors and ground-state degeneracy depend on boundary conditions and on the manifold's genus. Frequently omitted, and its omission makes topological specifications ill-posed | **L1 model** |
| `defect_tolerance` | **Move** — an acceptance criterion | **L0 acceptance region** |
| `lifetime` | **Move, and elevate** — essential, because most targets are metastable (§8.4, Prop. 8.8) | **L0 acceptance region**, mandatory for driven and metastable targets |
| `verification_witness` | **Remove as an input.** The witness set is an *output* of the compiler (§9.9). Hand-specifying it defeats design-for-verifiability. Admissible only as a *constraint* ("must be certifiable by non-destructive means") | **Compiler output**, or a constraint |

#### 9.3.2 The corrected IR

```
L0  SPECIFICATION  (substrate-free)
      observables       : [ (estimand, protocol, acceptance interval) ]
      invariants        : [ (name, expected integer value, witness class) ]
      order_parameters  : [ (definition, threshold, protocol) ]
      correlation_length: (range, at what T, extracted how)
      gap               : (which gap, value, at what size, how measured)
      lifetime          : (minimum, under what conditions)
      defect_tolerance  : (density bound, defect class)
      alternatives      : Alt(S)  -- the named hypotheses the witness set must exclude
      delta, beta       : false-accept / false-reject bounds
      scaling_hypothesis: sizes to be measured and the expected finite-size behaviour

L1  ABSTRACT MODEL  (substrate-free)
      mode_class        : spin-S | fermionic | bosonic | mixed
      symmetry_group    : (spatial, internal, antiunitary)
      dimensionality    : (d, effective d)
      lattice/graph     : interaction graph, unit cell, boundary conditions, genus
      coupling_tensor   : {J_ij, U, V, t, Δ, ...} in a DECLARED operator basis
      filling           : (typed on mode_class)
      dissipators       : {L_k} with rates
      drive             : (frequency, amplitude, protocol) for Floquet targets

L2  ENCODED MODEL  (substrate-class-specific)
      after: fermion→qubit mapping, locality reduction, Trotterization,
             universal-Hamiltonian encoding, bosonic truncation

L3  SUBSTRATE-NATIVE  (backend-specific)
      lattice depth, detunings, scattering length, tweezer coordinates,
      twist angle, circuit layout, cavity parameters

L4  CONTROL SIGNALS  (device-specific)
      pulse shapes, ramps, holograms, gate schedules
```

#### 9.3.3 The compilation pipeline

```
 S_Q ──► Q-IR(L0,L1) ──► H_eff ──► ℒ ──► physical controls ──► fabrication/preparation
                                                                        │
                                                                        ▼
                                                            verification ──► attestation
```

with the compile-time feedback edge of §7.7 closing back on L1 when models disagree.

> **PROPOSITION 9.4.** L0 and L1 are substrate-free; the L1→L2 passes are universal as
> *transformations* and non-universal as *costs*; L2→L3 is backend-specific by
> construction. **Classification: ESTABLISHED** — every named pass exists (see §40.3).
> The LLVM analogy holds architecturally and fails on cost uniformity: compiling a
> strongly correlated fermionic model onto a Rydberg array can be infeasible, not
> merely slow, chiefly because of perturbative-gadget coupling-ratio blowup.

---

### 9.4 Hamiltonian and Lindbladian synthesis — quantum control audit

The directive requires that no theorem be generalized beyond its assumptions. Each
claim below is stated with its scope.

#### 9.4.1 Spectral invariance

> **THEOREM 9.5 (scope-stated).** For a **closed** quantum system evolving under any
> time-dependent Hamiltonian H(t) — i.e. ρ(t) = U(t)ρ₀U(t)† with U unitary — the
> eigenvalue spectrum of ρ is invariant. Hence purity Tr ρ², von Neumann entropy, and
> every spectral function are conserved. **ESTABLISHED (elementary).**
>
> **Scope conditions, all necessary:** (i) closed system — no coupling to an
> unmodelled environment; (ii) the full state is tracked, not a reduced state of a
> larger system; (iii) no measurement; (iv) exact unitarity, not approximate.
>
> **What it does NOT say:** it does not constrain open-system evolution. Under a CPTP
> map purity may increase or decrease. Under a *selective* (post-selected) measurement
> the conditional state's purity typically increases; under a *non-selective*
> measurement the ensemble-averaged state typically becomes more mixed. With an
> ancilla and a global unitary, the reduced state of the system can be purified at the
> cost of entropy in the ancilla.

> **ENGINEERING CRITERION 9.6 (compile-time screen).** If the declared control set is
> purely unitary and spec(ρ₀) ≠ spec(ρ_T) for every ρ_T ∈ [S], return
> INFEASIBLE with a spectral certificate. The target then *types* as requiring an
> entropy-changing resource: a colder reservoir, engineered dissipation, measurement
> with feedback, or an ancilla with subsequent discard. **This is why entropy removal
> — not laser power or gate fidelity — is the binding constraint in cold-atom
> many-body preparation.**

#### 9.4.2 Reachability under the three control modes

| Mode | Reachable set from ρ₀ | Invariant |
|---|---|---|
| **Closed / unitary, LARC satisfied** | the full unitary orbit {UρU†} | spec(ρ) exactly |
| **Open / CPTP with unrestricted channels and ancillas** | *everything* — you can discard and re-prepare. Physically vacuous without resource restriction | none |
| **Open / CPTP with quasi-local bounded-weight generators** | the physically meaningful case; characterized only partially (see §9.6) | locality structure |
| **Measurement + feedforward** | conditionally purifying; enables constant-depth preparation of long-range-entangled states, beating the Lieb–Robinson depth floor, because classical communication is not velocity-limited | the classical record must be retained or erased at Landauer cost |

> **KNOWN LIMIT 9.7.** Any statement of the form "state X is reachable" is meaningless
> without declaring the admissible generator class and the resource budget. The
> unrestricted-CPTP answer ("everything") demonstrates that the content of a
> reachability claim lives entirely in the restriction.

#### 9.4.3 Thermomajorization — scope audit

> **THEOREM 9.8 (scope-stated).** For states **block-diagonal in the energy
> eigenbasis**, interconversion under **thermal operations** (energy-conserving global
> unitaries with a thermal bath at fixed temperature) is governed by
> **thermomajorization**, a partial order. **ESTABLISHED.**
>
> **Scope conditions:** (i) block-diagonal (zero coherence between energy
> eigenspaces); (ii) exact, single-shot, no error tolerance; (iii) thermal operations
> specifically — not arbitrary channels; (iv) no catalysts.
>
> **What it does NOT say:**
> - It is **not** a universal law of quantum-state reachability. It says nothing about
>   states with energy coherence, for which *additional* constraints from
>   time-translation covariance (the resource theory of asymmetry) apply and
>   thermomajorization is insufficient.
> - **With catalysts** the conditions change, yielding a *family* of generalized
>   second laws (Rényi-divergence conditions) rather than a single criterion.
> - It is a partial order, so most pairs of states are simply **incomparable** —
>   which means it usually yields no verdict at all.
> - It models an idealization with free access to arbitrary bath structures and says
>   nothing about **kinetics**, which is where feasibility is actually decided (§3.4).
>
> **Classification of the scoped theorem: ESTABLISHED. Classification of the unscoped
> folk version ("thermomajorization determines what quantum transformations are
> possible"): ILL-POSED.**

#### 9.4.4 Controllability

> **THEOREM 9.9 (scope-stated).** For a finite-dimensional bilinear control system
> ρ̇ = −i[H₀ + Σ_j u_j H_j, ρ] on a compact Lie group, the system is completely
> controllable iff the Lie algebra generated by {iH₀, iH_j} is the full algebra
> (LARC). **ESTABLISHED.**
>
> **What it does NOT give:** time-optimality, robustness, bounded control amplitude,
> any statement about open systems, or any statement about *how long* the required
> control sequence is. A system can satisfy LARC and still require exponentially long
> pulses.

#### 9.4.5 Quantum control landscapes and "trap-free" results

> **CONTESTED.** Results asserting that quantum control landscapes are generically
> free of local traps rest on assumptions that constrained real systems violate:
> (i) full controllability / surjectivity of the end-point map, (ii) **unconstrained**
> control amplitudes and bandwidth, (iii) a full-rank Jacobian of the end-point map.
> Under realistic amplitude, bandwidth, and duration constraints, traps have been
> reported, and the general claim is disputed in the literature.
> **Classification: CONTESTED. Do not rely on trap-freedom in a compiler design.**

#### 9.4.6 Algorithmic cooling

Worth stating because it is a clean instance of the scoping discipline: heat-bath
algorithmic cooling can concentrate purity into selected qubits, but entropy is
*redistributed*, not destroyed, and the achievable polarization is bounded — without a
bath, by a limit on unitary entropy shuffling; with a bath, by the bath's
polarization. **ESTABLISHED.** It is a legitimate entropy-changing resource and it
obeys the accounting of §9.4.1.

---

### 9.5 Quantum state preparation

**Theorem-grade obstructions**, each stated with scope:

| Obstruction | Statement | Scope |
|---|---|---|
| **Lieb–Robinson** | In a local Hamiltonian system, correlations spread at finite velocity; preparing a state with correlation length ξ from a product state by **local unitary** evolution needs time ≥ ξ/v. A long-range-entangled state cannot be prepared by a constant-depth local **unitary** circuit | Local Hamiltonians; unitary circuits only |
| **Kibble–Zurek** | Crossing a continuous transition at finite rate yields defect density n ~ τ_q^(−dν/(1+νz)) | Continuous transitions; quasi-adiabatic driving |
| **QMA-hardness** | Preparing ground states of *arbitrary* local Hamiltonians is at least as hard as the QMA-complete LOCAL HAMILTONIAN decision problem | Worst case, arbitrary instances, 1/poly promise gap |
| **Eastin–Knill** | No QEC code has a universal transversal gate set | Codes with a transversal gate group |
| **No-cloning** | The prepared state cannot be copied for multiple measurements | Always |

**Escapes**, with their scope:

| Escape | What it buys | Scope |
|---|---|---|
| **Measurement + feedforward** | Constant-depth preparation of long-range-entangled states, beating the Lieb–Robinson floor. **DEMONSTRATED** — D₄ non-Abelian order via adaptive circuit on 27 trapped-ion qubits | Requires fast, high-fidelity mid-circuit measurement and classical feedforward within the coherence window. **F-12 tests whether the asymptotic win survives real measurement latency** |
| **Dissipative preparation** | Gap-agnostic (governed by Δ_L not Δ); autonomous and self-correcting | Restricted to targets reachable by quasi-local dissipators (§9.6) |
| **Counterdiabatic / shortcuts** | Compressed adiabatic schedules | The exact counterdiabatic term is generically non-local and as hard as the original problem; local approximations work over limited ranges |
| **Route around the transition** | Avoid gap closure by taking a different path through a multidimensional phase diagram | A compiler routing problem — and precisely what GSCON formalizes (§8.23) |
| **Floquet engineering** | Effective Hamiltonians unavailable statically | Prethermal window only; heating sets a lifetime |

---

### 9.6 Dissipative stabilization

> **PROPOSITION 9.10 (scope of the dissipative strategy).** With **quasi-local jump
> operators of bounded weight**, the pure states preparable as unique steady states
> are essentially those that are unique ground states of **frustration-free** local
> parent Hamiltonians — the condition under which a local dissipative process can
> drive every local term to its minimum simultaneously. **Frustrated targets are not
> generally reachable this way.** With unrestricted jump operators the problem is
> trivial and physically vacuous.
> **Classification: ESTABLISHED for the frustration-free sufficiency; the exact
> characterization of the preparable class under bounded-weight dissipators is
> OPEN.** *SOURCE VERIFICATION REQUIRED for the precise necessary-and-sufficient
> statement.*

Advantages, stated without inflation: the target is an attractor, so preparation is
*autonomous* (no measurement, no feedback) and *self-correcting* (perturbations decay);
convergence is governed by Δ_L rather than Δ, which can be far more favorable.

Costs: Δ_L is itself hard to bound and can close; engineered dissipation consumes
control resources and injects its own noise; and the steady state is only as good as
the engineered jump operators' fidelity.

---

### 9.7 Quantum metrology

**STRONGLY SUPPORTED and deployed**, and the framework should use it at Stage 2 rather
than waiting for Use 1.

| Modality | Capability | Relevance |
|---|---|---|
| NV magnetometry / scanning NV | nanoscale magnetic-field mapping at room temperature | In-situ characterization of magnetic order, current distributions, spin textures |
| NV-based NMR | spectroscopy at nanoscale volumes | Chemical identification without bulk samples |
| Squeezed light | sub-shot-noise displacement sensing, deployed in gravitational-wave detectors | Precision dimensional metrology |
| Entanglement-enhanced clocks / spectroscopy | Heisenberg-limited frequency estimation | Frequency-based property witnesses |
| Quantum gas microscopy | site-resolved occupation and spin readout in optical lattices | **Direct verification of a compiled lattice model's state** — the most complete verification channel in any substrate |

---

### 9.8 Quantum verification

#### 9.8.1 The structure

Tomography is the wrong primitive: full state tomography of a *d*-dimensional state
to trace distance ε requires Θ(d²/ε²) copies — exponential in qubit number.
**ESTABLISHED.** Nobody specifies a state (§39.2); specifications name observables,
invariants, and responses.

| Specified object | Certification route | Copy complexity |
|---|---|---|
| **Topological invariant** | quantized transport; anyon interferometry; ground-state degeneracy | **Best case in physics** — integer estimand, O(1) margin; quantized Hall conductance is measured to parts in 10⁹ and defines the SI ohm |
| **Long-range entanglement** | topological entanglement entropy via Renyi-2 swap tests | polynomial, but expensive in practice |
| **Braiding statistics** | interferometric phase around an anyon world-line | demonstrated |
| **Order parameter** | structure factor, susceptibility, direct site-resolved imaging | polynomial |
| **Gap** | spectroscopy, response functions, dynamic structure factor | polynomial |
| **m specified local observables** | classical shadows | O(log m / ε²), independent of Hilbert-space dimension |
| **Fidelity to a structured target** | direct fidelity estimation; stabilizer/graph-state verification | polynomial for structured targets |
| **Symmetry indicators** | symmetry eigenvalues at high-symmetry points | very cheap when applicable |
| **Defect content** | scattering, imaging, transport signatures | cheap |
| The **full state** | tomography | **exponential — excluded** |

> **PROPOSITION 9.11 (Integer estimands).** When the specification names a
> topological invariant, the acceptance region is a neighborhood of an integer and the
> property map is quantized. The verification margin is therefore O(1) rather than
> O(tolerance), and sample complexity is set by readout noise alone, independent of
> system size. **Classification: ESTABLISHED.** *Consequence: exotic quantum phases can
> be cheaper to certify than an aerospace alloy, because integers are easier to measure
> than a first-percentile strength bound at 95% confidence.*

#### 9.8.2 The minimum sufficient measurement set

> **DEFINITION 9.12.**
> ```
> find W ⊆ ℳ minimizing  Σ_{w∈W} cost(w)·n_w
> s.t. for every a ∈ Alt(S), the predicted values of W under [S] and under a differ
>      by ≥ margin relative to the estimator's standard error, at joint confidence
>      1−δ over |W| tests, within the copy and destructiveness budget
> ```

#### 9.8.3 Submodularity audit — the claim Drafts 1–3 overstated

Drafts 1–3 asserted that this is "a test-cover problem: NP-hard, submodular, greedily
(1−1/e)-approximable." **That is correct for one formulation and incorrect for
another, and the two were conflated.**

> **PROPOSITION 9.13 (What is and is not submodular).**
>
> **(a) Coverage formulation — submodular. ESTABLISHED.** Define
> f(W) = |{ a ∈ Alt(S) : some w ∈ W separates [S] from a at the required margin }|.
> This is a set-coverage function: monotone and submodular. Greedy achieves
> (1−1/e) for max-coverage under a cardinality/budget constraint, and a ln|Alt|
> approximation for min-cost cover. **The guarantee is real for this formulation.**
>
> **(b) Information formulation — NOT submodular in general. The Draft 1–3 claim was
> wrong here.** Mutual information I(m ; y_W) as a function of the measurement set W
> is not submodular in general. Entropy is submodular; mutual information between a
> parameter and a *set* of observations is submodular under conditional-independence
> structure (observations conditionally independent given the parameter), and
> otherwise only *weakly* submodular, with a submodularity ratio γ ∈ (0,1] giving a
> (1−e^{−γ}) greedy guarantee that degrades as γ → 0.
>
> **Consequence for the compiler:** use the coverage formulation when a guarantee is
> required, and the information formulation when the objective genuinely is expected
> information gain — reporting in the latter case that the guarantee is conditional on
> a submodularity ratio that must be estimated, not assumed.
> *SOURCE VERIFICATION REQUIRED for the specific submodularity-ratio bounds cited.*

#### 9.8.4 The alternative-hypothesis discipline

> **KNOWN LIMIT 9.14.** A minimum sufficient measurement set is minimal **against the
> declared alternatives Alt(S)**. It says nothing about alternatives nobody listed.
> This is the mechanism behind essentially every retracted claim of a new phase: the
> witness set was sufficient against the alternatives the authors considered, and an
> unconsidered alternative explained the data.

> **ENGINEERING CRITERION 9.15.** Alt(S) is a first-class, adversarially reviewed
> artifact of the specification. The compiler should **refuse** a phase specification
> that does not carry one. "This is a spin liquid" is not a specification; "these
> observables lie in these regions, with this scaling, against these named
> alternatives — valence-bond solid, spin glass, magnetically ordered,
> disorder-dominated" is.

---

### 9.9 Physical Design-for-Verifiability

> **DEFINITION 9.16.** Physical Design-for-Verifiability (PDfV) is the joint synthesis
> of an artifact and its acceptance test:
> ```
> (π*, Ψ*) = argmin_{π,Ψ}  [ 𝔍(π) + λ_V · C_verify(Ψ) ]
> subject to  Pr[A(Y)=PASS ∧ Φ(σ_T) ∉ 𝒦] ≤ δ
> ```
> — selecting, among physically equivalent routes, the one whose outcome is cheapest
> to distinguish from its named failure modes.

#### 9.9.1 Examples, by substrate

| Substrate | Design move | Why it lowers C_verify |
|---|---|---|
| Pharmaceutical solids | Choose a polymorph with a distinctive PXRD signature over one near-isostructural with a likely impurity phase | Phase ID by a $200 non-destructive scan instead of destructive sectioning |
| Metals | Add a witness coupon with the same thermal history | Destructive testing moves off the part |
| Quantum matter | Specify a target whose phase is certified by a **quantized** response rather than a local order parameter | Integer estimand; O(1) margin (§9.11) |
| Quantum devices | Engineer a resonance whose frequency shifts measurably if a single dopant is misplaced | Converts intractable structural verification into a cheap functional measurement |
| Photonics | Include on-chip test structures and loop-back paths | Non-destructive electrical/optical probe |
| Semiconductors | Scribe-line test structures, embedded calibration features | In-line metrology without sacrificing die |
| Magnetic materials | Choose a composition whose target order has a distinct magnetic signature | Magnetometry instead of neutron scattering |
| Any | Embedded calibration standards co-processed with the artifact | Bounds instrument drift without a separate campaign |

#### 9.9.2 Novelty audit — prior art stated plainly

**This concept is NOT novel in general engineering.** Close and substantial prior art:

- **Design for testability (DFT) in VLSI** — scan chains and built-in self-test date
  to the 1960s–70s and are universal industrial practice. The core idea — accept a
  penalty in area/performance to obtain polynomial rather than exponential test-vector
  counts — is exactly PDfV, forty years earlier, in one substrate.
- **Design for inspection** in aerospace and pressure-vessel engineering: components
  are designed so that NDT can reach critical locations.
- **Design for metrology** in semiconductor manufacturing: scribe-line structures and
  metrology targets are standard.
- **Optimal experimental design** (Lindley; Chaloner–Verdinelli) and **active
  learning** supply the measurement-selection mathematics.
- **Design of experiments** and **acceptance sampling** supply the statistical
  decision theory.

> **PROPOSITION 9.17 (What is actually new).** Two things, and no more:
> **(i)** making verification cost an explicit term inside an *automated, cross-
> substrate synthesis objective*, so that the route search itself is verification-aware
> rather than verification being designed afterward by a different team; and
> **(ii)** applying the discipline to quantum matter, where the gap between "prepared"
> and "known to be prepared" is larger than anywhere else in physics and where no
> equivalent practice exists.
> **Classification: architectural synthesis with one plausibly new domain application.
> Not an invention.** Any patent strategy should be scoped accordingly (§24).

---

### 9.10 Platform feasibility

Order-of-magnitude platform bests from the published record. **These figures move on a
month-scale; re-check at time of use.**

| Platform | Operating T | T₂ (best, with DD) | 1Q / 2Q fidelity | Gate time | Scale demonstrated | Native family | Role |
|---|---|---|---|---|---|---|---|
| **Neutral atoms (tweezers/Rydberg)** | µK | ~1–13 s | ~99.98% / ~99.5% | 0.1–1 µs | **6,100 sites** (2025), ~23 min trap lifetime | Ising/XY with programmable geometry | Use 2; best scaling path for analog many-body |
| **Trapped ions** | µK ions | seconds to >1 h (clock qubits) | >99.9% / ~99.9% | µs–ms | 10–10² | long-range spin (tunable power law) | Use 2; highest fidelity; adaptive circuits |
| **Superconducting** | 10–20 mK | 0.1–0.5 ms | ~99.9% / 99.5–99.9% | 10–50 ns | 10²–10³; below-threshold surface-code memory demonstrated (d=7, 101 qubits, 0.143%±0.003% per cycle, Λ=2.14±0.02) | transmon lattice, tunable couplers, bosonic modes | Uses 1 and 2; strongest engineered dissipation |
| **Optical lattices** | nK | — | — | — | 10²–10³ sites with quantum gas microscopy | Bose/Fermi–Hubbard | Use 2; best verification channel |
| **Si spin qubits / donors** | 0.1–1.5 K | ms–s (²⁸Si) | >99.9% / ~99% | 10–100 ns | 10⁰–10¹ | Hubbard-like with real disorder | Use 2; semiconductor-fabricated |
| **NV centres** | **300 K** | 0.1–2 ms (e⁻); seconds (nuclear) | ~99.9% / ~99% | ns–µs | 1–10 per register | few-spin registers | **Uses 2 and 3**; the room-temperature resource |
| **Photonic** | **300 K** | loss-limited | high 1Q; probabilistic 2Q | ps–ns | 10¹–10² modes | Gaussian/bosonic | Uses 2 and 3; topological photonics is the most manufacturable quantum matter |
| **Moiré / solid-state** | 0.1–300 K | n/a | n/a | n/a | device-scale | **not software-programmable** — λ fixed at fabrication | Use 2; the bridge to bulk materials |
| **Molecular spins** | 4–300 K | µs (RT) – ms (low T) | research-grade | ns–µs | 1–few | chemically tunable | Use 2; far from application |
| **Topological (non-Abelian hardware qubits)** | mK | — | — | — | **0 demonstrated** | — | **SPECULATIVE** — distinct from engineered topological order, which is demonstrated |

---

### 9.11 Limits and open problems

> **KNOWN LIMIT 9.18.** Passive 2D topological quantum memory at finite temperature is
> **forbidden** — the 2D toric code is thermally unstable, its memory time not growing
> with system size. The 4D toric code is thermally stable. **Whether a self-correcting
> quantum memory exists in 3D is OPEN**; partially self-correcting constructions exist
> with sub-exponential lifetime growth. **ESTABLISHED for 2D and 4D; OPEN for 3D.**

> **KNOWN LIMIT 9.19.** Mermin–Wagner forbids spontaneous breaking of a continuous
> symmetry at finite T in d ≤ 2 with short-range interactions. **Scope:** the
> thermodynamic limit; finite-size systems — including every tweezer array — evade the
> asymptotics, and anisotropy or long-range interactions evade the hypothesis.

> **KNOWN LIMIT 9.20.** Lieb–Schultz–Mattis and its higher-dimensional descendants
> forbid a trivial gapped symmetric ground state at half-odd-integer spin per unit cell
> with translation invariance and SU(2) symmetry. **This is a compile-time static
> check**: such a specification is infeasible by theorem and should fail with a
> certificate, exactly as a conservation violation does.

> **KNOWN LIMIT 9.21.** Nielsen–Ninomiya forbids a single chiral fermion on a local
> Hermitian translation-invariant lattice.

> **OPEN PROBLEM 9.22.** Verifying a quantum simulator's output precisely in the
> regime where classical simulation fails. Partial answers — cross-platform
> comparison, randomized benchmarking, self-testing, invariant-based checks — none
> complete. This is the deepest verification problem in the framework.

> **OPEN PROBLEM 9.23.** The exact characterization of the pure states preparable as
> unique steady states of bounded-weight quasi-local Lindbladians (§9.6). Frustration-
> freeness is sufficient; necessity is unresolved.

> **OPEN PROBLEM 9.24.** Is there a general procedure for inverse Hamiltonian design
> outside the tensor-network parent-Hamiltonian construction and the universal-
> Hamiltonian encoding — i.e. for targets specified by *phase* rather than by *state*?

> **OPEN PROBLEM 9.25.** The submodularity ratio γ for realistic measurement-selection
> objectives in quantum verification: is it bounded away from zero for physically
> structured alternative sets? If not, greedy witness selection has no guarantee and
> §9.8.3's coverage formulation is the only defensible route.

---

## Final synthesis for Parts III–IV

> **A Physical Synthesis Compiler does not compute every microscopic trajectory
> required to construct an artifact.**
>
> **It searches for physically realizable dynamics, controls, landscapes, and
> measurement protocols under which the desired physical property emerges with
> bounded risk and verifiable evidence.**

Three outcomes, typed:

| Outcome | Condition | Output |
|---|---|---|
| **COMPILE** | A route exists within the declared substrate and resource envelope, with a measurement plan bounding false acceptance at δ | (π*, H*, ℒ*, Ψ*, Π) |
| **OBSTRUCT** | A defensible obstruction exists, from one of the eight certificate classes of §7.2.1 | an independently checkable certificate |
| **MEASURE** | Available knowledge is insufficient, and the expected value of information from the best admissible experiment exceeds the expected value of committing now | e*, its EVOI, its cost, and what it would resolve |

and the conceptual progression:

```
Specify → Model → Compile → Engineer Dynamics → Let Physics Evolve
        → Measure → Verify → Attest
```

**This is a research architecture, not a universal compiler.** Its validity is tested
by progressively harder physical compilation benchmarks, twelve of which are specified
in [`15-audits-parts-III-IV.md`](15-audits-parts-III-IV.md), together with the claim
audit table, the novelty audit, and the adversarial review.
