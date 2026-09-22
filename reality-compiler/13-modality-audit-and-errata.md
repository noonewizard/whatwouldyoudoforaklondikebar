# Part XIII — Modality Audit, Errata, and Revised Program

Sections 35–38

---

## 35. The Modality Audit

Every boundary claim in this monograph, re-examined against the directive's question:
**is this boundary imposed by physics, mathematics, information theory, control
theory, current materials science, current fabrication technology, current
measurement technology, economics, or merely by today's engineering assumptions?**

The rightmost column is the honest answer. Entries where Draft 1 got the category
wrong are marked **← CORRECTED**.

### 35.1 Boundaries imposed by PHYSICS (theorem, conservation, or established bound)

| Boundary | The theorem or law | Consequence for the framework |
|---|---|---|
| Matter cannot be created from the specification | Conservation of baryon number, charge, energy | The compiler sorts and rearranges a supplied inventory. Non-negotiable |
| Local entropy reduction must be paid for | Second law | Quantified in §15; the cost is *small*, which favors ambition |
| Preparation of long-range entanglement from a product state by constant-depth **unitary** local circuit | Lieb–Robinson bound | Forbidden *for that method*; measurement + feedforward circumvents it |
| Adiabatic passage through a critical point in finite time | Gap closure; Kibble–Zurek scaling | Forbidden; route around the transition instead |
| Continuous-symmetry breaking at finite T in d ≤ 2, short-range interactions | Mermin–Wagner | A forbidden region of specification space. Finite-size systems evade the asymptotics |
| Trivial gapped symmetric ground state at half-odd-integer spin per unit cell | Lieb–Schultz–Mattis | Compile-time static check with a certificate |
| Single chiral fermion on a local Hermitian lattice | Nielsen–Ninomiya | Constrains lattice-model targets |
| Finite-temperature topological order in 2D / passive 2D topological memory | Alicki–Fannes–Horodecki (thermal instability of the 2D toric code) | **A real ceiling.** 3D self-correction is OPEN; 4D is a theorem that does not help |
| Universal transversal gate set in any QEC code | Eastin–Knill | Forces magic-state distillation or code switching |
| Copying the prepared quantum state for verification | No-cloning | Verification consumes copies; drives the shadow/invariant strategy |
| Speed of state transformation given energy | Mandelstam–Tamm, Margolus–Levitin | Real, and permissive at manufacturing energy scales |
| Precision costs dissipation | Thermodynamic uncertainty relation | Real; numerically far below practice |
| Positioned mechanochemical event rate | Vibrational/reorganization timescale ~10⁻¹³ s | Ceiling ~10¹²–10¹³ events/s/site. **Permissive: biology runs 11 orders below it** |

### 35.2 Boundaries imposed by MATHEMATICS / COMPUTABILITY

| Boundary | Basis | What it actually bounds |
|---|---|---|
| General hybrid-system reachability | Undecidable (Henzinger et al.) | **Bounds the compiler, not the substrate.** Matter is not constrained by our ability to decide questions about it |
| Spectral gap in the thermodynamic limit | Undecidable (Cubitt–Pérez-García–Wolf) | Kills a *general property oracle*. Does not prevent making a gapped material, and says nothing about any finite instance |
| Reaction-network reachability | Ackermann-complete | Bounds exhaustive route search; bounded-depth search is what is used |
| Ground-state energy estimation | QMA-complete | Bounds *general* certification. Particular instances are routinely solved. **← CORRECTED framing** |
| Ising ground state, lattice folding, min tile set | NP-complete/hard | Bounds exact optimization; approximation and instance structure are the escapes |
| Averaged-over-all-objectives optimizer performance | No free lunch | Bounds universality of optimization, not performance on structured real instances |

**The correction that matters across this whole table:** these are bounds on
*algorithms answering questions about arbitrary instances*. They are not bounds on
what matter can do, and not bounds on what can be achieved for a *particular* target.
Draft 1 stated this once and then let the results read as pessimism throughout. A
physical system reaching its ground state is not violating QMA-hardness; it is
declining to solve the general problem.

### 35.3 Boundaries imposed by INFORMATION THEORY

