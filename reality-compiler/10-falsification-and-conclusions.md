# Part X — Falsification, Priorities, and Conclusions

Sections 25–28 and the Final Answers (A–J)

---

## 25. Falsification Program

A framework without failure conditions is not a theory. Each experiment below has a
hypothesis, a protocol, a quantitative pass/fail threshold, an estimated cost and
duration, and an explicit statement of what is refuted by failure. They are ordered
by how much of the framework each one puts at risk.

---

### F-1 — Are `DERIVE` uncertainties calibrated against physical reality?
**Puts at risk: the entire `DERIVE` mode, i.e. every predictive claim the compiler makes.**

- **Hypothesis.** The compiler's predicted 90% credible intervals for physical
  properties contain the measured value ~90% of the time on targets outside its
  training distribution.
- **Protocol.** Pre-register 100 predictions with intervals across ≥3 property
  classes and ≥2 material families deliberately chosen outside the training
  distribution. Synthesize and measure to standardized protocols in an independent
  laboratory. Report empirical coverage and the calibration curve.
- **Pass:** empirical coverage ∈ [85%, 95%]. **Marginal:** [75%, 99%].
  **Fail:** <75% (overconfident) or >99% (useless intervals).
- **Cost/duration:** $1.5–4M, 12–18 months.
- **If it fails:** the compiler cannot honestly report `DERIVE` requirements; every
  such requirement must degrade to `VERIFY`, verification cost rises by ~the coverage
  gap, and the economic model of §22 loses most of its design-side value. **This is
  the highest-stakes single experiment in the program and should be run first.**

---

### F-2 — Does the simulation ⇄ verification exchange relation (E1) hold?
**Puts at risk: the complexity-vector theory of §16.7.**

- **Hypothesis.** For a fixed target confidence, there is a monotone decreasing
  trade-off curve between computational effort spent on prediction and physical
  measurements required for certification, and the curve has an identifiable
  economic minimum.
- **Protocol.** Fix a target specification and a substrate. For each of ≥6 levels of
  model fidelity (empirical correlation → CALPHAD → MLIP → DFT → DFT+corrections),
  compute the minimum number of physical tests needed to certify at δ = 0.05 given
  that model's calibrated uncertainty. Measure actual attained confidence.
- **Pass:** a statistically significant monotone decreasing relation
  (Spearman ρ ≤ −0.7, p < 0.01) between model cost and required sample size, with
  attained confidence ≥ nominal.
- **Fail:** no relation, or attained confidence below nominal at any level (which
  would indicate the models' uncertainty is not usable as a substitute for
  measurement at all).
- **Cost/duration:** $0.8–2M, 12 months.
- **If it fails:** the exchange-relation theory is wrong and the complexity vector is
  bookkeeping, not theory.

---

### F-3 — Does specification-language discipline improve real outcomes?
**Puts at risk: Stage 0, the framework's cheapest and most certain deliverable.**

- **Hypothesis.** Specifications expressed in the DSL contain fewer latent defects
  (ambiguous protocols, statistically unsatisfiable criteria, unguaranteed
  requirements) than the natural-language/spreadsheet incumbents, and formalization
  surfaces defects that experts confirm as real.
- **Protocol.** Take 200 real historical specifications from ≥3 industries. Translate
  to the DSL. Have the compiler report defects. Submit each reported defect to a
  blinded panel of three domain experts for adjudication as real / not real /
  ambiguous. Additionally, cross-check against the specifications' actual downstream
  change-order and nonconformance history.
- **Pass:** ≥0.2 expert-confirmed real defects per specification, precision ≥70%
  (fraction of reported defects confirmed real), and a statistically significant
  association between reported defects and historical nonconformances.
- **Fail:** <0.05 confirmed defects per specification or precision <40%.
- **Cost/duration:** $400–800k, 9 months.
- **If it fails:** specification discipline is not a real gap; Stage 0 has no value
  and the framework loses its entry wedge.

---

