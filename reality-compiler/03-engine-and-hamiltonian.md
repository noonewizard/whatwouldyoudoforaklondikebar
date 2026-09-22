# Part III — The Transition Engine and Landscape Engineering

Sections 7–8: Physical State Transition Engine · Hamiltonian / Energy-Landscape
Compilation

---

## 7. The Physical State Transition Engine (PSTE)

### 7.1 What the engine can and cannot be asked

The proposed API

```
TRANSITION(R_A, S, Ω) → (P*, Π_pred)
```

is the right shape with the wrong types. Corrected:

```
TRANSITION( 𝔟₀ : Belief[Σ],
            S  : Spec,
            Ω  : Substrate )
  → Result {
      status   : FEASIBLE | INFEASIBLE(certificate) | UNKNOWN(reason, budget_exhausted)
      plans    : List[ Plan ]           # ranked, diverse, not a single optimum
      forecast : ForecastBundle         # per plan
      evidence : ModelWitnesses         # which models, which validity domains
      gaps     : List[ KnowledgeGap ]   # what must be measured to decide
    }
```

Three changes matter.

**(a) Three-valued status, with certificates.** `INFEASIBLE` must carry a reason
that is checkable — a conservation violation, a thermodynamic bound, a
controllability obstruction, or a metrological impossibility. `UNKNOWN` must be a
first-class outcome; an engine that cannot say "I do not know" will say "feasible"
about things that are not.

**(b) Plans plural.** Per §6.5.

**(c) `gaps` as an output.** The most valuable thing the engine produces on a hard
problem is not a plan; it is a ranked list of the measurements that would most
reduce the uncertainty blocking a decision. This makes the engine useful *before*
it is capable, which matters enormously for the roadmap.

### 7.2 The engine's internal problem: a belief-space optimal control problem

Formally the PSTE solves a **partially observed, model-uncertain, chance-constrained
optimal control problem with discrete-continuous structure**. This is a POMDP over a
hybrid system with a distribution over dynamics. Each of those four adjectives
independently makes the problem hard:

| Feature | Consequence | Mitigation in practice |
|---|---|---|
| Partial observation | Optimal policy is over beliefs, not states; belief space is infinite-dimensional | Certainty-equivalent + robustification; moment-matched beliefs; scenario trees |
| Model uncertainty | Single-model optima are badly non-robust | Distributionally robust / minimax regret; active discrimination (§6.9) |
| Chance constraints on rare events | Naive Monte Carlo useless at δ≤10⁻⁴ | Importance sampling; extreme-value tail models; failure-boundary surrogates |
| Hybrid discrete/continuous | Mixed-integer optimal control; combinatorial explosion | Two-level decomposition (§6.5/§6.6): discrete route search then continuous parameter synthesis |

**The two-level decomposition is not merely convenient, it is the only thing that
makes the engine tractable, and it costs global optimality.** That cost should be
stated openly rather than papered over: the engine returns good plans, not optimal
ones, and §16.4 argues that optimality is the wrong goal anyway.

### 7.3 The eleven required outputs, audited

The proposal lists eleven determinations. Each is assessed for whether it is
well-posed and computable.

| # | Requested output | Well-posed? | Honest status |
|---|---|---|---|
| 1 | Feasibility | Only in bounded form (RC-0′) | Decidable when bounded; undecidable in general (§16.2) |
| 2 | Reachability | Yes, as a belief-space set | Computable exactly for linear/bilinear; over-approximable for nonlinear (reachability analysis: zonotopes, Taylor models); undecidable for general hybrid |
| 3 | Controllability | Yes | **ESTABLISHED and exactly checkable for bilinear systems** via the Lie algebra rank condition (Jurdjevic–Sussmann; Ramakrishna et al. for quantum). This is the one item on the list with a clean, classical, complete answer. Note: LARC gives controllability, *not* time-optimality or robustness. |
| 4 | Resource requirements | Yes | Computable given the plan; linear accounting |
| 5 | Transition pathway | Yes | The plan itself |
| 6 | Expected energy | Yes | Computable, and dominated by process inefficiency, not by thermodynamic minima (§15) |
| 7 | Entropy production | Yes | Computable as Ẇ_diss/T for the modeled dissipation; **note it is not independent of (6)** — see §7.4 |
| 8 | Defect probability | **Ill-posed as stated** | Needs a defect *definition* (a criterion on Φ), a population, and a statistic. Then it is a rare-event estimation problem. |
| 9 | Execution time | Yes | Computable; stochastic, so report a distribution |
| 10 | Uncertainty | Yes, and it is the most important output | Requires propagating both aleatoric (process noise) and epistemic (model) uncertainty, and these must be reported separately — collapsing them is a standard and serious error |
| 11 | Verification requirements | Yes | §11; and this is the output the framework uniquely contributes |

