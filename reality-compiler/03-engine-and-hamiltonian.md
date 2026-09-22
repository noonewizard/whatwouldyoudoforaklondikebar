# Part III — The Transition Engine, Landscape Engineering, and Attractor Compilation

Sections 7–8. **Rewritten in Draft 4** under adversarial review. This Part supersedes
the Draft 1 text entirely. Where it contradicts Draft 2's §32, Draft 4 governs; the
specific retraction is recorded in §8.7.3.

**Claim classification** used throughout: ESTABLISHED · STRONGLY SUPPORTED ·
PLAUSIBLE · OPEN PROBLEM · CONTESTED · SPECULATIVE · CONTRADICTED · UNDEFINED/ILL-POSED.
Boxes are marked DEFINITION, THEOREM (only for genuinely established results),
PROPOSITION, CONJECTURE, ENGINEERING CRITERION, KNOWN LIMIT, OPEN PROBLEM, and
FALSIFICATION TEST.

---

## 7. The Physical State Transition Engine

### 7.1 API and type system

```
TRANSITION(
    𝔟₀ : Belief[Σ],          -- epistemic state over the substrate's ontic states
    S  : Spec,                -- ⟨𝒪, 𝒦, δ, β, Alt⟩
    Ω  : Substrate            -- ⟨Σ, 𝒜, Λ_Ω, 𝒯, ℳ, 𝒞, ℰ, 𝔐⟩
) → Result {
    status   : FEASIBLE
             | INFEASIBLE(certificate : Obstruction)
             | UNKNOWN(reason : Reason, budget_exhausted : Bool)

    plans    : List[Plan]          -- ranked, diverse; never a singleton optimum
    forecast : ForecastBundle      -- per plan, with aleatoric/epistemic separated
    evidence : ModelWitnesses      -- which m ∈ 𝔐, with validity-domain check results
    gaps     : List[KnowledgeGap]  -- ranked by expected value of information
}
```

Four type-level commitments, each of which excludes a failure mode observed in real
process-development practice.

**(1) `plans` is a list.** Returning a single optimum presumes the model bundle is
correct. Under model uncertainty the correct output is a diverse set whose members
are ranked differently by different members of 𝔐, so that the disagreement is visible
and can be resolved by measurement (§7.7).

**(2) `forecast` separates aleatoric from epistemic uncertainty.** Aleatoric
(process noise, thermal fluctuation, disorder realization) is irreducible by more
data; epistemic (model error, parameter uncertainty, calibration drift) is reducible.
Collapsing them into a single error bar destroys the information needed to decide
whether to run more experiments or accept the variance. This distinction is standard
in uncertainty quantification and is routinely lost in inverse-design tooling.

**(3) `evidence` is mandatory output, not logging.** Every model invoked carries its
validity-domain predicate and the result of evaluating that predicate on the
candidate plan. A plan that exits a model's declared domain is reported as such and
its predictions are demoted from DERIVE to unsupported (§15 of the specification
layer).

**(4) `gaps` is a first-class return value.** On hard instances the most valuable
output is not a plan but a ranked list of measurements that would resolve the
blocking uncertainty. This makes the engine useful before it is capable, which
matters more for adoption than any single capability.

---

### 7.2 Three-valued compilation outcome

> **DEFINITION 7.1 (Compilation outcome).**
> Let 𝒫(Ω, L, T) be the set of plans over the action algebra 𝒜 of length ≤ L and
> horizon ≤ T that are admissible under 𝒯, 𝒞, ℰ. Define
>
> **FEASIBLE** — the engine exhibits a plan π and a measurement plan Ψ such that,
> for every m ∈ 𝔐 whose validity domain contains the plan's trajectory,
> Pr_m[A(Y)=PASS ∧ Φ(σ_T) ∉ 𝒦] ≤ δ.
> *Witness: the pair (π, Ψ) plus the model-validity certificates.*
>
> **INFEASIBLE(c)** — the engine exhibits an obstruction certificate c proving that
> **no** π ∈ 𝒫(Ω, L, T) satisfies the specification.
> *Witness: c, checkable independently of the search that produced it.*
>
> **UNKNOWN(r, b)** — neither witness was produced. r names what was missing; b
> records whether the search budget was exhausted.

> **KNOWN LIMIT 7.2.** The distinction between INFEASIBLE and UNKNOWN is not a
> presentation choice. `INFEASIBLE` is a claim about the world; `UNKNOWN` is a claim
> about the engine. An engine that reports the second as the first is unsound. An
> engine that reports the first as the second is merely incomplete. **Unsoundness is
> the failure to avoid; incompleteness is the normal condition**, because the general
> feasibility problem is undecidable (§16.2) and the bounded problem is at least
> PSPACE-hard (§16.4).

#### 7.2.1 What constitutes a valid infeasibility certificate

A certificate must be *independently checkable in time polynomial in its own size*,
without re-running the search. This is the standard from certifying algorithms and it
is achievable for the classes below.

| Obstruction class | Certificate | Checkable? | Class of the underlying bound |
|---|---|---|---|
| **Conservation** | A Farkas certificate (a nonnegative combination of the element/charge balance rows) proving the stoichiometric system infeasible over the declared inventory | **Yes**, LP duality, polynomial | Fundamental law |
| **Spectral (quantum, closed-system)** | A demonstration that spec(ρ₀) ≠ spec(ρ_T) for every ρ_T ∈ [S], given that the declared control set is purely unitary | **Yes**, compare spectra | Fundamental law, *conditional on the closed-system declaration* (§9.4.1) |
| **Thermodynamic** | ΔF computation with a bound on the model error, or — for the scoped quantum case — a thermomajorization violation with its assumptions discharged (§9.4.3) | **Yes**, given the model; the certificate is only as strong as the free-energy data | Fundamental law, model-relative |
| **Controllability** | For bilinear systems: a proof that the Lie algebra generated by {H₀, H_j} has dimension < required, i.e. the LARC fails. For linear systems: an unreachable subspace | **Yes**, finite computation | Control-theoretic |
| **Resource** | An accounting proof that any plan reaching [S] consumes more than 𝒞 of some resource — e.g. a lower bound on the number of required primitive operations times their unit draw | **Yes** when a valid lower bound exists; **such bounds are rare and this is the weakest certificate class** | Resource/economic |
| **Safety** | A reachability over-approximation (zonotope, Taylor model, barrier certificate) showing every admissible trajectory reaching [S] must pass through the unsafe set | **Yes** — barrier certificates are exactly this, and are checkable by SOS programming | Control-theoretic |
| **Metrological** | A proof that no subset of ℳ can distinguish [S] from a named alternative in Alt(S) at confidence 1−δ within budget — e.g. two members of 𝔐 predicting identical values for every available measurement | **Yes**, and this is the certificate class most under-used in practice | Information-theoretic |
| **Substrate-expressivity** | A proof that the target Hamiltonian/property lies outside the span of Λ_Ω — e.g. a symmetry the substrate cannot break, or a required interaction absent from the native family | **Yes** when the native family is declared algebraically | Fabrication/design |
| **Model-validity** | Not an infeasibility certificate. Every candidate plan exits every model's validity domain | **N/A** | → must return **UNKNOWN**, not INFEASIBLE |