### F-4 — Does compiling for verifiability actually reduce verification cost?
**Puts at risk: the framework's flagship novel claim (E2, §16.7; §11.5).**

- **Hypothesis.** Among routes that equally satisfy a specification, deliberately
  selecting for verifiability (distinctive signatures, functional witnesses, built-in
  test features) reduces total certification cost at equal confidence by ≥30%.
- **Protocol.** For ≥10 real specifications, generate two process routes each: one
  optimized for production cost only, one jointly optimized with λ_V > 0. Execute
  both. Certify both to δ = 0.05 using optimal verification plans. Compare total
  cost-to-certify and attained confidence.
- **Pass:** median reduction ≥30% in cost-to-certify at equal attained confidence,
  with production-cost penalty ≤10%.
- **Marginal:** 10–30% reduction.
- **Fail:** <10% reduction, or production-cost penalty exceeding the verification
  saving.
- **Cost/duration:** $2–5M, 18–24 months.
- **If it fails:** the framework's most distinctive idea is not economically real;
  it remains true in VLSI and does not generalize.

---

### F-5 — Can physical provenance detect process substitution?
**Puts at risk: the attestation layer's core security claim (T8, §14.2).**

- **Hypothesis.** An adversary who executes a cheaper substitute process while
  reporting the approved one can be detected from the article's measurable
  signatures plus the attested record, at detection rate ≥95% with false-positive
  rate ≤1%.
- **Protocol.** A genuine red-team exercise. Blue team specifies and attests a
  process. Red team, with full knowledge of the detection scheme and physical access
  to equivalent equipment, produces substituted articles intended to pass. Blue team
  adjudicates blind. Include at least three substitution classes: cheaper feedstock,
  shortened heat treatment, different supplier equipment.
- **Pass:** ≥95% detection at ≤1% false positive.
- **Fail:** <80% detection, or the red team demonstrates a systematically
  undetectable substitution class.
- **Cost/duration:** $1–2M, 12 months.
- **If it fails:** attestation proves only that a record was created, not that a
  process was followed; the provenance value proposition narrows sharply to
  *deterrence and forensics* rather than *prevention*, and the framework must say so.

---

### F-6 — Is the physical fingerprint binding (Assumption PUF-1) sound?
**Puts at risk: the record↔matter binding, hence all attestation.**

- **Hypothesis.** For a chosen substrate and fingerprint protocol, an adversary
  cannot produce a distinct article matching a target fingerprint within the
  acceptance threshold at cost below a stated bound; and legitimate articles remain
  matchable after realistic service exposure.
- **Protocol.** (a) Enroll 10⁴ articles; measure false-match and false-non-match
  rates. (b) Age/wear a subsample (thermal cycling, abrasion, corrosion) and re-match.
  (c) Red-team cloning attempt with full protocol disclosure.
- **Pass:** false-match ≤10⁻⁶, false-non-match ≤10⁻³ after service-representative
  ageing, no successful clone under a stated attacker budget.
- **Fail:** false-non-match >10⁻² after ageing (the binding does not survive
  service), or a successful clone.
- **Cost/duration:** $1–3M, 18 months.

---

### F-7 — Does Reality DSL compilation produce reproducible physical outcomes?
**Puts at risk: the claim that compilation is compilation at all.**

- **Hypothesis.** The same DSL specification, compiled and executed at ≥3
  independent facilities with conforming substrate declarations, produces articles
  whose specified properties are statistically indistinguishable within the declared
  tolerance.
- **Protocol.** An interlaboratory round-robin, structured exactly as ASTM E691 /
  ISO 5725 proficiency testing. ≥3 facilities, ≥5 specifications, ≥10 articles each.
  Compute repeatability (r) and reproducibility (R) statistics.
- **Pass:** between-laboratory variance contributes <30% of total variance, and all
  facilities' lots meet the specification at the declared δ.
- **Fail:** between-laboratory variance dominates, or facilities disagree on
  pass/fail for the same specification.
