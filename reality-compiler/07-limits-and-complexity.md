# Part VII — Limits

Sections 15–17: Thermodynamic Limits · Complexity Theory · Universality

---

## 15. Thermodynamic Limits

### 15.1 The question

Is there a lower bound `J_min(S, R_A, Ω)` on the cost of physical compilation? Yes,
and it decomposes cleanly. The more important finding is **how far below practice it
sits**, which determines whether the bound is worth optimizing against.

### 15.2 The decomposition

For an isothermal process at bath temperature T₀, transforming a feedstock ensemble
into an article satisfying S:

```
W_min  =  ΔF_state        (reversible free-energy difference; a state function; may be < 0)
        + W_ex(τ)         (finite-time excess work; → 0 as τ → ∞)
        + T₀ k_B ln2 · N_erase   (Landauer cost of logically irreversible control operations)
        + W_meas          (measurement and feedback cost, ≥ 0, bounded by information gained)
```

**Term 1 — ΔF_state.** **ESTABLISHED.** ΔF = ΔU − T₀ΔS. For an ordering
transformation (feedstock → specified structure), ΔS < 0 and the term is positive.
A useful closed form for the *configurational* part: if the specification pins N
constituents each to one of M distinguishable local states,

```
ΔS_config = − N k_B ln M,     W_config ≥ N k_B T₀ ln M.
```

This is the honest, defensible version of "information has an energy cost in
matter," and it is the correct generalization of Landauer from bits to atoms.

**Term 2 — W_ex(τ), the finite-time penalty.** **ESTABLISHED** in two complementary
forms:
- *Thermodynamic-length form:* for slow driving, W_ex ≈ 𝓛²/(2τ) where 𝓛 is the
  thermodynamic length of the protocol path in the metric induced by the friction
  tensor (Sivak–Crooks). Cost scales as 1/τ — **speed is expensive, quadratically in
  distance and inversely in time.**
- *Optimal-transport form:* for overdamped Langevin dynamics, the minimum
  dissipated work to transport a distribution ρ₀ → ρ_T in time τ is
  W_diss ≥ W₂²(ρ₀,ρ_T)/(μ τ), with W₂ the Wasserstein-2 distance and μ the mobility.
  This is exact and gives the optimal protocol.

**Term 3 — Landauer.** **ESTABLISHED, and experimentally confirmed** (single-bit
erasure at the k_BT ln2 bound demonstrated in colloidal and nanomagnetic systems).
k_BT₀ ln2 = 2.87 × 10⁻²¹ J at 300 K.

**Term 4 — measurement and feedback.** **ESTABLISHED** (Sagawa–Ueda; the
generalized second law with information). Feedback can extract up to k_BT·I work,
but acquiring and erasing the information costs at least as much over a cycle.
Relevant to the framework because closed-loop control is *information-consuming*,
and this term prices in-process metrology thermodynamically.

**Additional bounds that constrain the framework in shape if not in magnitude:**

- **Quantum speed limits.** τ ≥ πħ/(2ΔE) (Mandelstam–Tamm) and τ ≥ πħ/(2⟨E−E₀⟩)
  (Margolus–Levitin) bound the time to drive a state to an orthogonal one given
  available energy. **ESTABLISHED.** Implication: *fast state transformation requires
  energy scale, not just power* — a real constraint for coherent control, irrelevant
  for bulk manufacturing.
- **Thermodynamic uncertainty relation.** For a current-like observable in a
  nonequilibrium steady state, Var(J)/⟨J⟩² ≥ 2k_B/Σ with Σ the entropy production.
  **ESTABLISHED (2015–2016).** Rearranged: achieving relative precision ε requires
  Σ ≥ 2k_B/ε². **This is the single most conceptually relevant bound for the
  framework, because it is the first-principles statement that *tolerance costs
  dissipation*.** A process cannot be simultaneously precise and quasi-reversible.
- **Classical speed limits** relating entropy production, time, and the distance
  between distributions (Shiraishi–Funo–Saito) give the classical analogue of the
  quantum speed limit and are the right tool for bounding fast classical protocols.

### 15.3 The numbers, and the conclusion that follows