> **PROPOSITION 7.3.** Of the nine classes above, eight yield certificates checkable
> in polynomial time in the certificate size. The ninth (model-validity) is not an
> obstruction at all and must be routed to UNKNOWN. A compiler that returns
> INFEASIBLE without a certificate from one of the eight is reporting search failure.
> **Classification: ESTABLISHED** (each certificate class is a standard construction);
> **the architectural requirement that they be mandatory is PLAUSIBLE and proposed
> here.**

#### 7.2.2 The two errors, named

- **Type-I compiler error (unsound rejection):** reporting INFEASIBLE when an
  admissible plan exists. Cost: a real capability is abandoned. Cause: treating an
  exhausted heuristic as a proof.
- **Type-II compiler error (unsound acceptance):** reporting FEASIBLE when the plan
  does not meet the specification. Cost: a defective artifact with an attestation.
  Cause: a model used outside its validity domain, or a chance constraint estimated
  by naive Monte Carlo on a rare event.

Type-II is the dangerous one, because it is the one that produces a signed
certificate of a false claim (§12).

---

### 7.3 Belief-space planning

The Draft 1 characterization — "a partially observed, model-uncertain,
chance-constrained, hybrid optimal-control problem" — is audited word by word.

| Word | Justified? | Precise content |
|---|---|---|
| **Partially observed** | **Yes** | The controller never has σ; it has a history of measurement outcomes. The sufficient statistic is the belief 𝔟 ∈ Δ(Σ), and the optimal policy is a map from beliefs, not states. |
| **Model-uncertain** | **Yes**, and this is the dominant term | 𝔐 is a *set* of models, and the true dynamics are frequently outside it (structural, not merely parametric, misspecification). |
| **Chance-constrained** | **Yes** | The specification is Pr[Φ ∈ 𝒦] ≥ 1−δ, a probabilistic constraint, not a cost. |
| **Hybrid** | **Yes** | Discrete mode switches (tool change, phase transition, reagent addition, measurement) interleave with continuous evolution. |
| **Optimal control** | **Only with qualification** | For the continuous stage. The discrete stage is AI planning, not optimal control, and conflating them imports the wrong algorithms. Draft 1 used "optimal control" for the whole, which was loose. |

> **KNOWN LIMIT 7.4 (No single algorithm).** The composite problem — POMDP over a
> stochastic hybrid system with a distribution over dynamics and rare-event chance
> constraints — has no known general solution algorithm, and each of the four
> features independently places it beyond exact methods at useful scale. Finite-horizon
> POMDP policy existence is already PSPACE-complete for the finite discrete case
> (Papadimitriou–Tsitsiklis), and the continuous-state, continuous-observation case is
> undecidable in general. **Classification: ESTABLISHED.**
> *Any claim that the engine "solves" this problem is ILL-POSED. The engine
> approximates it by decomposition, and the approximation is where the engineering is.*

#### 7.3.1 Epistemic versus aleatoric, operationally

```
𝔟 over Σ        — aleatoric: where the system is, given known dynamics
p(θ) over Θ     — epistemic (parametric): which parameters the dynamics has
p(m) over 𝔐     — epistemic (structural): which model form is right
```

The forecast bundle must report these separately because they license different
actions: high aleatoric variance argues for process control and tighter tolerances;
high parametric uncertainty argues for calibration experiments; high structural
uncertainty argues for discriminating experiments (§7.7) or for refusing to compile.

> **ENGINEERING CRITERION 7.5.** Report the *epistemic fraction*
> ε_frac = Var_epistemic / (Var_epistemic + Var_aleatoric) with every forecast.
> When ε_frac > ~0.5, additional process optimization is premature: the dominant
> term is ignorance, and the correct next action is measurement, not search.
> *This is a heuristic, not a theorem; the threshold is a design choice.*

---

### 7.4 Hybrid discrete/continuous optimization and what the decomposition costs

The practical architecture:

```
   IR₁ ──► [DISCRETE ROUTE SEARCH] ──► IR₂ ──► [CONTINUOUS PARAMETER SYNTHESIS] ──► IR₃
              over 𝒜, symbolic                      optimal control / protocol design
              parameters                            under 𝔐, chance-constrained
                    ▲                                         │
                    └────────── model-discrimination ─────────┤
                                feedback (§7.7)               ▼
                                                    [EXECUTION + FEEDBACK] ──► [VERIFICATION]
```

> **PROPOSITION 7.6 (Cost of the decomposition).** Two-stage decomposition — fixing
> the discrete structure before optimizing continuous parameters — forfeits global
> optimality whenever the optimal continuous parameters for route r₁ make r₁ worse
> than route r₂ under r₂'s optimal parameters, but the discrete stage's evaluation of
> r₁ used a surrogate that ranked r₁ first. Formally, the decomposition is exact only
> if the discrete-stage surrogate ranking is *order-consistent* with the fully
> optimized objective. **In general it is not, and no bound on the resulting
> suboptimality is available without solving the full problem.**
> **Classification: ESTABLISHED** (this is the standard bilevel-optimization
> observation); **the claim that the loss is acceptable in practice is PLAUSIBLE and
> is what F-2 tests.**

