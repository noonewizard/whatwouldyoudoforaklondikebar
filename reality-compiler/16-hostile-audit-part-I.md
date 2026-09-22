# Hostile Audit of Part I (Draft 3, §§1–4)

Every substantive claim in Part I, audited against the classification scheme:
THEOREM · PROPOSITION · LEMMA · COROLLARY · DEFINITION · KNOWN RESULT ·
STRONGLY SUPPORTED · EMPIRICALLY SUPPORTED · PLAUSIBLE · CONJECTURE ·
OPEN PROBLEM · HEURISTIC · ENGINEERING ASSUMPTION · SPECULATIVE · FALSE · UNRESOLVED.

"ESTABLISHED" is **retired** as a classification. It was doing rhetorical work in
Drafts 1–3 — asserting consensus rather than naming a result — and every prior use is
reclassified below.

---

## A. The audit table

| # | Current wording (Draft 3) | Classification | Problem | Correct formulation | Evidence | Action |
|---|---|---|---|---|---|---|
| 1 | "Physical synthesis is compilation, and it is compilation at every level where it has been attempted." §1.3(A) | **EMPIRICALLY SUPPORTED**, not a theorem | "Is compilation" is not defined precisely enough to be true or false. Five existence proofs do not establish a universal property | *Five substrate domains (EDA, retrosynthesis, DNA origami, adjoint photonics, programmable simulators) independently converged on a pipeline of the form* spec → search → lower → execute → measure → accept. *This convergence is evidence that the pattern is substrate-general; it is not proof* | The five toolchains | Reworded; downgraded from an assertion about all physical synthesis to a statement about observed convergence |
| 2 | "Quantum matter scores higher on three of four compilability criteria." §1.3(B) | **PLAUSIBLE** | The four criteria were *proposed by this framework*, so the claim is partly circular. The scoring was qualitative | Same claim, with the circularity stated: *under this framework's own criteria, which are themselves a proposal (§10.3)*, quantum matter scores higher on three | §9.2.1 comparison | Circularity now flagged inline; criteria marked as the proposal they are |
| 3 | "Controlled emergence is the general primitive; instruction execution is its degenerate case." §1.3(C) | **CONJECTURE** with a falsifiable form | Presented with three arguments (scaling, containment, empirical) none of which is a proof. "Primitive" is undefined | *Conjecture: for target families with family-relative description ratio κ < κ*, landscape/attractor compilation achieves lower total verified cost than explicit instruction execution, with the crossover at κ* determined by production volume.* Falsifiable by F-3, F-14 | Tile complexity Θ(log N/log log N); industrial throughput ratios | Downgraded from thesis to conjecture; falsifiable form supplied |
| 4 | "Compilability is compressibility." §1.3(D), §39.8 | **Split: one PROPOSITION (proved below), one FALSE direction** | The claim conflated necessity and sufficiency. **Sufficiency is false** — counterexample CE-3 (§17.6) | *Necessity:* Proposition 16.1 below, proved by counting. *Sufficiency:* **FALSE.** Frustrated local Hamiltonians have O(N)-description targets whose ground-state preparation is QMA-hard and whose gap may close exponentially | Counting argument; CE-3 | **Major correction.** See §B.4 |
| 5 | "Targets are regions, not states — and for quantum matter this is forced." §4.4 | **PROPOSITION** (four independent arguments), of which three are rigorous | Argument 4 (phases don't exist at finite size) is correct but the conclusion drawn was slightly too strong: it forbids certifying *phase membership*, not certifying *observables* | Unchanged in substance; the fourth argument restated as: *phase membership is not a function of finite-size expectation values, so a phase specification must be a nested family of observable-acceptance regions plus a scaling hypothesis against a declared alternative set* | Chen–Gu–Wen phase equivalence under LU circuits | Wording tightened |
| 6 | "`R = (x,p,c,f,q)` is mathematically incorrect." §4.1 | **PROPOSITION** | Correct, and the reasoning is sound: the tuple mixes ontic, derived, and epistemic quantities and admits internally inconsistent states | Unchanged | Type-theoretic | Retained |
| 7 | "Proposition 3.1 (spectral invariance) ... belongs in the compile-time screen." §3.5 | **THEOREM** (elementary) for the physics; **ENGINEERING ASSUMPTION** for the architectural claim | The physics is a one-line consequence of unitarity. The claim that it "is not usually stated as a compile-time screen" is an assertion about the literature that was not checked | Physics: THEOREM, scope-stated. Architecture: proposed | Elementary; scope in §9.4.1 | Split into two claims with different classifications |
| 8 | "Quantum error correction is the only known constructed instance of the digital abstraction outside classical electronics." §3.6 | **FALSE as stated** (already softened in Draft 4 W-4) | "Only known" was never checked. Candidate counterexamples: classical error correction in digital logic; kinetic proofreading; DNA mismatch repair; multistable mechanical metamaterials; bistable chemical switches | *QEC is the clearest deliberately engineered quantum instance of a restoring, sufficient, contractive variable. Whether biological and mechanical analogues qualify depends on whether they exhibit gain and fan-out (§17.9)* | — | Corrected; converted into the Physical Logic Universality research problem (§17.9) |
| 9 | "The layers are stacked deterministically upward and degenerate downward." §4.2 | **PROPOSITION**, with one overstatement | "Deterministically upward" holds within Born–Oppenheimer and fails for non-adiabatic dynamics, conical intersections, and any case where electronic and nuclear motion couple | *Within the Born–Oppenheimer approximation, nuclear configuration determines electronic structure. The determination fails at conical intersections and in non-adiabatic regimes, which are exactly where photochemistry lives* | BO approximation and its known failures | Scope added |
| 10 | "Independent control exists only at the nonequilibrium handles." §4.2 | **HEURISTIC** | Stated as if derived; it is an observation about which parameters have produced surprises. "Only" is unsupported | *Search heuristic: control parameters that span a layer boundary (twist angle, strain, drive frequency, dissipation) are disproportionately likely to yield new accessible phases* | Empirical pattern over ~15 years | Downgraded to heuristic |
| 11 | "ℒ = I_verified/(K(π*)+C_ver) measures physical computational leverage." §2.6 | **Dimensionally inconsistent — defective as written** | Adds bits (I_verified, K) to cost (C_ver). Also depends on the universal machine through K, with an additive constant that can exceed the quantity for small objects | Replaced by an operational measure, §B.5 | — | **Replaced** |
| 12 | "Model error is not uniformly bounded and is largest where it matters most." §4.3 | **EMPIRICALLY SUPPORTED** | True and important; not a theorem | Unchanged in content | MLIP OOD behavior; CALPHAD extrapolation; continuum failure at interfaces | Classification corrected |
| 13 | "The binding layer is KINETIC or CONTROL, not thermodynamic." §3.4 | **EMPIRICALLY SUPPORTED** | An observation about which constraint binds in practice, stated with the force of a law | Unchanged in content | Diamond at STP; catalysis; entropy-limited cold-atom preparation | Classification corrected |
| 14 | Definition 2.4 (Physical State Compilation) | **DEFINITION** | Sound. One gap: it quantifies over "every model m ∈ 𝔐" without requiring that 𝔐 contain the truth — so a compiler can be sound relative to a uniformly wrong bundle | Add the explicit caveat: *soundness is relative to 𝔐 and is not a claim about reality* (now Theorem 17.5 and its corollary) | — | Caveat added; becomes the framework's central limitation |
| 15 | "There is no universal compiler for matter, but there is a universal interface." §17 | **PLAUSIBLE**; the positive half is a **KNOWN RESULT** | The negative half is an argument, not a proof. The positive half (universal Hamiltonian families) is a cited theorem | Negative: argued, not proved. Positive: KNOWN RESULT (Cubitt–Montanaro–Piddock) | — | Split by classification |
| 16 | "CQM(Ω)" (Definition 39.1) | **DEFINITION** (proposed) | Sound as a definition. The claim that it is "the right" class is not established | Unchanged, with the status note | — | Status note added |
| 17 | "Compilability of quantum matter rests on the area law." §39.9 | **PROPOSITION** (a derivation) resting on a **CONJECTURE** (area law in d ≥ 2) | Correctly flagged in Draft 3, but the chain has an unstated gap: area law ⟹ efficient MPS/PEPS description is a **theorem in 1D** and is *not* known to follow in higher d even given an area law (PEPS contraction is #P-hard) | *The chain holds in 1D. In d ≥ 2 it breaks in two places: the area law is conjectural, and even granting it, efficient classical representability does not follow — contracting PEPS is hard* | Hastings 1D area law; PEPS contraction hardness | **Material correction.** The higher-dimensional chain is weaker than Draft 3 implied |
| 18 | "Verification sample complexity is linear in specified properties" (Prop. 11.1) | **PROPOSITION** (correct under its hypotheses) | Hypotheses — bounded sensitivity, independent estimability, no unspecified failure modes — may fail badly. Draft 1 flagged this; Part I did not | Unchanged; hypotheses restated inline | Union bound | Hypotheses surfaced |
| 19 | "Integer estimands give O(1) verification margin" (Prop. 29.1) | **PROPOSITION** | Correct, but assumes the quantization is *exhibited at the working system size*, which for topological invariants requires that finite-size corrections be smaller than ½ — not automatic | Add the hypothesis: *provided finite-size and disorder corrections to the quantized value are below half the quantization unit* | Quantized Hall to 1 part in 10⁹ — at large size, low T, high mobility | Hypothesis added |
| 20 | "Extreme regimes are cheap at small volume, unavailable at large" §34.3 | **EMPIRICALLY SUPPORTED** | An engineering-economics observation | Unchanged | Diamond-anvil sample volumes; cryostat cooling power | Classification corrected |

**Summary of the audit.** Of twenty audited claims: one is **FALSE** as stated (#8);
one was half-**FALSE** (#4, the sufficiency direction); one was **dimensionally
defective** (#11); one contained a **material gap** (#17); three were **overstated in
scope** (#9, #10, #13); and eight had their classification corrected from "ESTABLISHED"
to a weaker and more accurate label. Six survive unchanged.

---

## B. The four load-bearing claims, audited individually

### B.1 Claim A — "Physical synthesis is compilation"

**Attack.** The claim is unfalsifiable as stated, because "is compilation" has no
truth conditions. Every purposive physical process can be described as
specification → execution → check if one is loose enough about the words.

**What survives.** A narrower, falsifiable statement:

> **CONJECTURE A′.** For a substrate satisfying the compilability conditions of §17,
> there exists a *single frontend semantics* — one specification language, one
> evidence calculus, one acceptance-decision procedure — that can target it alongside
> structurally unrelated substrates, with only the back end changing.
>
> **Falsifier.** Exhibit a substrate satisfying the conditions for which no such
> shared frontend is possible, or show that the shared frontend forces so much
> substrate-specific escape-hatching that it carries no semantics.
> **Test:** the three-backend prototype (§18) is the minimal positive test. It passes
> or fails observably.

**Classification: CONJECTURE, with the prototype as its first test.** Downgraded from
Draft 3's assertion.

### B.2 Claim B — "Quantum systems satisfy the proposed substrate requirements"

**Attack 1 (circularity).** The four criteria are this framework's own proposal. A
claim that quantum matter satisfies them is a claim about the criteria as much as
about quantum matter.

**Attack 2 (the third criterion is the one that matters, and it is mixed).**
Metrology with polynomial sample complexity holds for quantized and local-observable
targets and **fails** for generic states and for the regime where classical
simulation fails (Open Problem 9.22). Scoring it "mixed, and excellent for quantized
targets" was generous.

**Attack 3 (yield statistics).** "Each shot prepared fresh, no history dependence" is
true of the *state* and false of the *apparatus*: laser calibration, magnetic field
drift, atom-loading statistics, and trap-depth drift are exactly process history, and
they are the dominant source of run-to-run variation in practice.

**What survives.**

> **PROPOSITION B′.** For the restricted target class of *lattice models with
> quantized or local-observable witnesses on programmable simulators*, the four
> compilability conditions hold with quantitative margins (§17.4 instantiation), and
> this class is non-empty and experimentally accessible.
>
> **Classification: PLAUSIBLE, and directly testable** by F-7.

The general claim "quantum matter is a better-posed target than structural materials"
is **downgraded to PLAUSIBLE with the circularity stated**, and the apparatus-drift
objection is added to §9.2.1.

### B.3 Claim C — "Controlled emergence produces compression / leverage"

**Attack.** Three defects. (i) "Primitive" was never defined. (ii) The containment
argument (a machine executing instructions is itself a physical system) proves nothing
about *engineering* primitives — by the same argument, everything reduces to quantum
field theory. (iii) The empirical argument (all scaled synthesis technologies are
emergent) is a survivorship observation, not a mechanism.

**What survives.** The information-theoretic core, which is provable:

> **PROPOSITION 16.1 (Specification information bound).** Let Ω be a fixed substrate
> and let {S₁,…,S_M} be a family of specifications with pairwise disjoint acceptance
> regions, each achievable on Ω by some control program. Let π_i be the program
> achieving S_i and let K_max = max_i |π_i| in bits. Then M ≤ 2^{K_max + 1}, hence
>
> ```
>   I(S) := log₂ M  ≤  K_max + 1 .
> ```
>
> *Proof.* The map π_i ↦ (acceptance region reached) is well defined once Ω is fixed:
> distinct disjoint outcomes require distinct programs, since identical programs on
> identical substrates produce identical outcome distributions and disjoint regions
> cannot both be hit with the required confidence by the same distribution. Hence
> i ↦ π_i is injective, and an injection from M items into binary strings of length
> ≤ K_max requires M ≤ 2^{K_max+1}. ∎
>
> **Classification: PROPOSITION, proved.** *Contribution note:* this is a counting
> argument of a standard kind (it is the physical-synthesis instance of a pigeonhole
> bound), and it is **not claimed as novel**. Its role is to make the compression
> thesis precise enough to be correct.

**Corollary.** The compiler cannot deliver more specification information than its
control program contains, relative to a fixed substrate. **Leverage therefore comes
entirely from the substrate's fixed structure Ω being amortized across the family** —
which is exactly why landscape compilation's advantage is volume-dependent (§8.13) and
why the naive "the physics gives you 10²³ bits for free" reading is wrong: the physics
gives them once, to everyone, for every target.

### B.4 Claim D — "Compilability is bounded by compressibility"

This was Draft 3's deepest claim and it is **half wrong**.

**Necessity: PROPOSITION 16.1 above. Proved.**

**Sufficiency: FALSE.** Counterexample:

> **CE-3 (short description, exponential preparation).** Let H be a frustrated local
> Hamiltonian on N spins — e.g. a random-coupling Ising instance on a 3D lattice, or
> any QMA-hard instance family. Then K(H) = O(N log(1/precision)) bits: the target is
> *fully specified* by a short description. But (i) deciding its ground energy is
> QMA-complete; (ii) the spectral gap may close exponentially in N, so adiabatic
> preparation takes exponential time; (iii) it is frustrated, so the
> quasi-local-dissipator route (§9.6) does not apply. **Short description, no
> efficient preparation.** ∎

**Therefore the six quantities the directive lists must be separated, and they are
genuinely distinct:**

| Quantity | Meaning | Relations proved | Relations refuted |
|---|---|---|---|
| K(S) | description length of the specification | — | — |
| K(π*) | description length of the control program | K(π*) ≥ I(S) − O(1) (Prop. 16.1) | — |
| K(Ψ) | description length of the verification protocol | — | — |
| PrepComplexity(S) | time/depth to reach [S] | — | **K small ⇏ Prep small (CE-3)** |
| VerifyComplexity(S) | copies/samples to certify | — | **Prep small ⇏ Verify small (CE-1)**; **Verify small ⇏ Prep small (CE-2)** |
| ControlComplexity(S) | actuated channels × resolution × bandwidth | ≥ I(S)/(bits per channel) | — |

> **CONJECTURE D′ (the surviving, falsifiable form).** Within a fixed substrate and a
> fixed target family, the *family-relative* description ratio
> κ = K(generating rule)/K(target) is **negatively associated** with total verified
> cost C_PSE, controlling for preparation complexity. κ is necessary-but-not-sufficient
> for leverage; where κ ≪ 1 and PrepComplexity is also polynomial, leverage is large.
>
> **Falsifier:** F-14 (rank correlation |ρ| < 0.3 across ≥30 targets and ≥4
> substrates), or a systematic class with κ ≪ 1 and no cost advantage.

**Action: the "compilability is compressibility" thesis is withdrawn as stated and
replaced by Proposition 16.1 (necessity, proved) plus Conjecture D′ (association,
testable).**

### B.5 Replacement for the leverage measure

The Draft 3 measure ℒ = I_verified/(K(π*)+C_ver) fails three tests:

1. **Dimensional consistency: FAILS.** Bits are added to cost.
2. **Representation independence: FAILS.** K depends on a universal machine through an
   additive constant that can exceed the quantity itself at small sizes.
3. **Measurement-basis independence: FAILS.** I_verified depends on which observables
   were chosen to certify, and the numerator and denominator do not move together.

> **DEFINITION 16.2 (Operational physical leverage — replacement).**
> ```
>            I_verified(S)          [bits]
>   Λ(S,Ω) = ───────────────────────────────
>             N_ctrl · b_ctrl       [bits]
> ```
> where **N_ctrl** is the number of independently actuated control channels used by
> the program and **b_ctrl** is the effective resolution per channel in bits
> (log₂ of distinguishable settings, limited by actuator noise, not by DAC width), and
> **I_verified(S)** = log₂(μ(Σ_accessible)/μ([S])) is the certified reduction in
> accessible state-space measure under a **declared** reference measure, counted only
> for claims the verification plan actually establishes at confidence 1−δ.
>
> Subject to the verification budget as a **constraint**, C_ver(Ψ) ≤ B, rather than as
> a denominator term in the wrong units.

**Why this is better.** Both numerator and denominator are in bits. The denominator is
*directly countable* — you can enumerate the actuators and measure their effective
resolution — with no appeal to Kolmogorov complexity. The reference measure and the
declared observables are explicit inputs, so the representation-relativity is
*visible* rather than hidden.

**What remains relative.** Λ still depends on the declared reference measure μ and on
the observable set. It is a *comparative* quantity — valid for ranking routes to the
same specification on the same substrate, and **not** an absolute physical constant.
Cross-substrate comparisons require a declared common reference and should be reported
as such.

**Classification: DEFINITION (proposed), with Λ ≫ 1 as the operational signature of
landscape/attractor compilation. Falsifier: F-14, F-19.**

---

## C. Retired vocabulary

| Retired | Reason | Replacement |
|---|---|---|
| "ESTABLISHED" | Asserts consensus rather than naming a result | THEOREM / KNOWN RESULT / STRONGLY SUPPORTED / EMPIRICALLY SUPPORTED, as appropriate |
| "compilability is compressibility" | Sufficiency is false (CE-3) | Proposition 16.1 + Conjecture D′ |
| "the only known instance" (of anything) | Never checked against a literature review | "the clearest deliberately engineered instance", or a comparison table |
| ℒ = I_verified/(K+C_ver) | Dimensionally inconsistent | Λ (Definition 16.2) |
| "physics computes it for free" | Ill-posed; and Prop. 16.1 shows the free structure is amortized, not per-target | "the substrate's fixed structure is amortized across the target family" |
