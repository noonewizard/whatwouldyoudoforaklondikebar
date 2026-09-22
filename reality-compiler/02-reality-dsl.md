# Part II — The Specification Language and Compilation Semantics

Sections 5–6: Reality DSL · Compilation Semantics

---

## 5. Reality DSL

### 5.1 Design position

The single most defensible near-term deliverable of the whole program is a
specification language. It is Stage 0 of the roadmap (§21), it requires no new
physics, and it is where the framework can be *right* before it can be *useful*.

But a DSL is only worth building if it makes illegal states unrepresentable. A
language that merely provides syntax for what engineers already write in
spreadsheets is a reformatting exercise. The following five rules are what give the
language its content; each corresponds to a real, expensive, recurring class of
industrial error.

**Rule 1 — No property without a protocol.**
Every specified property is a pair (estimand, measurement protocol). `tensile_strength
>= 400 MPa` is a type error. `tensile_strength(ASTM_E8M, strain_rate: 1e-3/s,
T: 23 degC, orientation: L) >= 400 MPa` is well-formed. Rationale: §4.1(iii).

**Rule 2 — No tolerance without a statistic and a population.**
`defect_probability <= 1e-6` is ambiguous between per-unit, per-feature, per-hour,
and per-batch, and between a point estimate and an upper confidence bound. The
language requires: statistic (mean / quantile / upper confidence bound), population
(unit / lot / feature), and confidence.

**Rule 3 — Units and dimensions are part of the type system.**
Dimensional analysis is checked statically. This is table stakes and is nonetheless
absent from most scientific code.

**Rule 4 — Every requirement is either `VERIFY`, `ASSUME`, or `DERIVE`.**
`VERIFY` = will be measured on this article or a declared sample of the lot.
`ASSUME` = inherited from an upstream attestation or a supplier certificate.
`DERIVE` = predicted by a named model with a declared validity domain. **A
specification in which every requirement is `DERIVE` carries no physical
guarantee at all**, and the compiler reports this as a *coverage* figure. This is
the DSL's most important single feature.

**Rule 5 — Objectives and constraints are syntactically distinct.**
`optimize(...)` inside a property assignment, as in the proposal's example, is
ambiguous: it does not say whether the material choice is a free variable subject to
the listed constraints, or an objective. Separate them.

### 5.2 Concrete grammar (preliminary, EBNF)

