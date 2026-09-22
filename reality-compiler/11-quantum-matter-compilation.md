# Part XI — Quantum Matter as a Native Compilation Domain

Sections 29–31. **This part supersedes the treatment of quantum systems in Part IV,
which was wrong in scope.** Draft 1 treated quantum resources as *tools* for
compiling classical matter (compile-time electronic structure, run-time metrology).
That inverted the actual situation. Quantum matter is the domain where the
Reality Compiler thesis is *best posed and best demonstrated*, and Part IV's framing
suppressed that.

The modality vocabulary used throughout this part:

| Label | Meaning |
|---|---|
| **FORBIDDEN** | Contradicts a theorem, conservation law, causality, or a well-established thermodynamic bound. Cite the theorem. |
| **ALLOWED / INACCESSIBLE** | No known physical prohibition; present technology cannot execute it. Name the missing capability. |
| **DEMONSTRATED** | Realized experimentally under some conditions, whether or not scalable. |
| **PLAUSIBLE** | Supported by established theory, compatible with known constraints, no decisive demonstration. |
| **SPECULATIVE** | Requires unverified mechanisms, regimes, or scaling laws. |
| **UNSUPPORTED / CONTRADICTED** | No physical basis, or inconsistent with evidence. |

---

## 29. Quantum Matter Synthesis

### 29.1 The correction that reframes everything

Draft 1's §10.1 argued from throughput arithmetic that atomically precise assembly
cannot make bulk matter. That argument is sound *for bulk matter* and it is
**irrelevant to quantum matter**, for a reason I failed to state:

> **A quantum phase is not a quantity of substance. It is a pattern of correlations
> over a length scale. You do not need a mole of it. You need enough sites that the
> correlation length, the topological sector, and the relevant excitations are
> faithfully represented.**

A 6,100-site array of individually addressed atoms, each placed by an optical
tweezer, held at 13 s coherence with 99.98% single-qubit control and ~23 minute trap
lifetimes, **is a piece of programmable quantum matter that was assembled
atom-by-atom.** It was reported in 2025. The Avogadro objection does not touch it,
because nobody wants 10²³ sites — they want the thermodynamic limit's *physics*,
which finite-size scaling extracts from 10²–10⁴ sites.

This is the single most important correction in this response. **Atom-by-atom
positional assembly is already a working manufacturing technology — for the class of
objects where the useful quantity is correlation structure rather than mass.**
Label: **DEMONSTRATED.**

### 29.2 The second correction: the Avogadro argument overreached even for bulk

Draft 1 claimed serial assembly "is not a manufacturing technology for bulk matter
and never will be," labeled ESTABLISHED by arithmetic. The arithmetic is right; the
conclusion overreached, because it silently assumed serial operation and then
generalized to all positional assembly.

**Counter-existence-proof: the ribosome.** A ribosome is a programmable positional
assembler executing template-directed synthesis at ~5–20 residues per second with
error rates of ~10⁻³–10⁻⁴ per residue. An *E. coli* cell carries ~2×10⁴ of them; a
1,000 L fermenter at 10¹² cells/L holds ~10¹⁵ cells and therefore ~2×10¹⁹ ribosomes
operating in parallel, producing kilogram quantities of sequence-specified polymer
per day. A human body runs ~10¹⁸–10¹⁹ active ribosomes and turns over ~250–300 g of
sequence-specified protein daily.

So the correct statement is:

> **Massively parallel, programmable, positional molecular assembly at kilogram-per-day
> throughput is EXPERIMENTALLY DEMONSTRATED — by biology, continuously, at industrial
> scale.** What is not demonstrated is a *general-purpose* artificial version with an
> arbitrary chemistry palette.

The throughput objection is therefore not a bound on positional assembly. It is a
bound on *serial* positional assembly with macroscopic probes. The real questions
are: (a) can the parallelism factor of ~10¹⁹ be achieved artificially outside
ribosomal chemistry; (b) can the palette be widened beyond peptide and nucleotide
bonds. Both are **ALLOWED / INACCESSIBLE** — no theorem forbids either, and the
biological instance proves the parallelism regime is physically occupiable.

Constraints that *are* real, and should replace the overreach:
- **Attempt-frequency ceiling.** A positioned mechanochemical event cannot proceed
  faster than the relevant vibrational/reorganization timescale, ~10⁻¹³–10⁻¹² s, so
  ~10¹²–10¹³ events/s/site is a physics-grade ceiling. Ribosomes run 11 orders below
  it. This is a bound, and it is permissive.
- **Error-rate/throughput coupling.** Kinetic proofreading buys error suppression at
  the cost of energy and time (Hopfield, Ninio). Real, quantified, and already priced
  by biology at ~2 extra ATP per residue.
