# Substrate Compilability: A Formalism and a Theorem Program

Section 17 (new). The central mathematical problem of the framework.

---

## 17.1 The predicate

> **DEFINITION 17.1 (Compilability predicate).** For a substrate Ω, a specification
> *class* 𝒮, an error budget ε, a horizon T, and a resource budget C, write
> ```
>   Compilable(Ω, 𝒮, ε, T, C)
> ```
> to mean: there exists an algorithm 𝔠 such that for every S ∈ 𝒮, 𝔠(S) terminates
> within the declared compute budget and returns exactly one of
> **COMPILE(π,Ψ,Π)**, **OBSTRUCT(c)**, or **MEASURE(e)**, such that
>
> **(Soundness)** if 𝔠(S) = COMPILE(π,Ψ,Π) then the executed pair (π,Ψ) has
> false-accept probability ≤ ε under every model in 𝔐 that passes its own validity
> test on π's trajectory, with π respecting T and C;
> **(Certificate soundness)** if 𝔠(S) = OBSTRUCT(c) then c is independently checkable
> in time poly(|c|) and proves no admissible program of horizon ≤ T exists;
> **(Honesty)** otherwise 𝔠(S) = MEASURE(e), with e an admissible experiment.
>
> Note what is *not* required: completeness. A compiler that always returns MEASURE is
> trivially compilable-sound and useless, so §17.5 adds a non-triviality condition.

The predicate is over a *class* 𝒮, not a single specification. A substrate is not
"compilable" full stop; it is compilable for a class at a budget.

---

## 17.2 The ten conditions

Throughout, Ω = ⟨Σ, 𝒜, Λ, 𝒯, ℳ, 𝒞, ℰ, 𝔐⟩.

> **(R) Representation.** There is a set 𝔇 with a finite or finitely parameterized
> encoding, and an abstraction map α : Σ → 𝔇, such that every S ∈ 𝒮 is a decidable
> predicate on 𝔇, and the specified functionals factor through α:
> Φ = Φ̄ ∘ α for some Φ̄ : 𝔇 → ℝᵏ.
> *Failure mode:* the specification names a property not expressible at any level of
> description the substrate's models support.

> **(Ctrl) Controllability.** For each S ∈ 𝒮 there exists λ(·) ∈ 𝒯 with
> [S] ∩ Reach(𝔟₀, Λ, ∞) ≠ ∅. *Asymptotic; no time bound.*

> **(Rch) Reachability.** The stronger, time-bounded version:
> [S] ∩ Reach(𝔟₀, Λ, T) ≠ ∅.

> **(MV) Model validity.** There is m ∈ 𝔐 whose declared validity domain contains
> the candidate trajectory and whose error bound ε_m satisfies ε_m < margin(S) — the
> distance from the predicted operating point to ∂𝒦.

> **(St) Stability.** [S] contains a set that is invariant, or metastable with
> lifetime ≥ τ_spec, under the actual noise process, with stability radius B
> exceeding the realized parameter dispersion.

> **(Obs) Observability.** The measurement algebra separates the target from its
> declared alternatives: for every a ∈ Alt(S) there is w ∈ ℳ with
> p_w(· | [S]) ≠ p_w(· | a). *Qualitative.*

> **(Ver) Verifiability.** The quantitative form: there is W ⊆ ℳ and a sample
> allocation n with Σ cost ≤ B_ver such that the decision rule on W has false-accept
> ≤ δ and false-reject ≤ β against Alt(S).

> **(Cmp) Compositionality.** For multi-step programs: each intermediate acceptance
> region 𝒦_i is a **sufficient statistic** for downstream behavior — the map from an
> intermediate article to the next step's outcome distribution factors through Φ_i as
> a Markov kernel — and each step's false-accept functional is Lipschitz on 𝒦_i.

> **(Res) Resource boundedness.** The program's draws on 𝒞, T, and ℰ are within
> budget, and this is checkable before execution.

> **(Ref) Refinement.** Each lowering IR_i ⇝ IR_{i+1} carries a witness — a proof or a
> measured calibration with an error bound — such that the acceptance guarantee is
> preserved within a declared per-step budget δ_i.

---

## 17.3 Dependency analysis — which conditions imply which

This is where the formalism earns its keep: the ten are **not independent**, and the
implications are not all obvious.

> **LEMMA 17.2.** Ver ⟹ Obs, and the implication is strict.
> *Proof.* If a decision rule on W achieves false-accept ≤ δ < ½ against a ∈ Alt(S),
> then p_W(· | [S]) ≠ p_W(· | a), which is Obs. Strictness: full-state verification of
> a generic *n*-qubit target is observable (tomography separates any two distinct
> states) but requires Θ(4ⁿ) copies, so Obs holds and Ver fails at any realistic
> budget. ∎

