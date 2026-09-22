# Audits for Parts III–IV

Claim audit table · Falsification program · Novelty audit · Adversarial review ·
Literature additions

---

## A. Claim audit table

Every load-bearing claim in Parts III–IV. "Falsifier" states what observation or proof
would overturn it.

| # | Claim | Classification | Assumptions | Evidence | Falsifier |
|---|---|---|---|---|---|
| **III-1** | INFEASIBLE requires an independently checkable certificate; eight classes supply one | PLAUSIBLE (architectural); the certificate constructions are ESTABLISHED | Certificates checkable in poly time in their own size | LP duality, barrier certificates, LARC, indistinguishability arguments | A compilation problem class where no certificate form exists yet infeasibility is provable |
| **III-2** | INFEASIBLE and UNKNOWN must never be conflated; unsoundness is the failure to avoid | ESTABLISHED (logic) | — | — | Not falsifiable; definitional |
| **III-3** | The composite PSTE problem has no general solution algorithm | ESTABLISHED | Finite-horizon POMDP policy existence is PSPACE-complete; continuous case undecidable | Papadimitriou–Tsitsiklis; hybrid-automata undecidability | A general algorithm with proven guarantees at useful scale |
| **III-4** | Two-stage decomposition forfeits global optimality with no available bound | ESTABLISHED (bilevel optimization) | Discrete surrogate not order-consistent with the optimized objective | Standard | A surrogate class proven order-consistent for physical route search |
| **III-5** | The specification is a constraint, not a weighted cost term | ESTABLISHED as practice; PLAUSIBLE as a universal design rule | Regulatory and contractual structure of real specifications | GD&T, ASTM, ICH, process windows | A domain where partial specification satisfaction is genuinely tradeable against cost |
| **III-6** | CVaR is not a universal default; it inherits the error of the least-validated part of 𝔐 | PLAUSIBLE | Model tails are least validated | Coherent-risk-measure theory; standard model-validation practice | Evidence that tail models in this domain are as well validated as bulk models |
| **III-7** | Chance constraints at δ ≤ 10⁻⁶ require structural knowledge of the failure mode | ESTABLISHED | Crude MC relative error ~1/√(Np) | Rare-event simulation literature | A general variance-reduction method needing no structural input |
| **III-8** | MEASURE is a correct compiler outcome, not a failure | ESTABLISHED as decision theory; PLAUSIBLE as compiler architecture | EVOI criterion | Lindley; Howard; self-driving labs | A setting where committing always dominates measuring |
| **III-9** | Basin volume is an inadequate robustness descriptor | ESTABLISHED | Measure-dependence; barrier-governed escape; high-dimensional concentration | Freidlin–Wentzell; Kramers | — |
| **III-10** | 𝔄 = (Δ, B, τ, κ, Γ, D) — six axes suffice and are the right six | PLAUSIBLE (proposed) | Individual trade-offs are ESTABLISHED | — | **F-6**: a substrate where the six fail to predict attractor performance |
| **III-11** | Most engineering targets are metastable, not attracting | ESTABLISHED as description | — | Steel, glass, diamond, polymorphs, Floquet phases | — |
| **III-12** | The ground state is not an attractor of unitary dynamics | ESTABLISHED | Closed system | Spectral invariance | — |
| **III-13** | **RETRACTED:** sub-exponential τ_relax is the sole criterion for landscape advantage | **CONTRADICTED** (by Draft 2's own materials examples) | — | Metastable products | Already falsified; retained as a correction record |
| **III-14** | Replacement: total verified cost/time (C_PSE, T_PSE) with amortized setup | ENGINEERING CRITERION (not a theorem) | Costs commensurable in one numeraire | — | **F-3**: a case where C_PSE ranks routes worse than τ_relax does |
| **III-15** | τ_relax still binds when the specification requires the true attractor | ESTABLISHED within scope | No metastable member of [S] is acceptable | — | — |
| **III-16** | "Physics computes for free" is ILL-POSED; physical evaluation ≠ algorithmic speedup | ESTABLISHED (logical) | A speedup claim needs a named computational problem both procedures solve | — | A synthesis task reformulable as a computational problem where the physical route provably beats the best classical algorithm |
| **III-17** | Of nine landscape cases, two support computational-advantage claims and one a depth separation | STRONGLY SUPPORTED | Per-case scoping in §8.8.2 | Per-case citations | New evidence in any of the six control-leverage rows |
| **III-18** | ℒ = I_verified/(K(π*)+C_ver) measures physical computational leverage | PLAUSIBLE (proposed); representation-relative | A declared representation | — | **F-14**: κ/ℒ fails to predict cost-to-compile |
| **III-19** | **Draft 1's "verifying (a) is QMA-hard" WITHDRAWN.** The relevant problem (APX-SIM) is P^QMA[log]-complete | ESTABLISHED | 5-local H, 1-local observable, 1/poly promise gap | Ambainis 2014 (hardness proof gap later repaired); Gharibian–Yirka; Gharibian–Piddock–Yirka | — |
| **III-20** | GSCON (ground-state connectivity) ranges QCMA-complete to PSPACE-complete, NEXP-complete succinctly | ESTABLISHED | Path length and error parameters | Gharibian–Sikora | — |
| **III-21** | All hardness results bound algorithms over arbitrary instances, not physical achievability | ESTABLISHED (scope) | — | — | Not falsifiable; a statement about what theorems say |
| **III-22** | The five-level hierarchy is ordered by κ, not by capability | PLAUSIBLE (proposed) | κ well-defined in a declared representation | Tile complexity Θ(log N/log log N) | **F-14** |
| **III-23** | Emergence compilation is established in exactly one substrate (tile assembly) | STRONGLY SUPPORTED | Comprehensive search not performed | aTAM universality; DNA demonstrations | A general global→local rule compiler in another substrate |
| **IV-1** | Quantum matter is better posed than structural materials on 3 of 4 compilability criteria | PLAUSIBLE | The criteria themselves are a proposal | §9.2.1 comparison | Criteria shown to be the wrong ones |
| **IV-2** | Quantum matter escapes the Avogadro constraint | ESTABLISHED | A phase is a correlation structure | 6,100-site arrays; finite-size scaling practice | — |
| **IV-3** | Spectral invariance holds for closed systems only | ESTABLISHED (scoped) | Closed, full state, no measurement, exact unitarity | Elementary | — |
| **IV-4** | Entropy removal, not control fidelity, binds cold-atom many-body preparation | STRONGLY SUPPORTED | — | Fermi–Hubbard antiferromagnet experiments | Demonstration of the target entropy by purely coherent means |
| **IV-5** | Thermomajorization is NOT a universal reachability law | ESTABLISHED (scoping) | Block-diagonal, exact single-shot, thermal operations, no catalysts | Resource-theory literature | — |
| **IV-6** | "Any state can be a steady state" is ILL-POSED without a generator class; bounded-weight quasi-local dissipators reach essentially frustration-free targets | ESTABLISHED (sufficiency); **OPEN** (necessity) | Bounded jump-operator weight | Verstraete–Wolf–Cirac; Poyatos–Cirac–Zoller | Characterization proving a wider preparable class |
| **IV-7** | Trap-free quantum control landscapes are CONTESTED under realistic constraints | CONTESTED | Unconstrained amplitude/bandwidth, surjective end-point map | Constrained-control counterexample literature | Resolution of the dispute either way |
| **IV-8** | Integer estimands give O(1) verification margin | ESTABLISHED | The specification names a quantized invariant | Quantized Hall conductance to 1 part in 10⁹ | — |
| **IV-9** | **Drafts 1–3 overstated submodularity.** Coverage formulation is submodular; information formulation is not in general | ESTABLISHED | Conditional independence for the MI case | Coverage functions; weak-submodularity literature | — |
| **IV-10** | Alt(S) must be a mandatory part of a phase specification | ENGINEERING CRITERION | — | The retraction record in phase-discovery claims | A phase claim robustly established without an explicit alternative set |
| **IV-11** | PDfV is architectural synthesis, not invention; DFT is 40-year prior art | ESTABLISHED | — | Scan chains, BIST, design for inspection, design for metrology | — |
| **IV-12** | What is new in PDfV: verification cost inside an automated cross-substrate synthesis objective, and the quantum-matter application | PLAUSIBLE | Comprehensive prior-art search not performed | — | Prior art showing either |
| **IV-13** | Passive 2D topological memory at finite T is forbidden; 3D is open | ESTABLISHED (2D, 4D); OPEN (3D) | — | Thermal instability of the 2D toric code; 4D stability | A 3D self-correcting construction |
| **IV-14** | Non-Abelian topological *order* is demonstrated; non-Abelian *hardware qubits* are not | ESTABLISHED | The distinction is between engineered order and an intrinsic material phase | D₄ on ions; Fibonacci on superconducting | — |
| **IV-15** | No known theorem bounds T_c | ESTABLISHED (absence of a theorem) | — | McMillan-type limits were empirical and were exceeded | A proof of an upper bound |

---

## B. Falsification program

Twelve experiments. Each states hypothesis, setup, variables, baseline, success and
failure criteria, confounders, expected result, and consequence for the theory.

---

### F-1 — PSTE feasibility prediction

- **Hypothesis.** The engine's FEASIBLE/INFEASIBLE/UNKNOWN verdicts agree with
  experimental outcome at ≥85%, and **no INFEASIBLE verdict is ever overturned**.
- **Setup.** 150 pre-registered specifications across ≥2 substrates (one classical, one
  quantum-matter), spanning feasible, infeasible, and borderline cases. Attempt every
  case experimentally, including those declared INFEASIBLE.
- **Independent variable.** Specification difficulty (distance from known-feasible).
  **Dependent.** Verdict accuracy; certificate validity.
- **Baseline.** Expert judgment by three domain scientists, blinded to the engine.
- **Success.** ≥85% agreement on FEASIBLE/INFEASIBLE; **zero overturned INFEASIBLE
  verdicts**; UNKNOWN rate ≤40% and concentrated on genuinely borderline cases.
- **Failure.** Any overturned INFEASIBLE with a valid certificate (an unsoundness
  bug), or <65% agreement, or UNKNOWN uncorrelated with difficulty.
- **Confounders.** Experimental failures mistaken for true infeasibility — every
  negative must be attempted ≥3 times by ≥2 operators. Selection of easy cases.
- **Expected.** ~70–80% agreement; unsoundness rate near zero if certificates are
  enforced; high UNKNOWN rate early.
- **If it fails.** A single overturned certificate falsifies the certificate
  discipline of §7.2.1 and means INFEASIBLE cannot be trusted — the engine's most
  valuable output collapses to advisory.

---

### F-2 — Route ranking and the cost of decomposition

- **Hypothesis.** Two-stage decomposition returns routes whose realized cost is within
  25% of the best route found by an unrestricted joint search given 10× the compute.
- **Setup.** 30 specifications; run (a) two-stage, (b) mixed-integer joint
  optimization with a large budget. Execute the top-3 from each experimentally.
- **Baseline.** Expert-designed route.
- **Success.** Median realized-cost gap ≤25%, and the decomposed top-3 contains the
  joint-search optimum in ≥60% of cases.
- **Failure.** Gap >60%, or the decomposed set misses the optimum in >70% of cases.
- **Confounders.** Compute budget asymmetry; surrogate quality dominating the result.
- **Expected.** 15–40% gap, highly instance-dependent.
- **If it fails.** Proposition 7.6's optimism is unwarranted; the engine must carry a
  joint-optimization back end for high-value targets, at large compute cost.

---

### F-3 — Landscape versus explicit trajectory control

- **Hypothesis.** C_PSE (total verified cost, §8.13) ranks landscape and trajectory
  routes correctly, and ranks them **differently from τ_relax** in the metastable-target
  cases.
- **Setup.** 12 targets: 6 requiring the true attractor, 6 admitting a metastable
  member of [S]. For each, develop both a landscape route and a trajectory-control
  route; execute both; measure realized total cost and time including verification and
  failures.
- **Independent variable.** Whether [S] admits a metastable member; production volume
  (1, 10², 10⁴ units, by amortization modelling).
- **Success.** C_PSE predicts the cheaper route in ≥80% of cases; τ_relax predicts it
  in ≤60% of the metastable cases, demonstrating the retraction of III-13 was
  warranted.
- **Failure.** τ_relax matches or beats C_PSE — the retraction was unnecessary.
- **Confounders.** Unequal engineering effort on the two routes; setup cost allocation.
- **Expected.** C_PSE wins clearly on metastable targets; the two agree on
  true-attractor targets.
- **If it fails.** §8.7.3's replacement criterion is not an improvement and should be
  reverted.

---

### F-4 — Verification-aware route selection

- **Hypothesis.** Joint (π, Ψ) optimization reduces cost-to-certify by ≥30% at equal
  attained confidence, with ≤10% production-cost penalty.
- **Setup.** 10 real specifications. Two routes each: cost-only, and jointly optimized
  with λ_V > 0. Execute both; certify both to δ = 0.05 with optimal measurement plans;
  measure realized cost-to-certify and attained confidence by held-out testing.
- **Success.** Median ≥30% reduction, penalty ≤10%. **Marginal** 10–30%.
- **Failure.** <10% reduction, or the production penalty exceeds the verification
  saving.
- **Confounders.** The joint route being *better* rather than more verifiable —
  control by matching production cost. Attained confidence must be measured, not
  assumed.
- **Expected.** 20–50% in substrates with distinctive signatures; near zero where all
  routes share a verification method.
- **If it fails.** PDfV is true but immaterial; the framework's distinctive economic
  claim does not cash out.

---

### F-5 — Active model discrimination

- **Hypothesis.** The engine's selected experiment e* reduces posterior model
  uncertainty more per unit cost than (a) expert-chosen experiments and (b) random
  admissible experiments.
- **Setup.** 40 cases where ≥2 models in 𝔐 disagree beyond tolerance. Run e*, an
  expert choice, and a random choice. Measure posterior entropy reduction per unit
  cost.
- **Success.** e* beats random by ≥3× and matches or beats expert choice, in ≥70% of
  cases.
- **Failure.** e* fails to beat random, or is beaten by expert choice in >60% of cases.
- **Confounders.** Model set 𝔐 not containing the truth — record whether the
  discriminated winner survives later evidence. Cost model errors.
- **Expected.** Beats random comfortably; roughly matches experts, winning on
  throughput rather than insight.
- **If it fails.** The MEASURE branch has no advantage over existing practice and is
  an architectural relabeling, not a capability.

---

### F-6 — Attractor robustness prediction

- **Hypothesis.** 𝔄 = (Δ, B, τ, κ, Γ, D) predicts realized yield and service-life
  survival better than any single component, and better than basin volume.
- **Setup.** ≥25 attractor-compiled targets across ≥2 substrates (e.g. polymorph
  selection and a dissipatively stabilized quantum state). Measure all six components
  independently, then measure realized yield and accelerated-aging survival.
- **Success.** The six-vector model achieves out-of-sample R² ≥ 0.6 for yield, beating
  the best single component by ≥0.2 and basin volume by ≥0.3.
- **Failure.** R² < 0.3, or a single component does as well as all six — the
  multidimensional descriptor is unnecessary.
- **Confounders.** Components hard to measure independently (τ and Δ are related for
  normal dynamics); limited target diversity.
- **Expected.** B and Γ dominate; κ matters for parameter-drift-limited processes; D
  matters only for quenched targets.
- **If it fails.** Proposition 8.11 is wrong and 𝔄 should be reduced.

---

### F-7 — Quantum-Matter IR compilation

- **Hypothesis.** An automated compiler maps L0/L1 specifications to realizable L3
  parameters, producing the specified phase on first attempt in ≥70% of cases, for
  ≥20 targets including ≥3 outside the training distribution.
- **Setup.** A programmable simulator (Rydberg array or optical lattice). Specify 20
  targets at L0/L1; compile; execute; verify by the compiler's own emitted witness set.
- **Baseline.** Hand-derived parameters by a domain expert.
- **Success.** ≥70% first-attempt, including ≥3 novel; compiled parameters within the
  expert's tolerance in ≥80% of overlapping cases.
- **Failure.** <40%, or success confined to targets already in the literature
  (retrieval, not compilation).
- **Confounders.** Literature leakage — the novel cases must be genuinely absent from
  training data and pre-registered.
- **Expected.** 50–70%, with failures concentrated on frustrated and gapless targets.
- **If it fails.** The Q-IR is a notation, not a compiler.

---

### F-8 — Dissipative state preparation versus coherent preparation

- **Hypothesis.** For targets within the frustration-free class, dissipative
  preparation achieves higher fidelity per unit wall-clock time than the best coherent
  protocol, and is more robust to parameter drift.
- **Setup.** ≥6 targets on a platform with engineered dissipation (superconducting or
  trapped-ion). Compare dissipative, adiabatic, and optimal-control preparation.
  Deliberately detune parameters by 1%, 5%, 10% and re-measure.
- **Success.** Dissipative wins on fidelity-per-time in ≥4 of 6, and degrades more
  slowly under detuning in ≥5 of 6.
- **Failure.** Dissipative loses on both axes, or Δ_L proves uncontrollably small.
- **Confounders.** Engineered-dissipation overhead not counted in wall-clock time;
  unequal tuning effort.
- **Expected.** Dissipative wins decisively on robustness, less clearly on speed.
- **If it fails.** §9.6's strategic preference for dissipative routes is unfounded.

---

### F-9 — Physical design-for-verifiability in quantum matter

- **Hypothesis (sharpened F-4 for quantum targets).** Among physically equivalent
  routes, those specified by quantized/topological observables cost ≥10× less to
  certify at equal confidence than those specified by local order parameters.
- **Setup.** ≥6 target pairs where the same physics admits both a quantized witness
  and a local-order-parameter witness. Certify both to δ = 0.05; count copies,
  wall-clock, and apparatus cost.
- **Success.** Median ≥10× reduction. **Marginal** 3–10×.
- **Failure.** <3×.
- **Confounders.** Quantized witnesses may require different apparatus with its own
  capital cost — amortize explicitly.
- **Expected.** 5–50×, dominated by the O(1) margin of integer estimands.
- **If it fails.** Proposition 9.11 is correct in theory and does not dominate real
  verification budgets.

---

### F-10 — Compositional certification

- **Hypothesis.** For multi-step routes, realized end-to-end false-accept rate is
  bounded by Σδ_i + Σ L_i·diam(𝒦_i) as predicted by the composition theorem (§D
  below), and the non-uniformity term is measurable.
- **Setup.** ≥8 three-step routes with certified intermediates. Measure per-step
  false-accept rates and end-to-end rates on ≥200 articles per route. Independently
  estimate the Lipschitz constants L_i by perturbing intermediates within 𝒦_i.
- **Success.** Realized end-to-end rate ≤ predicted bound in ≥90% of routes, and the
  bound is not vacuous (within 5× of realized).
- **Failure.** Realized exceeds the bound (the theorem's hypotheses fail in practice),
  or the bound exceeds realized by >100× (vacuous).
- **Confounders.** Insufficient articles to estimate rare false-accepts — use stress
  conditions to inflate rates, with a declared extrapolation model.
- **Expected.** The bound holds but is loose; the sufficiency hypothesis will fail for
  at least one route, which is the informative outcome.
- **If it fails (bound violated).** Intermediate specifications are not sufficient
  statistics, and multi-step routes cannot be certified compositionally — forcing
  end-to-end validation and destroying the framework's scaling story for multi-step
  synthesis. **This is the highest-stakes experiment for the framework's long-run
  architecture.**

---

### F-11 — Alternative-hypothesis sufficiency

- **Hypothesis.** Specifications carrying an adversarially reviewed Alt(S) produce
  fewer overturned phase/property claims than specifications without one.
- **Setup.** Retrospective: take ≥40 published claims of novel phases or materials,
  half later disputed. Reconstruct the implicit Alt(S) in each. Test whether the
  disputed claims systematically had smaller or less adversarial alternative sets.
  Prospective: 20 new claims, half with mandated adversarial Alt(S) review.
- **Success.** Statistically significant association (p < 0.01) between Alt(S)
  adequacy and claim survival.
- **Failure.** No association — alternative-set discipline does not predict claim
  durability.
- **Confounders.** Hindsight bias in reconstructing Alt(S) — reconstruction must be
  blinded to outcome.
- **Expected.** Strong association; this is the mechanism behind most retractions.
- **If it fails.** §9.15's mandatory Alt(S) requirement is bureaucratic overhead.

---

### F-12 — Adaptive preparation at realistic measurement latency

- **Hypothesis.** Constant-depth measurement-plus-feedforward preparation beats the
  best unitary protocol in *final fidelity at equal wall-clock time* on real hardware,
  with the margin growing with system size.
- **Setup.** ≥3 long-range-entangled target classes at ≥2 system sizes on a platform
  with fast mid-circuit measurement (trapped-ion or superconducting). Include
  measurement latency, readout error, and classical feedforward delay in the
  wall-clock accounting.
- **Success.** Adaptive wins on ≥3 target classes at both sizes, with the margin
  increasing with size.
- **Failure.** Latency and readout error consume the depth advantage at all accessible
  sizes.
- **Confounders.** Unequal optimization effort; hardware chosen to favor one route.
- **Expected.** Adaptive wins at larger sizes; the crossover is the interesting
  measurement.
- **If it fails.** The asymptotic theorem is true and irrelevant at realizable scale,
  and §32.3's polynomial-advantage row does not cash out in practice.

---

## C. Novelty audit

For each concept: is it already established, what are the close precedents, what is
genuinely new, what is synthesis, and what should be renamed.

| Concept | Already established? | Close precedents | Genuinely new | Merely synthesis | Terminology recommendation |
|---|---|---|---|---|---|
| **Physical State Compilation** | The *idea* yes, under other names | Inverse design; design automation; self-driving labs; closed-loop synthesis; EDA; CAM; retrosynthesis | The unification **with an evidence calculus** and a three-valued outcome | Most of it | Keep. It is descriptive and does not overclaim |
| **Reality Compiler** | — | — | Nothing | — | **Drop from technical writing.** It asserts that the object of compilation is reality, which is false. Retain as a program name only |
| **Attractor Compilation** | Components yes | Inverse statistical mechanics for targeted self-assembly; reservoir engineering (Poyatos–Cirac–Zoller 1996); dissipative state preparation; control-Lyapunov design; energy-landscape design in protein folding | The unified statement **across classical and quantum with a common six-axis figure of merit and an explicit verifiability condition (A6)** | The individual mechanisms | Keep, but cite the precedents prominently. Do not present the mechanism as new |
| **Landscape Compilation** | **Yes** | "Energy landscape engineering" is established terminology in protein folding and self-assembly; "inverse statistical mechanics" (Torquato and others) | Nothing at the concept level | All of it | Consider renaming to **landscape design** to avoid implying novelty, or cite the existing term explicitly on first use |
| **Quantum-Matter IR** | Partially | OpenFermion, Qiskit Nature, Bloqade/Pulser and other analog-simulator toolchains; every L1→L2 pass exists | The **L0 layer** — acceptance regions, alternative hypothesis sets, scaling hypotheses — which existing toolchains lack | L1–L4 | Keep, but claim only L0 as a contribution |
| **Physical Design-for-Verifiability** | **Yes, in substance** | Design for testability (scan chains, BIST — 1960s–70s); design for inspection (aerospace NDT); design for metrology (semiconductor scribe-line structures); optimal experimental design; acceptance sampling | (i) verification cost inside an **automated cross-substrate synthesis objective**; (ii) the quantum-matter application | The principle | Keep the name but **state DFT prior art on first use**. Do not claim invention |
| **Epistemic type system (VERIFY/ASSUME/DERIVE)** | **Closely anticipated** | **GUM Type A vs Type B uncertainty** (Type A = statistical analysis of observations ≈ VERIFY; Type B = other means including models ≈ ASSUME/DERIVE) — an international metrology standard; assume-guarantee reasoning; proof-carrying code; assurance cases / GSN; model cards | The **three-way refinement as a compiler type system with mechanical errors** | The distinction itself | Keep, and **cite GUM explicitly** — alignment with an international metrology standard strengthens rather than weakens the contribution |
| **Knowledge-gap compilation (COMPILE/OBSTRUCT/MEASURE)** | The method yes | Bayesian optimal experimental design; value of information (Howard 1966); active learning; conformant/contingent and epistemic planning; self-driving labs | Making MEASURE a **typed compiler outcome** alongside success and failure | The measurement-selection mathematics | Keep, claiming only the typing |
| **Verification-aware synthesis** | Partially | DFT; test-cost-aware design in VLSI; measurement-constrained experimental design | Cross-substrate generalization | Most | Merge with PDfV; two names for one idea is a defect |
| **Compositional physical compilation** | The machinery yes | Assume-guarantee reasoning; probabilistic refinement; approximate bisimulation; simulation relations; Markov kernels | Application with **physical sufficient statistics**, and the identification of the sufficiency condition as the binding one | The formal machinery | Keep; cite the formal-methods precedents |

> **Summary of the novelty audit.** Of ten concepts, **none is a new mechanism**. Two
> (the L0 specification layer; the typed three-way compiler outcome) are plausibly new
> *architectural* contributions. One (the six-axis attractor figure of merit) is a new
> *unification*. The rest are existing ideas transported across substrates, and the
> transport is the value. **A manuscript claiming more than this is attackable on
> prior art alone, and the patent strategy should be scoped to the physical layer
> accordingly (§24).**

---

## D. Compositionality: the strongest honest theorem

Draft 1 asserted δ_total ≤ δ₁ + δ₂ + ε without specifying ε. Working it out changes
the conclusion.

> **THEOREM D.1 (Sequential composition, uniform case).** Let P₁ be a process-and-test
> pair with false-accept probability ≤ δ₁ with respect to 𝒦₁, and let P₂ have
> false-accept probability ≤ δ₂ with respect to 𝒦₂ **uniformly over every admissible
> input in 𝒦₁**. Then the composite P₂∘P₁ (accepting only if both accept) has
> false-accept probability ≤ δ₁ + δ₂.
>
> *Proof sketch.* Decompose the joint false-accept event on whether the intermediate
> lies in 𝒦₁. If it does, the failure is bounded by δ₂ from P₂'s uniform guarantee; if
> it does not, P₁ accepted an out-of-spec intermediate, bounded by δ₁. Union bound. ∎
>
> **So ε = 0 when the downstream guarantee is uniform on 𝒦₁.** **Classification:
> ESTABLISHED (elementary).**

> **PROPOSITION D.2 (Where ε comes from).** In practice P₂'s guarantee is validated at
> a *nominal* intermediate σ̂ ∈ 𝒦₁, not uniformly. If the false-accept functional
> FA₂(·) is Lipschitz with constant L in a metric d on the specified intermediate
> functionals, then
> ```
>   ε ≤ L · diam_d(𝒦₁)
> ```
> and for an n-step route, δ_total ≤ Σᵢ δᵢ + Σᵢ Lᵢ·diam(𝒦ᵢ).
> **ε is therefore the price of non-uniform validation, and it is measurable** — L by
> perturbing intermediates within 𝒦₁, diam by the specification itself.
> **Classification: PLAUSIBLE; the Lipschitz hypothesis must be checked per route
> (F-10).**

> **PROPOSITION D.3 (The binding condition is sufficiency, not Lipschitz).** Both
> results require that P₂'s outcome distribution depend on the intermediate **only
> through the specified functionals Φ₁** — i.e. the map (intermediate) → (P₂ outcome
> distribution) factors as a Markov kernel through Φ₁. If it does not, "input ∈ 𝒦₁"
> constrains P₂ not at all and **no bound whatsoever follows from the specification**;
> only end-to-end validation is available.
> **Classification: ESTABLISHED (measure-theoretic).**
> *This is the formal explanation for the industrial fact that chemical and
> pharmaceutical process routes must be validated end to end: the intermediate
> specifications are not sufficient statistics for downstream behavior.*

> **COROLLARY D.4 (The digital-abstraction condition).** If in addition each step is
> *contractive* on the specified metric (Lᵢ < 1), the non-uniformity terms form a
> convergent geometric series and arbitrarily long composition is possible with bounded
> total error. This is exactly why digital logic composes — voltage with noise margins
> is both sufficient and restoring — and it is the precise statement of Open Problem 1
> (does such a variable exist for non-electronic substrates?).

---

## E. Adversarial review

Eight hostile readings, with the response. Where the objection lands, it is converted
into a bounded research question rather than defended.

**A skeptical condensed-matter physicist.**
*"You are describing what experimentalists already do — tune parameters, measure order
parameters, do finite-size scaling — and calling it compilation. What does the
formalism buy?"*
**Response: the objection largely lands.** The formalism buys three things and no
more: mandatory alternative-hypothesis sets (§9.15), an evidence-coverage figure
distinguishing measured from modelled claims, and automated witness-set selection.
Whether those are worth the overhead is F-7 and F-11, not a matter of argument.
**Bounded question:** does the formalism improve claim durability (F-11) or first-
attempt success (F-7)?

**A control theorist.**
*"You have written down a POMDP over a stochastic hybrid system with distributional
uncertainty and rare-event chance constraints, then proposed solving it by a heuristic
two-stage decomposition with no suboptimality bound. That is not a contribution."*
**Response: correct, and now stated as such** (Prop. 7.6, Open Problem 7.7). The
contribution is not an algorithm; it is the problem formulation plus the certificate
discipline and the three-valued outcome. **Bounded question:** is there a decomposition
with a provable bound for any useful class (Open Problem 7.7)?

**A compiler researcher.**
*"Your 'IR' has no formal semantics, no correctness theorem for its passes, and no
account of what it means for a pass to be sound. LLVM has all three."*
**Response: the objection lands.** Draft 4 has semantics only for the specification
layer (§5.7) and refinement obligations only informally (§6.7). The L1→L2 passes have
*error bounds* from physics but no soundness proofs in the compiler sense.
**Bounded question:** state and prove a soundness criterion for at least one pass
(e.g. Trotterization with a commutator bound is the most tractable candidate).

**A complexity theorist.**
*"You cite QMA and undecidability results decoratively. Half of them concern promise
problems over arbitrary instances with 1/poly gaps and have no bearing on your
engineering claims."*
**Response: the objection landed in Drafts 1–3 and is addressed in §8.9.1.** Draft 1's
"verifying (a) is QMA-hard" was withdrawn; the correct result is P^QMA[log]-completeness
for APX-SIM, and GSCON supplies the result actually relevant to attractor routing.
Proposition 8.26 states the scope discipline explicitly. **Residual exposure:** the
P^NP[log] classification for the classical analogue is marked SOURCE VERIFICATION
REQUIRED and should not be cited until checked.

**A statistical decision theorist.**
*"Your objective mixes a sup over a model set with a tail-conditional risk measure,
which double-counts conservatism in an uncontrolled way; and your chance constraints at
δ = 10⁻⁶ are unestimable."*
**Response: both land, and both are now stated** (Prop. 7.10; Known Limit 7.11). The
recommendation is entropic risk with a declared KL radius, or explicit chance
constraints, over CVaR when 𝔐 is a set. **Bounded question:** what is the right
risk functional for single-shot high-consequence synthesis, and is there empirical
evidence for any choice?

**An experimental materials scientist.**
*"Your models will be out of distribution most of the time, your uncertainty estimates
will be overconfident, and your compiler will produce confident errors at high
throughput — which is exactly what happened to the autonomous-laboratory literature."*
**Response: this is the framework's most serious exposure** and it is why F-1
(calibration of DERIVE uncertainties, Draft 1 numbering) is the first experiment to
fund. The architectural response — mandatory validity-domain predicates, demotion of
out-of-domain DERIVE claims, and a coverage figure — is necessary and may not be
sufficient. **Bounded question:** can out-of-distribution calibration be achieved at
all in this domain?

**A quantum-information theorist.**
*"Your scoping of dissipative universality is still loose; the exact preparable class
under bounded-weight quasi-local dissipators is not characterized, and you assert
frustration-freeness as if it were necessary."*
**Response: correct.** §9.6 now states frustration-freeness as *sufficient* and marks
necessity OPEN with SOURCE VERIFICATION REQUIRED (Open Problem 9.23). **Bounded
question:** characterize the class.

**A manufacturing engineer.**
*"Nothing here addresses qualification cost, which is the actual barrier. A better
compiler that produces an unqualified process is worth nothing."*
**Response: the objection lands and is the reason §22.6 identifies qualification cost
as the dominant adoption barrier.** The framework's economic argument is precisely
that verification-cost reduction attacks that barrier — which is F-4's hypothesis.
If F-4 fails, this objection is fatal to the commercial thesis, though not to the
scientific one.

---

## F. Literature additions for Parts III–IV

Confidence and evidence conventions as in `CITATIONS.md`.

| Work | Type | Conf. | Relevance |
|---|---|---|---|
| Ambainis, A. (2014). "On physical problems that are slightly more difficult than QMA." *IEEE Conference on Computational Complexity*. | T | **[verified]** | §8.22. APX-SIM is P^QMA[log]-complete. A gap in the original hardness proof was subsequently repaired. |
| Gharibian, S., Yirka, J. (2019). "The complexity of simulating local measurements on quantum systems." *Quantum* 3:189. | T | **[verified]** | §8.22. P^QMA[log]-completeness for more physical Hamiltonians and for two-point correlation functions; repairs the Ambainis proof. |
| Gharibian, S., Piddock, S., Yirka, J. (2020). "Oracle complexity classes and local measurements on physical Hamiltonians." *STACS*. | T | **[verified]** | §8.22. Extension to physically realistic Hamiltonians. |
| Gharibian, S., Sikora, J. (2018). "Ground state connectivity of local Hamiltonians." *ACM Trans. Comput. Theory* 10(2) (ICALP 2015). | T | **[verified]** | §8.23. GSCON is QCMA-complete to PSPACE-complete, NEXP-complete succinctly. **The complexity result most directly relevant to attractor-compilation routing.** |
| Gosset, D., Mehta, J., Vidick, T. (2017). "QCMA hardness of ground space connectivity for commuting Hamiltonians." *Quantum* 1:16. | T | [medium] | §8.23. Strengthening for commuting Hamiltonians. |
| Papadimitriou, C., Tsitsiklis, J. (1987). "The complexity of Markov decision processes." *Math. Oper. Res.* 12(3):441–450. | T | [high] | §7.4. Finite-horizon POMDP policy existence is PSPACE-complete. |
| Poyatos, J. F., Cirac, J. I., Zoller, P. (1996). "Quantum reservoir engineering with laser cooled trapped ions." *Phys. Rev. Lett.* 77:4728. | T | [high] | §8.3, novelty audit. **The origin of reservoir engineering** — essential prior art for attractor compilation. |
| Kraus, B., Büchler, H. P., Diehl, S., Kantian, A., Micheli, A., Zoller, P. (2008). "Preparation of entangled states by quantum Markov processes." *Phys. Rev. A* 78:042307. | T | [high] | §9.6. Conditions for pure steady states of local Lindbladians. |
| Freidlin, M., Wentzell, A. *Random Perturbations of Dynamical Systems* (Springer, 3rd ed. 2012). | T | [high] | §8.5. Quasi-potential; escape rates ~ exp(−ΔV/ε) — the rigorous basis for the stability-radius component B. |
| Hänggi, P., Talkner, P., Borkovec, M. (1990). "Reaction-rate theory: fifty years after Kramers." *Rev. Mod. Phys.* 62:251. | R | [high] | §8.5, §8.6. Escape-rate theory for Γ. |
| Krause, A., Guestrin, C. (2005). "Near-optimal nonmyopic value of information in graphical models." *UAI*. | T | [medium] | §9.8.3. Conditions under which mutual information is submodular. |
| Das, A., Kempe, D. (2011). "Submodular meets spectral: greedy algorithms for subset selection, sparse approximation and dictionary selection." *ICML*. | T | [medium] | §9.8.3. Submodularity ratio γ and the (1−e^{−γ}) guarantee for weakly submodular objectives. |
| Nemhauser, G., Wolsey, L., Fisher, M. (1978). | T | [high] | §9.8.3. (1−1/e) for monotone submodular maximization — **valid for the coverage formulation only.** |
| JCGM 100:2008, *Evaluation of measurement data — Guide to the expression of uncertainty in measurement* (GUM). | S | [high] | Novelty audit. **Type A vs Type B uncertainty is direct prior art for VERIFY vs ASSUME/DERIVE.** |
| Williams, M. J. Y., Angell, J. B. (1973). "Enhancing testability of large-scale integrated circuits via test points and additional logic." *IEEE Trans. Computers* C-22(1):46–60. | E | [medium] | Novelty audit. Early design-for-testability; establishes ~50-year prior art for PDfV. |
| Torquato, S. (2009). "Inverse optimization techniques for targeted self-assembly." *Soft Matter* 5:1157. | T | [high] | Novelty audit. Inverse statistical mechanics — direct prior art for attractor/landscape compilation in the classical case. |
| Bukov, M., D'Alessio, L., Polkovnikov, A. (2015). "Universal high-frequency behavior of periodically driven systems: from dynamical stabilization to Floquet engineering." *Adv. Phys.* 64:139. | R | [high] | §8.2. Floquet–Magnus validity window and prethermal lifetimes. |
| Abanin, D., De Roeck, W., Ho, W. W., Huveneers, F. (2017). "Effective Hamiltonians, prethermalization, and slow energy absorption in periodically driven many-body systems." *Phys. Rev. B* 95:014112. | T | [medium] | §8.2. Exponentially long prethermal windows at high drive frequency. |
| Schulman, L. J., Vazirani, U. (1999). "Molecular scale heat engines and scalable quantum computation." *STOC*. | T | [medium] | §9.4.6. Limits on unitary entropy shuffling (algorithmic cooling). |
| Rabitz, H. et al. — quantum control landscape trap-freedom; and the constrained-control counterexample literature (e.g. de Fouquieres & Schirmer). | T | **[check]** | §9.4.5. **CONTESTED.** Locate both sides before citing; do not rely on trap-freedom. |

### Source-verification flags raised in Parts III–IV

1. **§8.25** — the P^NP[log] classification for the classical ground-state-property
   analogue is a reasonable inference from the APX-SIM construction, **not a cited
   theorem.** SOURCE VERIFICATION REQUIRED.
2. **§9.6 / Open Problem 9.23** — the precise necessary-and-sufficient characterization
   of pure states preparable by bounded-weight quasi-local dissipators. SOURCE
   VERIFICATION REQUIRED.
3. **§9.8.3** — the specific submodularity-ratio bounds for quantum measurement
   selection. SOURCE VERIFICATION REQUIRED.
4. **§9.4.5** — the trap-freedom dispute. Both sides must be located and represented.
5. **Platform figures in §9.10** have month-scale half-lives and must be re-checked at
   time of use.
