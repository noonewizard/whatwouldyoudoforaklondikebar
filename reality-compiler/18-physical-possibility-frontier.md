# The Physical Possibility Frontier

Section 18. The boundary between **physically forbidden** and **physically possible
but technologically unrealized**, with every obstacle classified.

---

## 18.1 The obstruction taxonomy

Eighteen categories, never collapsed. Categories **1–6** are boundaries physics or
mathematics imposes; **7–11** are boundaries the *substrate* imposes and which change
with substrate choice; **12–17** are boundaries *we* impose and which move with time
and money; **18** is honesty.

| # | Category | Moves with technology? | Test for membership |
|---|---|---|---|
| 1 | Mathematical impossibility | **No** | A theorem, with its hypotheses checked against the case |
| 2 | Fundamental physical law | **No** | A law of the Standard Model, GR, or QM |
| 3 | Conservation constraint | **No** | A conserved charge is violated |
| 4 | Thermodynamic constraint | **No** | Second law, or a resource-theory monotone |
| 5 | Information-theoretic constraint | **No** | A counting, Holevo, or no-cloning argument |
| 6 | Quantum-mechanical constraint | **No** | A structural theorem of QM (no-go, uncertainty, LR bound) |
| 7 | Controllability limitation | Substrate-dependent | The target lies outside Reach(Λ_Ω) |
| 8 | Observability limitation | Substrate-dependent | No element of ℳ separates target from Alt(S) |
| 9 | Dynamical instability | Substrate-dependent | The target is not an invariant or metastable set |
| 10 | Kinetic limitation | Substrate-dependent | Barrier height / rate at accessible conditions |
| 11 | Metastability / nucleation barrier | Substrate-dependent | Nucleation rate below the required throughput |
| 12 | Fabrication limitation | **Yes** | No process exists; none is forbidden |
| 13 | Materials limitation | **Yes** | No material with the needed property is known |
| 14 | Measurement limitation | **Yes** | No instrument reaches the needed precision |
| 15 | Computational limitation | **Yes** | No algorithm/compute reaches the needed accuracy |
| 16 | Current instrumentation | **Yes** | Equipment does not exist yet |
| 17 | Economic limitation | **Yes** | Price, not physics |
| 18 | Unknown | — | We do not know which of 1–17 applies |

> **Discipline.** A claim of impossibility must cite a category 1–6 obstruction *with
> its hypotheses verified against the case at hand*. A category 12–17 obstruction is
> a cost curve, and stating it as impossibility is an error. Category 18 is a
> legitimate and common answer, and saying "unknown" is not a failure.

---

## 18.2 The ultimate ceilings

Before the frontier table: what bounds the *maximally ambitious* compiler, when every
engineering limit is removed? Four ceilings, all astronomically far from practice —
which is the substantive finding.

> **KNOWN LIMIT 18.1 (Specification density — Bekenstein).** The information that can
> be held in a region of radius R enclosing energy E is bounded by
> S ≤ 2πkER/(ħc). For 1 kg in a 10 cm sphere this is ~10⁴² bits.
> **Category 2.** *Practical specifications are ~10¹⁰ bits. The ceiling is 32 orders
> above.*

> **KNOWN LIMIT 18.2 (Computation rate — Margolus–Levitin / Lloyd).** A system of
> energy E performs at most 2E/(πħ) orthogonal state changes per second: ~10⁵⁰ s⁻¹
> for 1 kg. **Category 2/6.** *If the substrate computes its own dynamics, this is the
> ceiling on "physics does the work," and it is ~35 orders above any digital
> simulator.*

> **KNOWN LIMIT 18.3 (Selection cost — Landauer plus configurational entropy).**
> Imposing I(S) bits irreversibly costs ≥ k_BT ln2 · I(S). **Category 4.**
> *For 1 kg of matter pinned to one of ten local alternatives per atom: ~0.1 MJ/kg,
> two to six orders below real process energies (§15.3).*

> **KNOWN LIMIT 18.4 (Control propagation — causality and Lieb–Robinson).** Control
> information propagates at ≤ c classically, and correlations at ≤ v_LR in a local
> quantum system. A specification with correlation length ξ cannot be imposed by local
> unitary means in time below ξ/v_LR. **Category 2/6.** *Circumvented for preparation
> by measurement + classical feedforward, which is limited by c, not v_LR.*

**Reading.** All four ceilings are enormously permissive. **Nothing in the ultimate
physics of information forbids a compiler of essentially unlimited ambition.** The
binding constraints are all in categories 7–17, and that is the single most important
result of this section.

---

## 18.3 Theorems that genuinely block otherwise attractive ideas

The directive asks that a fundamental theorem blocking a powerful idea be treated as
an equally valuable finding. Eleven such, with scope.

