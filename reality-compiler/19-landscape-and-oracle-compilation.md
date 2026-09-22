# Landscape Compilation, Physical Universality, and the Oracle Compiler

Section 19. The escalation: how far the compilation paradigm goes when engineering
limits are removed and only physics and mathematics are allowed to say no.

---

## 19.1 The attractiveness functional

Drafts 2–4 gave six axes 𝔄 = (Δ, B, τ, κ, Γ, D) and no scalar to optimize. That was
a gap: `argmax_λ Attractiveness` needs an objective. Here is one.

> **DEFINITION 19.1 (Attractiveness).** For a controlled stochastic dynamics with
> generator ℒ_λ, an initial belief 𝔟₀, an acceptance region 𝒦, and an operating window
> T_op, define
> ```
>                    1    ⌠T_op
>   A(𝒦; λ, T_op) = ───   ⎮      Pr[ σ(t) ∈ 𝒦 | 𝔟₀, λ ] dt
>                   T_op  ⌡0
> ```
> — the **expected occupation fraction** of the acceptance region over the operating
> window. Dimensionless, valued in [0,1], estimable by sampling trajectories or by
> direct measurement on the physical system.
>
> The landscape-compilation problem is
> ```
>   λ* = argmax_{λ ∈ Λ_Ω}  A(𝒦; λ, T_op)   subject to resource and safety constraints.
> ```

**Why this is the right scalar.** It is the unique functional that handles all ten
objects of §8.4.1 without special cases:

| Object | Behaviour of A |
|---|---|
| Attracting set containing 𝒦 | A → 1 as T_op grows |
| Metastable set | A ≈ 1 for T_op ≪ escape time; decays as T_op → ∞ |
| Stationary distribution | A → π(𝒦), the stationary mass |
| Lindbladian steady state | A → Tr(P_𝒦 ρ_ss) |
| Limit cycle intersecting 𝒦 | A → duty fraction |
| Unreachable set | A = 0 |
| Reachable but unstable | A small and decreasing in T_op |

> **PROPOSITION 19.2 (The six axes are diagnostics of one objective).** Δ, B, τ, κ, Γ
> and D are not independent design targets; they are the leading terms in the
> sensitivity of A. Specifically τ controls A's rise time, Γ controls its decay, B and
> κ control ∂A/∂λ, D controls the defect-induced shortfall of the reachable A from 1,
> and Δ enters through both τ and Γ. **The engineering content of the six axes is
> preserved; the objective is A.**
> **Classification: PROPOSITION (proposed here).** *This corrects Draft 2's "basin
> volume" instinct and Draft 4's unordered six-tuple in one move.*

> **COROLLARY 19.3 (Metastability is not a degenerate case).** Because A is defined on
> a finite operating window, a metastable target with escape time ≫ T_op scores as
> highly as a globally attracting one. **The objective automatically treats diamond,
> steel, and Floquet phases as first-class**, which the earlier gap- and
> basin-based formulations did not.

> **COROLLARY 19.4 (The chance constraint is a special case).** The specification
> constraint Pr[Φ(σ_T) ∉ 𝒦] ≤ δ is the terminal-time form of A ≥ 1 − δ with
> T_op → {T}. Landscape compilation and the risk-constrained formulation of §7.5 are
> the same problem read at different operating windows.

---

## 19.2 The sharp dividing line: when is the landscape route available at all?

This is the central new result of this section.

> **PROPOSITION 19.5 (Landscape-reachability bound).** Let a landscape family be
> parameterized by p continuous controls, each realizable to b effective bits
> (log₂ of the number of distinguishable settings, limited by actuator noise). Let a
> family of targets {S₁,…,S_M} have pairwise disjoint acceptance regions, all to be
> reached on the same substrate by landscape engineering alone. Then
> ```
>   I(S) := log₂ M  ≤  p · b .
> ```
> *Proof.* By Proposition 16.1, distinct disjoint outcomes require distinct control
> programs. Under landscape-only control, the program *is* the parameter vector λ, of
> which at most 2^{p·b} are distinguishable. An injection from M targets into that set
> requires M ≤ 2^{p·b}. ∎
>
> **Classification: PROPOSITION, proved.**

> **COROLLARY 19.6 (The control/landscape dividing line).** A target family is
> reachable by landscape engineering **iff its specification information fits the
> landscape's parameter budget.** When I(S) > p·b, no landscape parameterization
> whatsoever suffices and explicit per-degree-of-freedom addressing is *required* —
> not merely preferable.