- **Heat removal** at 10¹⁹ parallel exothermic sites in a finite volume. Real, and
  solved biologically by working in water at low volumetric rates.

### 29.3 Why quantum matter is the *best*-posed compilation domain

Draft 1 gave four compilability criteria (§10.3). Score quantum matter against them
honestly, and it beats structural alloys:

| Criterion | Structural alloy | Programmable quantum matter |
|---|---|---|
| **Discrete composable action set with local, characterized effects** | Poor: process history is path-dependent, microstructure is emergent, effects are non-local | **Excellent**: lattice depth, detuning, tweezer position, twist angle, drive frequency, field strength — each maps to a term in H with a known coefficient |
| **Forward model accurate to ≪ tolerance, with known validity domain** | Poor: constitutive models fail at interfaces, cracks, free surfaces | **Excellent in principle**: the model *is* the Schrödinger/Lindblad equation. No constitutive guesswork. The difficulty is solving it, not knowing it |
| **Metrology with polynomial sample complexity** | Moderate | **Mixed, and excellent for the properties that matter most** — see §29.7 |
| **Stationary characterized yield statistics** | Good (mature processes) | **Excellent**: each shot is prepared fresh from a known initial state; no history dependence, no aging, no batch drift |

The third row is the only genuine weakness, and §29.7 shows it is far less severe
than Draft 1's tomography discussion implied.

**Conclusion: on three of four compilability criteria, quantum matter is a
*better* compilation target than the structural materials Draft 1 proposed as the
near-term bridge.** The thing that makes it feel harder — it needs cryostats,
lasers, vacuum — is an infrastructure cost, not a compilability deficit, and
infrastructure cost is exactly the category the directive warns against confusing
with physical limits.

### 29.4 The expanded compiler signature

```
Compile( S, Ω, ℰ )  →  ( P*, H*, U*, Ψ, Π )
```

with, for quantum targets, **H\* as the primary output**. The compiler's job is not
to compute a trajectory through Hilbert space; it is to find a *realizable
Hamiltonian and dissipator* whose dynamics carries the system into the acceptance
region, plus the protocol that gets there.

Formally, the target is stated on the Lindblad evolution

```
ρ̇ = −(i/ħ)[H(λ(t)), ρ] + Σ_k ( L_k(λ) ρ L_k(λ)† − ½{L_k(λ)†L_k(λ), ρ} )
```

where λ(t) ∈ Λ_Ω are the *physically available* control parameters — and the
acceptance condition, per Part I's reformulation, is on observables, not on states:

```
find  λ(·) ∈ Λ_Ω ,  T ≤ T_max
s.t.  Tr(ρ_T O_i) ∈ 𝒦_i   for i = 1..k,  at confidence 1−δ
      and, where the specification names a phase rather than a state,
      the invariant/order-parameter/response conditions of §29.6 hold.
```

Four sub-problems, with genuinely different characters:

| Sub-problem | Statement | Character |
|---|---|---|
| **Q1 Hamiltonian synthesis** | Find H ∈ realizable family whose ground state / phase / steady state has the specified property | Inverse problem over a *low-dimensional* parameter space (this is the key asymmetry — see §31.3) |
| **Q2 Preparation** | Drive ρ₀ → target sector | Adiabatic, dissipative, measurement-based, variational, Floquet; each with different complexity (§29.5) |
| **Q3 Stabilization** | Keep it there against decoherence | Engineered dissipation, gap protection, error correction, active feedback |
| **Q4 Certification** | Prove membership in the acceptance region | §29.7 |

Note that Q1 is often *easy* while Q2 is hard, which is the opposite of the classical
materials case where finding the composition is hard and making it is routine. That
inversion is worth stating because it means the compiler's effort budget allocates
differently.

### 29.5 Preparation: the real theorems, and the real escapes

This is where physics genuinely constrains, and where Draft 1's pessimism about
"landscape engineering" needs replacing with precise statements.

**Theorem-grade obstructions (FORBIDDEN, for the stated method):**

1. **Lieb–Robinson bound.** In a local Hamiltonian system, correlations spread at a
   finite velocity v. Preparing a state with correlation length ξ from a product
   state by *local unitary evolution* requires time ≥ ξ/v. Consequently a
   long-range-entangled state (topological order) **cannot** be prepared from a
   product state by a constant-depth local unitary circuit — this is essentially the
   definition of long-range entanglement.
   **Label: FORBIDDEN for that method.**

2. **Kibble–Zurek.** Crossing a continuous phase transition at finite rate produces
   a defect density n ~ τ_q^(−dν/(1+νz)). Adiabatic preparation *through* a critical
   point is impossible in finite time; the gap closes.
   **Label: FORBIDDEN to cross adiabatically; the scaling is a quantitative
   prediction, not a prohibition on reaching the phase.**