Order-of-magnitude comparisons at T₀ = 300 K.

**Compilation (the computation itself).** A 1 GB compiled plan is 8×10⁹ bits; erasing
all of it costs 8×10⁹ × 2.87×10⁻²¹ J ≈ **2×10⁻¹¹ J**. Real datacenter compute for a
demanding compile: 10⁴ core-hours ≈ 10⁹ J. The gap is **twenty orders of magnitude**.
Landauer is irrelevant to the compiler: a single CMOS switching event already costs
~10⁴× the bound, and system-level energy per floating-point operation in a real
datacenter is ~10⁸× it. Neither gap will close through thermodynamic design.

**Matter (the configurational bound).** 1 kg of a solid with mean atomic mass 60
contains ≈ 10⁲⁵ atoms. Pinning each to one of ~10 local alternatives:

```
W_config ≥ 10²⁵ × 1.38×10⁻²³ J/K × 300 K × ln 10 ≈ 9.5 × 10⁴ J/kg ≈ 0.1 MJ/kg.
```

Against real processes:

| Process | Actual energy | Ratio to configurational bound |
|---|---|---|
| Steel, primary production | ~20–25 MJ/kg | ~10² |
| Aluminum, primary | ~170–210 MJ/kg | ~10³ |
| Pharmaceutical API (high PMI) | ~10²–10³ MJ/kg | ~10³–10⁴ |
| Leading-edge logic wafer | ~10⁴–10⁵ MJ/kg of silicon | ~10⁵–10⁶ |

**Conclusion, and it is important: thermodynamic bounds are not the binding
constraint on physical compilation — by two to six orders of magnitude for matter
and twenty for computation.** A research program that optimizes against Landauer is
optimizing against a constraint that will not bind this century.

**What binds instead**, in order of severity:

1. **Kinetics.** Rates are exponential in barrier height: a 1 eV barrier at 300 K is
   a factor e^{−38.7} ≈ 10⁻¹⁷ on the attempt frequency. Raising temperature to
   accelerate everything destroys selectivity. **The entire difficulty of synthesis
   is that you need one barrier lowered and the others not** — which is why catalysis
   is the central technology of the chemical industry and why "thermodynamically
   favorable" says almost nothing about achievability.
2. **Selectivity.** Yield losses to competing pathways, not energy, dominate cost in
   chemistry and biology.
3. **Metrology.** The cost of *knowing* often exceeds the cost of *doing*: in
   aerospace and pharmaceutical manufacturing, qualification and testing routinely
   exceed unit production cost, sometimes by an order of magnitude.
4. **Heat removal.** Where thermodynamics does bite industrially, it bites as an
   entropy-export *rate* constraint — you must dump T₀ΔS_env of heat through a finite
   interface — not as an energy-total constraint.

### 15.4 Is there a "generalized thermodynamic compilation cost"?

A defensible and modest version exists:

> **Proposition 15.1.** For a specification S with configurational content
> I(S) = log₂|Σ| − log₂|[S]| bits (the information required to select the spec cell
> [S] out of the accessible state space), executed in time τ over a substrate with
> friction tensor g, the minimum work satisfies
>
> ```
> W ≥ ΔF_state + k_B T₀ ln2 · I(S) · χ + 𝓛²_g / (2τ)
> ```
>
> where χ ∈ [0,1] is the fraction of the selection that is performed *irreversibly*
> (i.e. by discarding alternatives rather than by reversible steering).

Two honest caveats. First, χ is where all the content hides: a process that selects
by rejecting 99% of attempts pays the full information cost, while one that steers
reversibly pays little — and real processes are overwhelmingly of the first kind.
Second, I(S) is defined relative to a coarse-graining, so the bound is only as
meaningful as the declared level of description. That relativity is not a flaw in
the proposition; it is a true feature of thermodynamic information accounting, and
§4.3's insistence on declared levels of description is what makes it well-defined.

**Label: PLAUSIBLE as stated, with the component inequalities ESTABLISHED
individually.** Sharpening χ into a computable quantity for a given process class
is a genuine research problem and is listed as Open Problem 5 (§27).

---

## 16. Complexity Theory: Physical Compilation Complexity