### 7.4 The objective functional, corrected

The proposed functional

```
J[P] = ∫₀ᵀ [ α E(t) + β Ṡ_irr(t) + γ D(t) + δ C(t) + ε T(t) ] dt
```

has five defects.

1. **Dimensional inconsistency.** `∫ E(t) dt` has units of energy·time, not energy.
   If E is meant as a power, say so; if as an energy, do not integrate it.
   Likewise `∫ T(t) dt` where T is "execution time" integrates time over time.
2. **Double counting.** For an isothermal process at bath temperature T₀, the
   dissipated work and the irreversible entropy production are the same quantity:
   W_diss = T₀ ΔS_irr. Carrying both αE and βṠ_irr with independent weights means
   the same physical cost appears twice with an arbitrary relative weight.
3. **`D(t)` is not a rate.** Defect probability is a terminal quantity. If defect
   *formation* is meant, it is a hazard rate and the correct terminal quantity is a
   survival probability, not a time integral.
4. **No terminal cost.** The whole point is the terminal state's membership in 𝒦;
   there must be a terminal term (or a chance constraint, which is better).
5. **No verification cost, no risk aversion, no model uncertainty.**

**Corrected formulation.** Separate *hard constraints* from *costs*, put the
specification in the constraints where it belongs, and price risk explicitly:

```
minimize over policies π:

  𝔍[π] = sup_{m ∈ 𝔐}  {  𝔼_m[ C_energy + C_material + C_time + C_tooling ]
                        + λ_V · 𝔼_m[ C_verify(Ψ) ]
                        + λ_R · CVaR_α,m[ C_total ]           }

subject to:
  Pr_m[ Φ(σ(T)) ∈ 𝒦 ] ≥ 1 − δ          ∀ m ∈ 𝔐        (specification)
  Pr_m[ σ(t) ∈ Safe  ∀t ]  ≥ 1 − δ_s   ∀ m ∈ 𝔐        (safety)
  resource draws ≤ 𝒞                                    (inventory, linear)

where
  C_energy   = ∫₀ᵀ  p_E · Ẇ_in(t) dt                    (Ẇ_in = input power; price p_E)
  C_material = Σ_s  p_s · n_s                            (species consumed)
  C_time     = p_T · T           (+ option value of the machine's time)
  C_verify   = Σ_j  cost(ψ_j) · n_j                      (from Ψ; see §11)
  C_total    = the realized total including rework/scrap on FAIL
```

Notes on the changes, each with a reason:

- **The specification is a constraint, not a cost.** You do not trade off "how much
  the part meets spec" against energy. You meet spec at confidence 1−δ or you do
  not ship. This reflects actual industrial practice and removes the arbitrary
  weight γ.
- **Entropy production is not a separate term.** It enters through C_energy, because
  dissipated work is what you pay for. If a separate environmental-burden term is
  wanted (waste heat load, coolant), add it as an explicit *resource* with a price,
  not as an abstract Ṡ_irr.
- **CVaR (conditional value at risk) rather than expectation.** Manufacturing
  economics is dominated by tail events (a scrapped lot, a contaminated reactor, a
  field failure). Minimizing the mean is the wrong objective and is why
  simulation-optimal processes often lose to conservative empirical ones. Risk
  measures are ESTABLISHED tooling; their absence from inverse-design objectives is
  a real gap.