```ebnf
program        ::= { import | unit_def | protocol_def | material_def
                   | object_def | process_def | assert_block } ;

(* ---------- declarations ---------- *)
import         ::= "IMPORT" string [ "AS" ident ] ;
unit_def       ::= "UNIT" ident "=" quantity_expr ;
protocol_def   ::= "PROTOCOL" ident "(" [ param_list ] ")" "{" { kv_pair } "}" ;
material_def   ::= "MATERIAL" ident "{" { kv_pair } "}" ;

(* ---------- the central construct ---------- *)
object_def     ::= "OBJECT" ident [ ":" ident ] "{" { obj_clause } "}" ;

obj_clause     ::= geometry_clause | composition_clause | requirement
                 | free_var | objective | env_clause | provenance_clause
                 | verification_clause | nested_object ;

free_var       ::= "FREE" ident ":" type_expr [ "IN" domain_expr ] ;
objective      ::= ( "MINIMIZE" | "MAXIMIZE" ) expr [ "WEIGHT" number ] ;

requirement    ::= mode property_ref rel_op quantity [ stat_qualifier ]
                   [ "AT" confidence ] [ "OVER" population ] ";" ;
mode           ::= "VERIFY" | "ASSUME" | "DERIVE" "(" model_ref ")" ;
property_ref   ::= ident "(" protocol_ref { "," named_arg } ")" ;
rel_op         ::= ">=" | "<=" | "==" | "IN" | "WITHIN" ;
stat_qualifier ::= "MEAN" | "MIN" | "MAX" | "QUANTILE" "(" number ")"
                 | "UCB" | "LCB" ;
confidence     ::= number ;                     (* e.g. 0.95 *)
population     ::= "UNIT" | "LOT" "(" number ")" | "FEATURE" | "BATCH" ;

geometry_clause::= "GEOMETRY" ( brep_ref | csg_expr | lattice_expr )
                   [ "TOLERANCE" gdt_block ] ;
gdt_block      ::= "{" { gdt_callout } "}" ;      (* ASME Y14.5 / ISO GPS subset *)
composition_clause ::= "COMPOSITION" "{" { species "=" range } "}" ;

env_clause     ::= "ENVIRONMENT" "{" { kv_pair } "}" ;
verification_clause ::= "VERIFICATION" "{" { verif_stmt } "}" ;
verif_stmt     ::= "METHOD" ident "(" [ named_arg_list ] ")"
                   [ "SAMPLES" expr ] [ "DESTRUCTIVE" bool ]
                   [ "COST" quantity ] ";" ;
provenance_clause ::= "PROVENANCE" "{" { prov_stmt } "}" ;
prov_stmt      ::= "REQUIRE" ( "ATTESTED_INPUTS" | "ATTESTED_MACHINE"
                             | "CALIBRATION_WITHIN" quantity
                             | "CHAIN_DEPTH" number
                             | "POLICY" string ) ";" ;

(* ---------- processes and substrate ---------- *)
process_def    ::= "PROCESS" ident "(" [ param_list ] ")" "{"
                     { "REQUIRES" predicate ";" }
                     { "ENSURES"  predicate ";" }
                     { "CONSUMES" resource_expr ";" }
                     { "DURATION" quantity ";" }
                     { "MODEL" model_ref "VALID_IN" domain_expr ";" }
                   "}" ;

substrate_def  ::= "SUBSTRATE" ident "{" { "ACTION" ident ";" }
                     { "MEASURE" ident ";" } { "INVENTORY" kv_pair }
                     { "ENVELOPE" kv_pair } "}" ;

(* ---------- expressions ---------- *)
quantity       ::= number unit_expr ;
expr           ::= (* dimensioned arithmetic, comparison, let-binding,
                      aggregation over populations *) ;
```

Notes on what the grammar deliberately does **not** include:

- **No imperative control flow in a specification.** Specifications are declarative;
  processes are where sequencing lives. Mixing them is how CAM languages became
  unanalyzable.
- **No direct material naming inside a requirement.** Materials are either fixed by
  a `MATERIAL` declaration or left `FREE`. This prevents the ambiguity in the
  proposal's `material = optimize(...)`.

### 5.3 Example program 1 — the proposal's structural component, repaired