| Theorem | Blocks | Scope / escape |
|---|---|---|
| **Kramers–Kronig / causality** | Arbitrary dispersion engineering. Low loss *and* strong dispersion over a broad band is forbidden in any causal passive medium. Broadband passive invisibility cloaking is forbidden: integrated over all wavelengths, total scattering of a linear, passive, causal, non-diamagnetic cloak **necessarily increases** relative to the uncloaked object (Monticone–Alù, PRX 2013) | **Category 2.** Escapes: narrowband operation; active (non-passive) media, which pay energy; gain media, which pay noise |
| **Wannier obstruction** | Exponentially localized Wannier functions do not exist for bands with nonzero Chern number | **Category 1.** Blocks "compile a topological band structure in a localized orbital basis." Escape: use a non-localized or hybrid basis; accept power-law tails |
| **Nielsen–Ninomiya** | A single chiral fermion on a local Hermitian translation-invariant lattice | **Category 1/6.** Escapes: domain-wall fermions, which put the partner on the other wall |
| **Lieb–Schultz–Mattis** (+ higher-d) | A trivial gapped symmetric ground state at half-odd-integer spin per unit cell with translation and SU(2) | **Category 1.** A compile-time static check: such a specification is infeasible by theorem |
| **Mermin–Wagner** | Continuous-symmetry breaking at T>0 in d ≤ 2 with short-range interactions | **Category 2.** Escapes: anisotropy; long-range interactions; **finite size** — every tweezer array evades the asymptotics |
| **Alicki–Fannes–Horodecki** | Passive 2D topological quantum memory at finite T | **Category 4/6.** 4D toric code is stable; **3D is OPEN**; active correction works in 2D |
| **Eastin–Knill** | A universal transversal gate set in any QEC code | **Category 1.** Escape: magic-state distillation, code switching |
| **No-cloning / no-broadcasting** | Copying the prepared state for repeated measurement | **Category 6.** Verification consumes copies; drives the whole shadow/invariant strategy |
| **Holevo bound** | Extracting more than n classical bits from n qubits | **Category 5.** Caps what any measurement plan can learn per copy |
| **Stability of matter (Dyson–Lenard, Lieb–Thirring)** | Arbitrarily dense ordinary matter; energy per particle is bounded below | **Category 2.** Sets the density ceiling for chemically bound matter |
| **Nuclear stability** | Stable nuclei beyond the drip lines; the composition space is finite and bounded | **Category 2/3.** Fixes the periodic table as the compiler's alphabet |

> **PROPOSITION 18.5 (The Kramers–Kronig constraint is the most under-appreciated
> obstruction in materials compilation).** Every specification of the form "a medium
> with response X over bandwidth B and loss below L" must be checked against the
> causality sum rules before search begins. A large class of metamaterial
> specifications that appear to be engineering problems are **forbidden by causality**,
> and no fabrication advance will produce them.
> **Action: this belongs in the compiler's static feasibility screen** as a ninth
> certificate class, alongside conservation and spectral invariance. It is cheap to
> check and it currently is not checked anywhere in the prototype.
> **Classification: PROPOSITION (the theorem is KNOWN; its use as a compile-time
> screen is proposed here).**

---

## 18.4 The Physical Possibility Frontier

**Current bottleneck** = what stops it today. **Ultimate bottleneck** = what would
stop it given arbitrarily advanced but physically lawful infrastructure. The
distinction is mandatory; "same" means the current obstruction is also the ultimate
one.