- **Cost/duration:** $2–4M, 18 months.
- **If it fails:** "compilation" is a misnomer — the output is facility-specific
  process development, not a portable artifact — and the entire architecture must be
  re-scoped to single-facility optimization. **This is the experiment that tests
  whether the word "compiler" is earned.**

---

### F-8 — Does physical relaxation solve useful optimization problems faster?
**Puts at risk: §VIII's computational-leverage claim (which this document has already
judged largely CONTRADICTED; the experiment is the honest way to close it).**

- **Hypothesis.** For a defined family of design problems arising in the framework,
  a physical annealer/relaxer reaches a target solution quality faster than the best
  classical algorithm, with a scaling advantage.
- **Protocol.** Pre-register the problem family, the target quality, and the
  classical baselines *before* running. Use the established methodology for defining
  and detecting quantum speedup (scaling of time-to-solution with instance size, not
  wall-clock at a single size). Require the classical baseline to be tuned by an
  independent party.
- **Pass:** a demonstrated scaling advantage that survives an independent classical
  re-optimization attempt over 12 months.
- **Fail (expected):** no scaling advantage. The prior literature strongly suggests
  this outcome for generic instances.
- **Cost/duration:** $300–700k, 12 months.
- **If it fails:** remove all computational-leverage claims for annealing from the
  framework and retain landscape engineering strictly as a *control* strategy (§8.2,
  reading I). This does not damage the framework; it removes an unsupported limb.

---

### F-9 — Can automated characterization be trusted?
**Puts at risk: Stages 2–8 (every closed-loop stage).**

- **Hypothesis.** Automated phase/structure identification with calibrated
  abstention matches expert consensus and, critically, *knows when it does not know*.
- **Protocol.** Blind set of ≥500 diffraction/spectroscopic datasets including
  deliberately hard cases (impurity phases, preferred orientation, amorphous
  content, near-isostructural pairs). Compare automated calls to a three-expert
  consensus. Measure accuracy on non-abstained cases and the abstention rate on
  cases where experts disagree.
- **Pass:** ≥95% accuracy on non-abstained cases, and abstention rate on
  expert-disagreement cases ≥80%.
- **Fail:** accuracy <90%, or abstention uncorrelated with difficulty (i.e.,
  confident errors).
- **Cost/duration:** $500k–1M, 12 months. **This is cheap and should be done
  immediately**; the published critique of a flagship autonomous-lab result turned on
  exactly this failure mode.

---

### F-10 — Does closed-loop assembly hit tolerances that open-loop cannot?
**Puts at risk: §10.4's in-process control requirement.**

- **Hypothesis.** In-process sensing plus correction reduces out-of-tolerance rate by
  ≥5× versus best-tuned open-loop on the same equipment and specification.
- **Protocol.** Matched-pairs comparison, ≥200 articles per arm, on a substrate with
  known process drift (LPBF or precision machining are good candidates).
- **Pass:** ≥5× reduction in nonconformance at equal or better cycle time.
- **Fail:** <2× reduction.
- **Cost/duration:** $1–2M, 12 months.

---

### 25.1 What this program costs and what it buys

Total: roughly **$11–26M over 24 months** to test every load-bearing claim in the
framework. That is small relative to the program's ambition and it is the correct
first expenditure. **F-1, F-3, and F-9 together cost under $6M, take under 18 months,
and between them determine whether the framework has a foundation.** They should be
funded before anything else.

---

## 26. Research Priorities

Ranked by (impact on the framework) × (tractability) ÷ (current attention).

**P1 — Calibrated, validity-aware forward models.**
Everything depends on models that know their own domain of applicability and report
honest uncertainty. This is *not* the same as improving model accuracy, and the
accuracy-focused literature is much larger than the calibration-focused one. A model
that is 95% accurate and silently wrong on the remaining 5% is worse for this
framework than one that is 85% accurate and abstains correctly. **Highest priority.**