This is the precise content of the directive's intuition that "less control plus
better landscape engineering may beat more control." It says exactly when:

```
   I(S) ≤ p·b      →  landscape route exists; cost ~ p·b, independent of N_dof
   I(S) > p·b      →  landscape route does not exist; cost ~ I(S) addressing operations
```

**Worked instances.**

| Target | I(S) | p·b available | Verdict |
|---|---|---|---|
| A crystal of 10²⁵ atoms on a lattice | ~10 bits (space group + lattice constants + composition) | ~10 knobs × 12 bits = 120 | **Landscape.** The huge product is generated, not specified |
| A specified quantum phase from a 10-parameter Hamiltonian family | ~10–30 bits | ~120 | **Landscape** |
| An N×N algorithmic tile pattern | Θ(log N/log log N) tile types × bits each | depends on sequence budget | **Landscape** (this is the tile-complexity theorem in the same units) |
| A bespoke CAD part with 10⁶ independent toleranced features | ~10⁷ bits | ~10² | **Explicit addressing required.** No landscape exists, and none will |
| A specific protein sequence of 300 residues | 300 × log₂20 ≈ 1300 bits | ~10²  (culture conditions) | **Explicit addressing required** — which is why DNA synthesis, not fermentation conditions, specifies sequence |

> That last row is the proposition earning its keep: it *derives*, from a counting
> argument, the observed fact that sequence is specified by synthesis and structure is
> specified by conditions. The dividing line is not a matter of taste.

---

## 19.3 The control-versus-landscape optimization

> **DEFINITION 19.7 (Total realization cost).**
> ```
>   C_total(route) = C_control + C_verification + C_stabilization
> ```
> with
> - **C_control** ~ (number of addressed degrees of freedom) × (operations each) ×
>   (unit cost), **plus** the amortized one-time cost of solving the inverse problem
>   divided by production volume;
> - **C_verification** from the measurement plan (§9.8);
> - **C_stabilization** = the cost of maintaining A ≥ 1−δ over the service life —
>   active correction, environmental control, or nothing if the target is a robust
>   attractor.
>
> The compiler minimizes C_total subject to Pr[Φ(σ_T) ∉ 𝒦] ≤ δ.

> **PROPOSITION 19.8 (Crossover).** Let N be the number of degrees of freedom to be
> placed explicitly and V the production volume. Landscape compilation is preferred
> when
> ```
>   C_design_once / V  +  p·b·c_λ  +  C_stab  <  N·c_addr  +  C_stab'
> ```
> Since the left side is independent of N and the right grows linearly in it, **there
> is always a crossover N\*(V), and it moves to smaller N as volume grows.**
> Landscape engineering is therefore a *volume-dependent* strategy — which the
> relaxation-time criterion of Draft 2 could not express, and which is why the same
> physics favours CNC for one part and casting for a million.
> **Classification: PROPOSITION (an accounting identity, not a physical law).**

**The third term is where most of the argument actually lives.** A robust attractor
has C_stab ≈ 0: the target maintains itself. An explicitly placed configuration in a
noisy environment has C_stab growing with service life. **This is the strongest
argument for attractor compilation and it is not about computation at all** — it is
that attractors are *free to maintain* and placed configurations are not.

---

## 19.4 Physical universality: is there a synthesis algebra?

> **DEFINITION 19.9 (Synthesis algebra).** For a substrate Ω, let 𝔖 be a set of
> primitive operations (transport, deposition, removal, bonding, dissociation,
> crystallization, phase transition, field application, pressure and temperature
> modulation, confinement, irradiation, excitation, cooling, annealing, measurement,
> feedback, dissipation engineering, interaction engineering) closed under sequential
> and parallel composition. Write Reach(𝔖) for the induced reachability relation and
> Perm(Ω) for the relation permitted by conservation alone.
>
> **𝔖 is universal over Ω** iff Reach(𝔖) = Perm(Ω).

**Results, honestly separated:**

> **KNOWN RESULT 19.10 (Quantum spin sector: universality holds).** Certain simple
> spin-lattice models are universal simulators, reproducing the spectrum, partition
> function, and dynamics of any other spin Hamiltonian with polynomial overhead
> (Cubitt–Montanaro–Piddock). **For spin Hamiltonians, a finite primitive set is
> universal.** This is the only proved physical-universality result in the framework.