```
IMPORT "std/protocols/mechanical" AS mech;
IMPORT "std/gdt" AS gdt;

OBJECT structural_bracket {

  GEOMETRY  brep("bracket_rev7.step")
    TOLERANCE {
      gdt.profile(surface: "A_datum", zone: 0.15 mm);
      gdt.position(hole: "H1", zone: 0.05 mm, datums: [A, B, C]);
    }

  FREE alloy : MaterialChoice IN catalog("aero_alloys_v3");
  FREE route : ProcessRoute   IN routes(alloy, GEOMETRY);

  # --- requirements -------------------------------------------------
  VERIFY mech.tensile_strength(
           protocol: mech.ASTM_E8M,
           T: 23 degC, strain_rate: 1e-3 /s, orientation: L)
         >= 400 MPa  QUANTILE(0.01)  AT 0.95  OVER LOT(30);

  VERIFY mech.tensile_strength(
           protocol: mech.ASTM_E8M, T: 150 degC,
           strain_rate: 1e-3 /s, orientation: ST)
         >= 310 MPa  QUANTILE(0.01)  AT 0.95  OVER LOT(30);

  DERIVE(model: "thermal.gnn_v4")
         thermal_conductivity(protocol: mech.ASTM_E1461, T: 23 degC)
         IN [120, 180] W/(m*K)  AT 0.90;

  VERIFY mass(protocol: gravimetric(resolution: 0.1 g)) <= 850 g  MAX  OVER UNIT;

  VERIFY porosity(protocol: xct(voxel: 20 um, threshold: "ASTM_E1441"))
         <= 0.2 %  MEAN  OVER UNIT;

  # Defect requirement, made well-formed: a per-unit probability of a
  # critical (>0.5 mm) internal flaw, as a 95% upper confidence bound.
  VERIFY critical_flaw_rate(protocol: xct(voxel: 20 um),
                            criterion: "flaw_diameter > 0.5 mm")
         <= 1e-4  UCB  AT 0.95  OVER UNIT;

  ENVIRONMENT { service_T: [-55 degC, 150 degC];
                atmosphere: "air"; cycles: 1e5; }

  # --- objectives ---------------------------------------------------
  MINIMIZE cost(route)                 WEIGHT 1.0;
  MINIMIZE verification_cost(route)    WEIGHT 0.4;
  MINIMIZE embodied_energy(route)      WEIGHT 0.2;

  VERIFICATION {
    METHOD xct(voxel: 20 um)  SAMPLES 1  DESTRUCTIVE false  COST 40 USD;
    METHOD tensile(mech.ASTM_E8M) SAMPLES 3 DESTRUCTIVE true COST 900 USD;
    METHOD ebsd(step: 0.5 um) SAMPLES 1 DESTRUCTIVE true COST 1200 USD;
  }

  PROVENANCE {
    REQUIRE ATTESTED_INPUTS;
    REQUIRE ATTESTED_MACHINE;
    REQUIRE CALIBRATION_WITHIN 30 d;
    REQUIRE POLICY "aerospace/AS9100/rev_D";
  }
}
```

The compiler's first output on this program is not a process. It is a
**specification report**:

```
SPEC COVERAGE  structural_bracket
  requirements: 6   VERIFY: 5 (83%)   DERIVE: 1 (17%)   ASSUME: 0
  DERIVE coverage gap:
    thermal_conductivity — model thermal.gnn_v4 declared valid for
      Al/Ti/Ni alloys, 200-500 K, porosity<1%. Candidate route R3 exits
      the validity domain (porosity 1.4%). Requirement UNGUARANTEED on R3.
  verification sample complexity (see §11):
    tensile QUANTILE(0.01) AT 0.95 OVER LOT(30) is INFEASIBLE:
      a 1st-percentile lower bound at 95% confidence from n=30
      requires a distributional assumption. With normality assumed:
      k-factor = 3.06 -> required mean margin >= 3.06 sigma.
      Nonparametric: n >= 299 for a distribution-free 1%/95% bound.
    ACTION REQUIRED: declare a distributional assumption, raise n, or
      relax the quantile.
```

That report — produced by static analysis, before any physics is simulated — is
already worth money. It catches the single most common real defect in engineering
specifications: statistically unsatisfiable acceptance criteria. Note that the
nonparametric bound is exact and elementary: the probability that the minimum of n
i.i.d. samples exceeds the p-quantile is (1−p)ⁿ, so a distribution-free
one-sided 95% bound on the 1st percentile needs (0.99)ⁿ ≤ 0.05, i.e. n ≥ 299.
**ESTABLISHED** (order statistics).

### 5.4 Example program 2 — a chemical target, showing route freedom