**P2 — Attractor engineering as a unified theory.**
The observation that classical self-assembly and quantum dissipative state
preparation are the same mechanism — make the target a robust attractor of
engineered dynamics, and pay for robustness in basin depth / spectral gap rather
than in control bandwidth — is the strongest genuinely novel synthesis available
here (§9.7). A unified formalism with a common figure of merit, and a theory of
when the five conditions of §8.5 hold, would be a real theoretical contribution.

**P3 — Compositionality of physical guarantees.**
Desideratum 3.1. Without it, multi-step synthesis cannot be reasoned about, and
hierarchical assembly cannot scale. The specific question: for which substrates does
a *restoring, sufficient, contractive* intermediate variable exist? This is the
physical analogue of the digital abstraction and it is the deepest open problem in
the framework.

**P4 — Verification-cost-aware design.**
Formalize, then test (F-4). If E2 generalizes beyond VLSI, it is the framework's
economic engine.

**P5 — Synthesizability prediction.**
The gap between "thermodynamically stable" and "makeable" is the field's central
practical blocker (§20.3) and is squarely the kinetic screen of §6.4.

**P6 — Automated characterization with calibrated abstention.**
Cheap, tractable, immediately valuable, and currently the documented failure point
of autonomous laboratories.

**P7 — Physical-digital binding primitives.**
Assumption PUF-1 (§12.3). A materials-science problem with a cryptographic
specification — an unusual and fertile combination.

**P8 — Rare-event certification.**
Certifying δ ≤ 10⁻⁶ from feasible sample sizes requires physics-informed extreme-value
methods rather than brute sampling. Underdeveloped relative to its importance.

**P9 — Quantum sensing for in-process metrology.**
The near-term quantum contribution to the framework, and it is ahead of quantum
computing on the relevant timeline (§9.3).

**Deliberately deprioritized:** quantum annealing for design optimization (F-8);
room-temperature macroscopic coherence (§9.2); universal assemblers (§10.1);
thermodynamic-limit optimization (§15.3).

---

## 27. Open Problems

**OP-1 (The digital abstraction problem for matter).** For a given substrate, does
there exist a *sufficient, restoring, contractive* intermediate variable — a quantity
that (i) determines downstream behavior, (ii) is driven back toward nominal by the
process dynamics, and (iii) has margins that compose? Digital electronics has one
(voltage with noise margins) and is therefore compositional; chemistry and
metallurgy largely do not. Characterizing which substrates admit one, and whether
they can be *engineered* to admit one, is the deepest question in the framework.

**OP-2 (Spec-cell measure and hardness).** Is there a quantitative relation between
the measure of the acceptance region [S] relative to the accessible state space and
the compilation complexity vector? Conjecture: C_search and C_ver scale with
−log μ([S]) under a suitable reference measure. Proving or refuting this would give
the framework its first genuine complexity theorem.

**OP-3 (Compositional confidence).** Under what conditions does the interaction term
ε in Desideratum 3.1 admit a computable bound? Related to OP-1 and to error
propagation in hierarchical self-assembly.

**OP-4 (Attractor engineering figure of merit).** Is there a single quantity —
generalizing the spectral gap, the basin volume, and the kinetic-trapping time —
that governs when landscape engineering succeeds across classical and quantum
substrates?

**OP-5 (The irreversibility fraction χ).** Make χ in Proposition 15.1 computable for
a given process class. This would turn the thermodynamic bound from a curiosity into
a design tool by identifying which parts of a process pay the full information cost.

**OP-6 (Verification of unspecified properties).** Is there a principled way to bound
the risk from properties nobody specified? Damage-tolerance doctrine (§11.3) is the
best existing answer and it is domain-specific. A general theory would be valuable
and may not exist.

**OP-7 (Detectability of process substitution).** Under what conditions does a
process leave a signature in the product that is (a) measurable at reasonable cost
and (b) hard to forge? This is a well-posed question at the intersection of
materials characterization and cryptography, and essentially nobody is working on it.

