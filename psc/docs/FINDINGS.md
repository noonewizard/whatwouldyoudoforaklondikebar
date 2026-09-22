# Prototype Findings

What building the compiler taught that writing the monograph did not. Every entry is
a finding from a program that actually ran, with the run output as evidence.

---

## F-1. The first benchmark failed, and I changed the model afterwards

**This is the most important entry in this file, and it is a methodological red flag
about my own process, reported rather than buried.**

Sequence:

1. `benchmark_verifiability.py` (F-28, prototype scale) was run. Result: **0% cost
   reduction on all three backends. FAIL.**
2. Diagnosis: `Witness.separates` was a *static* dictionary, so a witness's power to
   discriminate the target from an alternative did not depend on where in observable
   space the route landed. That is physically wrong — a correlator separates an
   ordered state from a paramagnet in proportion to the order itself, and separates
   nothing at all near criticality.
3. The model was corrected (`separation_fn`, operating-point dependent) and the
   benchmark re-run. Result: **median 70% cost reduction. PASS.**

**Status: the result does not count.** Changing the instrument after seeing a null
and re-running to a pass is the exact pattern that invalidates an experiment,
regardless of whether the change was independently justified. The correct status of
F-28 is:

> **Instrument corrected; the first valid test has not yet been run.** A pre-registered
> re-run on a model frozen before the result is seen is required. The 70% figure
> should not be cited.

What the episode *does* legitimately establish is a precondition, below.

---

## F-2. Compile-for-verifiability has a precondition nobody stated

The null result was informative. With static witness power, every route to a given
target is certified by the same witness at the same cost, so the verification-aware
objective has **nothing to optimise over** and the hypothesis is vacuous rather than
false.

> **ENGINEERING CRITERION.** Compile-for-verifiability can only pay when the route
> space contains routes whose *acceptance predicates differ in cost* — i.e. when
> witness discriminating power varies over the reachable observable space. Substrates
> with a single dominant characterisation method offer no verification-aware
> optimisation, however good the compiler.

The geometry backend still shows **0%** after the fix, because its witnesses remain
operating-point independent. That is the criterion demonstrating itself.

**Consequence for the monograph.** §9.9 and F-4/F-28 must add this precondition, and
any industrial pilot must first check that it holds in the chosen substrate. This was
not stated anywhere in Drafts 1–4.

---

## F-3. The compiler exploited an under-constrained specification

The quantum specification asked for long-range order, a spectral gap, and a low energy
density. The compiler returned **h = 0** — the classical Ising limit, at the edge of
the parameter box. That satisfies every stated requirement and contains no quantum
physics whatsoever.

Nothing in the framework detects this. The specification was under-constrained and the
optimiser did what optimisers do.

> **OPEN PROBLEM (new).** The framework has no notion of a *trivial* or *degenerate*
> solution. Acceptance regions bound what must be true; they do not bound what must
> be *interesting*. A specification language for physics needs either (a) mandatory
> non-degeneracy constraints, or (b) a compiler warning when the returned solution
> lies in a known degenerate limit of the model class.

This is a real gap and it generalises: every inverse-design system is vulnerable to it.

---

## F-4. Boundary-limited optima in all three backends

After adding the check, all three backends reported `BOUNDARY-LIMITED`: the optimum sat
on the edge of the declared parameter box. The reported margin is therefore a lower
bound, and the true optimum may lie outside Λ_Ω.

This is not a bug — it is the compiler correctly reporting that the *substrate
declaration*, not the physics, is binding. Without the check it was silent, which
means Drafts 1–4's architecture would have shipped a compiler that hides which
constraint is active.

---

## F-5. A witness model and a measurement model can silently disagree

The geometry backend initially reported a verification z-score of **5276** — absurd.
Cause: the witness declared σ = 0.004 for `mass` while the backend's sampling model
used a relative σ of 0.002 × value. Every sample-size computation for that witness was
unsound.

Now a typed error: `INCONSISTENT-MEASUREMENT-MODEL`. The general lesson is that the
measurement algebra ℳ appears **twice** in any such system — once in the witness
declaration used for planning, once in the sampling model used for execution — and
nothing forces them to agree. A real implementation must derive one from the other or
check them.

---

## F-6. Certificate strength is not uniform, and the difference is visible in code

Implementing the eight certificate classes made the strength ordering concrete:

- **Conservation, resource, spectral, metrological** — analytic. The certificate is a
  small witness structure that re-verifies in microseconds, independent of the search.
- **Grid exhaustion** — search-based. It proves non-existence only within the declared
  box and only under an *empirical* Lipschitz estimate, which is not a bound. The
  `caveat` field says so, and `TestPhysicsErrors::test_unreachable_target_obstructs`
  asserts the caveat is non-empty.

This gap matters: the physically interesting obstructions (is this phase reachable?)
fall in the weak class. **Strengthening search-based obstruction certificates — e.g.
by interval arithmetic over the parameter box rather than sampling — is the highest-
value engineering improvement available to the prototype.**

---

## F-7. Provenance replay works, and is cheap

`Provenance.replay()` reproduces the acceptance verdict from the record alone, with a
SHA3-based chained digest that is tamper-evident (asserted by
`test_provenance_digest_is_tamper_evident`). Nine nodes per compilation; negligible
cost. F-30 passes at prototype scale.

The useful surprise is the **assumption list**. Every compilation surfaces, without
being asked:

```
requirements independent (union bound)
grid resolution 0.1 in each of 2 parameters
COVERAGE formulation: monotone submodular, ln|Alt| approximation.
    NOT the mutual-information formulation, which is not submodular in general.
normal approximation to the estimator's sampling distribution
```

That list is the framework's actual product. It is what an auditor, a regulator, or a
reviewer would want and what no existing inverse-design tool emits.

---

## F-8. The three-valued outcome earns its place immediately

`MEASURE` fires whenever two models' predictions spread wider than the acceptance
margin (`test_model_disagreement_forces_MEASURE`). Without it the compiler would pick
whichever model happened to be listed first and return `COMPILE` with a signed
certificate. The tests confirm it refuses.

---

## F-9. Λ (leverage) is not yet measurable

Implemented values: 0.278 (quantum), 1.25 (CRN), 1.00 (geometry). These are dominated
by each backend's declared `reference_log_volume`, which is a modelling choice, not a
measurement. **Definition 16.2 is operational in form and not yet operational in
practice.** F-19 and F-24 (representation dependence) cannot be run until the
reference measure is derived from something physical rather than declared.

---

## F-10. What the prototype did not test

Honest scope statement. Untested conditions from §17.2:

| Condition | Status |
|---|---|
| R — representation | tested (three substrates, one frontend) |
| Rch — reachability | tested within the declared box only |
| Ver — verifiability | tested |
| Res — resource boundedness | tested (mass budget certificate) |
| MV — model validity | **partially** — validity domains are enforced, calibration is not |
| St — stability | **untested** — no dynamics, no noise, no lifetime |
| Cmp — compositionality | **untested** — single step only |
| Ref — refinement | **untested** — no lowering stage |

So Proposition 17.5's minimal set {R, Rch, Ver, Res} is the part the prototype
exercises, which is consistent but not a coincidence: it is the minimal set precisely
because it is what a measure-everything compiler needs, and that is what was built
first.

---

## F-11. The attractiveness functional's predicted defect appeared immediately

§19.7 of the monograph raised an objection against Definition 19.1 before any code
existed: *A rewards time-in-region and is blind to how that time is distributed, so a
shallow well entered quickly and left often could beat a deep well entered slowly and
held.*

The landscape search confirmed it on the first run. `argmax_λ A` returned the
**lowest** barrier in the parameter box (0.200) together with the **highest** escape
rate in the top-five table (Γ = 0.065, against 0.049 for a deeper well at nearly the
same A). Re-running with Γ ≤ 0.06 as a separate constraint trades a few percent of A
for a materially lower escape rate.

> **Resolution.** A is the correct objective for **terminal-acceptance** targets
> (did the article end up in 𝒦?) and requires an auxiliary continuity constraint for
> **service-life** targets (does it stay there, without cycling?). This is now stated
> in the definition rather than discovered by a user.

The useful part is the order of events: the objection was written down, then the code
reproduced it. That is the falsification program working as intended at the smallest
possible scale.

---

## F-12. Metastability is first-class, demonstrated

With the target prepared in the right well and the **left** well thermodynamically
favoured:

| T_op | A (barrier = 3.0) | A (barrier = 0.5) |
|---|---|---|
| 2 | 1.000 | 0.985 |
| 10 | 1.000 | 0.950 |
| 50 | 1.000 | 0.832 |
| 200 | 1.000 | 0.510 |

Same thermodynamics, different kinetics, and only the second fails. A stays at 1.000
across four decades of operating window for the high barrier. **Corollary 19.3 holds
numerically:** an occupation-fraction objective on a finite window treats diamond,
steel and Floquet phases as legitimate targets without special-casing, which the
gap-based and basin-volume formulations of Drafts 2–4 could not.

---

## F-13. A doc/code inconsistency the prototype caught

The monograph's §19.2 worked-instances table checks each target against a
*target-appropriate* landscape budget. The first version of the demo checked every
target against the double-well backend's own 20-bit budget, and therefore reported
"EXPLICIT ADDRESSING REQUIRED" for a quantum phase that the monograph lists as
landscape-reachable.

The code was right about what it computed and wrong about what it claimed. **The
budget p·b is per substrate and a comparison is only meaningful against the substrate
that would actually be used.** Fixed in the demo; the lesson is that Proposition 19.5
is easy to misapply, and any implementation must carry the substrate identity
alongside the bit count.

The corrected table puts algorithmic tile assembly **inside** the bound (60 bits of
pattern against 144 bits of sequence budget), which is the tile-complexity theorem
restated in information units, and protein sequence and CAD parts far **outside** it
— which is why sequence is specified by synthesis and structure by conditions.

---

## Prototype gap register

| ID | Gap | Source |
|---|---|---|
| **P-1** | The Model Discrimination Engine detects disagreement *between* models but not unanimous disagreement *with the data* — the case where every model is wrong, which is the only architectural route to mechanism discovery | §18.6 |
| **P-2** | No Kramers–Kronig / causality screen. A large class of metamaterial specifications is forbidden by causality and the compiler would happily search for them | §18.3 |
| **P-3** | No effective-realization rewriting library (strain → pseudo-magnetic field, Floquet → synthetic gauge field), so the compiler demands literal regimes | §19.5 |
| **P-4** | Query complexity is not measured. The oracle-compiler framing (§19.6) says the right cost measure is experiments, not FLOPs, and nothing in the prototype counts them | §19.6 |
| **P-5** | A carries no continuity constraint by default | F-11 |