- **Verification cost is in the objective.** This is the framework's distinctive
  move. With λ_V > 0 the compiler will prefer, among routes that both meet spec, the
  one whose acceptance predicate is cheaper to certify — e.g. a route that produces a
  distinctive, easily measured XRD signature over one whose product requires
  destructive sectioning to distinguish from a near-isostructural impostor.
  **PLAUSIBLE, and testable:** see Falsification Experiment F-4 (§25).
- **sup over 𝔐.** Robustness to model misspecification, per §6.6.

### 7.5 Well-posedness of the corrected functional

Is this a meaningful optimization problem? Conditions under which it is:

- **Existence of a minimizer** requires compactness of the admissible control set
  (bounded amplitudes, bounded bandwidth — physically true) and lower
  semicontinuity of 𝔍 (holds for the integral terms; the chance constraint's
  feasible set must be closed, which requires the distribution to have no atoms on
  ∂𝒦 — generically true, and the compiler should check it).
- **The sup over 𝔐 is attained** if 𝔐 is compact and m ↦ 𝔼_m[·] is upper
  semicontinuous. If 𝔐 is a finite model set (the practical case) this is automatic.
- **Non-convexity is generic.** There is no reason to expect convexity, and the
  quantum-control landscape literature's "trap-free" results hold under
  assumptions (surjectivity of the end-point map, unconstrained controls) that
  constrained real systems violate. Claims that quantum control landscapes are
  generically trap-free should be treated as **PLAUSIBLE but contested**, not
  established; the counterexample literature under control constraints is
  substantial.

**Verdict on §VII of the proposal: the architecture is sound; the objective as
written is not, and the corrected version above is materially different in
behavior, not merely in notation.**

---

## 8. Hamiltonian and Energy-Landscape Compilation

### 8.1 The proposition

> Instead of explicitly computing every microscopic transition, can the compiler
> engineer the physical energy landscape so that the desired state becomes
> dynamically favored?

This is the most scientifically interesting section of the proposal and it deserves
a precise answer rather than an enthusiastic one.

**Short answer: yes, this is a real and in places dominant engineering strategy; and
no, it does not generally provide a computational speedup. The two claims are
independent and conflating them is the central error to avoid.**

### 8.2 The distinction that must be kept

Two different things are being proposed under one heading:

**(I) Landscape engineering as a *control strategy*.** Rather than steering each
degree of freedom, design the potential/Hamiltonian so that the system relaxes to
the target. This saves *control authority and bandwidth* — you do not need 10²³
actuators — and it is how essentially all of chemistry, crystallization, and
self-assembly works. **ESTABLISHED and enormously important.** Crystallization from
solution is landscape engineering: you do not place the molecules, you set the
supersaturation and the temperature profile so that they place themselves.

**(II) Landscape engineering as a *computational* strategy.** Let the physical
system's relaxation "solve" an optimization problem that would be expensive to solve
on a digital computer. **This is where the claims go wrong.**

### 8.3 When does physical relaxation give genuine computational leverage?

The honest taxonomy:

**Case A — Simulating quantum dynamics. GENUINE ADVANTAGE, STRONGLY SUPPORTED.**
A controllable quantum system evolving under a programmable Hamiltonian reproduces
dynamics that are believed to require exponential classical resources. This is the
original Feynman argument and it is the best-supported instance of physical
computational leverage. Analog quantum simulators (neutral-atom arrays, trapped
ions, superconducting processors) have reached regimes where classical simulation is
at least severely strained. The caveat is significant: *verification* of the
simulator's output in exactly the regime where classical simulation fails is itself
an open problem, and several claimed advantages have been substantially eroded by
improved classical algorithms (tensor-network and sparse-sampling attacks on
supremacy claims). Call it: **genuine, narrower than advertised, and not yet
harnessed for synthesis.**

**Case B — Combinatorial optimization by annealing. LARGELY CONTRADICTED in the
strong form.** The hope that a physical spin system relaxes to the ground state of a
hard Ising instance faster than a classical algorithm finds it has not survived
careful benchmarking. Glassy landscapes have exponentially many metastable minima
and relaxation times that grow at least as fast as the algorithmic runtimes they
compete with; no consistent scaling advantage for quantum annealing on generic hard
instances has been demonstrated under fair comparison, and several early claims were
withdrawn or reinterpreted after classical baselines were improved. Specific
structured problem families may retain advantages; generic ones do not. **The
framework should not rest any load on this.**