3. **Mermin–Wagner.** No spontaneous breaking of a continuous symmetry at finite
   temperature in d ≤ 2 with short-range interactions.
   **Label: FORBIDDEN** for that class of target in that dimension. Escapes exist and
   are legitimate: anisotropy (breaks the continuous symmetry), long-range
   interactions, finite-size systems (where the theorem's asymptotics do not bind —
   and a tweezer array *is* finite), or BKT-type quasi-long-range order.

4. **Lieb–Schultz–Mattis and its higher-dimensional descendants.** A spin system with
   half-odd-integer spin per unit cell, translation invariance, and SU(2) symmetry
   cannot have a trivial gapped symmetric ground state — it must be gapless, break a
   symmetry, or be topologically ordered.
   **Label: a FORBIDDEN region in specification space.** This is an example of
   something the compiler should check *statically*: a specification asking for a
   trivial gapped symmetric state at half filling per cell is infeasible by theorem,
   and should fail at compile time with a certificate, exactly as a conservation
   violation does.

5. **Nielsen–Ninomiya.** A local, Hermitian, translation-invariant lattice
   Hamiltonian cannot host a single Weyl/chiral fermion — they come in pairs.
   **Label: FORBIDDEN**; constrains lattice-model compilation targets directly.

6. **Thermal instability of 2D topological order.** The 2D toric code has no
   finite-temperature topological order: its memory time does not grow with system
   size (Alicki–Fannes–Horodecki and related). The 4D toric code *is* thermally
   stable (Dennis–Kitaev–Landahl–Preskill). Whether a true self-correcting quantum
   memory exists in 3D is **OPEN**; Haah's cubic code achieves partial
   ("marginal") self-correction with memory time growing but sub-exponentially.
   **Label: a room-temperature passive topological quantum memory in 2D is FORBIDDEN;
   in 3D it is OPEN; in 4D it is a theorem that does not help us.** This is a genuine
   physical wall and the framework should name it rather than route around it.

7. **Eastin–Knill.** No quantum error-correcting code has a universal transversal
   gate set. Constrains fault-tolerant architecture; magic-state distillation or code
   switching is forced.
   **Label: FORBIDDEN** (as stated).

8. **No-cloning.** You cannot copy the prepared state to measure it many ways.
   Verification must consume copies. This is why §29.7's structure matters.

**The escapes, which are real and which Draft 1 under-weighted:**

- **Measurement plus feedforward beats Lieb–Robinson.** Local measurements with
  classical feedforward can prepare long-range-entangled states — including the toric
  code and certain non-Abelian orders — in **constant depth**, because classical
  communication is not bounded by the Lieb–Robinson velocity. This is a genuine,
  theorem-backed, *asymptotic* separation between unitary and adaptive preparation,
  and it has been executed: D₄ non-Abelian topological order was created on 27 qubits
  of a trapped-ion processor via an adaptive circuit with per-site fidelity >98.4%,
  and anyon interferometry around Borromean rings detected an intrinsically
  non-Abelian braiding process. **DEMONSTRATED (2024).**
  This is the sharpest available instance of the framework's whole thesis: a
  declarative target (a topological order), compiled to a protocol (an adaptive
  circuit), executed, and *verified by a topological observable*.
- **Dissipative preparation is gap-agnostic.** Engineering Lindblad operators whose
  unique steady state is the target makes the target an *attractor*: convergence is
  governed by the Liouvillian gap, not the Hamiltonian gap, and the state is
  self-correcting without measurement. **DEMONSTRATED** for entangled steady states
  in ions and superconducting circuits; **DEMONSTRATED** for bit-flip suppression in
  dissipatively stabilized cat qubits.
- **Counterdiabatic / shortcut-to-adiabaticity driving** compresses the adiabatic
  schedule. The exact counterdiabatic term is generally nonlocal (and that
  nonlocality is the honest caveat from Draft 1), but local approximations work over
  useful ranges. **DEMONSTRATED** in several platforms.
- **Avoid the transition.** If the gap closes on the path, take another path. Phase
  diagrams are multidimensional; a target phase can often be reached without crossing
  a critical point by routing through an extra control axis. This is a *compiler
  routing problem* and it is exactly the kind of thing a compiler is good at.
- **Floquet engineering** synthesizes effective Hamiltonians unavailable statically —
  artificial gauge fields, topological band structures, sign-reversed couplings — at
  the price of heating, which sets a lifetime rather than a prohibition.
  **DEMONSTRATED** (synthetic magnetic fields and Harper–Hofstadter bands in cold
  atoms; Floquet-engineered topological bands).

### 29.6 Target taxonomy