| # | Capability | Physics permits? | Math obstruction? | Known route? | Current bottleneck | **Ultimate bottleneck** | Confidence |
|---|---|---|---|---|---|---|---|
| 1 | Ambient-pressure room-T superconductor | **Yes** — no theorem bounds T_c | None known | No | 13 materials / 15 computational (search space) | **Unknown (18)** — possibly none | medium |
| 2 | Metallic hydrogen recovered to ambient | **Yes** | None | No | 11 metastability / 16 instrumentation | **11 kinetic** — whether any barrier suffices is open | low-medium |
| 3 | Arbitrary spin-Hamiltonian realized on a lattice | **Yes** — universal Hamiltonian theorem, poly overhead | None | **Yes, proved** | 12 fabrication / 17 economic (gadget overhead constants) | **17 economic** — overhead is polynomial but the constants are brutal | high |
| 4 | Bulk matter assembled atom-by-atom (1 kg) | **Yes** | None | Biology at 10¹⁹ parallelism | 12 fabrication (parallelism) | **10 kinetic** — attempt frequency ceiling ~10¹²–10¹³ s⁻¹/site | high |
| 5 | Passive broadband invisibility cloak | **No** | — | — | — | **2 causality (Kramers–Kronig sum rule)** | high |
| 6 | Narrowband cloak / scattering cancellation | **Yes** | None | **Yes, demonstrated** | 13 materials (loss) | **2 causality** sets the bandwidth–performance trade | high |
| 7 | Passive 2D topological quantum memory at T>0 | **No** | — | — | — | **4/6 thermal instability (AFH)** | high |
| 8 | Self-correcting quantum memory in 3D | **Unknown** | **OPEN** | Partial (marginal self-correction) | 15 computational / 18 | **18 unknown** | low |
| 9 | Macroscopic superposition at 300 K | **Yes** — decoherence is a rate, not a law | None | No | 16 instrumentation (isolation) | **4 thermodynamic** — entropy export from the isolation system | medium |
| 10 | Deterministic single-dopant array, 10⁶ sites | **Yes** | None | Demonstrated at 10⁰–10¹ | 12 fabrication (throughput) / 14 measurement (buried verification) | **8 observability** — non-destructive subsurface atomic verification | medium-high |
| 11 | Non-Abelian topological order, engineered | **Yes** | None | **Yes, demonstrated** (D₄ on 27 ions; Fibonacci on superconducting) | 12 fabrication (qubit count) | **4 thermal** for a passive version; none for an actively corrected one | high |
| 12 | Non-Abelian anyons in a *bulk material* at useful T | **Yes** | None | No | 13 materials | **4 thermal (AFH in 2D)**; 3D open | low-medium |
| 13 | Arbitrary 100-qubit pure state, verified | **Yes** to prepare | None | Circuit exists | 12 fabrication | **5/8 information-theoretic** — verification of a *generic* target needs resources scaling with Hilbert dimension (CE-1) | high |
| 14 | Arbitrary *structured* 100-qubit state, verified | **Yes** | None | **Yes** — shadows, stabilizer verification | 12 fabrication | **12 fabrication** only | high |
| 15 | Arbitrary designed enzyme, specified k_cat/K_M | **Yes** | None | Partial (deep generative design) | 15 computational / 13 materials | **10 kinetic** — transition-state stabilization is the hard part | medium |
| 16 | Whole-organ morphogenesis to geometric spec | **Yes** | None | No | 15 computational (no forward model) / 8 observability | **8 observability** — certifying tissue architecture non-destructively | low |
| 17 | Room-temperature quantum processor, ≥10³ logical qubits | **Yes** | None | No | 16 instrumentation / 13 materials | **4 thermodynamic** — entropy removal rate at 300 K | low-medium |
| 18 | Self-replicating general-purpose assembler | **Unknown** | None known | Biology, for *its* chemistry | 12 fabrication / 18 | **18 unknown** — whether a general-chemistry replicator is thermodynamically closable is open | low |
| 19 | Entropy reduction without compensating export | **No** | — | — | — | **4 second law** | high |
| 20 | Arbitrary elastic tensor (incl. extreme Poisson ratio) | **Yes** within stability bounds | Positive-definiteness of the elasticity tensor | **Yes, demonstrated** | 12 fabrication | **1 mathematical** — thermodynamic stability bounds the achievable tensor cone | high |
| 21 | Negative absolute temperature state | **Yes** — needs a bounded spectrum | None | **Yes, demonstrated** | 13 materials (bounded spectrum) | **2** — unbounded-spectrum systems cannot host it | high |
| 22 | Programmable matter reconfiguring on command, mm scale | **Yes** | None | Partial (swarm robotics, DNA) | 12 fabrication / 13 materials | **4 thermodynamic** — actuation energy and heat removal per unit volume | medium |
| 23 | Isotopically pure bulk material, any element | **Yes** | None | **Yes** (²⁸Si at 99.995%) | 17 economic | **17 economic** — separation energy scales with mass ratio | high |
| 24 | Room-temperature BEC of a massive particle | **Yes** for quasiparticles — **demonstrated** (organic polariton condensates) | None | Yes, for polaritons | 13 materials | **2** for atoms: T_c ∝ n^{2/3}/m makes 300 K require densities above material stability | high |
| 25 | Compiling a *previously unknown* stable phase | **Yes** | None | Partial (generative + DFT screening) | 15 computational / 11 synthesizability | **10 kinetic** — predicting a route, not a structure | medium |
| 26 | Arbitrary 3D complete photonic bandgap | **Yes** | None | **Yes, demonstrated** | 12 fabrication | **13 materials** — index contrast required is bounded by available dielectrics | high |
| 27 | Verification of a quantum simulator in the classically-hard regime | **Unknown** | **OPEN** | Partial (cross-platform, self-testing) | 15 computational | **18 unknown** | low |
| 28 | Compiling the *environment* (fields, reservoirs) as a designed component | **Yes** | None | Partial (reservoir engineering) | 16 instrumentation | **4 thermodynamic** — reservoir capacity and heat rejection | medium-high |