### 16.1 The claim to be corrected

The proposal states: *"Exact physical compilation is NP-hard."*

**Verdict: CONTRADICTED as a characterization.** It fails in three distinct ways.

1. **It is far too weak.** The general problem is not in NP and is not merely
   NP-hard; several natural formulations are **undecidable**. NP-hardness would be
   *good news*.
2. **It is not a single problem.** Optimization, simulation, control, reachability,
   and verification are different problems with different, incomparable
   complexities. No single class characterizes their combination.
3. **"Exact" is doing illegitimate work.** Exact compilation to a *point* target is
   not merely hard, it is ill-posed (§4.4). The meaningful problems are approximate
   and probabilistic from the start, and their complexity is different in kind.

### 16.2 Undecidability results that bound the framework

**Reachability for hybrid systems.** Physical compilation with discrete mode
switching and continuous dynamics is a hybrid automaton reachability problem.
Reachability is decidable for timed automata and for a narrow class of rectangular
hybrid automata, and becomes **undecidable** just above that boundary — for instance
for piecewise-constant-derivative systems in dimension ≥ 3 and for linear hybrid
automata with modest expressive power. **ESTABLISHED (mid-1990s).** Since any
realistic process model exceeds these thresholds, *general* physical reachability
is undecidable.

**Spectral gap undecidability.** Whether a translationally invariant
nearest-neighbor spin model on a 2D lattice is gapped or gapless in the
thermodynamic limit is **undecidable** — the question is equivalent to a halting
problem. **ESTABLISHED (2015).** Implication for the framework: "is this material an
insulator?" is not answerable in general by any algorithm. The result concerns the
infinite-size limit of a constructed family, not any finite instance, and its
practical force is bounded accordingly — but it definitively kills the idea of a
complete, general materials-property oracle.

**Undecidability in chemistry and tiling.** Whether a given tile set tiles the plane
is undecidable (Berger, 1966), and algorithmic self-assembly inherits Turing
universality (§18), so "does this tile set produce the target structure" is
undecidable in general.

**What this means, practically.** Undecidability of a general problem is not fatal
to an engineering discipline — program verification is undecidable and program
verification tools are used daily. What it means is:

> **The framework must be built on a *bounded, approximate, per-instance* problem
> statement (RC-0′), and must return UNKNOWN honestly.** Any architecture that
> promises a total decision procedure is promising something that cannot exist.

### 16.3 Reaction networks: decidable but non-elementary

An important and rarely noted result. Discrete chemical reaction networks with
integer species counts are exactly **vector addition systems / Petri nets**, and
reachability for those is **decidable but Ackermann-complete** — recent work settled
matching Ackermannian upper and lower bounds (2021). Ackermann-complete is
non-primitive-recursive: worse than any tower of exponentials.

**Implication:** "can this set of reactions, starting from this inventory, reach a
state containing the target molecule?" is decidable in principle and hopeless in
general. Retrosynthesis works in practice *only because* it restricts to bounded
depth, curated reaction templates, and heuristic guidance — i.e., exactly the
RC-0′ restriction. This is a clean, rigorous vindication of the bounded formulation.

### 16.4 Classical hardness results

| Problem | Complexity | Relevance |
|---|---|---|
| Ground state of an Ising spin glass (3D, or planar with fields) | **NP-complete** (Barahona, 1982) | Inverse design over discrete configurations; "let it relax to the ground state" is NP-hard to verify |
| Protein folding in standard lattice models (HP model) | **NP-complete** | Structure prediction from sequence, in the model |
| Motion planning with multiple movable objects ("warehouseman's problem") | **PSPACE-hard** | Robotic assembly planning |
| Generalized mover's problem | **PSPACE-hard** (Reif, 1979) | Same |
| Propositional (STRIPS) planning with bounded state | **PSPACE-complete** | Route/process synthesis (§6.5) |
| Planning with unbounded numeric fluents | **Undecidable** | Resource-aware process synthesis |
| Minimum tile set for a target shape (aTAM) | **NP-hard** | Self-assembly compilation (§18) |
| Mixed-integer optimal control | **NP-hard**, often much worse | Stage-4 parameter synthesis with discrete mode choices |
| Optimal experimental design (submodular maximization) | **NP-hard**, but (1−1/e)-approximable greedily | §6.9; the approximation guarantee is a genuine gift |