Every target class the directive names, with the compilation specification, the
substrate that realizes it, and an honest modality label. "Specification" is what
the DSL would state; "control handle" is what the compiler actually varies.

| Target | Specification | Control handle (λ) | Substrate | Modality |
|---|---|---|---|---|
| **Engineered lattice model** (Hubbard, Ising, XY, Heisenberg, t–J) | H = specified operator with specified couplings | lattice depth, detuning, scattering length, tweezer geometry | optical lattices; Rydberg tweezer arrays; trapped ions; superconducting arrays | **DEMONSTRATED** — Bose- and Fermi-Hubbard realized; antiferromagnetic correlations in the Fermi-Hubbard model observed |
| **Ground-state manifold of a given H** | energy density, degeneracy, symmetry sector | adiabatic ramp / dissipative / variational | all of the above | **DEMONSTRATED** for gapped, small systems; **ALLOWED / INACCESSIBLE** at large N with small gaps (QMA-hardness bounds general certification, not particular instances) |
| **Symmetry-broken phase** (magnetic order, CDW, nematic) | order parameter ≥ threshold, correlation length | temperature, doping, field, strain | cold atoms; solids | **DEMONSTRATED**; constrained by Mermin–Wagner in d≤2 for continuous symmetries |
| **Topological order (Abelian)** — toric code, Z₂ | topological entanglement entropy; anyon braiding phase; ground-state degeneracy on a torus | adaptive circuit; Rydberg blockade geometry | superconducting processors; Rydberg arrays; trapped ions | **DEMONSTRATED** — toric-code ground states prepared and verified on superconducting hardware; spin-liquid signatures on Rydberg arrays |
| **Non-Abelian topological order** | braiding matrix is non-commuting; fusion rules | adaptive circuits with measurement + feedforward | trapped ions (D₄ on 27 qubits); superconducting (Fibonacci string-net) | **DEMONSTRATED (2023–2024)** as engineered topological order with braiding verified. **Distinct from** a *material* hosting non-Abelian anyons intrinsically, which remains **PLAUSIBLE/contested** |
| **Topological invariant** (Chern number, ℤ₂ index, winding) | integer value | band-structure engineering; twist angle; field; Floquet drive | moiré materials; cold atoms; photonic lattices | **DEMONSTRATED**; invariants measured directly |
| **Fractionalized phases / FQAH** | fractional Hall conductance at zero field | twist angle, filling, displacement field | twisted bilayer MoTe₂, twisted WSe₂ | **DEMONSTRATED (2023)** — fractional quantum anomalous Hall states at ν = −2/3, −3/5 at zero magnetic field |
| **Quantum Hall / IQHE** | σ_xy = ν e²/h to 10⁻⁹ | field, density, mobility | 2DEGs, graphene | **DEMONSTRATED**; underpins the SI ohm |
| **Quantum spin liquid** | no order down to T→0; fractionalized excitations; topological entanglement entropy | frustrated lattice geometry, exchange ratios | candidate materials; Rydberg arrays; Kitaev materials | **In materials: SPECULATIVE/contested** — no confirmed material. **On engineered arrays: DEMONSTRATED at the level of signatures** |
| **Frustrated magnetism** | specified exchange ratios on a specified lattice | tweezer geometry; chemistry | Rydberg arrays; pyrochlores, kagome compounds | **DEMONSTRATED** (arrays); materials route mature but not designable |
| **Superconducting phase** | Tc ≥ T₀, gap symmetry, critical current/field | composition, pressure, doping, strain, interface | cuprates, hydrides, interfaces | **DEMONSTRATED** to ~203 K (H₃S, 155 GPa) and ~250 K (LaH₁₀, 170 GPa), with active disputes over some magnetization analyses. **Ambient-pressure room-T: UNSUPPORTED** (three high-profile retractions). **Critically: there is no known theorem bounding Tc** — so ambient room-T superconductivity is **ALLOWED / undiscovered**, not forbidden |
| **Correlated-electron states** (Mott, heavy fermion, strange metal) | resistivity scaling, spectral weight, effective mass | doping, U/t via lattice depth or twist | cold atoms; moiré; oxides | **DEMONSTRATED**; predictive design **ALLOWED / INACCESSIBLE** (many-body problem) |
| **Moiré systems** | twist angle θ ± 0.05°, filling, displacement field | mechanical stacking; gating | 2D heterostructures | **DEMONSTRATED** — twist angle is a genuinely *compilable continuous parameter* selecting among correlated, superconducting, and topological phases. This is the closest thing in solid-state to a tunable Hamiltonian knob |
| **Artificial gauge fields** | synthetic flux per plaquette | Floquet drive; Raman coupling; strain | cold atoms; photonics; strained graphene | **DEMONSTRATED** — including strain-induced pseudo-magnetic fields exceeding 300 T, far beyond any laboratory magnet. A clean case where *engineering the effective theory* beats brute force by orders of magnitude |
| **Spin textures** (skyrmions, merons) | topological charge, size, density | DMI via interface, field, temperature | multilayers, B20 magnets | **DEMONSTRATED**; interface engineering is the compiler handle |
| **Orbital ordering** | orbital occupancy pattern | strain, epitaxy, pressure | oxides | **DEMONSTRATED**; designability **PLAUSIBLE** |
| **Excitonic phases / excitonic insulator** | condensate order parameter, gap | doping, layer separation, temperature | Ta₂NiSe₅, TiSe₂, bilayer 2D | **Contested / PLAUSIBLE** — signatures reported, interpretation disputed |
| **Polaritonic states / condensates** | condensate fraction, coherence length, at T | cavity Q, detuning, material oscillator strength | organic microcavities, GaN | **DEMONSTRATED at room temperature** — this is genuine room-temperature macroscopic quantum coherence (§29.8) |
| **Photonic/phononic topological matter** | edge-mode spectrum, invariant | lattice geometry, gyrotropy, drive | photonic crystals; metamaterials | **DEMONSTRATED**; and the most *manufacturable* topological matter, since it is lithography |
| **Quantum metamaterials** | effective ε, μ, dispersion, nonlinearity | unit-cell geometry | superconducting arrays; photonic | **DEMONSTRATED** |
| **Nonequilibrium phases / time crystals** | subharmonic response rigid to perturbation | drive amplitude/frequency, disorder | superconducting processors; trapped ions; NV ensembles | **DEMONSTRATED (2021)** — discrete time-crystalline order observed on multiple platforms. Note: a *phase of matter that does not exist in equilibrium*, compiled by driving. Strong support for the directive's thesis |
| **Dissipatively stabilized states** | steady state of a designed Liouvillian | engineered reservoir coupling | ions; superconducting circuits | **DEMONSTRATED** |
| **Metastable quantum phases** | lifetime ≥ τ, property X | quench protocol, photoexcitation | photoinduced phases in solids | **DEMONSTRATED** (photoinduced hidden phases, light-induced ferroelectricity); light-induced superconductivity signatures **contested** |
| **Defect-engineered quantum states** (NV, SiV, divacancy, single dopants) | defect species, site, orientation, charge state, count | implantation, annealing, STM lithography | diamond, SiC, silicon | **DEMONSTRATED** — deterministic single-dopant placement achieved; this is the flagship Stage-7 target from Draft 1 and it belongs here |
| **Atomically precise interfaces** | layer sequence, termination, sub-ML control | MBE/ALD/PLD with in-situ RHEED | oxide and semiconductor heterostructures | **DEMONSTRATED** — monolayer-precision heterostructures are routine industrial capability |
| **Quantum-confined structures** | confinement energy, level spacing, g-factor | dot size, composition, gating | quantum dots, nanowires | **DEMONSTRATED** |
| **Programmable atomic arrays** | site occupancy pattern, geometry, connectivity graph | tweezer holograms | neutral atoms | **DEMONSTRATED at 6,100 sites** with 13 s coherence |
| **Ultracold-atom matter** | temperature/degeneracy, statistics, interaction strength | evaporative cooling, Feshbach resonance | BEC/DFG | **DEMONSTRATED**; including in orbital microgravity |
| **Josephson / superconducting circuit architectures** | circuit Hamiltonian (E_J, E_C, couplings) | lithographic geometry | superconducting foundries | **DEMONSTRATED** — and note this is literally *lithographic compilation of a Hamiltonian*: you draw a layout, and the Hamiltonian you specified is what you get. The most mature quantum-matter compiler in existence |
| **Majorana-like regimes** | zero-bias peak with the right quantization, non-local correlations, braiding | nanowire/proximity architecture, field, gating | semiconductor–superconductor hybrids | **Hardware qubits: SPECULATIVE** — poor replication record including a retraction. Distinguish sharply from the engineered non-Abelian order above, which *is* demonstrated |