| Boundary | Basis | Status |
|---|---|---|
| Verification sample complexity scales with specified-property count | Prop. 11.1, union bound | Real, favorable, and the framework's main lever |
| Full quantum state tomography | Θ(d²/ε²) copies | Real — and avoidable by specifying observables (§29.7) |
| Targets with κ ≈ 1 admit no compilation leverage | Prop. 31.1 (description length) | Real, and it explains the paradigm's scope |
| Specification information must be imposed on matter | §15.4, W ≥ k_BT ln2 · I(S) · χ | Real; numerically negligible |

### 35.4 Boundaries imposed by CONTROL THEORY

| Boundary | Basis | Status |
|---|---|---|
| Controllability of a bilinear system | Lie algebra rank condition | Exactly checkable; when satisfied, *any* target unitary is reachable. **A positive result, under-used in Draft 1** |
| Actuator count vs degrees of freedom | N_actuators ≪ N_dof for bulk matter | The reason landscape compilation is the primitive (§33) |
| Partial observability | Belief-space control is infinite-dimensional | Handled by approximation; not fundamental |

### 35.5 Boundaries that are CURRENT MATERIALS SCIENCE, not physics ← the corrections

| Boundary | Draft 1 implied | Correct status |
|---|---|---|
| Coherence times | Treated as platform constants | **τ_dec is a rate set by environmental coupling — an engineering parameter.** Isotopic purification, vacuum, DD, DFS, and engineered dissipation have each bought orders of magnitude **← CORRECTED** |
| Room-temperature superconductivity at ambient pressure | Not addressed | **No known theorem bounds Tc.** ALLOWED and undiscovered. Early McMillan-type "limits" were empirical and were broken by the hydrides |
| Material purity and defect density | Implicit ceiling | ²⁸Si at 99.995% is industrial; purity is a cost curve |
| Quantum spin liquid in a material | — | No confirmed material; no prohibition. Search problem |
| Recovering a metastable hydride to ambient pressure | — | Unsolved chemistry; no prohibition |

### 35.6 Boundaries that are CURRENT FABRICATION, not physics ← the corrections

| Boundary | Draft 1 said | Correct status |
|---|---|---|
| **Serial atomic assembly cannot make bulk matter** | "ESTABLISHED by arithmetic," generalized to all positional assembly | **The arithmetic holds for serial assembly only.** Massively parallel programmable positional assembly at kg/day is DEMONSTRATED by ribosomal synthesis at ~10¹⁹-fold parallelism. Artificial general-palette versions are ALLOWED / INACCESSIBLE **← CORRECTED, and this was the worst error in Draft 1** |
| "There is no universal assembler and never will be" | Stated as settled | **Over-stated.** No universal assembler *exists*; none is *forbidden*. The honest claims: (a) serial assembly cannot scale; (b) parallelism to 10¹⁹ is physically occupied by biology; (c) a general-chemistry artificial assembler is undemonstrated **← CORRECTED** |
| Probe-array parallelization ceiling | Implied ~10³ | An engineering figure with no physical basis stated. Should be an open question, not a bound |
| Tweezer-array size | Implied "hundreds" | **6,100 sites with 13 s coherence and 23-minute trap lifetimes, 2025.** The figure moved by an order of magnitude while Draft 1 was being written **← CORRECTED** |
| DSA defectivity floor | Treated as near-fundamental | A kinetic-trapping problem with active approaches; no theorem |
| Sample volume at extreme pressure | Not addressed | ~10⁻⁴ mm³ — a genuine and decisive engineering limit for *bulk*, irrelevant for quantum matter |

### 35.7 Boundaries that are CURRENT METROLOGY

| Boundary | Status |
|---|---|
| Non-destructive subsurface atomic-resolution imaging at throughput | No technique exists; no prohibition. A major open capability |
| Automated phase identification reliability | Documented failure point of autonomous labs; entirely an algorithms-and-data problem |
| Rare-event certification at δ ≤ 10⁻⁶ | Statistics plus physics-informed tail models; an open methods problem |
| Verifying a quantum simulator in the regime where classical simulation fails | Genuinely open; partly addressed by invariants and cross-platform comparison |

### 35.8 Boundaries that are ECONOMICS