Note the last row. **Several of the framework's stages have constant-factor
approximation guarantees even though exact solution is NP-hard.** Greedy
sensor/experiment selection under submodularity is the clearest case, and it is
directly applicable to the verification-plan synthesis of §11.5. This is an example
of the general point that hardness of the exact problem says little about the
usefulness of the approximate one.

### 16.5 Quantum hardness results

| Problem | Complexity | Relevance |
|---|---|---|
| k-local Hamiltonian ground-energy estimation, k ≥ 2 | **QMA-complete** (Kitaev for k=5; Kempe–Kitaev–Regev for k=2) | "What is this material's ground-state energy?" is as hard as anything verifiable on a quantum computer |
| Local Hamiltonian on physically realistic 2D lattices with fixed interactions | **QMA-complete** (Oliveira–Terhal, and the subsequent classification program) | Hardness is not an artifact of artificial models; it survives restriction to realistic geometries |
| Classification of 2-local qubit Hamiltonians | Complete classification into P / NP-hard / StoqMA-complete / QMA-complete (Cubitt–Montanaro, 2016) | **A genuine structural result: the landscape of hardness for physical Hamiltonians is now mapped.** This is the closest existing thing to the "Physical Compilation Complexity" taxonomy the proposal asks for, and the framework should build on it rather than beside it |
| Existence of *universal* Hamiltonian families that simulate all others with polynomial overhead | **Proved** (Cubitt–Montanaro–Piddock, 2018) | See §17.3 — the one place universality genuinely holds |
| Simulating local Hamiltonian dynamics | **BQP-complete** | Quantum computers are exactly as powerful as needed for this, no more |
| Determining if a gapped ground state can be prepared adiabatically in poly time | Related to QMA-hardness; gap can be exponentially small | Bounds the adiabatic strategy of §8 |
| Full state tomography | Θ(d²/ε²) copies — **exponential in qubit count** | §11.2 |
| Shadow/property estimation for m local observables | O(log m /ε²) copies | §11.2 — the escape |

**Reading of the table.** The proposal asks whether the problem is NP-hard. A more
accurate summary: *the natural decision problems attached to physical ground states
are QMA-complete; the natural simulation problems are BQP-complete; the natural
discrete planning problems are PSPACE-complete to undecidable; the natural
reachability problems are undecidable or Ackermann-complete; and the natural
verification problems range from O(1) samples to exponentially many depending on
whether the target is a property or a state.* There is no single class. Asserting
one is the error.

### 16.6 The proposed taxonomy — is `PCC` meaningful?

The proposal suggests
`PCC = {PCC_classical, PCC_quantum, PCC_analog, PCC_biological, PCC_hybrid}`.

**Verdict: not meaningful as stated, for a precise reason.** Complexity classes are
defined by *computational resources and machine models*, not by substrate. "Quantum"
is a legitimate axis because it corresponds to a machine model (BQP, QMA).
"Biological" is not: a biological process is a physical process, and there is no
biological model of computation distinct from the classical/quantum ones.
"Analog" is a machine model but a treacherous one: without a noise/precision bound,
analog models spuriously exceed Turing computability, and with one, they collapse to
BPP-like classes. Cutting a taxonomy by substrate mislabels the real structure.

**The multidimensional formulation the proposal offers as an alternative is the
right one**, and it can be made precise.

### 16.7 Proposal: the compilation complexity vector and its exchange relations