> **LEMMA 17.3.** Rch ⟹ Ctrl. Ctrl ⇏ Rch.
> *Proof.* First is immediate. For the converse: a bilinear system satisfying the Lie
> algebra rank condition is completely controllable, yet the time to reach a given
> target may exceed T — LARC gives no time bound (§9.4.4). ∎

> **LEMMA 17.4 (Independence of the remaining pairs).** No implication holds among
> {Ctrl, St, Obs, MV, Res}, as witnessed by the counterexamples of §17.6:
> - **Ctrl ∧ ¬Ver** — CE-1 (prepare any state; certify none).
> - **Ver ∧ ¬Ctrl** — CE-2 (ambient room-temperature superconductivity: trivially
>   verifiable by four standard measurements; no known control route).
> - **Ctrl ∧ ¬St** — CE-4 (prepare an unstable equilibrium; it leaves).
> - **St ∧ ¬Rch** — CE-5 (a deep basin behind an insurmountable barrier).
> - **R ∧ ¬MV** — a specification perfectly expressible in a representation for which
>   every available model is out of validity domain.
> ∎

### The feature lattice

The important structural result is that the conditions are **feature-conditioned**, not
uniformly required:

| Compiler feature used | Conditions required |
|---|---|
| Single-step, pure-VERIFY (measure everything, model nothing) | **R, Rch, Ver, Res** |
| + DERIVE-mode claims (any model-based guarantee) | + **MV** |
| + lifetime / service-life requirements | + **St** |
| + multi-step routes | + **Cmp** |
| + multi-level abstraction (phase → H_eff → microscopic → controls) | + **Ref** |
| + optimization over routes | *(none new — a refinement of search)* |

> **PROPOSITION 17.5 (Minimal condition set).** For single-step compilation of
> specifications whose every requirement is mode VERIFY, the conditions
> **{R, Rch, Ver, Res}** are sufficient, and each is necessary.
>
> *Sufficiency.* Given R, the specification is a decidable predicate on 𝔇. Given Rch,
> a program exists within T. Given Res, it is affordable. Given Ver, a decision rule
> with false-accept ≤ δ exists within budget. A compiler that enumerates programs of
> horizon ≤ T (finite, since 𝒜 is finite and T bounded), pairs each with the
> verification plan from Ver, and returns the first pair passing the budget test, is
> sound by construction — it *exhibits* a witness, and the guarantee is discharged by
> measurement, not by a model.
> *Necessity.* Drop R: the specification is not a predicate on anything the compiler
> can reason about. Drop Rch: no program exists. Drop Res: no program is affordable.
> Drop Ver: by Lemma 17.2's strictness, no decision rule meets the budget, so no
> COMPILE verdict can be sound. ∎
>
> **Classification: PROPOSITION, proved.** *Contribution note:* the proof is routine;
> the content is the identification of the minimal set and, in particular, that
> **MV is not among them**. A compiler that promises nothing it has not measured needs
> no valid model at all.

> **COROLLARY 17.6 (The price of prediction).** Model validity enters the requirement
> set exactly when the compiler makes a claim it has not measured. Every DERIVE
> requirement converts an empirical burden into a modeling burden. **The
> evidence-coverage figure of §5 is therefore not a reporting nicety: it is a readout
> of which conditions the compilation actually depends on.**

---

## 17.4 The soundness theorem, and its central limitation

> **THEOREM 17.7 (Conditional soundness).** Let 𝒮 be the class of specifications that
> are conjunctions of k predicates on bounded-sensitivity functionals, let Ω satisfy
> R, Rch(T), Res(C), Ver(δ/k per predicate), and let every DERIVE requirement be
> covered by MV with ε_m < margin. Then the compiler 𝔠 that (i) type-checks, (ii)
> searches programs of horizon ≤ T, (iii) synthesizes a verification plan per §9.8.2,
> and (iv) returns COMPILE only on exhibiting both, satisfies: for every m ∈ 𝔐 passing
> its validity test,
> ```
>   Pr_m[ A(Y) = PASS ∧ Φ(σ_T) ∉ 𝒦 ] ≤ δ .
> ```
> *Proof.* Union bound over the k predicates, each certified at δ/k by Ver; DERIVE
> requirements contribute bounded error ε_m < margin by MV, which by definition does
> not move the predicted operating point across ∂𝒦. Soundness follows because the
> verdict is issued only on exhibition of a witness. ∎
>
> **Classification: THEOREM (elementary).**