| Boundary | Note |
|---|---|
| Qualification cost (~$10⁷–10⁸ per aerospace alloy) | The dominant adoption barrier. Pure economics |
| Launch cost to orbit | Fallen ~20× in two decades. Explicitly not a physical limit |
| Helium-3 supply, isotope enrichment, facility access | Prices |
| Fab capital (~$10¹⁰ per node) | Price |
| Value capture ratio for a compiler layer | Market structure |

### 35.9 Boundaries that were merely ENGINEERING ASSUMPTION ← the corrections

| Assumption in Draft 1 | Correction |
|---|---|
| The compiler's substrate is fixed and ambient | **ℰ is a free variable to be costed** (§34) |
| Quantum resources are tools for compiling classical matter | **Quantum matter is a native and better-posed compilation domain** (§29) **← CORRECTED** |
| The action algebra 𝒜 is an instruction set of placements | **𝒜 is an algebra of conditions** (§33.4) **← CORRECTED** |
| Verification is uniformly expensive | **Topological targets are exceptionally cheap to certify** (Prop. 29.1) **← CORRECTED** |
| The near-term bridge is structural materials | Quantum matter scores higher on three of four compilability criteria (§29.3) **← CORRECTED** |
| "No universal compiler" settles universality | Universal *Hamiltonian* families exist and are proven; programmable simulators approach a universal compiler for a broad class (§35.10) **← PARTIALLY CORRECTED** |

### 35.10 Universality, revisited

Draft 1's §17 concluded "no universal compiler, only a universal interface." That
stands for *matter at large*. It under-reported one genuine positive result:

- **Universal Hamiltonian families exist** — certain simple spin-lattice models can
  reproduce the full physics (spectrum, partition function, dynamics) of *any* other
  spin Hamiltonian with polynomial overhead. **Proved.**
- Consequently a sufficiently controllable programmable quantum simulator is, for the
  class of spin systems, something very close to a **universal compiler for quantum
  matter**. That is a much stronger universality claim than Draft 1 allowed, it is
  theorem-backed, and its restriction (to spin Hamiltonians, with polynomial
  overhead that may be large) should be stated rather than the whole result being
  omitted.

Revised verdict:

> Universality fails for fabrication of bulk matter, for optimization, for
> representation, and for verification. It **holds, provably, for the simulation and
> synthesis of spin-Hamiltonian physics**, and therefore a universal compiler exists
> for a well-defined and physically rich sub-domain. Outside it, interfaces are
> universal and back ends are not.

---

## 36. Errata to Draft 1

Consolidated, so that the corrections are locatable rather than distributed.

| # | Location | Draft 1 text | Correction | Severity |
|---|---|---|---|---|
| E1 | §10.1 | "Serial atomic assembly is not a manufacturing technology for bulk matter and never will be. ESTABLISHED by arithmetic." | The arithmetic bounds *serial* assembly. Parallel programmable positional assembly at kg/day is demonstrated biologically. See §29.2 | **Major** — a physical prohibition was asserted where only an architectural one exists |
| E2 | §9.2 | "Room-temperature macroscopic quantum coherence: CONTRADICTED" | Conflates four distinct claims with labels ranging from DEMONSTRATED to CONTRADICTED. See §29.8 | **Major** |
| E3 | Part IV generally | Quantum treated as compile-time tool and metrology | Quantum matter is a native compilation domain and the framework's best-posed one. See Part XI | **Major** — a scope error, not a factual one |
| E4 | §9.1, §9.5 | "Topological: SPECULATIVE — none demonstrated" | True for *Majorana hardware qubits*. False for *engineered topological order*: Abelian and non-Abelian topological order have been prepared and braiding verified on trapped-ion and superconducting processors (2023–2024) | **Major** |
| E5 | §8.2–8.3 | Landscape engineering as computation "largely CONTRADICTED" | Correct only for glassy optimization — one of five advantage regimes. Exponential advantage holds for many-body state preparation. See §32.3 | **Moderate** |
| E6 | §17 | "No universal physical compiler" | Under-reports the proven existence of universal Hamiltonian families. See §35.10 | **Moderate** |
| E7 | §11 | Verification framed as uniformly costly | Topological/quantized targets are exceptionally cheap to certify. See Prop. 29.1 | **Moderate** |
| E8 | §2.3, §5.2 | 𝒜 defined as placement-like primitive actions | 𝒜 should be an algebra of *conditions*; placement is the degenerate case. See §33.4 | **Moderate** |
| E9 | §16 throughout | Undecidability/QMA results presented without repeated scope qualification | They bound algorithms over arbitrary instances, not physical achievability of particular targets. Stated once in §16.8; should govern every use | **Moderate** |
| E10 | §21 | ℰ treated as a constraint | ℰ is a costed free variable. See §34 | **Moderate** |
| E11 | §15.3 | Numeric slips: configurational bound and CMOS ratio | Corrected in place (0.1 MJ/kg; ~10⁴× device-level, ~10⁸× system-level) | Minor |