### 29.7 Verification of quantum matter — better than Draft 1 implied

Draft 1 emphasized that full tomography costs Θ(d²/ε²) copies and is hopeless. True,
and beside the point, because nobody specifies a state. Applying Part V's own
reformulation to quantum matter gives a surprisingly favorable picture:

| What is specified | How it is certified | Sample complexity |
|---|---|---|
| **A topological invariant** | quantized transport; anyon interferometry; ground-state degeneracy | **The best verification problem in physics.** The estimand is an *integer*: you need only resolve to ±0.5, so the margin is enormous. Quantized Hall conductance is measured to parts in 10⁹ and defines the SI ohm |
| **Long-range entanglement** | topological entanglement entropy via Renyi-2 (swap test); constant-depth circuits with measurement | Polynomial; demonstrated on hardware |
| **Braiding statistics** | interferometric phase around an anyon world-line | Demonstrated on trapped-ion and superconducting processors |
| **An order parameter** | structure factor, susceptibility, direct imaging | Polynomial; quantum gas microscopes image site-resolved occupation directly |
| **A gap** | spectroscopy, response functions, dynamic structure factor | Polynomial |
| **k specified observables** | classical shadows | O(log k / ε²) — independent of Hilbert-space dimension |
| **Fidelity to a target state** | direct fidelity estimation; for stabilizer/graph targets, efficient verification protocols | Polynomial for structured targets |
| **The full state** | tomography | Θ(d²/ε²) — **do not do this** |