**OP-8 (Model-error-aware optimal control).** Existing robust control handles bounded
uncertainty in parameters; the framework needs robustness to *structural* model
error, where the true dynamics are outside the model class. Largely open.

**OP-9 (Analog verification).** Can a physical experiment serve as a *proof* rather
than as evidence — i.e., is there a substrate-level analogue of an interactive proof
system where the physical system convinces a bounded verifier? Speculative, and if it
worked it would be transformative for §11.

**OP-10 (Throughput-precision frontier).** Is there a fundamental bound on the
product of placement precision and throughput for physical assembly, analogous to a
rate-distortion bound? §10.1 gives a scaling argument; a theorem would be better.

---

## 28. Conclusions

1. **The central thesis survives, narrowed.** Physical synthesis can be formalized as
   compilation, but the target is an acceptance region over measurable functionals,
   the guarantee is probabilistic, and the compiler's contract covers a process-plus-
   test pair rather than a state.

2. **The complexity claim was wrong in the optimistic direction.** General physical
   reachability is undecidable; reaction-network reachability is Ackermann-complete;
   ground-state problems are QMA-complete; planning is PSPACE-complete. The correct
   object is a complexity *vector* with exchange relations, not a class.

3. **Thermodynamics is not the binding constraint.** Bounds sit 2–6 orders of
   magnitude below practice for matter and ~20 for computation. Kinetics, selectivity,
   and metrology bind.

4. **There is no universal compiler, but there is a universal interface.** The
   specification language, the evidence calculus, and the attestation schema can be
   substrate-independent. Everything below them cannot.

5. **Verification is the framework's centre of gravity** — scientifically (it is the
   hardest layer), architecturally (it gates every closed-loop stage), and
   commercially (the verification market is ~15× the design-tool market).

6. **Self-assembly and photonic inverse design are existence proofs.** The pipeline
   has been demonstrated end-to-end in DNA nanotechnology, in nanophotonics, and —
   at industrial scale for forty years — in EDA. The framework's task is to
   generalize a working pattern, not to invent one.

7. **The quantum substrate as proposed does not exist and is not needed.** Quantum
   resources enter as compile-time models (2030s) and as metrology (now).

8. **The economics are large and finite.** Best case ~$100–250B enterprise value;
   extraordinary case ~$1T; the quintillion figure is off by a factor of ~500 against
   the capitalized output of civilization.

9. **The program's first $6M should go to F-1, F-3, and F-9.** They are cheap, fast,
   and between them determine whether there is a foundation.

10. **The most valuable single idea in the proposal, once corrected, is
    compile-for-verifiability.** It is testable (F-4), it has a strong precedent
    (design-for-testability in VLSI), it is absent from materials, chemistry, and
    biology, and it attacks the cost line that actually dominates.

---

# Final Answers

> **The question:** What is the strongest scientifically defensible version of the
> Reality Compiler that could actually be built, and what sequence of breakthroughs
> would be required to get there?

**Answer.** The strongest defensible version is a **substrate-targeted physical
synthesis compiler**: a specification language with an evidence calculus, a
portfolio of substrate-specific back ends that emit closed-loop process programs
together with costed verification plans, and an independent attestation layer that
records exactly what was proven and what was assumed. It compiles to an acceptance
region, not a state; it returns UNKNOWN honestly; it treats verification cost as a
first-class objective; and it is universal only in its interfaces. Everything in it
can be built with today's physics; the required breakthroughs are in model
calibration, compositionality of physical guarantees, synthesizability prediction,
and automated characterization — not in quantum coherence or molecular assemblers.

---

### A. The minimal viable Reality Compiler

**Buildable in 18 months for ~$8M. No new physics.**

A materials/process advisory compiler that:
1. ingests specifications in the DSL, type-checks units, protocols, and statistics;
2. reports VERIFY/ASSUME/DERIVE coverage and flags statistically unsatisfiable
   acceptance criteria;
3. proposes ranked candidate composition × route pairs with **calibrated** credible
   intervals;