### Reading of the table

**Of 28 capabilities: 3 are physically forbidden** (#5 broadband passive cloak, #7
passive 2D topological memory at T>0, #19 free entropy reduction). **3 are unknown**
(#8, #18, #27). **The remaining 22 are physically permitted**, and their ultimate
bottlenecks distribute as:

| Ultimate bottleneck | Count | Comment |
|---|---|---|
| Thermodynamic (4) | 6 | Entropy export, heat rejection, reservoir capacity |
| Kinetic (10) / metastability (11) | 5 | **The single most common ultimate obstruction** |
| Observability (8) | 3 | Non-destructive verification at the required resolution |
| Information-theoretic (5) | 1 | Generic-state verification |
| Materials (13) | 3 | And these are *not* ultimate — they are category-13 today and may vanish |
| Economic (17) | 2 | Price curves |
| Mathematical (1) / physical law (2) | 5 | Causality, stability bounds, spectrum boundedness |
| Unknown (18) | 3 | Honest |

> **PROPOSITION 18.6 (The frontier's shape).** For physically permitted targets, the
> ultimate obstruction is **kinetic or observational far more often than
> thermodynamic or informational.** Thermodynamics bounds the *cost* and almost never
> the *possibility*; information theory bounds *verification* of generic targets and
> almost never structured ones. **The compiler's hard problems are therefore route
> discovery (category 10/11) and non-destructive certification (category 8), and the
> research program should be weighted accordingly.**
> **Classification: PROPOSITION**, supported by the tabulation above; the tabulation
> is a judgment over 28 cases and not a survey.

---

## 18.5 Beyond today's materials: the admissible design space

Targets defined by (composition, structure, pressure, strain, field, dimensionality,
topology, nonequilibrium condition). Classification, never collapsed:

| Class | Definition | Example | Compiler action |
|---|---|---|---|
| **Physically allowed** | No category 1–6 obstruction | Ambient room-T superconductor | Admit to search |
| **Thermodynamically stable** | On the convex hull at the operating condition | Most known compounds | Standard route search |
| **Metastable** | Above the hull, barrier ≥ service requirement | Diamond, steel, most polymorphs, all Floquet phases | **First-class target type**; lifetime is a specified property |
| **Kinetically inaccessible** | No route at accessible conditions | Many predicted-stable compounds | Report as category 10 with the barrier estimate |
| **Synthesizable in principle** | A route exists in the model but not in the lab | — | MEASURE branch |
| **Extreme-conditions only** | Requires regimes available at small volume only | Hydride superconductors at 170 GPa | Admit, with the volume ceiling attached |
| **Presently inaccessible** | Category 12–17 | Room-T quantum processor | Admit to the theory; exclude from the roadmap |
| **Unsupported by known physics** | Requires new physics | — | Reject |

> **ENGINEERING CRITERION 18.7.** The compiler must attach a **volume ceiling** to
> every environment demand. 400 GPa is nearly free at 10⁻⁴ mm³ and unavailable at any
> price for 1 kg; millikelvin costs ~$10⁶ for a few litres and is unavailable for a
> factory. **Extreme-regime targets and bulk targets therefore have opposite
> environmental economics and must not share a cost model** — which is why quantum
> matter and bulk materials belong in separate back ends (§34.3).

---

## 18.6 Compiler-induced discovery

The directive asks whether inverse compilation can find genuinely unexpected routes.
The honest structure of the answer:

> **What is established.** Inverse design routinely produces *non-intuitive* solutions
> that outperform human designs — adjoint-optimized nanophotonic devices are the
> clearest case, and their geometries are not ones a person would draw. **KNOWN
> RESULT.**

> **What is not established.** That a compiler can discover a *new phase of matter* or
> a *new mechanism*. Every demonstrated case searches within a model class that
> already contains the answer. A compiler cannot find physics outside its model
> bundle 𝔐, by construction.

> **PROPOSITION 18.8 (Discovery is bounded by the model class, except through the
> MEASURE branch).** A simulation-only compiler can discover new *instances* within
> 𝔐 and cannot discover new *mechanisms* outside it. The only architectural route to
> mechanism discovery is the MEASURE branch: when no model in 𝔐 predicts the observed
> outcome, the compiler has detected that 𝔐 is wrong. **A compiler that reports
> model-bundle falsification is a discovery engine; one that only optimizes is not.**
> **Classification: PROPOSITION (proposed here).**
>
> **Implementation consequence, and a gap in the prototype:** the Model Discrimination
> Engine currently detects *disagreement between models*. It does not detect
> *unanimous disagreement with the data* — the case where every model is wrong. That
> is a distinct and more valuable signal, and adding it is a small change with a large
> conceptual payoff. **Logged as prototype gap P-1.**