> **PROPOSITION 19.11 (Classical matter: universality fails, and the obstruction is
> kinetic, not algebraic).** For classical substrates, Reach(𝔖) ⊊ Perm(Ω). The gap is
> not because the primitive set is too small — transport, bonding and dissociation
> suffice *combinatorially* to reach any conservation-permitted configuration on a
> lattice model — but because the rates are not there: composition is limited by
> barrier heights, not by expressiveness.
> **Classification: PROPOSITION.** *Consequence, and it is a clean reframing: the
> search for a universal classical synthesis algebra is misdirected. The algebra is
> already universal; the* **kinetics** *are not. Effort belongs in catalysis and route
> discovery, not in enlarging the primitive set.*

> **OPEN PROBLEM 19.12.** Is there a *rate-annotated* synthesis algebra — primitives
> with composition rules that carry rate bounds — for which a poly-time reachability
> theory exists? Chemical reaction networks give the decidability picture
> (Ackermann-complete, §17.17) but no rate theory. This is the missing mathematics
> between the algebra and practice.

---

## 19.5 Compiling the environment

> **DEFINITION 19.13.** The compilation target is the **environment–object dynamical
> system** (𝒪, ℰ), with ℰ = (temperature, pressure, electromagnetic and optical
> fields, acoustic fields, chemical atmosphere, vacuum, confinement, interfaces,
> reservoirs, engineered dissipation, radiation, gravity where relevant) treated as a
> designed component with its own resource accounting: reservoirs have finite
> capacity, fields cost power, and heat must be rejected through a finite interface.

Three consequences that are not cosmetic:

**(1) Environment compilation is the prerequisite for gain, hence for composability.**
§17.9 established that gain requires an energy source and an open system. A passive
substrate cannot restore signals and therefore cannot compose indefinitely. **The
environment is where the energy source lives.** Compiling ℰ is therefore not an
optional extension of the framework; it is how a substrate acquires the digital
abstraction's missing property.

**(2) It changes the conservation accounting.** Closed-system conservation checks are
necessary but no longer sufficient: the reservoir's capacity, and the entropy export
rate through the interface, become constraints with certificates of their own.

**(3) It supplies the effective-parameter escape.** When a specification demands an
extreme parameter, the compiler should first search for an *effective* realization:
strain-induced pseudo-magnetic fields exceeding 300 T on a benchtop; synthetic gauge
fields by Floquet driving; proximity-induced superconductivity. **This is a rewriting
rule over Hamiltonian terms** — maintain a library mapping (required term → alternative
physical realizations) and search it before demanding the literal regime. Directly
implementable, and absent from the prototype.

---

## 19.6 The oracle compiler: physical-in-the-loop changes the complexity class

This is the most consequential escalation in this section, and it invalidates a
framing I have used since Draft 1.

**The observation.** Every hardness result cited in this monograph — QMA-completeness
of LOCAL HAMILTONIAN, P^QMA[log]-completeness of APX-SIM, QCMA/PSPACE-completeness of
GSCON, PSPACE-hardness of bounded planning — bounds an algorithm that must *decide or
predict* the outcome. They bound a **simulating compiler**.

A physical-in-the-loop compiler does not predict. It **queries**.

> **DEFINITION 19.14 (Oracle compiler).** A compiler with access to a physical oracle
> 𝒪_Ω: given λ, 𝒪_Ω returns a sample from the outcome distribution of running the
> substrate at λ, at a cost of one experiment. Its complexity measure is **query
> complexity**, not time complexity.

> **PROPOSITION 19.15 (The hardness results bound the wrong compiler).** QMA-hardness
> of ground-state problems bounds the resources a classical (or quantum) machine needs
> to *compute* the answer. It places **no lower bound** on the number of physical
> queries an oracle compiler needs to *find* a satisfying λ, because the substrate
> evaluates its own ground state at unit cost per query.
>
> The oracle compiler's complexity is governed instead by the **sample complexity of
> landscape identification**: how many experiments suffice to locate a λ with
> A(𝒦; λ) ≥ 1−δ. For a p-parameter landscape with Lipschitz-continuous A and
> resolution ε, naive grid search needs O((1/ε)^p) queries; Bayesian optimization with
> a well-specified prior needs far fewer in practice; and no QMA-type lower bound
> applies to either.
> **Classification: PROPOSITION.** *The observation that physical evaluation sidesteps
> simulation hardness is not new — it is what experiments are. What is proposed here
> is the architectural consequence: the compiler's complexity analysis must be stated
> in query complexity, and Drafts 1–4 stated it in the wrong measure throughout.*