> **Definition 16.1 (Compilation complexity vector).** For a compilation instance
> ⟨𝔟₀, S, Ω⟩ and a target confidence δ, define
>
> ```
> 𝒞(𝔟₀, S, Ω, δ) = ( C_search, C_sim, C_ctrl, C_ver, C_fab, C_E, C_τ, C_I )
> ```
>
> - **C_search** — cost of finding a candidate plan (discrete route search)
> - **C_sim** — cost of evaluating a candidate plan's predicted outcome
> - **C_ctrl** — cost of synthesizing and executing the control (number of control
>   degrees of freedom × bandwidth × precision)
> - **C_ver** — *number of physical measurements* to certify S at confidence 1−δ
> - **C_fab** — number of physical operations/articles consumed, including
>   sacrificial and rework
> - **C_E** — energy; **C_τ** — wall-clock time; **C_I** — information that must be
>   transferred to the substrate (§15.4's I(S))

The content of the theory is not the vector; it is the claim that **the components
are not independent, and the compiler's real job is to choose a point on an exchange
surface.** Proposed exchange relations, each of which is a research hypothesis with
a concrete test:

**(E1) Simulation ⇄ verification.** `C_sim · C_ver ≳ const` for a fixed confidence.
You can predict the outcome (expensive computation, few measurements) or measure it
(cheap computation, many measurements). Choosing where to sit on this curve *is*
process development. **PLAUSIBLE; testable directly (F-2, §25).**

**(E2) Fabrication ⇄ verification (design for testability).** Increasing C_fab by
adding built-in self-test structures, witness features, or deliberately distinctive
signatures reduces C_ver, often by orders of magnitude. **STRONGLY SUPPORTED by the
VLSI precedent** and untested elsewhere. This is the framework's flagship hypothesis.

**(E3) Control ⇄ search (landscape engineering).** Spending effort to engineer a
landscape in which the target is a robust attractor (raising C_search at compile
time, once) reduces C_ctrl at run time (fewer actuators, less bandwidth), for every
article made thereafter. This is the formal content of §8 and it explains *why*
landscape engineering pays: the compile-time cost amortizes over production volume.
**PLAUSIBLE, and it yields a quantitative prediction**: landscape engineering wins
when production volume exceeds C_search/(ΔC_ctrl per unit).

**(E4) Time ⇄ energy.** W_ex ~ 𝓛²/(2τ) (§15.2). **ESTABLISHED.**

**(E5) Tolerance ⇄ dissipation.** Σ ≥ 2k_B/ε² (TUR). **ESTABLISHED** for the
steady-state current setting; its generalization to manufacturing tolerance is
**SPECULATIVE** and worth pursuing.

**(E6) Specification size ⇄ everything.** C_ver grows linearly in |𝒪| (Prop. 11.1);
C_search grows at least linearly in the number of constraints; C_I grows with the
information content of [S]. **Over-specification is uniformly costly**, and the
compiler should report a per-requirement marginal cost. This is directly
implementable and immediately useful.

**Assessment.** Is a multidimensional complexity theory more appropriate than a
single class? **Yes, decisively**, and the exchange relations are what make it a
theory rather than a bookkeeping scheme. Whether E1–E3 hold with the quantitative
forms conjectured is exactly the kind of question that makes this a research program.
A name for the resulting discipline is proposed in Final Answer J.

### 16.8 Where tractability actually comes from

Assembling the escape routes, since the hardness results otherwise read as despair:

1. **Bounded horizon.** RC-0′. Converts undecidable to decidable.
2. **Coarse-graining.** Working at the level of description where the specification
   lives, never finer. Converts exponential state spaces to manageable ones — at the
   price of model error, which must be measured, not assumed.
3. **Tolerance.** [S] is a fat set (§4.4). Approximation is not a compromise; it is
   the problem statement.
4. **Structure.** Real instances are not worst case: molecules are sparse graphs,
   materials are periodic, processes are near-decomposable, physical interactions
   are local. Locality in particular buys enormous algorithmic leverage (tensor
   networks, area laws, sparse solvers, domain decomposition).
5. **Amortization.** A compilation is solved once and executed 10⁶ times. This
   changes the economics of every hardness result on the list and is the reason EDA
   can afford to run NP-hard placement heuristics for days.
6. **Physical evaluation.** When simulation is intractable, run the experiment
   (Case C, §8.3). Expensive per query, but it is an oracle with the *correct*
   physics — its "model error" is zero by construction.
7. **Approximation guarantees where they exist.** Submodularity in experiment
   selection; convex relaxations in control; semidefinite bounds in ground-state
   energy.

**The correct summary sentence for the framework's complexity story:** *Physical
compilation is undecidable in general, intractable in most bounded formulations, and
routinely solved in practice — because practice restricts the horizon, accepts
tolerance, exploits locality, amortizes over volume, and substitutes measurement for
proof. A theory of physical compilation is a theory of those five substitutions.*

---

## 17. Universality

### 17.1 The five meanings, separated

The proposal correctly warns that the five senses of universality are not
equivalent. They are assessed individually.

### 17.2 Universal representation — **NO**, but a universal interface — **YES**

There is no single state representation adequate across the quantum, atomistic,
mesoscale, and continuum levels; the levels are related by information-destroying
maps that are not invertible (§4.3). Any claim of a universal representation is
either false or vacuous (a bit string is universal and useless).

**But** a universal *specification language* and a universal *attestation format* are
achievable, because both range over *measurable functionals and evidence*, which are
substrate-independent by construction. A tensile strength, a purity, a defect rate,
and a fidelity are all the same kind of thing at the level of the interface: an
estimand with a protocol, a tolerance, and a confidence.

**This asymmetry — no universal representation, yes universal interface — is the
architectural key to the whole program**, and it has an exact precedent: LLVM has no
universal machine model, and a universal IR plus target back ends nonetheless works.

### 17.3 Universal optimization — **NO**

No-free-lunch results say that averaged over all objective functions no optimizer
beats any other; more concretely, the hardness results of §16.4–16.5 say that no
single algorithm handles ground-state search, route planning, and optimal control
efficiently. Optimization must be portfolio-based and structure-exploiting.
**ESTABLISHED.**

### 17.4 Universal physical control — **YES, in a precise and limited sense**

This is the one place where a genuine universality theorem exists and it should be
foregrounded.

- **Quantum control:** for a bilinear control system on a compact Lie group, the Lie
  algebra rank condition is necessary and sufficient for complete controllability.
  When it holds, *any* unitary is reachable. This is a real universality statement
  with an effective test. **ESTABLISHED.** It says nothing about *time* or
  *robustness*, which is where practice lives.
- **Universal Hamiltonians:** certain simple spin-lattice models are *universal
  simulators* — they can reproduce the full physics (spectrum, partition function,
  dynamics) of any other spin Hamiltonian with polynomial overhead. **ESTABLISHED
  (2018).** This is a striking result and is exactly the kind of universality the
  proposal hoped for, realized in the one domain where it is provable.
- **Classical control:** no comparable general theorem. Controllability of nonlinear
  systems has local results (Chow–Rashevskii, accessibility rank conditions) but
  global controllability over realistic actuation is an empirical question per
  substrate.

**Honest reading:** universality of control holds where the system is a
finite-dimensional bilinear system with sufficient actuation. Bulk matter is not
such a system, because you do not have one control per degree of freedom (§8.5,
condition 1). Universality of control therefore does *not* transfer to manufacturing.

### 17.5 Universal fabrication — **NO**

Settled in §10.1 by throughput arithmetic, not by principle. Even granting arbitrary
positional chemistry, serial assembly cannot produce bulk matter, and parallel
processes are substrate-specific. There is no universal assembler; there are
universal *interfaces to* families of assemblers.

### 17.6 Universal verification — **NO**

Two independent obstructions: (i) checking an arbitrary property of an arbitrary
physical system is undecidable in general (§16.2); (ii) even for decidable
properties, sample complexity is method- and property-specific, and some properties
(no flaw anywhere; behavior at t = 30 years) are not certifiable by measurement at
all, only by model. **Verification is irreducibly substrate- and property-specific.**

### 17.7 The verdict on universality

> **Reality compilation is substrate-specific in its physics and universal only in
> its interfaces.** The universal layer is: the specification language, the evidence
> calculus (VERIFY/ASSUME/DERIVE with confidences), the complexity vector as a
> reporting format, and the attestation schema. Everything below that layer — state
> representation, models, search, control, fabrication, metrology — is and will
> remain substrate-specific.

This is a narrowing of the original thesis, and it is also the form in which the
thesis becomes *buildable*, *standardizable*, and *defensible*. A universal compiler
is not available; a universal *contract* between specification and substrate is, and
it is worth more, because it is what lets substrate-specific back ends compose,
compete, and be audited against each other.