> **COROLLARY 17.8 (The central limitation — and the honest content of the whole
> framework).** Theorem 17.7 bounds false acceptance **relative to 𝔐**. It says
> nothing if the truth lies outside 𝔐. Since MV is an *empirical* claim about a set of
> models and no formal machinery establishes it,
>
> > **the compiler can be sound relative to its model bundle and cannot be sound
> > relative to reality.**
>
> The framework's formal content is therefore *thin* and its empirical burden is
> *heavy*: Theorem 17.7's proof is a union bound, and everything difficult is in
> discharging Ver and MV experimentally. **This is not a defect to be hidden; it is the
> correct allocation, and it is why the calibration experiment (F-1) is the first thing
> to fund.**

> **PROPOSITION 17.9 (Non-triviality condition).** A compiler is *non-trivial* on
> (Ω, 𝒮) if its MEASURE rate on 𝒮 is below a declared threshold and its COMPILE
> verdicts are confirmed at the claimed rate. Soundness alone is worthless — the
> constant-MEASURE compiler is sound. **Every claim of compilability must report the
> MEASURE rate.**

---

## 17.5 The theorem program: attempt and result

The directive asks whether a result of this shape is provable:

> *"If a substrate admits a finite semantic abstraction, sound refinement, bounded
> model error, sufficient controllability, robust target regions, and a statistically
> valid observation channel, then a compiler can provide sound compilation guarantees
> over a restricted specification class."*

**Result: TRUE, and weaker than it sounds.** Theorem 17.7 is exactly this statement,
and its proof is a union bound. The reason it is not a deep theorem is structural:
*soundness by witness exhibition is cheap.* A compiler that only ever claims what it
can demonstrate is sound almost trivially; the hypotheses (Ver, MV) carry all the
weight, and they are empirical.

**The deep theorem would be a completeness result**, and here the news is bad:

> **PROPOSITION 17.10 (No general completeness).** There is no algorithm that, for
> arbitrary Ω and 𝒮, returns COMPILE or a valid OBSTRUCT certificate and never
> MEASURE. *Proof.* Take Ω a hybrid dynamical substrate and 𝒮 a reachability class;
> deciding membership is undecidable (§7.16). Bounding the horizon restores
> decidability but at PSPACE-hardness, and the bounded problem's OBSTRUCT certificates
> must then certify non-existence over an exponentially large program space, for which
> no polynomial certificate is known (and would imply coNP ⊆ NP for the propositional
> fragment). ∎

> **COROLLARY 17.11.** MEASURE is not an engineering concession. It is **forced** by
> the complexity of the compilation problem. A three-valued compiler is the only kind
> that can exist.

*This is, in the author's assessment, the strongest genuinely new formal statement in
the framework: the three-valued outcome is not a design choice but a consequence of
undecidability plus certificate hardness.*

---

## 17.6 Counterexamples

Each is a substrate/target pair demonstrating that a condition does not follow from
the others.

**CE-1 — Controllable, not verifiable.** A fully controllable 50-qubit register
(LARC satisfied) with specification "fidelity ≥ 0.99 to a fixed Haar-random |ψ⟩."
Preparation: a circuit exists. Verification: for a generic target with no structure,
certification requires resources scaling with Hilbert-space dimension — 2⁵⁰ ≈ 10¹⁵.
**Ctrl ∧ ¬Ver.** *Moral: controllability is not the binding condition; verifiability
usually is.*

**CE-2 — Verifiable, not controllable.** "A superconductor with T_c ≥ 300 K at ambient
pressure." Verification is cheap and standard: zero resistance, Meissner expulsion,
specific-heat anomaly at T_c, isotope shift. Control: no known route. **Ver ∧ ¬Ctrl.**
*Moral: the verification layer can be ready long before the synthesis layer, and the
framework should say which is missing.*