> **COROLLARY 19.16 (What actually bounds the oracle compiler).** Four things, none of
> them QMA:
> 1. **Query cost and latency** — an experiment costs hours and dollars, not
>    microseconds. This is the binding resource.
> 2. **Query complexity in p** — exponential in the number of landscape parameters
>    under adversarial landscapes; benign for smooth, low-dimensional Λ_Ω. Proposition
>    19.5 says p is *small* exactly when the landscape route exists, so the two
>    results reinforce.
> 3. **Verification cost per query** — each query's answer must itself be certified
>    (§9.8), and this often dominates the query.
> 4. **Safety** — an oracle compiler executes physical actions during search, so the
>    safety envelope constrains exploration in a way simulation never does.
>
> **None of these is a complexity-theoretic obstruction. All are engineering and
> economics.** This is the strongest single argument in the entire monograph that the
> compilation paradigm can be pushed much further than the hardness results suggest.

> **COROLLARY 19.17 (Compilation and computation become the same process).** When the
> substrate serves as its own simulator, the distinction between "compiling the
> artifact" and "computing how to make it" collapses: each query is simultaneously a
> computation step and a fabrication step. Self-assembly is the limiting case, where
> the computation *is* the product (§8.8.2, case F). **This is the defensible version
> of "physics computes for free": not that information is created, but that the
> evaluation and the fabrication are one event.**

**Honest limits on the oracle framing.** The oracle is expensive, high-latency,
stochastic, gives one sample per query, answers only what was measured, and admits no
counterfactuals. Query complexity that is polynomial but with a 10⁴-query constant is
a decade of laboratory work. The escape from QMA is real and it is not free.

---

## 19.7 Self-attack

Applying the eight questions to this section's own claims.

**Against Definition 19.1 (attractiveness as occupation fraction).**
*What would falsify it?* A target where A is high and the article is nevertheless
useless — e.g. a system that spends 99% of its time in 𝒦 while cycling rapidly in and
out, destroying the artifact each time. **The objection lands:** A is insensitive to
the *pattern* of occupancy. Fix: for targets where residence continuity matters,
constrain the exit rate Γ separately rather than folding it into A.
**Status: A is correct for terminal-acceptance targets and needs an auxiliary
continuity constraint for service-life targets.**

**Against Proposition 19.5 (landscape-reachability bound).**
*What theorem could contradict it?* None — it is a counting argument. *But is it
tight?* No, and the gap matters: it bounds *distinguishable* targets, not achievable
*precision*. A landscape with p·b = 120 bits could address 2¹²⁰ disjoint regions only
if the map λ ↦ outcome were injective and the regions were resolvable. In practice
the map is many-to-one and noisy, so the achievable I(S) is **far below** the bound.
**The proposition gives a hard ceiling and a weak floor**, and the floor is what an
engineer needs. Stated as a limitation rather than repaired.

**Against Proposition 19.15 (oracle compiler escapes QMA).**
*What could rule it out?* The strongest objection: **verification is still hard.**
Escaping the simulation of the ground state does not escape the need to certify that
the state you got is the one you wanted, and for generic targets that is
information-theoretically hard (CE-1) regardless of how it was prepared. So the
oracle compiler escapes category-15 (computational) obstructions and **inherits
category-5/8 (information/observability) ones unchanged.** The escape is real and
partial. *This is the correct statement and it narrows the corollary materially.*

*Second objection:* an oracle compiler can only search within Λ_Ω, and building the
apparatus that realizes a wide Λ_Ω is itself the hard fabrication problem. The oracle
framing moves the difficulty from computation to apparatus, it does not remove it.
**Also lands.**

**Against Proposition 19.11 (classical universality fails kinetically, not
algebraically).** *What would falsify it?* Exhibiting a conservation-permitted
transformation unreachable by the primitive set for a *combinatorial* rather than
kinetic reason. Plausible candidates exist — topological obstructions in knotted
polymer or entanglement-of-chains settings, where reachability may fail for reasons of
configuration-space connectivity rather than rate. **Unresolved; the proposition should
be read as covering compositional and positional targets, not topological ones.**