**Proposition 29.1 (Topological targets are cheap to certify).** When the
specification names a topological invariant, the acceptance region 𝒦 is a
neighborhood of an integer and the property map is quantized. The verification
margin is therefore O(1) rather than O(tolerance), and sample complexity is set by
readout noise alone, independent of system size.

This inverts a natural intuition and is worth stating loudly: **exotic quantum
phases can be easier to certify than an aerospace alloy**, because integers are
easier to measure than 400 MPa at the first percentile with 95% confidence. It also
means the Draft 1 objective's λ_V term (verification cost) *favors* topological
specifications over local-order-parameter ones — a concrete, non-obvious design
consequence of compile-for-verifiability.

### 29.8 Re-labeling the macroscopic-coherence claim

Draft 1 §9.2 labeled "room-temperature macroscopic quantum coherence" as
**CONTRADICTED**. That was sloppy and conflated distinct claims. Corrected:

| Claim | Correct label | Basis |
|---|---|---|
| Macroscopic quantum coherence (off-diagonal long-range order) at room temperature | **DEMONSTRATED** | Organic polariton condensates at 300 K; superconductivity is itself macroscopic phase coherence, at 203 K under pressure |
| Superposition of macroscopically distinct states of a massive object at 300 K, strongly coupled to a thermal environment | **ALLOWED / INACCESSIBLE** | No theorem forbids it. Decoherence theory gives *rates*, not prohibitions, and the rate depends on environmental coupling — an engineering parameter. The frontier is moving: quantum interference has now been demonstrated with sodium nanoparticles of >7,000 atoms and >170 kDa, and levitated nanospheres of ~10⁸ amu have been cooled to the motional ground state |
| A many-qubit, long-coherence quantum processor embedded in a bulk ambient-condition manufactured article | **ALLOWED / INACCESSIBLE** | Requires isolation engineering far beyond current capability; no prohibition |
| Warm, wet biological systems performing useful quantum *computation* | **CONTRADICTED** | Photosynthetic coherences are vibrational, ~10²fs at physiological T. Radical-pair magnetoreception remains a live *sensing* hypothesis |

The general principle the directive demands, stated explicitly:

> **Decoherence is a rate, not a law.** τ_dec ~ (coupling)⁻² × (environmental spectral
> density)⁻¹. Every term in it is an engineering variable. Cryogenics, vacuum,
> isotopic purification, dynamical decoupling, decoherence-free subspaces, and
> engineered dissipation have each bought orders of magnitude. Nothing in quantum
> mechanics fixes the ceiling. Treating today's τ as a constant of nature is the
> exact error the directive names.

### 29.9 The 12-point falsification, applied

As required, for three flagship quantum-matter targets.

---

#### Target A — Programmable Fermi–Hubbard ground state at specified doping, N ≥ 10³ sites

| # | Constraint | Assessment |
|---|---|---|
| 1 Conservation | Particle number, energy, momentum conserved; no issue |
| 2 Thermodynamic | Entropy per particle must be driven below ~0.3 k_B for antiferromagnetic order. **This is the binding constraint.** Entropy removal in a lattice is the hard part, not cooling power |
| 3 Quantum-mechanical | Gap scales as the superexchange J = 4t²/U, which is small; states are fragile to heating |
| 4 Causality/locality | Lieb–Robinson sets preparation depth ≥ ξ/v |
| 5 Controllability | Excellent — lattice depth and Feshbach tuning give direct access to t and U |
| 6 Decoherence | Photon scattering from lattice beams; technical heating. Sets a lifetime, not a prohibition |
| 7 Preparation complexity | Adiabatic through the Mott transition; entropy redistribution schemes; **the open problem is entropy engineering, not control** |
| 8 Measurement complexity | Excellent: quantum gas microscopy gives site-resolved occupation and spin |
| 9 Scaling | Currently 10²–10³ sites with the required entropy; 10⁴ **ALLOWED / INACCESSIBLE** |
| 10 Resources | Laser power, vacuum, magnetic-field stability — all conventional |
| 11 Unknowns | Whether the d-wave superconducting phase exists in the pure Hubbard model at all — this is *the* open question the experiment is meant to answer |
| 12 Falsifiable prediction | At the target entropy, the spin structure factor peaks at (π,π) with amplitude ≥ X; pairing correlations exceed non-interacting baseline by ≥ Y at doping p. **Failure of both at achieved entropy would falsify the claim that this substrate can settle the Hubbard question** |