Mitigations that preserve some of the loss bound: keep k diverse routes rather than
one (so the discrete stage's ranking errors are recoverable); use optimistic
surrogates in the discrete stage (so pruning is conservative); and re-rank after
continuous synthesis.

> **OPEN PROBLEM 7.7.** Is there a class of physical synthesis problems for which
> the discrete surrogate can be made provably order-consistent — i.e. a decomposition
> with a suboptimality guarantee? Branch-and-bound over mixed-integer optimal control
> gives one in principle; whether it is computationally viable at the scale of real
> route search is unresolved.

---

### 7.5 The objective and risk functional

#### 7.5.1 The functional

> **DEFINITION 7.8 (Compilation objective).**
> ```
> 𝔍[π] = sup_{m ∈ 𝔐} {  𝔼_m[ C_energy + C_material + C_time + C_tooling ]
>                      + λ_V · 𝔼_m[ C_verify(Ψ) ]
>                      + λ_R · ϱ_m( C_total ) }
> ```
> subject to
> ```
>   Pr_m[ Φ(σ(T)) ∈ 𝒦 ] ≥ 1 − δ        ∀ m ∈ 𝔐      (specification)
>   Pr_m[ σ(t) ∈ Safe  ∀ t ∈ [0,T] ] ≥ 1 − δ_s       (safety)
>   resource draws ≤ 𝒞                                (inventory; linear)
> ```
> where ϱ is a **declared** risk functional (§7.5.3) and C_total includes rework,
> scrap, and downstream failure cost.

#### 7.5.2 Dimensional and structural audit

| Issue | Draft 1 | Correction |
|---|---|---|
| `∫E(t)dt` with E an energy | energy·time — dimensionally wrong | `C_energy = ∫ p_E · Ẇ_in(t) dt` with Ẇ_in a **power** and p_E a price; result in currency |
| `∫T(t)dt` with T a time | time² | `C_time = p_T · T` — a **terminal** cost, not a running one |
| `αE + βṠ_irr` | double counts | For an isothermal process W_diss = T₀ ΔS_irr. Dissipation is paid once, through C_energy. A separate environmental-burden term may be added as an explicit *resource* (waste heat, coolant) with its own price — never as an abstract entropy term with a free weight |
| `γD(t)` with D a probability | probabilities are not rates and do not integrate | Defect *formation* is a hazard rate λ(t); the terminal quantity is a survival probability exp(−∫λ dt). Defect probability enters the **chance constraint**, not the cost |
| No terminal term | — | The specification is a terminal chance constraint; safety is a path constraint |
| No verification cost | — | λ_V·C_verify — the framework's distinctive term |
| No model uncertainty | — | sup over 𝔐 |
| Specification as a weighted cost | conceptually wrong | **The specification is a constraint.** You do not trade "how much the artifact meets spec" against energy; you meet it at confidence or you do not ship. This removes an arbitrary weight and matches industrial and regulatory practice |

**All terms must be in a single numeraire.** Converting energy, time, and material to
currency embeds an economic model inside a physical objective; that is legitimate for
a production decision and inappropriate for a research decision, which is why §8.8
defines a separate, price-free research objective.

#### 7.5.3 Risk functionals: when each is correct

Draft 1 asserted CVaR. That was an unjustified default.

> **ENGINEERING CRITERION 7.9 (Choice of risk functional).** ϱ is a modeling
> decision that must be **declared and justified**, not inherited.

| ϱ | Appropriate when | Inappropriate when | Note |
|---|---|---|---|
| **Expectation** 𝔼[C] | High volume, independent repetitions, losses small relative to the firm's balance sheet; the law of large numbers applies | Single-shot; correlated failures; ruinous tails | The default in optimization and usually wrong for one-off synthesis |
| **Worst case** max_ω C(ω) | Safety-critical; adversarial; regulatory hard limits | The uncertainty set is loose — gives absurd conservatism and often infeasibility | Robust control's default |
| **CVaR_α** | Tail-sensitive but not worst-case-driven; a coherent risk measure with convex programming formulations | **The tail of the model is untrustworthy** — which is exactly the regime here | **The serious objection to CVaR in this setting:** it is an expectation *conditional on the tail*, so it inherits all the error of the least-validated part of the model. Using it presumes tail fidelity you do not have |
| **Entropic risk** (1/γ)log𝔼[e^{γC}] | Constant absolute risk aversion; and its dual is exactly worst-case expectation over a KL ball — so it *is* a distributionally robust formulation | Heavy-tailed C (the exponential moment may not exist) | The cleanest link between risk aversion and model uncertainty |
| **Chance constraint** Pr[C > c] ≤ p | A hard requirement with a tolerated violation rate (regulatory, contractual) | Needs rare-event estimation; generally non-convex | Already used for the specification itself |
| **Multi-objective Pareto** | Weights are genuinely contested or unknown; the decision belongs to a human | Automated pipelines needing a total order | **Recommended default for the research engine**: return the Pareto set and let the weights be an explicit, auditable input |

> **PROPOSITION 7.10.** Because model tails are the least-validated part of 𝔐,
> tail-conditional risk measures (CVaR) and the sup over 𝔐 are partially redundant
> and can compound conservatism unpredictably. Prefer **entropic risk with a declared
> KL radius**, or an explicit chance constraint, over CVaR when 𝔐 is a set rather
> than a point. **Classification: PLAUSIBLE** — the duality underpinning it is
> ESTABLISHED; the recommendation is a judgment.

#### 7.5.4 Rare-event estimation

> **KNOWN LIMIT 7.11.** A chance constraint at δ = 10⁻⁶ cannot be verified by naive
> Monte Carlo: the relative error of a crude estimator of a probability p scales as
> 1/√(Np), requiring N ≫ 10⁶ samples of a simulation that may cost hours. Viable
> methods — importance sampling with a good tilting, subset simulation, large-deviation
> asymptotics, extreme-value tail fits, failure-boundary surrogates — all require
> structural knowledge of the failure mode. **Classification: ESTABLISHED.**
> *Consequence: the engine's stated δ is only as good as its rare-event method, and
> that method must be part of the attestation (§12).*

---

### 7.6 Verification-aware optimization

The term λ_V·C_verify is what distinguishes this objective from standard
process optimization. Its content is developed as Physical Design-for-Verifiability
in §9.9; here only the engine-side statement.

> **DEFINITION 7.12.** Verification-aware synthesis is the joint problem
> ```
> minimize over (π, Ψ)    𝔍[π] + λ_V · C_verify(Ψ)
> subject to              Pr[A(Y)=PASS ∧ Φ(σ_T) ∉ 𝒦] ≤ δ
> ```
> — i.e. the route and its acceptance test are chosen **together**, so that routes
> whose outcomes are cheaply distinguishable from their failure modes are preferred
> over routes that are equally good physically but require expensive or destructive
> certification.

> **FALSIFICATION TEST 7.13.** See F-4 (§ audits file): among routes that equally
> satisfy a specification, does joint optimization reduce cost-to-certify by ≥30% at
> equal attained confidence, with ≤10% production-cost penalty? A result below 10%
> falsifies the economic claim, leaving the idea true-but-immaterial.

---

### 7.7 Knowledge-gap generation: the third compiler outcome

> **DEFINITION 7.14 (Knowledge-gap experiment).** When 𝔐 = {m₁,…,m_k} and members
> disagree about whether a candidate plan satisfies the specification by more than
> the tolerance band, the engine emits
> ```
> e* = argmax_{e ∈ E}  I(m ; y_e)     subject to  cost(e) ≤ B, safety(e)
> ```
> the experiment maximizing mutual information between model identity and outcome,
> subject to cost and safety.

This makes the architecture three-way rather than two-way:

```
    ┌──────────┐        ┌────────────┐        ┌──────────┐
    │ COMPILE  │        │  OBSTRUCT  │        │ MEASURE  │
    ├──────────┤        ├────────────┤        ├──────────┤
    │ (π,Ψ,Π)  │        │ certificate│        │ e*, EVOI │
    │ with     │        │ from the   │        │ and what │
    │ bounded  │        │ eight      │        │ it would │
    │ risk     │        │ classes    │        │ resolve  │
    └──────────┘        └────────────┘        └──────────┘
```

> **PROPOSITION 7.15.** The MEASURE branch is not a failure mode. It is the correct
> output whenever the expected value of information from the best available
> experiment exceeds the expected value of committing to the best available plan.
> Formally, emit MEASURE when
> ```
>   EVOI(e*) − cost(e*)  >  𝔼[gain from executing the current best plan now]
> ```
> **Classification: ESTABLISHED** as decision theory (the value-of-information
> criterion is standard); **PLAUSIBLE** as an architectural principle for compilers.

**Prior art, stated so no novelty is claimed:** this is Bayesian optimal experimental
design (Lindley 1956; Chaloner–Verdinelli 1995), value of information (Howard 1966),
and active learning, with close operational precedent in self-driving laboratories
which already select maximally informative experiments. **What is proposed here is
narrower and is an architectural claim, not a methodological one: that MEASURE should
be a typed compiler outcome alongside success and failure, so that a compiler's
inability to decide is reported as an actionable experiment rather than as an error.**

> **CAUTION (audited).** I(m; y_e) as a function of an experiment *set* is **not
> submodular in general**; submodularity holds under conditional-independence
> structure, and otherwise only weakly, with a submodularity ratio γ giving a
> (1−e^{−γ}) greedy guarantee. See §9.8.3 for the full audit. Claims that greedy
> experiment selection carries a (1−1/e) guarantee are **only valid for the coverage
> formulation**, not the information formulation.

---

### 7.8 Formal limits of the engine

> **KNOWN LIMIT 7.16.** General feasibility over hybrid dynamics is **undecidable**
> (reachability for piecewise-constant-derivative systems in dimension ≥3 and for
> linear hybrid automata above the rectangular class). Bounded feasibility is
> decidable and at least PSPACE-hard (propositional planning is PSPACE-complete;
> with unbounded numeric fluents it becomes undecidable again). **ESTABLISHED.**

> **KNOWN LIMIT 7.17.** Discrete chemical reaction networks are vector addition
> systems; reachability is decidable and **Ackermann-complete** — non-primitive-
> recursive. **ESTABLISHED (2021).** Retrosynthesis is viable only because it
> restricts to bounded depth with curated templates, which is precisely the RC-0′
> restriction.

> **KNOWN LIMIT 7.18 (Scope discipline).** Every hardness result above bounds
> **algorithms deciding questions about arbitrary instances.** None bounds physical
> achievability of a particular target. A system reaching its ground state is not
> violating QMA-hardness; it is declining to solve the general problem. This sentence
> governs every complexity claim in this monograph.

---

## 8. Hamiltonian, Landscape, and Attractor Compilation

### 8.1 Landscape engineering: the preserved insight

> **PROPOSITION 8.1.** Landscape engineering is a **control strategy** — it
> substitutes designed dynamics for per-degree-of-freedom actuation — and it does not,
> by itself, constitute a computational speedup. These are independent claims and
> conflating them is the central error to avoid in this area.
> **Classification: ESTABLISHED as a distinction; the separation is developed in §8.8.**

As control, landscape engineering is how essentially all synthesis at scale works:
crystallization does not place molecules, it sets supersaturation and a thermal
profile. **ESTABLISHED and economically dominant.**

### 8.2 Hamiltonian compilation

> **DEFINITION 8.2 (Hamiltonian compilation).** Given a specification S and a
> substrate with realizable control-parameter space Λ_Ω, find λ* ∈ Λ_Ω such that the
> effective Hamiltonian H_eff(λ*) — obtained from the microscopic Hamiltonian by a
> declared reduction (Schrieffer–Wolff, Floquet–Magnus averaging, adiabatic
> elimination, or a low-energy projection) — has spectral or dynamical properties
> placing the target in [S].

Two things must be declared with every H_eff, and usually are not:

1. **The reduction's small parameter and its validity window.** Schrieffer–Wolff is
   perturbative in (coupling/detuning); Floquet–Magnus converges only for drive
   frequency above a bandwidth-dependent threshold and the series is generically
   asymptotic, not convergent. Outside the window, H_eff is not an approximation of
   anything.
2. **The heating channel.** Floquet engineering has no static ground state; a driven
   many-body system generically heats toward infinite temperature. What is engineered
   is a **prethermal** window whose lifetime scales (for high-frequency drives)
   exponentially in ω/J. The target is therefore metastable by construction, and the
   specification must carry a lifetime requirement. **ESTABLISHED.**

### 8.3 Dissipative compilation

> **DEFINITION 8.3.** Find engineered jump operators {L_k(λ)} such that the
> Liouvillian ℒ_λ(ρ) = −(i/ħ)[H(λ),ρ] + Σ_k D[L_k(λ)]ρ has a steady state ρ_ss ∈ [S],
> unique within the relevant sector, with mixing time 1/Δ_L ≤ T_max.

**Scoped correctly (audited):**

> **THEOREM 8.4 (scope of dissipative universality).** Dissipative dynamics is a
> universal resource for state preparation and computation: for suitable engineered
> Markovian generators, target states can be prepared as unique steady states, and
> the preparation is *autonomous* — no measurement or feedback is required, and the
> state is restored after perturbation. **ESTABLISHED** (Verstraete–Wolf–Cirac 2009;
> reservoir engineering originating with Poyatos–Cirac–Zoller 1996).
>
> **What this does NOT say.** It does not say "any state can be made a steady state"
> without qualification. The admissible generator class matters decisively:
> - With **arbitrary (possibly non-local, possibly high-weight) jump operators**,
>   preparing any pure target as a unique fixed point is essentially trivial and
>   physically vacuous.
> - With **quasi-local jump operators of bounded weight**, the preparable pure states
>   are essentially those that are unique ground states of **frustration-free** local
>   parent Hamiltonians — the condition under which a local dissipative process can
>   drive every local term to its minimum simultaneously.
> - **Frustrated** targets are not generally reachable this way, and this is the
>   honest boundary of the dissipative strategy.
> **Classification of the scoped statement: ESTABLISHED. Classification of the
> unscoped folk version: ILL-POSED.**

> **KNOWN LIMIT 8.5.** Dissipative preparation trades the Hamiltonian gap Δ for the
> Liouvillian gap Δ_L. This is a genuine advantage (Δ_L can be large where Δ is
> small) and not a free lunch: Δ_L is itself hard to bound, can close, and engineered
> dissipation costs control resources and injects its own noise.

### 8.4 Attractor compilation

> **DEFINITION 8.6 (Attractor Compilation).** Given a controlled dynamical system —
> classical ẋ = f_θ(x) + noise, or quantum ρ̇ = ℒ_θ(ρ) — find physically realizable
> θ* ∈ Λ_Ω such that
>
> **(A1) Reachability** — [S] is reachable from the declared initial belief under the
> dynamics generated by θ*;
> **(A2) Attractivity** — [S] contains an *invariant set that is attracting* in the
> declared sense (below), or a metastable set with lifetime ≥ the specified service
> life;
> **(A3) Rate** — convergence into [S] occurs within T_max;
> **(A4) Selectivity** — competing invariant sets either lie inside [S] or have
> escape/capture rates below a declared bound;
> **(A5) Robustness** — the margins of §8.5 exceed the declared perturbation classes;
> **(A6) Observability** — the resulting state is measurable and verifiable within
> budget (§9.9). *A6 is not decoration: an attractor that cannot be distinguished
> from its competitors is not a compilation target.*

#### 8.4.1 The generalized usage, made explicit

"Attractor" is used above in a generalized sense that must be defined, because the
objects below are genuinely different and are routinely conflated.

| Object | Setting | Precise meaning | Relevant stability notion |
|---|---|---|---|
| **Equilibrium** | ẋ = f(x) | f(x*) = 0 | none by itself |
| **Stable equilibrium** | deterministic ODE | Lyapunov-stable; asymptotically stable if nearby trajectories converge | eigenvalues of Df; Lyapunov function |
| **Attractor (proper)** | deterministic | compact invariant set with an open basin, minimal under invariance | basin of attraction |
| **Limit cycle** | deterministic | isolated periodic orbit | Floquet multipliers |
| **Strange attractor** | deterministic | attracting invariant set with sensitive dependence | Lyapunov exponents. **Almost never a synthesis target**, and listing it is a warning, not a menu item |
| **Metastable state** | stochastic / thermal | a set with long escape time but *not* invariant; the true stationary measure puts mass elsewhere | Kramers/Arrhenius escape rate; quasi-potential |
| **Stationary distribution** | classical Markov | invariant measure of the generator | spectral gap → mixing time |
| **Lindbladian steady state** | open quantum | ρ_ss with ℒ(ρ_ss)=0 | Liouvillian gap Δ_L |
| **Ground state** | closed quantum, T=0 | lowest eigenvector of H | spectral gap Δ. **Not an attractor of unitary dynamics** — closed-system evolution does not converge to it (§9.4.1) |
| **Free-energy minimum** | classical thermal | minimizer of F = U − TS in a coarse-grained coordinate | barrier height ΔF‡ |
| **Nonequilibrium attractor** | driven | e.g. a time-crystalline orbit or a driven steady state; no equilibrium counterpart | Floquet/Liouvillian spectrum; heating time |

> **KNOWN LIMIT 8.7 (The ground state is not an attractor).** A closed quantum system
> does not relax to its ground state; unitary evolution preserves energy and the
> spectrum of ρ. "Cooling into the ground state" requires coupling to a colder
> reservoir, engineered dissipation, or measurement. Calling the ground state an
> attractor is a category error that licenses invalid protocol designs.
> **ESTABLISHED.** (See §9.4.1 for the spectral-invariance statement and its exact
> scope.)

> **PROPOSITION 8.8 (Most engineering targets are metastable, not attracting).**
> Steel, glass, most polymers, diamond at ambient conditions, most pharmaceutical
> polymorphs, and every Floquet-engineered phase are **metastable**: the true
> stationary state is something else, and the artifact exists because the escape rate
> is small. Attractor compilation must therefore treat metastability as a *first-class
> target type*, with lifetime as a specified property — not as a degenerate or
> undesirable case. **Classification: ESTABLISHED** (as a description of materials
> practice). *This corrects a bias in Drafts 1–2, which treated attracting sets as the
> goal and metastability as a nuisance.*

### 8.5 Stability and robustness: the figure of merit

Basin volume is the intuitive descriptor and is insufficient.

> **PROPOSITION 8.9 (Why basin volume fails).** Basin volume is (i) not invariant
> under coordinate change — it depends on a choice of measure on state space, which
> is arbitrary; (ii) uninformative at finite temperature, where escape is governed by
> *barrier height and attempt frequency*, not by volume; (iii) dominated in high
> dimension by directions irrelevant to the dynamics, so it concentrates on
> geometrically large but dynamically inert regions; (iv) irrelevant to the actual
> preparation, which arrives along a specific trajectory — what matters is whether a
> *tube around the prepared trajectory* lies in the basin, not the basin's total
> measure. **Classification: ESTABLISHED** (each point is standard).

> **DEFINITION 8.10 (Attractor figure of merit).**
> ```
>   𝔄 = ( Δ , B , τ , κ , Γ , D )
> ```
>
> | Component | Definition | Operational measurement |
> |---|---|---|
> | **Δ** — gap | Quantum closed: spectral gap E₁−E₀. Open: Liouvillian gap (slowest nonzero decay rate). Classical stochastic: spectral gap of the generator. Classical deterministic: the negative real part of the least-stable eigenvalue of the linearization | spectroscopy; decay of an initialized perturbation; correlation-time fits |
> | **B** — stability radius | The minimum perturbation, in a declared norm on the **control parameters**, that destroys A2 (target ceases to be attracting or metastable-with-required-lifetime): B = inf{‖δθ‖ : A2 fails at θ*+δθ} | parameter sweeps to the boundary; for stochastic systems, the Freidlin–Wentzell quasi-potential barrier ΔV, with escape rate ~ exp(−ΔV/ε) |
> | **τ** — relaxation time | Time to enter [S] from the declared initial belief, to within the specified tolerance. For Markovian dynamics τ ≈ 1/Δ; **in general τ ≠ 1/Δ** when the dynamics is non-normal or the initial state is far from the slow manifold | direct kinetics measurement |
> | **κ** — conditioning | Relative sensitivity of the *specified property* to the control parameters: κ = ‖∂Φ/∂θ‖·‖θ‖ / ‖Φ‖, a relative condition number | finite-difference or adjoint sensitivity |
> | **Γ** — competing-transition rate | Total rate of escape into any competing invariant set outside [S]; the Arrhenius/Kramers rate summed over exit channels | accelerated aging; polymorph conversion studies |
> | **D** — defect-generation rate | Density of topological or structural defects produced by the preparation itself. For a quench through a critical point, Kibble–Zurek gives n ~ τ_q^(−dν/(1+νz)) | direct imaging; scattering |

> **PROPOSITION 8.11 (Six axes, not one).** No single scalar orders attractors for
> engineering purposes, because the six components respond to different perturbation
> classes and are traded against each other by design choices. A deep, narrow basin
> (large B, small basin volume) is excellent against parameter drift and poor against
> large disturbances; a fast relaxation (small τ) frequently comes with a large
> defect rate D (Kibble–Zurek); high selectivity (small Γ) typically requires a large
> Δ, which constrains the accessible λ. **Classification: PLAUSIBLE** — the individual
> trade-offs are ESTABLISHED; the claim that six axes suffice and are the right six is
> a proposal, tested by F-6.

#### 8.5.1 Six stability notions, kept distinct

| Notion | Question | Governed by |
|---|---|---|
| **Thermodynamic stability** | Is it the global free-energy minimum? | ΔF relative to competing phases |
| **Kinetic stability** | How long until it leaves? | barrier heights, Γ, attempt frequencies |
| **Dynamical stability** | Do nearby trajectories converge? | Lyapunov/Floquet spectrum, Δ |
| **Robustness to parameter perturbation** | Does it survive θ → θ+δθ? | **B**, κ |
| **Robustness to environmental noise** | Does it survive stochastic forcing at the actual noise spectrum? | Δ vs noise power spectral density; not Δ alone |
| **Robustness to manufacturing variation** | Does the *population* meet spec? | the chance constraint, over the realized θ distribution |

Diamond is thermodynamically unstable, kinetically extremely stable, and that
combination is why it exists as a product. Any framework whose stability descriptor
cannot express that is inadequate.

### 8.6 Competing attractors

> **ENGINEERING CRITERION 8.12.** Selectivity requires, for each competing invariant
> set C_j ∉ [S]:
> ```
>   (i)  capture: Pr[trajectory lands in C_j's basin | preparation protocol] ≤ p_j
>   (ii) escape:  rate from [S] into C_j  ≤ Γ_j , with Σ_j Γ_j · t_service ≤ ε
> ```
> Both are needed: (i) is a *preparation* property and (ii) is a *service-life*
> property, and passing one says nothing about the other.

The industrially canonical instance is **polymorph selection**: the desired
pharmaceutical form is frequently metastable, competing forms are thermodynamically
favored, and control is entirely kinetic — via supersaturation profile, solvent,
seeding, and shear. A compiler reasoning only about thermodynamic ground states
returns the wrong polymorph and passes its own simulated check. **ESTABLISHED as a
failure mode.**

### 8.7 Kinetic feasibility, and the retraction of the τ_relax criterion

#### 8.7.1 The claim under audit

Draft 2 (§32.3) asserted: *"The criterion is the relaxation time, and nothing else."*

#### 8.7.2 Audit

**Is sub-exponential τ_relax necessary?** **No.** The compiler must reach the
*acceptance region*, not the attractor. If [S] contains a long-lived metastable set
reachable by a quench, then the relaxation time to the *global* attractor being
exponential is not an obstacle — **it is the mechanism of the product**. Diamond,
steel, glass, and every Floquet phase exist because relaxation to equilibrium is
exponentially slow. Requiring sub-exponential relaxation would exclude most of
materials engineering.

**Is sub-exponential τ_relax sufficient?** **No.** Fast relaxation does not help if
the attractor is outside [S] (A2 fails), if the quench generates defects above
tolerance (D fails), if competing attractors capture the trajectory (A4 fails), or if
the result cannot be verified (A6 fails).

#### 8.7.3 Retraction and replacement

> **RETRACTION.** Draft 2's claim that relaxation time is the sole criterion for
> landscape-compilation advantage is **withdrawn**. It conflated *reaching the
> attractor* with *reaching the acceptance region*, and metastability — on which most
> of the materials industry depends — is the counterexample. The error was a
> reasoning error, not a citation error.

> **ENGINEERING CRITERION 8.13 (Total verified cost and time).** Landscape
> compilation is preferred over explicit trajectory control when, against the best
> available alternative route,
> ```
>   C_PSE = C_setup + C_control + C_relaxation + C_measurement + C_failure
>   T_PSE = T_setup + T_relaxation + T_verification
> ```
> are jointly lower at equal attained confidence — where C_setup includes the
> **amortized** compile-time cost of solving the inverse problem, divided by
> production volume.
>
> This is an engineering criterion, not a theorem. It has three consequences the
> τ_relax criterion missed:
> - **Amortization decides.** A one-time inverse-design cost divided by 10⁶ units is
>   negligible; divided by 1 unit it dominates. Landscape compilation's advantage is
>   therefore *volume-dependent*, which τ_relax does not capture.
> - **Metastable targets are admissible** and often preferred, with lifetime as a
>   specified property.
> - **Verification cost is inside the criterion**, so a route that relaxes quickly to
>   a state nobody can certify does not win.

> **PROPOSITION 8.14 (Where τ_relax still binds).** Sub-exponential relaxation
> remains **necessary** in the specific case where the specification requires the
> *true* attractor (equilibrium phase, ground state, or unique steady state) and no
> metastable member of [S] is acceptable. That case includes ground-state preparation
> for quantum matter and equilibrium phase determination, and excludes most bulk
> materials engineering. **Classification: ESTABLISHED within the stated scope.**

### 8.8 Physical computational leverage

#### 8.8.1 The distinction that must be maintained

> **DEFINITION 8.15 (Physical evaluation).** A physical experiment returns a *sample*
> from the system's own dynamics. It is an oracle with a substrate-determined cost per
> query, high latency, stochastic output, and an output restricted to what was
> measured. It admits no counterfactuals.

> **DEFINITION 8.16 (Algorithmic speedup).** A demonstrated asymptotic separation in
> resource scaling between a physical procedure and the best known classical algorithm
> **for the same well-posed computational problem**, with the problem, the input
> encoding, the precision model, and the success criterion all specified.

> **PROPOSITION 8.17 (The comparison is usually a category error).** For *synthesis*,
> physical evaluation and algorithmic computation are not commensurable: physical
> evaluation yields the artifact, while simulation yields a prediction about an
> artifact one must still make. Claiming a "speedup" for the former requires first
> naming a computational problem that both solve, and for synthesis there generally
> is none. **Classification: ESTABLISHED as a logical point.**
> *Consequence: "physics computes it for free" is ILL-POSED as stated. What is true
> and much narrower: the substrate performs, at unit cost per query, an evaluation
> whose simulation may be intractable — and that is valuable without being a speedup.*

#### 8.8.2 Case-by-case audit

| Case | Is there a genuine algorithmic speedup? | Classification |
|---|---|---|
| **A. Physical self-organization** (crystallization, phase separation) | No computational problem is being solved; the leverage is *control-theoretic* (10²³ degrees of freedom placed with ~10 knobs) | **ESTABLISHED as control leverage; ILL-POSED as speedup** |
| **B. Physical computation** (analog/in-memory kernels) | Constant-factor energy and latency advantages for specific kernels; no asymptotic separation under realistic noise/precision models | **STRONGLY SUPPORTED (constant factor); CONTRADICTED (super-Turing claims)** |
| **C. Quantum simulation of quantum dynamics** | Yes — BQP-complete; believed to require exponential classical resources. This is the strongest case in the table | **STRONGLY SUPPORTED**, conditional on BQP ⊄ BPP; with the caveat that verification in the hard regime is itself open |
| **D. Analog computation** | Super-Turing claims rely on infinite precision; under any realistic noise model they collapse | **CONTRADICTED** (super-Turing); **PLAUSIBLE** (constant factors) |
| **E. Combinatorial optimization by annealing** | No consistent scaling advantage on generic hard instances under fair comparison with tuned classical baselines | **CONTESTED, tending CONTRADICTED** for generic instances; structured families remain open |
| **F. Self-assembly** | Yes in a precise and unusual sense: the computation *is* the fabrication, and programmed assembly is provably exponentially more compact than explicit assembly (Θ(log N/log log N) tile types for an N×N square) | **ESTABLISHED** |
| **G. Dissipative state preparation** | Yes for depth: adaptive/dissipative routes can beat unitary-circuit depth lower bounds; and the state is prepared, not merely predicted | **ESTABLISHED** (depth separation for measurement-assisted preparation); **the dissipative case is gap-agnostic rather than asymptotically separated** |
| **H. Catalytic selectivity** | No computational claim. The leverage is selective barrier lowering — the most economically important landscape-engineering technology in existence | **ESTABLISHED as control leverage** |
| **I. Materials processing** | No computational claim; kinetic control of metastable states | **ESTABLISHED as control leverage** |

> **PROPOSITION 8.18.** Of nine cases, **two** (C, F) support a defensible
> computational-advantage claim and one (G) supports a depth separation. Six are
> control leverage with no computational content. **Generic "physics computes for
> free" and generic quantum-annealing-speedup claims are not supported by this
> table and should not appear in the manuscript.**

#### 8.8.3 The research objective

> **DEFINITION 8.19 (Physical computational leverage).**
> ```
>              I_verified(S)
>   ℒ(S,Ω) = ─────────────────────
>             K(π*) + C_ver(Ψ)
> ```
> — certified information content of the specification, divided by the description
> length of the control program plus the verification cost.
>
> ℒ is price-free (unlike 𝔍) and therefore appropriate for ranking *research*
> directions rather than production routes. ℒ ≫ 1 indicates the physical dynamics are
> carrying the load; ℒ ≈ 1 indicates explicit instruction execution and no leverage.
> **Classification: PLAUSIBLE** — proposed here; tested by F-14 (Draft 3).
>
> **Honest caveat.** I_verified and K both require a declared representation, so ℒ is
> representation-relative. It is a useful comparative measure within a fixed
> representation and not an absolute quantity. Claims of the form "ℒ = 10²²" are
> order-of-magnitude illustrations, not measurements.

### 8.9 Inverse Hamiltonian design and its neighbors

These six problems are routinely conflated and have different inputs, outputs, and
complexity.

| Problem | Given | Find | Character |
|---|---|---|---|
| **Inverse Hamiltonian design** | a target state or phase | H in a realizable family with that ground/steady state | Constructive for tensor-network targets (parent-Hamiltonian construction); search otherwise |
| **Materials inverse design** | target macroscopic properties | composition + structure + process | Enormous combinatorial search; the classical case |
| **Hamiltonian learning** | experimental data from an unknown system | the H that generated it | Estimation/identification; sample-complexity results exist |
| **Quantum control** | a known H and a target state | the pulse sequence u(t) | Optimal control; LARC settles controllability |
| **Phase engineering** | a target phase | λ in a phase diagram | Requires the phase diagram, which is the expensive part |
| **Materials synthesis** | a target compound | a route that actually makes it | Kinetics; the field's central unsolved problem |

> **PROPOSITION 8.20.** The compiler's effort allocation differs by target class: for
> classical materials the search (inverse design) dominates and the making is routine;
> for quantum matter the *making* (preparation, §9.5) dominates and the search over a
> low-dimensional Λ_Ω is comparatively easy. **Classification: PLAUSIBLE**, argued
> from the dimensionality of the respective search spaces.

#### 8.9.1 Complexity audit of the inverse problem

The Draft 1 sentence *"even verifying (a) is QMA-hard"* is **withdrawn as imprecise**.
The correct statements, with their exact decision problems:

> **KNOWN LIMIT 8.21 (Local Hamiltonian).** The *k*-LOCAL HAMILTONIAN problem —
> a promise problem: given a *k*-local H with bounded-norm terms and thresholds
> a < b with b − a ≥ 1/poly(n), decide λ_min(H) ≤ a or λ_min(H) ≥ b — is
> **QMA-complete** for k ≥ 2, including on physically realistic 2D lattices.
> **ESTABLISHED.** *This is a statement about ground-state* **energy**, *not about
> ground-state properties.*

> **KNOWN LIMIT 8.22 (Ground-state properties: the correct result).** The problem
> actually relevant to condition A2 — estimating a local observable's expectation on
> the ground state (APX-SIM) — is **P^QMA[log]-complete**: decidable by a polynomial-
> time machine making logarithmically many adaptive queries to a QMA oracle. This
> holds for 5-local H with a 1-local observable, and P^QMA[log]-completeness has been
> extended to more physically realistic Hamiltonians and to two-point correlation
> functions. **ESTABLISHED** (Ambainis 2014, with a gap in the original hardness proof
> subsequently repaired; Gharibian–Yirka; Gharibian–Piddock–Yirka).
> *So the relevant problem is not QMA-complete — it sits in a class believed to
> strictly contain QMA. Draft 1 understated it while appearing to overstate it.*

> **KNOWN LIMIT 8.23 (Routing between states: ground-state connectivity).** GSCON —
> deciding whether two ground states are connected by a sequence of local unitaries
> staying within the low-energy space — ranges from **QCMA-complete** to
> **PSPACE-complete** depending on the allowed path length and error parameters, and
> is **NEXP-complete** in a succinct encoding. **ESTABLISHED** (Gharibian–Sikora).
> *This is the exact formalization of the attractor-compilation routing question
> "can I deform to the target without closing the gap," and it is the most directly
> relevant complexity result to §8.4 that Drafts 1–3 did not cite.*

> **KNOWN LIMIT 8.24 (Thermodynamic-limit gap).** Whether a translationally invariant
> nearest-neighbour 2D model is gapped or gapless in the thermodynamic limit is
> **undecidable**. **ESTABLISHED.** Scope: a constructed family, in the infinite-size
> limit. It forecloses a general property oracle and says nothing about any finite
> instance, every one of which is decidable and possibly intractable.

> **KNOWN LIMIT 8.25 (Classical analogue).** Finding the ground state of a 3D Ising
> spin glass is **NP-hard**; the decision version (∃ configuration with energy ≤ E)
> is **NP-complete**. The analogue of APX-SIM — deciding a property of the ground
> configuration — sits naturally at **P^NP[log]** by the same construction.
> **ESTABLISHED for the first two; the third is a reasonable inference and is marked
> PLAUSIBLE pending a citation.** *SOURCE VERIFICATION REQUIRED for the
> P^NP[log] classification.*

> **PROPOSITION 8.26 (Scope, restated).** All five limits above concern
> **worst-case decision problems over arbitrary instances with a promise gap of
> 1/poly**. They do not imply that any particular Hamiltonian's ground-state
> properties are hard to determine, that any particular material cannot be made, or
> that heuristics fail on structured instances. Structured physical instances are
> not worst-case instances, and the evidence that DMRG, quantum Monte Carlo, and
> tensor-network methods succeed routinely is evidence about instance structure, not
> a contradiction of these theorems.

### 8.10 Emergence compilation, and the status of the hierarchy

> **The proposed hierarchy.**
> ```
> Instruction Compilation → Trajectory Compilation → Landscape Compilation
>                        → Attractor Compilation → Emergence Compilation
> ```
> **This is a proposed theoretical hierarchy, not an established result.** Its status,
> level by level:

| Level | What it means | Status |
|---|---|---|
| **Instruction compilation** | Emit a sequence of placements/operations; one actuator per degree of freedom | **ESTABLISHED** — CNC, pick-and-place, lithographic layout, SPM manipulation. Mature and throughput-limited |
| **Trajectory compilation** | Emit a time-dependent control signal steering a known system along a computed path | **ESTABLISHED** — GRAPE/Krotov/CRAB pulse design, NMR, model-predictive control, adjoint-optimized processes |
| **Landscape compilation** | Emit parameters shaping the energy/free-energy/effective-Hamiltonian surface so the target is favored | **ESTABLISHED as practice** (crystallization, catalysis, directed self-assembly, reservoir engineering) — **but the general inverse problem is open**, and "landscape engineering" is existing terminology in protein folding and self-assembly, not a new concept |
| **Attractor compilation** | Emit dynamical parameters satisfying A1–A6 with declared robustness margins | **SYNTHESIS, not invention.** Its components exist: inverse statistical mechanics for targeted self-assembly, reservoir engineering, dissipative state preparation, control-Lyapunov design. What is proposed here is a **unified problem statement with a common six-axis figure of merit across classical and quantum substrates and an explicit verifiability condition (A6)**. That unification is **PLAUSIBLE and untested** |
| **Emergence compilation** | Emit *local rules* whose collective dynamics produce a global target not present in any rule | **ESTABLISHED in exactly one substrate** — algorithmic self-assembly, where tile sets are Turing-universal and programmed assembly is provably exponentially more compact than explicit assembly. **SPECULATIVE as a general capability**; there is no general theory mapping global specifications to local rules outside tile assembly and a few reaction–diffusion cases |

> **PROPOSITION 8.27 (What the hierarchy actually orders).** The levels are ordered by
> **the ratio of specification length to product complexity** — equivalently by the
> compression ratio κ = K(generating rule)/K(target) — and *not* by capability, power,
> or sophistication. Instruction compilation is not primitive because it is crude; it
> is the correct choice when κ ≈ 1 and there is nothing to compress. **Classification:
> PLAUSIBLE** — proposed; tested by F-14.

> **OPEN PROBLEM 8.28.** Is there a general procedure mapping a global specification
> to local rules whose dynamics realize it — an "emergence compiler" — outside tile
> assembly? Partial answers: inverse statistical mechanics for targeted self-assembly,
> pattern selection in reaction–diffusion, and parent-Hamiltonian construction for
> tensor-network states are each a restricted instance. A general theory does not
> exist, and its absence is the principal gap between the hierarchy's top level and
> engineering practice.

---

*Continues in Part IV (`04-quantum-substrate.md`). The claim audit table,
falsification program, novelty audit, and adversarial review for Parts III–IV are in
[`15-audits-parts-III-IV.md`](15-audits-parts-III-IV.md).*