```
IMPORT "std/chem" AS chem;

OBJECT api_lot : ChemicalSubstance {
  IDENTITY  chem.inchikey("RZVAJINKPMORJF-UHFFFAOYSA-N");

  VERIFY chem.assay(protocol: chem.HPLC_UV(column:"C18", lambda: 254 nm))
         IN [98.0, 102.0] %  MEAN  AT 0.95  OVER BATCH;
  VERIFY chem.impurity(protocol: chem.HPLC_UV(...), any_single)
         <= 0.10 %  MAX  OVER BATCH;
  VERIFY chem.impurity(protocol: chem.HPLC_UV(...), total)
         <= 0.50 %  MAX  OVER BATCH;
  VERIFY chem.residual_solvent(protocol: chem.GC_HS, species: "DCM")
         <= 600 ppm  MAX  OVER BATCH;              # ICH Q3C class 2
  VERIFY chem.polymorph(protocol: chem.PXRD(range: [5,40] deg))
         == "Form_I"  OVER BATCH;
  VERIFY chem.enantiomeric_excess(protocol: chem.chiral_HPLC)
         >= 99.0 %  MIN  OVER BATCH;

  FREE route : SynthesisRoute IN retro(IDENTITY, depth: 6,
                                       catalog: "commercial_2026Q1");

  CONSTRAINT route.avoids(reagent_class: ["azide", "diazomethane"]);
  CONSTRAINT route.pmi <= 60 kg/kg;                # process mass intensity
  CONSTRAINT route.max_step_temperature <= 140 degC;

  MINIMIZE route.cost  WEIGHT 1.0;
  MINIMIZE route.steps WEIGHT 0.3;
  MINIMIZE route.verification_cost WEIGHT 0.5;

  PROVENANCE { REQUIRE ATTESTED_INPUTS; REQUIRE CHAIN_DEPTH 6; }
}
```

The polymorph requirement is the interesting one. It is a *kinetic* requirement
disguised as a compositional one — Form I may be metastable — and the compiler must
recognize that satisfying it constrains the crystallization protocol, not the
molecular target. A compiler that reasons only about thermodynamic ground states
would return the wrong polymorph and pass its own simulated check. This is a
concrete instance of §3.2's conclusion that the kinetic layer binds.

### 5.5 Example program 3 — a self-assembly target

```
OBJECT dna_tile_array : SelfAssembled {
  GEOMETRY  lattice(kind: "square", dims: [64, 64], pitch: 5.8 nm);
  PATTERN   bitmap("sierpinski_64.png");

  SUBSTRATE dna_tam {
    ACTION anneal(T_start, T_end, rate);
    INVENTORY strands: catalog("idt_2026"), buffer: "TAE/Mg2+ 12.5mM";
    ENVELOPE  T: [20,95] degC, ramp: [0.01, 5] degC/min;
  }

  DERIVE(model: "ktam_v2") yield(correct_tile_placement) >= 0.999 AT 0.90;
  VERIFY afm_pattern_fidelity(protocol: afm(mode:"tapping", pixel: 2 nm))
         >= 0.98  MEAN  AT 0.95  OVER LOT(20);

  MINIMIZE strand_count WEIGHT 1.0;
  MINIMIZE anneal_time  WEIGHT 0.2;
}
```

Here the compiler's job is genuinely the one the framework describes: translate a
global pattern into **local rules** (tile types and sticky-end sequences) whose
equilibrium/kinetic assembly realizes the pattern. This is the one domain where the
full Reality-Compiler pipeline has already been demonstrated end-to-end
(§18). It is the framework's existence proof, and it should be foregrounded rather
than buried under speculative material.

### 5.6 Type system

The type system carries the framework's physics. Sketch:

- **Dimensioned scalars** `Q[d]` where d ∈ ℤ⁷ (SI base dimensions), with checked
  arithmetic. Affine units (°C, gauge pressure) are a distinct kind from ratio units
  and cannot be multiplied. This catches a real and expensive error class.
- **Estimands** `Est[τ, 𝒫]` — a property type τ indexed by a protocol 𝒫. Two
  estimands of the same τ but different 𝒫 are *not* interchangeable and do not
  unify. Conversion requires an explicit, declared, and attestable bridging relation
  (e.g., a hardness-to-tensile correlation, which is a model, with a validity domain
  and an error, and therefore `DERIVE`).
- **Conservation-typed inventories.** A linear (substructural) type discipline over
  species counts: material consumed cannot be reused, and the element balance of a
  route is checked statically. This is the type-theoretic encoding of §2.1's
  conservation constraint and it is cheap to implement. **A specification requiring
  an element absent from the inventory fails to type-check** — which is the correct
  and typed way to reject "software creating matter."