**Case C — Physical systems solving *their own* dynamics at unit cost. TRUE BUT
OFTEN MISREAD.** It is trivially true that a beaker of reagents "computes" its own
trajectory for free, in the sense that you get the answer without simulating. The
leverage is real but bounded by three things: you only get *one sample* per
experiment; you only learn the observables you measure; and you cannot ask
counterfactuals. Formally, the beaker is an oracle with expensive queries, high
latency, and stochastic answers. That is genuinely useful — it is what an experiment
*is* — but it is not a general-purpose accelerator, and the framework's economics
must account for query cost, not treat physical evaluation as free.

**Case D — Analog computation with continuous variables. CONTRADICTED as a source of
super-Turing power; PLAUSIBLE as a source of constant-factor and
energy-efficiency advantage.** Claims that analog physical systems exceed Turing
computability rely on infinite-precision idealizations; under any realistic noise
model, precision is finite and the advantage collapses to constant factors. Those
constant factors can still be large (analog photonic and in-memory matrix
multiplication), and energy-per-operation advantages of 10²–10³× are credible for
specific kernels. This is an engineering win, not a complexity-theoretic one.

**Case E — Self-assembly as computation. ESTABLISHED, and the framework's best
case.** Algorithmic self-assembly is Turing-universal (§18) and performs, in
parallel and at molecular scale, a computation whose *output is the structure
itself*. Here the physical computation and the fabrication are the same event, which
is precisely the leverage the framework is looking for. It is also the case with the
strongest experimental record. **This, not quantum annealing, is where the
landscape-engineering thesis should be anchored.**

### 8.4 The engineering repertoire, assessed

| Technique | What it engineers | Maturity | Honest assessment |
|---|---|---|---|
| **Adiabatic state preparation** | Ground state of a target H via slow interpolation | Theory mature; hardware limited | Runtime scales as ~1/Δ² in the minimum gap; gaps are exponentially small for hard instances. Rigorous and ESTABLISHED as a *universal model* of quantum computation (equivalent to the circuit model up to polynomial overhead) — but universality ≠ speedup. |
| **Quantum annealing (open system)** | Same, with a thermal bath | Deployed hardware | See Case B. Useful as a heuristic sampler; no established generic speedup. |
| **Optimal coherent control (GRAPE/Krotov/CRAB)** | Time-dependent H to steer a specific transition | Mature, widely used | **ESTABLISHED and genuinely valuable.** This is the workhorse: pulse shaping in NMR, quantum gates, cold-atom transport, laser control of chemistry. Its limitation is that it needs an accurate H. |
| **Dissipative / reservoir engineering** | Designed Lindblad operators whose *steady state* is the target | Demonstrated in ions, superconducting circuits, atoms | **STRONGLY SUPPORTED and strategically important.** Dissipative preparation is *autonomous and self-correcting*: the target is an attractor, so errors are pumped out without measurement or feedback. Verstraete–Wolf–Cirac showed dissipative dynamics is a universal resource for quantum computation and state engineering. For the framework, this is the cleanest formal realization of "engineer the dynamics so the answer is the attractor." |
| **Shortcuts to adiabaticity (counterdiabatic driving)** | Auxiliary control fields that suppress excitations, enabling fast transfer | Demonstrated in several platforms | Real, useful, and *not* a free lunch: the exact counterdiabatic term is typically nonlocal and as hard to implement as solving the problem. Approximate/local versions work over limited ranges. **PLAUSIBLE with strong caveats.** |
| **Reaction–diffusion / Turing patterning** | Spatial pattern from local kinetics | Theory 1952; chemical demonstrations (CIMA/BZ); biological relevance contested | Pattern *classes* are controllable; specific patterns with defect control are not. **PLAUSIBLE at the level of stripe/spot selection; SPECULATIVE for arbitrary targets.** |
| **Directed self-assembly (block copolymers on chemical/topographic guides)** | Free-energy landscape shaped by a pre-pattern | Industrially evaluated for lithography | The most industrially advanced landscape-engineering technology outside chemistry. Density multiplication demonstrated; defectivity (targets of <1 defect/cm²) has been the persistent blocker. **STRONGLY SUPPORTED as a technique; defectivity is the open issue** — and it is an instance of the general lesson that landscape engineering is limited by *kinetic trapping*, not by thermodynamics. |
| **Catalysis** | Selective lowering of specific barriers | Foundational industrial technology | The single most economically important landscape-engineering technology in existence. Note what it demonstrates: **selective kinetic control, not thermodynamic control.** Reinforces §3.2. |
| **Molecular machines / ratchets** | Directed motion from designed asymmetry + energy input | Nobel-recognized; laboratory scale | Real, elegant, and far from manufacturing-relevant throughput. |
| **Metastable-state trapping (quenching, tempering)** | Deliberately *avoiding* equilibrium | The basis of steel, glass, and most polymers | Worth naming explicitly because it is the counterexample to landscape-as-thermodynamics thinking: most engineering materials are kinetically trapped and would be destroyed by equilibration. |