4. emits the single most discriminating experiment when models disagree;
5. synthesizes a cost-optimal verification plan with a proven operating
   characteristic;
6. attests the whole chain with model versions, validity-domain checks, calibration
   records, and raw-data digests.

It does not fabricate anything. Its output is the report in §20.4. It is useful on
day one, it is falsifiable (F-1, F-3), and every later stage is an extension of it.

### B. The 10-year research target (≈2036)

A **closed-loop compiler for two or three substrate classes** — most plausibly
(i) inorganic solid-state synthesis, (ii) organic synthesis/flow chemistry, and
(iii) nanophotonic/metasurface devices — in which a specification compiles to an
executed process on attested equipment, with automated characterization that knows
when it does not know, achieving interlaboratory reproducibility (F-7 pass) and
first-pass specification satisfaction ≥70% on novel targets. Plus: protein and
protein-assembly design as a fourth, biological back end, which is already closest
to working. Plus: quantum sensing integrated into in-process metrology.

### C. The 25-year research target (≈2051)

**Compositional physical compilation.** Multi-step, multi-substrate synthesis where
guarantees compose across steps with bounded confidence loss (OP-1, OP-3 solved for
at least one substrate class) — enabling hierarchical assembly from molecular to
millimeter scale, programmable microstructure with verified spatial property
gradients, and engineered multicellular constructs to specification. Plus fault-
tolerant quantum computation contributing accurate Hamiltonians for strongly
correlated systems at compile time. Compilation *coverage* — the fraction of an
industry's specifications that compile end-to-end — becomes the headline metric.

### D. The extreme theoretical endpoint

Not a universal assembler; §10.1 and §17 rule that out. The endpoint is:

> **A complete, machine-checkable map of the reachable set of a declared substrate,
> and a compiler that is complete relative to that map** — i.e., for any
> specification, it either emits a certified process or emits a *proof* that no
> process over that substrate satisfies it. Composed over many substrates and open
> to competitive back ends, this is the physical-synthesis analogue of a complete
> decision procedure for a decidable fragment of arithmetic: not universal, but
> exhaustive within a declared boundary, and honest about the boundary.

That endpoint is consistent with undecidability (it is relative to a bounded
declared substrate), consistent with thermodynamics, and would be a genuine
civilizational capability.

### E. The five hardest unsolved problems

1. **The digital abstraction for matter** (OP-1) — does a restoring, sufficient,
   contractive intermediate variable exist for a given substrate? Everything
   compositional depends on it.
2. **Calibrated model uncertainty outside the training distribution** (P1) — the
   problem the entire `DERIVE` mode rests on, and the one machine learning is worst
   at.
3. **Synthesizability prediction** (P5) — the gap between stable and makeable.
4. **Verification of what was not specified** (OP-6) — the irreducible residual risk.
5. **Kinetic inverse design** — designing a landscape whose target is reachable in
   available time without kinetic trapping. Condition 3 of §8.5, and the reason most
   landscape-engineering proposals fail.

### F. The three claims most likely to be wrong

**In the original proposal:**
1. That quantum coherence in an accessible-condition substrate can do useful
   computational work for synthesis. Likely wrong; §9.
2. That physical relaxation provides generic optimization speedup. Likely wrong;
   §8.3 Case B, testable by F-8.
3. That a universal physical compiler is the goal. Wrong in the author's judgment;
   §17.

**In *this document*, the three claims most likely to be wrong are:**
1. **That compile-for-verifiability generalizes beyond VLSI** (E2, §16.7). It is the
   framework's flagship and it rests on one precedent from one industry. F-4 tests
   it; it may simply not transfer.
2. **That verification sample complexity is linear in the number of specified
   properties in practice** (Prop. 11.1). The proposition is correct as stated, but
   its hypotheses — bounded sensitivity, independent estimability, no unspecified
   failure modes — may fail badly in the substrates that matter, making the practical
   complexity much worse than the theorem suggests.