**CE-3 — Compressible, not preparable.** A frustrated local Hamiltonian instance:
K(H) = O(N), target fully specified, ground-state energy decision QMA-complete, gap
possibly exponentially small, frustration blocking the dissipative route.
**Short description ∧ ¬efficient preparation.** *Moral: kills the sufficiency
direction of the compressibility thesis (audit item #4).*

**CE-4 — Reachable and unstable.** An inverted-pendulum-type target, or a quantum
state at a saddle of the control landscape: reachable, and left immediately under the
actual noise. **Rch ∧ ¬St.** *Moral: St must be checked against the realized noise
spectrum, not against an idealized one.*

**CE-5 — Stable and unreachable.** A deep free-energy basin separated from the
accessible region by a barrier exceeding the substrate's thermal and control budget —
e.g. a thermodynamically favored polymorph with no nucleation pathway at accessible
conditions. **St ∧ ¬Rch.** *Moral: thermodynamic favorability is not a route.*

**CE-6 — Locally sound refinement, globally unsound.** A three-step route where each
step's guarantee was validated at a nominal intermediate, and the intermediate
specification is *not* a sufficient statistic: step 2's yield depends on a trace
impurity not named in 𝒦₁. Each step is individually sound; the composite is not.
**Ref ∧ ¬Cmp.** *Moral: this is Proposition D.3 from Part III, and it is why chemical
process routes are validated end-to-end.*

---

## 17.7 Complexity table for the ten problems

| Problem | Decision version | Input encoding | Precision model | Class / known hardness | Undecidability boundary | Approximation |
|---|---|---|---|---|---|---|
| **RC-0** Feasibility | ∃ program reaching [S]? | hybrid automaton + spec | exact reals | **Undecidable** for PCD systems in dim ≥ 3 | above rectangular hybrid automata | n/a |
| **RC-0′** Bounded feasibility | …with \|π\| ≤ L, T ≤ T_max | finite 𝒜, bounded horizon | bounded precision | **PSPACE-complete** (propositional planning); undecidable with unbounded numeric fluents | numeric fluents | domain-dependent |
| **RC-1** Reachability | σ_T ∈ [S]? | as above | bounded | over-approximable (zonotopes, Taylor models); exact undecidable | as RC-0 | over-approximation is the practical tool |
| **RC-2** Optimization | ∃ π with 𝔍 ≤ c? | as above | bounded | **NP-hard** minimum (Ising ground state); mixed-integer optimal control worse | — | submodular subproblems (1−1/e) |
| **RC-3** Refinement | does IR_{i+1} refine IR_i within δ_i? | two IRs + witness | bounded | checking a *given* witness: poly. Finding one: as hard as the synthesis | — | measured calibration substitutes for proof |
| **RC-4** Verification | does W certify at δ? | measurement models | statistical | **NP-hard** (test cover), submodular ⇒ ln-approx for min-cost cover | — | (1−1/e) max-coverage |
| **QR-0** Quantum feasibility | ∃ λ(·) with ρ_T ∈ [S]? | local H + spec | 1/poly promise gap | contains LOCAL HAMILTONIAN ⇒ **QMA-hard** | — | **OPEN**: if quantum PCP holds, constant-relative-error approximation stays QMA-hard *for the ground-energy problem*. See caveat below |
| **QR-1** Quantum reachability | ρ_T ∈ Reach? | generator class + budget | bounded | mode-dependent: unitary ⇒ spectrum-invariant (poly check); unrestricted CPTP ⇒ trivial; quasi-local dissipators ⇒ **OPEN** | — | — |
| **QR-2** Landscape synthesis | ∃ λ with the target as ground/steady state? | H family + spec | 1/poly | ground-state *property* estimation is **P^QMA[log]-complete** (APX-SIM); routing between ground states (GSCON) is **QCMA-complete to PSPACE-complete**, NEXP-complete succinct | — | constructive for MPS/PEPS parent Hamiltonians |
| **QR-3** Robust preparation | ∃ λ robust to Θ? | as QR-2 + uncertainty set | bounded | at least as hard as QR-2; minimax structure | — | scenario approximation |
| **QR-4** Quantum verification | does W certify ρ ∈ [S]? | observables + noise model | statistical | tomography Θ(d²/ε²); shadows O(log m/ε²); structured targets poly | — | coverage formulation submodular |

> **CAVEAT on quantum PCP (required by the directive).** The quantum PCP conjecture
> concerns the hardness of approximating the **ground-state energy** of a local
> Hamiltonian to constant *relative* error. It is **not** a statement about the
> hardness of preparing states, of estimating arbitrary ground-state properties, or of
> approximate compilation in general. Drafts 3–4 wrote that if qPCP holds then
> "approximate compilation is no easier than exact in the worst case." **That wording
> overreaches.** The defensible statement: *if qPCP holds, then the energy-threshold
> subproblem admits no constant-relative-error approximation in the worst case,
> removing one hoped-for general escape from QMA-hardness; it does not settle the
> complexity of the other four quantum problems.* **Classification: the conjecture is
> OPEN; the corrected implication is a KNOWN consequence; the Draft 3–4 wording is
> withdrawn.**

---

## 17.8 The minimal universal substrate

> **Question.** What is the smallest class of physical capabilities sufficient to
> express the semantics a general Physical State Compiler requires?

Working from the conditions rather than from hardware, the **semantic primitives of
physical programmability** appear to be:

| Primitive | Why required | Which condition it discharges |
|---|---|---|
| **Programmable interaction** — a parameterized family of couplings | Otherwise Λ is a point and nothing is compilable | Ctrl, Rch |
| **Entropy sink** — access to a colder reservoir, engineered dissipation, or measurement-with-erasure | By Theorem 9.5, unitary control alone cannot change spec(ρ); every preparation from a mixed initial state needs one | Rch (for entropy-changing targets) |
| **Readout** — a measurement channel separating targets from alternatives | Otherwise no verdict is possible even in principle | Obs, Ver |
| **Reset** — return to a known initial state | Needed for repeatability and hence for yield statistics | Ver (stationary statistics) |
| **Isolation** — a controllable coupling to the environment | Needed for St; without it the noise floor is not a design parameter | St |
| **Feedback** — conditional action on a measurement outcome within the relevant timescale | Buys constant-depth preparation (§9.5) and closed-loop correction; **not strictly necessary**, but removing it costs depth | Rch (depth), St |
| **Composition** — a way to join two prepared subsystems without destroying either | Required for Cmp and hence for multi-step | Cmp |

> **CONJECTURE 17.12 (Semantic minimality).** {programmable interaction, entropy sink,
> readout, reset, isolation, composition} is **sufficient** for a substrate to admit a
> non-trivial compiler over a non-empty specification class; and each is **necessary**
> in the sense that removing it excludes a non-empty target class.
> **Falsifier:** a substrate lacking one primitive that nonetheless compiles a class
> requiring it; or a substrate with all six that admits no non-trivial compiler.

**Substrate comparison against the six primitives:**

| Substrate | Prog. interaction | Entropy sink | Readout | Reset | Isolation | Composition |
|---|---|---|---|---|---|---|
| Classical digital electronics | ✔ | ✔ (dissipation) | ✔ | ✔ | ✔ | ✔ |
| Trapped ions | ✔ | ✔ (laser cooling) | ✔ | ✔ | ✔ | partial |
| Superconducting circuits | ✔ | ✔ (engineered) | ✔ | ✔ | ✔ | ✔ |
| Neutral atoms | ✔ (geometry) | ✔ (evaporative) | ✔ | ✔ | ✔ | partial |
| Photonics | partial | loss only | ✔ | ✔ | ✔ | ✔ |
| Chemical reaction networks | partial (rates) | ✔ (thermal) | ✔ (assay) | ✖ (no reset in a batch) | ✖ | partial |
| DNA self-assembly | ✔ (sequence) | ✔ | ✔ (imaging) | ✖ | partial | ✔ (hierarchical) |
| Mechanical metamaterials | ✖ (fixed at fabrication) | ✔ (damping) | ✔ | ✔ | ✔ | ✔ |
| Biological regulatory networks | ✔ (circuits) | ✔ | ✔ | partial | ✖ | ✔ |
| Moiré / solid-state | ✖ (fixed at fabrication) | ✔ | ✔ | ✖ | ✔ | ✖ |

**Reading.** Superconducting circuits and classical electronics are the only rows with
all six. Chemical and solid-state substrates fail *reset* and *isolation*, which is a
precise statement of why they are harder to compile for, and it is a more useful
diagnosis than "chemistry is messy."

---

## 17.9 Physical logic universality

> **Research problem.** Does a substrate possess the nine properties that make digital
> logic composable — robust state variable, noise margin, **gain**, restoration,
> fan-out, composition, reset, isolation, error correction?

**Gain is the discriminating property and it is the one usually forgotten.** Without
gain, signals attenuate under composition and no amount of margin saves you.

| Substrate | Robust variable | Noise margin | **Gain** | Restoration | Fan-out | Error correction |
|---|---|---|---|---|---|---|
| CMOS logic | voltage | ✔ | ✔ | ✔ | ✔ | ✔ (coding) |
| Quantum error correction | logical subspace | ✔ (code distance) | ✔ (syndrome-driven) | ✔ (every cycle) | ✔ (transversal, with Eastin–Knill caveat) | ✔ by construction |
| Chemical reaction networks | concentration | partial | ✔ (autocatalysis, enzymatic cascades) | partial | ✔ | partial (proofreading) |
| Biological regulatory networks | expression level | ✔ (ultrasensitivity) | ✔ (cooperativity, Hill coefficient) | ✔ | ✔ | ✔ (proofreading, repair) |
| Mechanical metamaterials | discrete stable configuration | ✔ (energy barrier) | ✖ (passive, no energy source) | ✖ | ✖ | ✖ |
| Topological phases | anyon charge / sector | ✔ (gap) | ✖ (passive) | ✖ in 2D at finite T | partial | ✔ under active correction |
| Dissipative quantum systems | steady state | ✔ (Liouvillian gap) | ✔ (dissipation supplies the energy) | ✔ | ? | ✔ (autonomous) |

> **PROPOSITION 17.13 (correcting audit item #8).** QEC is **not** the only constructed
> instance of the digital abstraction outside classical electronics. Biological
> regulatory networks exhibit all nine properties — robust variables with
> ultrasensitive thresholds, gain via cooperativity, restoration, fan-out, and
> error correction via proofreading and repair — and predate every engineered
> instance. **Classification: STRONGLY SUPPORTED.** What is true of QEC and not of
> biology is that QEC's properties are *designed to specification with a provable error
> threshold*, which is a different and narrower claim.

> **OPEN PROBLEM 17.14 (Physical Logic Universality).** Characterize the substrates
> admitting all nine properties. Conjecture: **gain requires an energy source and an
> open system** — passive substrates (mechanical metamaterials, undriven topological
> phases) cannot restore, hence cannot compose indefinitely. If true, this is a sharp
> and testable dividing line, and it explains why every composable substrate on the
> list above is driven or dissipative.

---

## 17.10 The falsification program (twenty tests)

Extending F-1…F-12 (Parts III–IV). Each states the target claim and the falsifying
observation.

| # | Test | Falsifies |
|---|---|---|
| **F-13** | Find a substrate satisfying R, Rch, Ver, Res, MV, St yet admitting no compositional compilation (Cmp fails for every intermediate specification one can write) | Feature-lattice completeness (§17.3) |
| **F-14** | Measure κ vs total verified cost across ≥30 targets, ≥4 substrates; require rank correlation \|ρ\| ≥ 0.6 | Conjecture D′ (§16 B.4) |
| **F-15** | Exhibit a highly compressible target requiring exponential preparation **that a compiler nonetheless reports FEASIBLE** | Soundness of the feasibility screen (not the thesis — CE-3 already settles the thesis) |
| **F-16** | Exhibit a highly verifiable state that is uncontrollable | *Already satisfied by CE-2 — this is a confirmed separation, not an open test* |
| **F-17** | Exhibit a controllable state whose verification provably requires exponential resources | *Already satisfied by CE-1* |
| **F-18** | Find a landscape whose attractor is stable but whose preparation time is exponential, and check whether 𝔄 (§8.10) predicts the failure | Adequacy of the six-axis figure of merit |
| **F-19** | Compute Λ (Def. 16.2) under three different declared reference measures for the same compilation; require rank order of routes preserved | Usability of Λ as a comparative measure |
| **F-20** | Find a quantum target where no low-complexity acceptance region captures the scientifically relevant property | Representability (R) for quantum matter; would restrict 𝒮 sharply |
| **F-21** | Find a case where model discrimination is impossible within budget — two models in 𝔐 predicting identical values on every available measurement | Completeness of the MEASURE branch; forces a fourth verdict (UNDECIDABLE-IN-PRINCIPLE) |
| **F-22** | Find a refinement chain where each step is locally sound and the composite is not | *Already satisfied by CE-6 — the open question is how often this occurs in practice* |
| **F-23** | Find a case where verification cost exceeds all physical preparation costs by >10× | *Expected to succeed routinely; the test is whether the compiler predicts it in advance* |
| **F-24** | Show Λ depends pathologically on representation — rank order of routes inverts under a reasonable change of reference measure | Definition 16.2 |
| **F-25** | Run the three-backend prototype: does one frontend semantics genuinely express quantum, chemical, and geometric targets without substrate-specific escape hatches? | **Conjecture A′** — the framework's central claim |
| **F-26** | Measure the MEASURE rate on a realistic specification corpus; require < 60% | Proposition 17.9 non-triviality |
| **F-27** | Adversarial specification suite (§ prototype tests): every ill-formed or physically impossible specification must produce a *typed* diagnostic, never a silent COMPILE | Type-system soundness |
| **F-28** | Compile-for-verifiability A/B: joint (π,Ψ) optimization vs sequential, measured cost-to-certify | Candidate B (the framework's strongest differentiator) |
| **F-29** | Landscape vs trajectory compilation head-to-head on the same target with matched effort | Conjecture C′ |
| **F-30** | Provenance replay: can an independent party recompute the acceptance decision from Π alone and reach the same verdict? | Machine-checkability of the physical certificate (Candidate G) |
| **F-31** | Cross-substrate frontend transfer: take a specification written for backend A, retarget to backend B, and check that the *semantics* (acceptance region, evidence obligations) are preserved even though no plan exists | Universality of the interface, independent of compilability |
| **F-32** | Calibration of DERIVE uncertainties on out-of-distribution physical tests; require 90% intervals to cover 85–95% | **MV**, and with it every model-based guarantee in the framework |

**F-32 remains the single most important experiment**, because Corollary 17.8 makes
every DERIVE-mode guarantee contingent on it.

---

## 17.11 Novelty ranking by evidence category

Not scored. Grouped by the *strength of the prior-art distinction*, after the audits in
Parts III–IV and here.

**Category 1 — Distinction is clear and defensible.**

- **Candidate B, Compile-for-verifiability**, *in the specific form* of joint (π, Ψ)
  synthesis inside one objective across substrates. Prior art: design-for-testability
  (~1973, VLSI only), design for inspection, optimal experimental design. **Exact
  distinction:** existing practice designs the test *after* the artifact, by a
  different team, with no shared correctness contract. **Unresolved overlap:**
  test-cost-aware synthesis in VLSI is close and should be read carefully.
  **Publishable:** yes, if F-28 shows an effect. **Implementation:** in the prototype.
- **Candidate G, Machine-checkable physical provenance.** Prior art: in-toto, SLSA,
  C2PA, GUM uncertainty budgets, ALCOA+ data-integrity requirements. **Exact
  distinction:** a typed provenance *graph* whose replay reproduces the acceptance
  decision, binding models, validity witnesses, and raw-data digests — not a log.
  **Unresolved overlap:** assurance cases / GSN come close. **Implementation:** in the
  prototype; F-30 tests it.
- **Corollary 17.11 (three-valued outcome is forced).** Prior art: three-valued logics,
  abstention in classification, conformant planning. **Exact distinction:** the
  derivation that MEASURE is forced by undecidability plus certificate hardness, rather
  than adopted for convenience. **Publishable:** yes, as a short formal note.

**Category 2 — Distinction is real but narrow.**

- **Candidate D, Substrate compilability criteria** (§17.2–17.3). Prior art: none
  directly, but each condition is standard in its own field. **Distinction:** the
  dependency analysis and the minimal set (Prop. 17.5), especially that MV is
  excluded. **Risk:** may be judged bookkeeping.
- **Candidate F, Model-discrimination compilation.** Prior art: Bayesian optimal
  experimental design, active learning, self-driving labs. **Distinction:** refusing
  to compile when model uncertainty dominates the acceptance margin — a *compiler
  error*, not an experiment-selection heuristic. **Implementation:** in the prototype.
- **Candidate E, Typed physical IR.** Prior art: OpenFermion, Qiskit Nature, pulse-level
  toolchains, ChemDraw/RXN formats, STEP/GD&T. **Distinction:** the L0 layer
  (acceptance regions, alternative-hypothesis sets, evidence modes) and dimensional +
  conservation + entropy typing.

**Category 3 — Primarily synthesis; claim accordingly.**

- **Candidate A, Physical State Compilation as a cross-domain abstraction.** Value is
  real; novelty is organizational.
- **Candidate C, Landscape compilation.** "Energy landscape engineering" and inverse
  statistical mechanics are established terms.
- **Candidate H, Compositional physical refinement.** The machinery is standard formal
  methods; the physical sufficiency condition (Prop. D.3) is the distinction.
- **Candidate I, Physical computational leverage.** Now Λ; representation-relative;
  useful comparatively.
- **Candidate J, Quantum Physical Synthesis as a first-class target.** Reframing, not
  invention — and the reframing has already paid (Part XI), but it is not a result.

---

## 17.12 The critical question

> **Is Physical State Compilation a new computational paradigm, or a useful vocabulary
> for inverse design, optimal control, scientific computing, and manufacturing
> automation?**

**Answer: it is a vocabulary, plus exactly one structural move — and the move is the
work.**

Strip the framework of the move and what remains is a recombination: inverse design
supplies the search, optimal control supplies the trajectories, formal methods supply
refinement, metrology supplies the statistics, and EDA supplies the pipeline metaphor.
Nothing in that recombination is new, and a reviewer who says so is right.

The move is this:

> **Verification, model validity, and refinement are placed inside the compiler's
> correctness contract.** The compiler's output is not an artifact. It is a triple
> (process, test, evidence) whose false-accept probability is bounded *relative to a
> declared model bundle*, with the relativity exposed in the record rather than hidden.

No source discipline does this. EDA has design-for-testability but verification is a
separate flow with its own team and no unified contract. Optimal control has no
verification contract at all. Formal methods have refinement without physical
measurement. Metrology has evidence without synthesis. Self-driving labs have the loop
without the contract.

**Therefore the centre of the work is the contract**, and the manuscript should be
organized around it rather than around the compilation metaphor. Everything else —
the IR, the landscape formalism, the leverage measure, the quantum-matter reframing —
is scaffolding that makes the contract expressible and checkable.

---

## 17.13 The three research questions, answered as far as they can be

**Q1. Under what conditions can physical reality, restricted to a declared substrate,
be treated as a programmable dynamical system with compiler semantics?**

> **Answer.** When {R, Rch, Ver, Res} hold for a non-empty specification class
> (Proposition 17.5), plus MV for any model-based claim, St for lifetime claims, Cmp
> for multi-step and Ref for multi-level. The resulting guarantee is
> **sound relative to 𝔐 and never relative to reality** (Corollary 17.8), and the
> compiler is necessarily three-valued (Corollary 17.11). **Compilability is a property
> of a (substrate, specification class, budget) triple, not of a substrate.**

**Q2. Can a desired quantum many-body phase be compiled into the laws, controls,
environment, and verification that make its emergence reliable?**

> **Answer.** For targets in CQM(Ω) (Def. 39.1) — those with a finite witness set
> against a declared alternative set, an efficient realizable parent Hamiltonian, a
> poly-time preparation route, and poly-sample witness estimation — **yes, and it has
> been demonstrated** for Abelian and non-Abelian topological order, Hubbard physics,
> and programmable spin models. For targets outside it, **no**, and the exclusions are
> characterized: generic states (counting), non-local parent Hamiltonians, exponential
> preparation depth (CE-3), and phases with no finite-size witness. The class's size in
> d ≥ 2 depends on the area law **and, separately, on efficient contractibility**,
> which does not follow from the area law — a gap Draft 3 elided (audit item #17).

**Q3. Can compilation itself become a physical design process in which the substrate
performs the majority of the computation through its own dynamics?**

> **Answer.** Partially, and Proposition 16.1 bounds how much. The substrate's fixed
> structure is amortized across a target family, not supplied per target: the compiler
> cannot deliver more specification information than its control program carries
> relative to a fixed Ω. So "the substrate does the computation" is true in the sense
> that **the control program's length is exponentially smaller than the product's
> description** (the κ ≪ 1 regime), and false in the sense that **information is being
> created**. The leverage is real, amortized, and bounded — measured by Λ
> (Definition 16.2), tested by F-14, F-19, F-24, F-29.

---

## 17.14 Prototype status

A working prototype implementing Proposition 17.5's minimal condition set
{R, Rch, Ver, Res} is in [`../psc/`](../psc/). It runs, and its findings are in
[`../psc/docs/FINDINGS.md`](../psc/docs/FINDINGS.md).

**What it establishes.** One frontend semantics (`Spec`/`Requirement`/`Estimand`/
`Mode`) targets a quantum spin system, a chemical reaction network, and a geometric
manufacturing problem with no substrate-specific escape hatches in the specification
layer. That is the first positive evidence for **Conjecture A′** (§16 B.1). Thirty
adversarial tests pass, every obstruction carries a re-checkable certificate, and
provenance replay reproduces the acceptance verdict.

**What it does not establish, and two corrections it forced.**

1. **F-28 must be re-run.** The verifiability benchmark first returned a null, the
   witness model was then corrected, and the re-run returned a pass. That sequence
   invalidates the result regardless of the correction's merit. The 70% figure is not
   citable; a pre-registered re-run is required.
2. **Compile-for-verifiability has an unstated precondition.** It can only pay when
   witness discriminating power *varies over the reachable observable space*. A
   substrate with one dominant characterisation method offers nothing to optimise.
   This belongs in §9.9 and in F-4/F-28's protocol, and it was absent from Drafts 1–4.
3. **A new open problem: degenerate solutions.** The compiler returned the classical
   limit (h = 0) of the quantum specification — satisfying every requirement while
   containing no quantum physics. Acceptance regions bound what must be true and not
   what must be non-degenerate. Every inverse-design system shares this vulnerability
   and the framework has no answer to it.