### 8.5 The conditions under which landscape engineering pays — stated precisely

Collecting the above into a usable criterion. Landscape engineering provides genuine
leverage over explicit trajectory computation when **all** of the following hold:

1. **Degrees of freedom vastly exceed available actuators.** (N_dof / N_actuators ≫ 1.)
   Otherwise, direct control is simpler and more reliable.
2. **The target is, or can be made, a *robust attractor*** of the engineered
   dynamics — a ground state with a finite gap, a deep free-energy minimum with a
   large basin, or a steady state of an engineered dissipative map. Robustness, not
   mere optimality, is the requirement.
3. **The relaxation time to that attractor is sub-exponential in system size at the
   operating conditions.** This is the condition that fails for glasses, for
   spin-glass optimization, for protein misfolding, and for most "just let it find
   the ground state" proposals. **It must be checked, not assumed, and checking it
   is hard.**
4. **Competing attractors are either absent or separated by barriers exceeding the
   thermal scale over the service lifetime.** (Otherwise you get polymorph
   conversion, recrystallization, aging.)
5. **The landscape parameters are within the substrate's control envelope** — you
   can only engineer the Hamiltonian terms you can physically apply.

When (1)–(5) hold, landscape engineering is not merely competitive; it is the only
viable strategy, because explicit trajectory control of 10²³ degrees of freedom is
not available at any price. When (3) fails, landscape engineering fails, and no
amount of compute fixes it.

**This five-condition test is, in the author's judgment, the most directly useful
technical output of the proposal's §VIII, and it is stated nowhere in the source
material.**

### 8.6 The compiler's role, restated

Given the above, the compiler's job in landscape mode is:

```
Given 𝒦 and the controllable Hamiltonian family { H(θ) : θ ∈ Θ_Ω },
find θ* such that:
   (a) argmin / steady-state of H(θ*) has Φ ∈ 𝒦          [target is an attractor]
   (b) gap / basin depth of that attractor ≥ margin       [robustness]
   (c) relaxation time τ(θ*) ≤ T_max                      [kinetic feasibility]
   (d) no competing attractor with basin volume > ε       [selectivity]
   (e) θ* ∈ Θ_Ω                                           [realizability]
```

This is an *inverse Hamiltonian design* problem. Its complexity is discussed in
§16.5; briefly, even verifying (a) is QMA-hard in the quantum case and NP-hard in
the classical Ising case, and verifying (b) in the thermodynamic limit is
undecidable. **But** — and this is the practical escape hatch that matters —
conditions (a)–(d) need only be verified *for the actual finite system at the actual
operating conditions*, where they become estimation problems that can be attacked
with sampling, and ultimately with experiments. The undecidability and QMA-hardness
results bound what can be *proven in general*, not what can be *achieved in
instances*. That distinction is what separates a usable theory from a paralyzed one,
and §16 develops it.