- **Confidence-carrying propositions.** A requirement's type records its mode
  (VERIFY/ASSUME/DERIVE) and its δ. Composition of requirements composes δ by union
  bound unless a tighter dependency structure is declared. This makes the coverage
  report a type-level computation.
- **Effect types for destructiveness and reversibility.** A `DESTRUCTIVE` measurement
  cannot appear in a plan that must deliver the measured article.

### 5.7 Semantics of the specification language

Denotationally, a specification denotes a triple:

```
⟦S⟧ = ( 𝒦 ⊆ ℝ^k ,  Φ = (Est₁,…,Est_k) ,  Ξ )
```

where Ξ is the **evidence obligation**: a mapping from each requirement to the mode
and confidence with which it must be discharged. Satisfaction is then defined
against a *lot* ℒ = (a₁,…,a_n) of articles and a record ℛ:

```
ℒ, ℛ ⊨ S   ⟺   ∀ r ∈ S.requirements :
                 mode(r) = VERIFY  ⟹  ℛ contains an admissible test for r
                                        whose decision is PASS at ≥ 1−δ_r
               ∧ mode(r) = ASSUME  ⟹  ℛ contains a valid upstream attestation for r
               ∧ mode(r) = DERIVE  ⟹  ℛ contains a model-validity witness and
                                        the model's prediction lies in 𝒦_r
```

Two consequences worth stating plainly:

1. **Satisfaction is a property of the (lot, record) pair, not of the article.**
   This is not a weakness of the formalism; it is the truth about how physical
   specifications work, and making it explicit is the framework's main contribution
   to specification practice.
2. **`DERIVE` requirements are discharged against a model, not against the world.**
   The record therefore shows exactly how much of the guarantee is epistemic. This
   is what the "coverage" figure measures, and it is the number a regulator, an
   insurer, or a customer should actually be asking for.

---

## 6. Compilation Semantics

### 6.1 Is the proposed pipeline S → G → M → H → P → U → R appropriate?

**Partly.** The staging is recognizably a compiler pipeline and that instinct is
right, but as drawn it is a *linear* pipeline, and physical compilation is not
linear. Three structural problems:

1. **It has no feedback edge.** Real physical compilation is closed-loop at compile
   time (design of experiments to reduce model uncertainty) and at run time
   (in-process control). A pipeline without a loop cannot express either.
2. **It places the material model M and the Hamiltonian H as sequential stages.**
   They are not stages; they are *the same object at two levels of resolution*, and
   which one is used is a per-decision choice, not a phase.
3. **It terminates at R (the realized state).** It should terminate at
   (record, decision), because R is never observed.

### 6.2 The corrected pipeline

```
                     ┌───────────────── model-uncertainty feedback ──────────────┐
                     ↓                                                            │
  S ──► [FRONT END] ──► IR₀ ──► [FEASIBILITY] ──► IR₁ ──► [ROUTE SEARCH] ──► IR₂ ──┤
        parse, type      spec     conservation,    pruned    discrete plan    route │
        check, desugar   graph    thermo, kinetic  spec      over 𝒜           +cand.│
                                  screens                                           │
                                                                                    │
  IR₂ ──► [PARAMETER SYNTHESIS] ──► IR₃ ──► [LOWERING] ──► U ──► [EXECUTION] ──► traces
          continuous optimal        timed     machine         signals    closed-loop
          control / protocol        program   binding,                   control
          design under 𝔐            + Ψ       calibration                     │
                                                                              ↓
  ──────────────────► [METROLOGY] ──► [DECISION] ──► [ATTESTATION] ──► Π ──────┘
                        execute Ψ      PASS/FAIL/    sign, chain
                                       INCONCLUSIVE
```

Five stages, plus a metrology/decision/attestation tail, plus two feedback edges
(compile-time model refinement; run-time control). Each is now defined.