**What Draft 1 got right and should not be softened:**

- The acceptance-region reformulation (targets are regions, not states).
- The thermodynamic accounting — which, correctly read, *supports* ambition: even
  extreme synthesis is energetically cheap, and energy will never be the wall.
- The verification-cost-in-the-objective thesis, which §29.7 strengthens rather than
  weakens by giving it a domain where it bites hardest.
- The complexity-vector-with-exchange-relations formulation.
- The economic arithmetic on the quintillion-dollar claim.
- The insistence on three-valued verification and on honest UNKNOWN.

---

## 37. Revised Program: The Quantum-Matter Track

Draft 1's roadmap had a single track ending at "generalized physical-state
compilation." Given §29.3, the program should run **two tracks with different
physics, different economics, and different environmental cost models** (§34.3).

```
TRACK A — BULK MATTER                    TRACK B — QUANTUM MATTER
(ambient envelope, large volume)         (extreme envelope, small volume)

A0 Specification language  ──────────────  shared front end, shared evidence calculus
A1 Materials advisory compiler           B1 Hamiltonian-target compiler
     (Draft 1 Stages 0-1)                     spec → H* over a realizable family
A2 Automated lab execution               B2 Preparation-protocol synthesis
A3 Closed-loop synthesis                      adiabatic / dissipative / adaptive
A4 Programmable microstructure           B3 Programmable array compilation
A5 Self-assembling systems                    (tweezers, lattices, circuits)
A6 Synthetic morphogenesis               B4 Phase-target compilation
A7 Atomically precise, parallel               (invariants, order parameters)
                                         B5 Solid-state quantum matter
                                              (moiré, defects, interfaces)
                                         B6 Stabilized nonequilibrium matter
```

### Track B stage detail

**B1 — Hamiltonian-target compiler.** Input: a specified Hamiltonian, phase, or
invariant. Output: parameters in a realizable family (lattice depth, detunings,
tweezer geometry, twist angle, circuit layout). *Required technology:* exists.
*Bottleneck:* the many-body inverse problem — mapping desired phase to realizable
couplings. *Milestone:* compile ≥20 target Hamiltonians to correct parameters,
verified experimentally, including ≥5 the designer did not anticipate.
*Falsification:* if compiled parameters fail to produce the target phase in >30% of
cases despite in-family realizability, the inverse map is not learnable at this
fidelity.

**B2 — Preparation-protocol synthesis.** Input: H* and initial state. Output: a
protocol (ramp, dissipative coupling, adaptive circuit). *Bottleneck:* entropy
management and gap navigation. *Milestone:* automated protocol synthesis matching or
beating hand-designed protocols on ≥10 targets by ≥2× in fidelity-per-unit-time.
*Falsification:* if automated protocols never beat expert hand-tuning, the
preparation problem is not compilable at present.

**B3 — Programmable array compilation.** *Milestone:* a declarative spec → tweezer
hologram → verified site occupancy at ≥10⁴ sites with ≥99% fill fidelity.

**B4 — Phase-target compilation.** *Milestone:* specify a topological invariant and
a gap; the compiler selects substrate, protocol, and an *interferometric verification
plan*; execution confirms the invariant to ±0.05. **This is the flagship
demonstration of the entire framework** — it is the first time a declarative physical
specification would be compiled, executed, and certified by a quantized observable.

**B5 — Solid-state quantum matter.** Twist angle, strain, interface, and defect
placement as compiler outputs. *Milestone:* predict-then-realize a correlated or
topological phase at a specified twist angle and filling, pre-registered.