**Verdict: ALLOWED, partially DEMONSTRATED, entropy-limited.** The limit is
thermodynamic and quantified, not a fabrication limit.

---

#### Target B — Non-Abelian topological order with braiding-verified statistics, on demand

| # | Constraint | Assessment |
|---|---|---|
| 1 Conservation | None violated |
| 2 Thermodynamic | **2D topological order has no thermal stability (Alicki–Fannes–Horodecki).** Any finite-T 2D realization is metastable and must be actively maintained. This is a real theorem and a real ceiling |
| 3 Quantum-mechanical | Long-range entanglement required; cannot arise from constant-depth unitary circuit |
| 4 Causality/locality | Circumvented by measurement + feedforward: classical communication is not Lieb–Robinson-limited |
| 5 Controllability | Demonstrated at 27 qubits |
| 6 Decoherence | Sets the window in which braiding and interferometry must complete |
| 7 Preparation complexity | **Constant depth with adaptive circuits** — a genuine asymptotic win |
| 8 Measurement complexity | Interferometric; polynomial; demonstrated |
| 9 Scaling | Qubit count and mid-circuit measurement fidelity |
| 10 Resources | Conventional quantum-processor infrastructure |
| 11 Unknowns | Whether a *material* hosting these phases intrinsically at useful temperature exists |
| 12 Falsifiable prediction | Braiding two anyons yields a non-commuting unitary on the degenerate subspace, detectable as a specific interference pattern; the pattern must be robust to local perturbations and must scale correctly with code distance |

**Verdict: DEMONSTRATED as engineered order (2023–2024). Passive thermal stability in
2D: FORBIDDEN. 3D self-correction: OPEN.**

---

#### Target C — Ambient-pressure room-temperature superconductor

| # | Constraint | Assessment |
|---|---|---|
| 1 Conservation | None violated |
| 2 Thermodynamic | None violated. Tc is set by pairing strength and phase stiffness |
| 3 Quantum-mechanical | **No known theorem bounds Tc.** Early "limits" from McMillan-type phonon arguments were empirical and were broken by the hydrides |
| 4 Causality/locality | No constraint |
| 5 Controllability | The search space (composition × structure × pressure × strain × interface) is enormous and only weakly navigable |
| 6 Decoherence | Not applicable — this is a thermodynamic phase, not a coherent superposition |
| 7 Preparation complexity | Synthesis of metastable hydrides at ambient pressure: the central materials challenge. Chemical pre-compression is the leading idea |
| 8 Measurement complexity | **This is where the field has failed.** Three retractions traceable to inadequate and non-reproducible characterization. Verification protocol — Meissner expulsion, zero resistance, specific-heat jump, isotope effect, all on the same sample with raw data published — is the actual bottleneck |
| 9 Scaling | Diamond-anvil samples are ~10⁻⁴ mm³; bulk synthesis of a recovered metastable phase is unsolved |
| 10 Resources | Megabar pressure is cheap; sample volume is the cost |
| 11 Unknowns | Whether a metastable hydride can be recovered to ambient pressure at all |
| 12 Falsifiable prediction | Specified Tc with a specific-heat anomaly at Tc, full Meissner fraction, and a predicted hydrogen-isotope shift Tc(H)/Tc(D) ≈ √2 for phonon-mediated pairing. **Absence of the isotope shift falsifies the mechanism; absence of the specific-heat jump falsifies the transition** |