### 6.3 Stage 1 — Front end

**Input** S (source). **Output** IR₀: a *specification graph* — nodes are estimands,
edges are dependencies and shared protocols, annotated with mode and δ.

Obligations discharged here: dimensional consistency; protocol resolution; estimand
unification; conservation typing of any fixed inventory; and the **coverage report**
of §5.3. Formally this stage is a total function; it either produces IR₀ or a typed
error. No physics yet. This is the whole of Stage 0 in the roadmap and it is
achievable with existing programming-language technology. **ESTABLISHED as
feasible.**

### 6.4 Stage 2 — Feasibility screening

**Input** IR₀ + Ω. **Output** IR₁, or a certificate of infeasibility.

Applies the permission stack of §3.2 *in order of cheapness*:

- **Conservation screen** — linear algebra over the element/charge balance.
  Complexity: polynomial. Produces an exact infeasibility certificate (a Farkas
  certificate on the stoichiometric matrix) when it fails. This is a genuinely nice
  property: *infeasibility here is provable and explainable*, not a search timeout.
- **Thermodynamic screen** — free-energy / convex-hull distance for the target
  composition; thermomajorization for quantum targets. Complexity: convex-hull
  construction, polynomial in the number of candidate phases. Yields "metastable by
  X meV/atom above hull," which is *not* a rejection (see the diamond point in §3.2)
  but a flag routed to the kinetic screen.
- **Kinetic screen** — does a route exist whose rate-limiting barrier is crossable in
  the allowed time at the allowed temperature? This is where feasibility is actually
  decided and it is the least automatable screen today. **This is the framework's
  primary scientific bottleneck**, restated in §26.
- **Control screen** — is the target in the reachable set of 𝒜? For bilinear
  systems (including quantum control) there is an exact, checkable criterion: the
  Lie algebra rank condition. For general nonlinear/hybrid systems there is not
  (§16.2).
- **Metrological screen** — is 𝒦 certifiable at δ with the available ℳ? Runs the
  sample-complexity computation of §11. **Placing this screen at compile time, before
  route search, is one of the framework's real contributions**: it fails fast on
  specifications that are unverifiable in principle, which today are discovered only
  after the process has been developed.

### 6.5 Stage 3 — Route search (discrete)

**Input** IR₁. **Output** IR₂: a set of candidate discrete plans over 𝒜, each a
partially ordered sequence of actions with symbolic parameters.

This is classical AI planning / retrosynthesis / process synthesis, and it is where
the combinatorics live. Formally: search over a labeled transition system whose
states are abstract resource/phase configurations and whose transitions are members
of 𝒜 with preconditions and effects. Complexity: PSPACE-complete for propositional
STRIPS-style planning with bounded state; worse with numeric resources; undecidable
with unbounded numeric fluents. **ESTABLISHED.**

The correct engineering posture, imported wholesale from retrosynthesis and from
EDA: **do not search for the optimum; search for a diverse set of good candidates
and defer the choice.** Returning k diverse routes with quantified model
uncertainty, and then choosing among them by a *measurement* (§6.9), converts an
intractable optimization into a tractable experimental design problem. This is the
single most practically important architectural decision in the compiler.

### 6.6 Stage 4 — Parameter synthesis (continuous)

**Input** IR₂ (a discrete plan with symbolic parameters). **Output** IR₃: a timed
program with concrete continuous protocols (temperature ramps, pressure profiles,
control pulses, flow rates) and an attached measurement plan Ψ.

This is optimal control under model uncertainty. Formally:

```
min_{u ∈ 𝒯}  sup_{m ∈ 𝔐}  𝔼_m [ 𝔍(σ, u) ]
s.t.  dσ = f_m(σ,u)dt + g_m(σ)dW,   σ(0) ~ 𝔟₀,
      path constraints (safety envelope),
      Pr[ Φ(σ(T)) ∈ 𝒦 ] ≥ 1 − δ.
```