**B6 — Stabilized nonequilibrium matter.** Compile a drive + dissipation pair whose
steady state has a specified property with no equilibrium counterpart. *Milestone:*
a designed nonequilibrium phase, specified in advance, realized and verified.

### Why Track B should be funded first

Against Draft 1's own criteria: better forward models, better action algebras, better
yield statistics, cheaper verification for topological targets, no qualification
cost, no regulatory cycle, and experimental turnaround in days rather than the years
an alloy takes. **Track B is where the framework's thesis can be proven or falsified
fastest**, and Draft 1's roadmap put it last.

---

## 38. Additional Falsification Experiments

Extending §25. Same format: hypothesis, protocol, quantitative threshold, what dies
if it fails.

**F-11 — Can a Hamiltonian target be compiled without expert intervention?**
*Hypothesis:* an automated compiler maps specified phases to realizable control
parameters at ≥70% first-attempt success across ≥20 targets on a programmable
simulator. *Pass:* ≥70%, including ≥3 targets outside the training distribution.
*Fail:* <40%, or success confined to targets already in the literature. *If it
fails:* Hamiltonian-target compilation is retrieval, not compilation. *Cost:*
$1–2M, 18 months.

**F-12 — Does adaptive (measurement-based) preparation beat unitary preparation in
practice, not just asymptotically?** *Hypothesis:* for long-range-entangled targets,
constant-depth adaptive circuits achieve higher final fidelity than the best unitary
protocol at equal wall-clock time, on real hardware with realistic measurement
latency and error. *Pass:* adaptive wins on ≥3 target classes at ≥2 system sizes,
with the margin growing with size. *Fail:* measurement latency and readout error
consume the depth advantage at all accessible sizes. *If it fails:* the theorem is
true and irrelevant at realizable scales, and §32.3's polynomial-advantage row does
not cash out. *Cost:* $500k–1M, 12 months.

**F-13 — Does compile-for-verifiability hold where it should bite hardest?**
*Hypothesis (a sharpened F-4):* among physically equivalent routes to a quantum
target, those specified by quantized/topological observables cost ≥10× less to
certify than those specified by local order parameters at equal confidence. *Pass:*
≥10× median reduction. *Fail:* <3×. *If it fails:* Prop. 29.1 is correct in theory
and does not dominate real verification budgets, which weakens the framework's
flagship economic claim in the one domain where it looked strongest.

**F-14 — Does the compression ratio κ predict compilability?**
*Hypothesis:* across ≥30 targets spanning ≥4 substrates, κ(S) predicts
cost-to-compile with rank correlation |ρ| ≥ 0.6. *Pass:* |ρ| ≥ 0.6, p < 0.01.
*Fail:* |ρ| < 0.3. *If it fails:* Prop. 31.1 is a metaphor, and §33's answer to the
instruction-vs-emergence question loses its quantitative backing. *Cost:*
$300–600k, 12 months, mostly analysis of existing data.

**F-15 — Can parallel positional assembly be pushed beyond biological chemistry?**
*Hypothesis:* a programmable, template-directed assembly system with a non-biological
monomer palette achieves ≥10¹² parallel sites with per-monomer error ≤10⁻³.
*Pass:* both thresholds. *Fail:* error rate cannot be driven below 10⁻² at any
useful parallelism. *If it fails:* the ribosome existence proof does not generalize
beyond its evolved chemistry, and E1's optimism about artificial parallel assembly is
unwarranted — which would *partially restore* Draft 1's pessimism, on proper grounds
this time. *Cost:* $5–15M, 3–5 years.

**F-16 — Is decoherence-limited performance an engineering variable or a ceiling?**
*Hypothesis:* for a fixed physical platform, targeted environmental engineering
(isotopic purification, vacuum, field stability, engineered dissipation) yields ≥10×
coherence improvement per decade of sustained effort, with no sign of saturation.
*Protocol:* a pre-registered longitudinal audit of published coherence records per
platform, plus one targeted improvement campaign. *Pass:* ≥10×/decade continuing.
*Fail:* clear saturation over ≥5 years across ≥3 platforms despite effort. *If it
fails:* decoherence *is* behaving like a ceiling for that platform class, and §29.8's
reframing — the core of the directive's correction — is wrong for that platform.
**This experiment is the directive's own falsification test, and it should be run.**