**Against §19.5 (environment compilation supplies gain).** *What thermodynamic
argument could rule it out?* None — gain from an open driven system is routine. But
the *cost* is bounded below by the entropy export required, and for a substrate with
N gain stages the heat rejection scales with N. **The claim survives; the scaling is
the constraint.**

---

## 19.8 The ultimate question

> **Is "physical reality as a compilation target" a useful metaphor, or is there a
> rigorous class of physical systems for which declarative specifications translate
> into executable, verifiable physical transformations with bounded error?**

**Answer: there is a rigorous class, it is characterized, and it is substrate-relative.
The metaphor reading is wrong, and so is the universal reading.**

The class is defined by Proposition 17.5 and the results above:

> **The maximal currently defensible class.** A (substrate, specification-class,
> budget) triple (Ω, 𝒮, ε, T, C) admits sound compilation iff
>
> **(i)** every S ∈ 𝒮 is a decidable predicate on a finitely representable abstraction
> of Σ **[R]**;
> **(ii)** [S] ∩ Reach(𝔟₀, Λ_Ω, T) ≠ ∅ **[Rch]**;
> **(iii)** a measurement plan with false-accept ≤ δ against a declared Alt(S) exists
> within budget **[Ver]**;
> **(iv)** the program fits 𝒞 **[Res]**;
>
> plus **MV** for any model-based claim, **St** for lifetime claims, **Cmp** for
> multi-step, **Ref** for multi-level. Soundness is **relative to 𝔐 and never to
> reality** (Corollary 17.8), and the compiler is necessarily three-valued
> (Corollary 17.11).

**The boundary, stated four ways:**

1. **By information (Prop. 16.1, 19.5).** The compiler cannot impose more
   specification information than its control program carries. Landscape routes exist
   iff I(S) ≤ p·b; beyond that, explicit addressing is required and its cost is
   linear in I(S). **This is the sharpest statement of the boundary and it is a
   theorem.**
2. **By verifiability (CE-1, Lemma 17.2).** Structured targets are certifiable in
   polynomial samples; generic targets are not, at any technology level.
   Verifiability, not controllability, is the usual binding condition.
3. **By kinetics (§18.4).** Among physically permitted targets, the most common
   ultimate obstruction is route existence at accessible conditions — category 10/11
   — not thermodynamics and not information.
4. **By theorem (§18.3).** Eleven identified results carve genuine holes in the
   design space: causality bounds dispersion engineering and forbids broadband passive
   cloaking; AFH forbids passive 2D topological memory at finite T; Mermin–Wagner,
   LSM, Nielsen–Ninomiya, Wannier and Eastin–Knill each forbid a specific and
   otherwise attractive target class.

**What the escalation actually changed.** Three things, and they push in opposite
directions, which is the sign that the analysis is not being steered:

- **Outward.** The oracle-compiler framing (§19.6) shows the complexity-theoretic
  obstructions I have been citing since Draft 1 bound a *simulating* compiler, not an
  *experimenting* one. The paradigm reaches substantially further than the hardness
  results suggest — the ultimate ceilings (Bekenstein, Margolus–Levitin, Landauer,
  causality) sit 30+ orders of magnitude above practice, and **nothing in the ultimate
  physics of information forbids a compiler of essentially unlimited ambition.**
- **Inward.** Proposition 19.5 draws a hard, provable line: targets whose
  specification information exceeds the landscape parameter budget can **never** be
  reached by landscape engineering, at any technology level. That is a permanent
  structural limit on the framework's most attractive idea, and it explains — from
  counting alone — why sequence is specified by synthesis and structure by conditions.
- **Sideways.** The dominant ultimate obstruction turns out to be **kinetic route
  existence and non-destructive verification**, not energy, not information, not
  computation. The research program should be weighted accordingly, and the previous
  drafts' emphasis on thermodynamic and complexity-theoretic limits was
  mis-proportioned.

**With explicit uncertainty.** The characterization of the class rests on:
Proposition 17.5 (proved, elementary); Corollary 17.8 (proved, and it is the honest
ceiling on what any such framework can claim); Propositions 16.1 and 19.5 (proved by
counting); the area-law dependence for quantum matter (**conjectural in d ≥ 2, with a
second gap at efficient contractibility**); and the frontier tabulation of §18.4 (a
judgment over 28 cases, not a survey). **Three of 28 tabulated capabilities are
physically forbidden; three are genuinely unknown; twenty-two are permitted and
blocked by categories 7–17.** That ratio is the quantitative answer to the ultimate
question, and it is the number to attack.