3. **That the EDA capture ratio is the right economic anchor** (§22.2). EDA may be
   *sui generis*: semiconductors are uniquely digital, uniquely standardized, and
   uniquely reliant on a design abstraction that no other industry has. If so, the
   capture ratio for other substrates could be 5–10× lower and the economics change
   materially.

### G. The three potentially transformative discoveries

1. **A restoring, contractive intermediate variable for a non-electronic substrate**
   (OP-1). If found for, say, polycrystalline metals or for hierarchical molecular
   assembly, it would do for that substrate what the digital abstraction did for
   electronics: make arbitrarily long composition possible. This is the one result
   that would change the ceiling of the entire program.
2. **A general, cheap physical-digital binding primitive** (OP-7, F-6). Would make
   every physical article self-authenticating, with consequences well beyond this
   framework: counterfeit elimination, supply-chain integrity, biosecurity, and
   materials recycling all change.
3. **Calibrated out-of-distribution uncertainty for physical models** (P1). Not
   specific to this framework — it would transform scientific machine learning
   generally — but this framework is where its value would be most directly
   monetizable, because it converts `DERIVE` from a liability into an asset.

### H. Formal definition suitable for a peer-reviewed paper

> **Definition (Physical Synthesis Compilation).**
> Let a *substrate* be a tuple Ω = ⟨Σ, 𝒜, 𝒯, ℳ, 𝒞, ℰ, 𝔐⟩ comprising an ontic state
> space, an action algebra with declared preconditions, effects, durations and
> resource draws, an admissible control-signal space, a measurement algebra with
> declared estimators and error models, an inventory, an environment envelope, and a
> model bundle whose members carry explicit validity domains and error bounds.
>
> Let a *specification* be S = ⟨𝒪, 𝒦, δ, β⟩ where 𝒪 = (O₁,…,O_k) is a finite vector
> of estimands, each a pair (property functional, measurement protocol); 𝒦 ⊆ ℝᵏ is a
> measurable acceptance region; and δ, β ∈ (0,1) bound false acceptance and false
> rejection.
>
> Given an initial belief 𝔟₀ ∈ Δ(Σ), the **physical synthesis compilation problem**
> is to produce a triple (P, Ψ, Π) where P is a policy over 𝒜 admissible under 𝒯, 𝒞,
> ℰ; Ψ is a measurement plan over ℳ together with a three-valued acceptance rule
> A : Y* → {PASS, FAIL, INCONCLUSIVE}; and Π is a verifiable record binding S, Ω, P,
> Ψ, the executed trace, the measurement outcomes, and the validity witnesses of every
> model in 𝔐 invoked; such that for every m ∈ 𝔐,
>
> ```
>   Pr_m[ A(Y) = PASS  ∧  Φ(σ_T) ∉ 𝒦 ]  ≤  δ,
>   Pr_m[ A(Y) = FAIL  ∧  Φ(σ_T) ∈ 𝒦 ]  ≤  β.
> ```
>
> A compiler is **sound** if every triple it emits satisfies the above under the
> declared 𝔐; **complete relative to Ω and a horizon L** if, whenever an admissible
> triple of length ≤ L exists, it emits one or a certificate of infeasibility; and
> **honest** if it emits UNKNOWN in all other cases. Soundness is relative to 𝔐, and
> Π must expose that relativity.

### I. Proposed mathematical notation system

Consolidated in [`NOTATION.md`](NOTATION.md). Summary:

| Symbol | Meaning |
|---|---|
| Ω = ⟨Σ, 𝒜, 𝒯, ℳ, 𝒞, ℰ, 𝔐⟩ | substrate declaration |
| σ ∈ Σ_ℓ | ontic state at level of description ℓ |
| 𝔟 ∈ Δ(Σ) | epistemic state (belief) |
| Φ : Σ × 𝒫 → ℝᵏ | property map; 𝒫 a measurement protocol |
| S = ⟨𝒪, 𝒦, δ, β⟩ | specification |
| [S] = Φ(·,𝒫)⁻¹(𝒦) | **spec cell** — the acceptance region in state space |
| P, Ψ, Π | process policy, measurement plan, provenance record |
| 𝔍[P] | compilation objective (§7.4) |
| 𝒞(𝔟₀,S,Ω,δ) | **compilation complexity vector** (§16.7) |
| ⊨_δ | probabilistic satisfaction at confidence 1−δ |
| ⟦S⟧ = (𝒦, Φ, Ξ) | denotation of a specification; Ξ the evidence obligation |
| χ | irreversibility fraction (§15.4) |
| E1…E6 | exchange relations between complexity components |

Conventions: ontic quantities in Latin lowercase (σ, x, u); epistemic in fraktur
(𝔟, 𝔐); spaces and algebras in script (Σ, 𝒜, ℳ, 𝒦); the spec cell always in square
brackets; confidences always as δ (false accept) and β (false reject), never as a
bare "confidence."

### J. Proposed name for the discipline

**Synthesis Complexity.**

The study of the resources — computational, control-theoretic, energetic,
metrological, and fabricational — required to bring a physical system into a
specified acceptance region, and of the exchange relations among them.

Rationale: it names the object (synthesis), commits to the central formal content
(complexity as a multidimensional resource account), and is legible to complexity
theorists, control theorists, and materials scientists simultaneously. It does not
overclaim.

Subsidiary terms, for consistent use:
- **Physical synthesis compilation** — the engineering activity.
- **Physical Synthesis Compiler (PSC)** — the artifact.
- **Spec cell** — the acceptance region [S].
- **Compilability** — the property of a substrate satisfying the four criteria of §10.3.
- **Attractor engineering** — the unified design strategy of §8/§9.7.
- **Verification-cost-aware design** — the discipline of exchange relation E2.

---

*"Reality Compiler" was the right ambition and the wrong name. What the ambition
actually names, once the physics and the complexity theory are taken seriously, is a
compiler for a declared substrate, a calculus of evidence, and an economics of
verification. That is a smaller claim and a far more valuable one, because it can be
built, tested, and — this is the point — proven wrong.*

---

## Draft 3 note on the Final Answers

Parts I and XIV revise three of the answers above. The revisions, in place of
rewriting:

**Answer H (formal definition) is superseded by Definition 2.4**, which carries the
generalized compilation object (𝔟₀, S, Ω) → (P*, H*, L*, Ψ, Π), the algebra of
conditions, the realizable control-parameter space Λ_Ω, and the environment as a
costed free variable.

**Answer D (the extreme theoretical endpoint)** is sharpened. Drafts 1–2 gave "a
complete machine-checkable map of a declared substrate's reachable set, and a compiler
complete relative to it." That stands, and §43 now supplies the scope: the endpoint is
attainable **on the compressible sector**, characterized for quantum matter as
CQM(Ω) (Definition 39.1) and resting structurally on the area law. The generic sector
is excluded by a counting argument that no technology can touch — and equally excludes
generic classical matter, which is why compressibility is the single criterion
covering both halves of the framework.

**Answer F (claims most likely to be wrong)** gains a fourth, which now outranks the
other three in consequence:

> **That the area law holds broadly enough in d ≥ 2 to make CQM a fat class.** It is
> proved in one dimension and conjectured above it. If it fails widely, the compilable
> sector of quantum matter is much thinner than §39 suggests and the program's scope
> contracts sharply. This is the framework's largest single external dependency, it is
> tracked in `CITATIONS.md`, and it is not something this program can resolve on its
> own.

**Answer J (the discipline name)** is unchanged — *Synthesis Complexity* — but the
subsidiary vocabulary is extended: **Physical State Compilation** for the field,
**Quantum Physical Synthesis** for target class C, **Landscape Compilation** for the
core architectural object, **physical computational leverage** ℒ for the research
objective, and **CQM(Ω)** for the compilable class.