The chance constraint is what makes this hard and what makes it correct. Standard
tooling: GRAPE and Krotov for quantum; direct collocation / multiple shooting for
classical; scenario approximation or distributionally-robust reformulation for the
sup over 𝔐. **ESTABLISHED methods, applied in a nonstandard combination.**

Two honest notes. First, the chance constraint over a rare event (δ = 10⁻⁶) cannot
be estimated by naive Monte Carlo; it needs importance sampling, large-deviation
approximations, or a surrogate on the failure boundary. Second, the sup over 𝔐 is
usually the dominant term and usually mis-specified: the true model is rarely in 𝔐.
Both are real research problems, not engineering details.

### 6.7 Stage 5 — Lowering and machine binding

**Input** IR₃ + a specific machine instance. **Output** U: signals.

This is where an abstract "anneal at 1.2 °C/min" becomes a setpoint trajectory for
a specific furnace with a specific thermal mass, a specific controller, and a
calibration record with a specific expiry. The key formal obligation:

> **Lowering correctness.** Each lowering step must be accompanied by a *refinement
> witness*: either a proof that the concrete program's behavior is contained in the
> abstract program's specified behavior, or a measured bound on the discrepancy
> (a calibration).

In practice almost all lowering witnesses will be measured, not proved. That is
fine, and it is exactly how EDA handles timing closure (static timing analysis
against characterized cell libraries — the library characterization is the measured
witness). The architectural requirement is that **the witness exists and is recorded
in Π**, so that an attestation says "executed on machine M whose calibration
witness for action a was measured on date d with residual ε."

### 6.8 Stage 6 — Execution with in-process control

Open-loop execution of a compiled physical program is a mistake in every substrate
where disturbances are not negligible, which is all of them. The compiled artifact
should therefore not be a signal but a **policy**: a feedback law plus a monitoring
predicate plus an abort/rework condition.

```
U := ⟨ π: (history, measurements) → actions,
       φ_safe: invariant that must hold throughout,
       φ_abort: condition triggering safe-state transition,
       Ψ_inline: in-process measurements ⟩
```

The safety invariant φ_safe is checked at run time by a monitor that is
*architecturally separate* from the controller and, in high-consequence settings,
implemented on independent hardware with a simpler and independently verified
model. This is standard safety-instrumented-system practice (IEC 61511) and it is
the correct answer to §14's actuator-compromise threat.

### 6.9 The compile-time feedback edge: active model discrimination

When two candidate routes in IR₂ are ranked differently by two models in 𝔐 whose
predictions differ by more than the tolerance band, the compiler should not pick one.
It should **emit an experiment** that maximally discriminates them:

```
e* = argmax_e  I( m ; y_e )            (mutual information between model identity
                                        and the outcome of experiment e)
     subject to cost(e) ≤ budget
```

This is Bayesian optimal experimental design (Lindley; Chaloner–Verdinelli), and it
is the formal object that turns "self-driving lab" from a slogan into a compiler
stage. **ESTABLISHED method**; its integration into a compilation pipeline as a
first-class stage is the novel part.

### 6.10 What "compilation" means here, precisely

A final definitional note, because the word is doing a lot of work.

In a conventional compiler, correctness is *semantic preservation*: the target
program's observable behavior refines the source program's. Physical compilation
cannot have that property, because the target "program" runs on hardware whose
semantics are only known statistically and approximately.

The correct analogue is **probabilistic refinement with an evidence obligation**:

> A physical compilation is *correct* if the emitted (program, measurement plan,
> record) triple has false-accept probability ≤ δ under the declared model bundle,
> **and** the record makes the declared model bundle and its validity witnesses
> auditable.

Note that correctness is therefore *relative to 𝔐* and the record must expose that
relativity. A compiler that hides its modeling assumptions cannot be correct in this
sense even if it is right. This is a strong and, I think, defensible position, and it
is the reason the attestation layer (§12) is not an add-on but a component of the
correctness criterion.