**Verdict: ALLOWED, undiscovered. The binding constraints are search-space size and
*verification integrity* — not physics.** This is the cleanest case in the document
where Draft 1's instinct (verification is the centre of gravity) and the directive's
instinct (don't mistake difficulty for prohibition) agree completely.

---

## 30. Independence of the Specification Layers

The directive asks which layers of

```
geometry → microstructure → atomic configuration → electronic structure
         → quantum state → collective phase
```

can be specified independently, which are coupled, and which can be compiled
jointly. This is answerable, and the answer is structurally informative.

| Layer pair | Coupling | Can they be specified independently? |
|---|---|---|
| geometry ↔ microstructure | Strong in processing (cooling rate follows geometry), weak in principle | **Partially.** In AM they are coupled through thermal history; in wrought processing they decouple |
| microstructure ↔ atomic configuration | Definitional (microstructure *is* coarse-grained atomic configuration) | **No** — one determines the other upward, not downward |
| atomic configuration ↔ electronic structure | Deterministic (Born–Oppenheimer): configuration fixes electronic structure | **No.** Electronic structure is not a free variable given the nuclei — it is a *consequence*. **This is a hard coupling and specifications that treat them independently are ill-formed** |
| electronic structure ↔ quantum many-body state | Strong; the state is a functional of H | **No**, except through temperature, driving, and preparation history — which is exactly the *nonequilibrium* freedom |
| quantum state ↔ collective phase | The phase is an equivalence class of states | **No** downward; **yes** in the sense that many states realize the same phase — this is the spec-cell degeneracy of Part I §4.4, and it is the resource |

**The structurally important finding:** the layers are *stacked deterministically
upward* and *degenerate downward*. This means:

1. **Compilation is inherently an inverse problem at every layer boundary**, and
   inverse problems are one-to-many. The compiler's output is always a set.
2. **The only places where genuinely independent control exists are the
   *nonequilibrium* handles**: temperature, driving, preparation history, and
   dissipation. Those handles do not follow from the atomic configuration. They are
   where the extra design freedom lives.
3. Consequently **joint compilation across layers is possible precisely where a
   nonequilibrium handle spans them** — e.g. Floquet driving spans electronic
   structure and collective phase; strain spans atomic configuration and electronic
   structure; twist angle spans geometry and electronic structure directly.

That last observation gives a concrete search heuristic: **look for control
parameters that reach across layer boundaries.** Twist angle, strain, drive
frequency, and dissipation are the known ones, and they are precisely the parameters
that have produced the last decade's surprises.

---

## 31. The Compression Argument

### 31.1 Why a specification can be shorter than its product

The framework's deepest claim is that a short specification can produce a large
structure. Self-assembly proved it discretely (Θ(log N / log log N) tile types for an
N×N square). The quantum-matter case proves it continuously and far more
dramatically.

Define the **compilation compression ratio**

```
κ(S) =  K( H* )  /  K( target state )
```

where K is description length (Kolmogorov complexity, in practice: the length of the
shortest parameter list that specifies the object in the declared representation).

- **Crystal from a Hamiltonian:** K(H*) is a few dozen numbers (composition, lattice
  parameters, exchange couplings). K(state) for 10²³ atoms is astronomical.
  κ ~ 10⁻²² . This is why crystallization works and why nobody places atoms in a salt
  crystal.
- **Toric code on N qubits:** K(H*) = the local stabilizer terms, O(1) to specify by
  a rule. K(state) is exponential to write down. κ → 0.
- **A random disordered structure with no generating rule:** κ ~ 1. There is no
  compression, and therefore **no compiler leverage** — you must specify everything.

> **Proposition 31.1 (Compressibility predicts compilability).** A target is
> compilable with leverage if and only if it admits a generating Hamiltonian (or
> local rule set) whose description is much shorter than the target's own. Targets
> with κ ≈ 1 admit no landscape compilation and must be built by explicit placement,
> which is where the throughput arithmetic of §29.2 bites.

This is, I believe, the correct general statement of when "controlled emergence"
works, it subsumes the self-assembly tile-complexity result and the
Hamiltonian-engineering case under one principle, and it is testable: **κ should
predict, across substrates, which specifications compile cheaply.** It is added to
the falsification program as F-14 (§35).

### 31.2 It also explains the framework's failures

κ ≈ 1 targets — an arbitrary CAD part with 10⁶ independent feature tolerances, a
specific protein sequence, a bespoke circuit layout — get no emergence leverage, and
correctly so. They are built by instruction execution, and their cost scales with
their description length. That is not a failure of the paradigm; it is the paradigm
correctly reporting that there is nothing to compress.

### 31.3 The asymmetry that makes quantum matter tractable

Returning to §29.4's observation that Q1 (Hamiltonian synthesis) is often easier than
Q2 (preparation): the reason is now clear. The realizable Hamiltonian family
Λ_Ω is *low-dimensional* — an optical lattice has perhaps ten independent knobs, a
moiré stack has three or four, a superconducting circuit has a lithographic layout
but a small number of physically distinct coupling types. Searching a ten-dimensional
space for a phase boundary is easy. Preparing the state once you know where to go is
the hard part.

**This is the inverse of the classical materials problem**, where the composition ×
process space is enormous and the making is routine. The compiler architecture should
reflect that inversion: for classical targets, spend the budget on search; for
quantum targets, spend it on preparation protocol synthesis and on entropy/decoherence
management.
