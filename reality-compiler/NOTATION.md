# Proposed Notation System

A single consistent notation for Synthesis Complexity. Conventions are stated first
so that the symbol table is unambiguous.

## Conventions

| Convention | Rule |
|---|---|
| **Ontic quantities** | Latin lowercase: σ (state), x (configuration), u (control), y (measurement outcome) |
| **Epistemic quantities** | Fraktur: 𝔟 (belief), 𝔐 (model bundle) |
| **Spaces, algebras, sets** | Script capitals: Σ, 𝒜, 𝒯, ℳ, 𝒞, ℰ, 𝒦, 𝒪, 𝒫 |
| **Structured declarations** | Angle brackets: Ω = ⟨…⟩, S = ⟨…⟩ |
| **Acceptance region in state space** | Always square brackets: [S] |
| **Confidences** | δ = bound on false accept; β = bound on false reject. Never a bare "confidence." |
| **Levels of description** | Subscript ℓ ∈ {quantum, micro, meso, macro}; Σ_ℓ, Φ_ℓ |
| **Denotation** | ⟦·⟧ |
| **Probabilistic satisfaction** | ⊨_δ |

## Symbol table

### Substrate

| Symbol | Type | Meaning |
|---|---|---|
| Ω | ⟨Σ,𝒜,𝒯,ℳ,𝒞,ℰ,𝔐⟩ | substrate declaration |
| Σ_ℓ | set | ontic state space at level ℓ |
| 𝒜 | algebra | action algebra: primitive control actions with preconditions, effects, durations, resource draws |
| 𝒯 | function space | admissible control signals u : [0,T] → 𝒰 with bandwidth/amplitude/slew limits |
| ℳ | algebra | measurement algebra: operations with estimators, precision, bias, cost, destructiveness |
| 𝒞 | vector | inventory: species counts, energy, time budgets |
| ℰ | record | environment envelope (T, P, atmosphere, vibration) with tolerances |
| 𝔐 | set of models | model bundle; each m ∈ 𝔐 carries a validity-domain predicate dom(m) and error bound err(m) |

### Specification

| Symbol | Type | Meaning |
|---|---|---|
| S | ⟨𝒪,𝒦,δ,β⟩ | specification |
| 𝒪 = (O₁…O_k) | estimands | each O_i = (property functional, protocol 𝒫_i) |
| 𝒦 ⊆ ℝᵏ | measurable set | acceptance region |
| 𝒫 | protocol | measurement conditions defining an estimand |
| Φ_ℓ : Σ_ℓ × 𝒫 → ℝᵏ | map | property map |
| [S] := Φ(·,𝒫)⁻¹(𝒦) ⊆ Σ | set | **spec cell** |
| Ξ | map | evidence obligation: requirement ↦ (mode, δ), mode ∈ {VERIFY, ASSUME, DERIVE} |
| ⟦S⟧ = (𝒦, Φ, Ξ) | triple | denotation of a specification |

### Process and epistemics

| Symbol | Type | Meaning |
|---|---|---|
| 𝔟 ∈ Δ(Σ) | measure | epistemic state (belief) |
| 𝔟₀ | measure | initial belief over feedstock state |
| P | policy | process program: (π, φ_safe, φ_abort, Ψ_inline) |
| π | map | feedback law (history, measurements) ↦ action |
| Ψ | plan | measurement and decision plan over ℳ |
| A : Y* → {PASS, FAIL, INCONCLUSIVE} | map | acceptance rule |
| Π | record | provenance/attestation record (a Merkle DAG of events E_i) |
| E_i | event | ⟨id, type, parents, article, subject, actor, context, policy, sig⟩ |

### Costs, bounds, complexity

| Symbol | Meaning |
|---|---|
| 𝔍[P] | compilation objective (§7.4): robust expected cost + CVaR risk term |
| CVaR_α | conditional value at risk at level α |
| W_min | thermodynamic lower bound on work (§15.2) |
| ΔF_state | reversible free-energy difference |
| W_ex(τ) | finite-time excess work; W_ex ≈ 𝓛²/(2τ) |
| 𝓛 | thermodynamic length of a protocol path |
| I(S) | information content of the spec cell: log₂\|Σ\| − log₂\|[S]\| |
| χ ∈ [0,1] | irreversibility fraction (§15.4) |
| Σ_ent | entropy production (subscripted to avoid clash with the state space Σ) |
| 𝒞(𝔟₀,S,Ω,δ) | **compilation complexity vector** = (C_search, C_sim, C_ctrl, C_ver, C_fab, C_E, C_τ, C_I) |
| E1…E6 | exchange relations among complexity components (§16.7) |

### Relations

| Notation | Meaning |
|---|---|
| σ ⊨ S | Φ(σ, 𝒫) ∈ 𝒦 (deterministic satisfaction; used only for idealized reasoning) |
| P ⊨_δ S | Pr[A(Y)=PASS ∧ Φ(σ_T) ∉ 𝒦] ≤ δ (the operational relation) |
| ℒ, ℛ ⊨ S | lot ℒ with record ℛ satisfies S (§5.7) |
| P₂ ∘ P₁ | sequential composition of process programs |
| 𝔟 ⤳_P 𝔟′ | belief transition induced by program P |
| Reach(𝔟₀, Ω, T) ⊆ Δ(Σ) | reachable belief set |

## A note on symbol collision

`Σ` is used for the state space and is conventionally also entropy production. This
document uses `Σ` for the state space and `Σ_ent` for entropy production. `δ` is the
false-accept bound throughout and is never a variation or a Dirac delta. `𝒞` is the
inventory in a substrate declaration and `𝒞(·)` with arguments is the complexity
vector; the arity disambiguates, but in a paper they should be renamed (suggested:
`Inv` for inventory).

---

# Draft 2 Addendum

## Revised and added symbols

| Symbol | Meaning | Note |
|---|---|---|
| **𝒜** | algebra of **conditions**, not placements | **Revised.** Primitives are "impose this boundary," "set this field," "couple to this reservoir," "hold this drive." Explicit placement is the degenerate element where one condition affects one degree of freedom. See §33.4 |
| ℰ | environment envelope, now a **free variable to be costed**, not a constraint | **Revised.** §34 |
| Λ_Ω | the realizable control-parameter space of a substrate: {λ : H(λ), L_k(λ) physically available} | New. The search space for quantum-matter compilation (§29.4) |
| H* | the compiled **effective Hamiltonian** — for quantum targets, the compiler's primary output | New. §29.4 |
| L_k(λ) | engineered Lindblad (dissipation) operators under control λ | New |
| U* | control sequence / field / pulse / interaction program realizing λ(t) | New |
| κ(S) = K(H*) / K(target) | **compilation compression ratio** | New. κ ≪ 1 ⟹ emergence leverage; κ ≈ 1 ⟹ explicit placement is correct. §31.1 |
| K(·) | description length in the declared representation (Kolmogorov-flavored, used operationally as shortest parameter list) | New |
| τ_relax | physical convergence time of engineered dynamics to the target basin | New. **The classifying variable for advantage class.** §32.3 |
| Δ | Hamiltonian spectral gap; **Δ_L** the Liouvillian gap for dissipative preparation | New. §32.5 |
| ξ | correlation length; sets the Lieb–Robinson preparation-depth floor ξ/v | New |
| ν, z | correlation-length and dynamic critical exponents (Kibble–Zurek) | New |

## Revised compiler signature

```
Compile( S, Ω, ℰ )  →  ( P*, H*, U*, Ψ, Π )
```

with H* primary for quantum targets and P* primary for bulk-matter targets. The two
back ends have opposite environmental cost models (§34.3) and opposite effort
allocation: bulk targets spend the budget on search, quantum targets on preparation
protocol synthesis (§31.3).

## Modality labels

Parts XI–XIII use FORBIDDEN / ALLOWED-INACCESSIBLE / DEMONSTRATED / PLAUSIBLE /
SPECULATIVE / UNSUPPORTED-CONTRADICTED alongside Draft 1's evidence labels. The two
vocabularies answer different questions — *how strong is the evidence* versus *what
imposes the boundary* — and both should be carried. A claim can be
**ESTABLISHED** (the evidence is solid) that something is **ALLOWED / INACCESSIBLE**
(no prohibition, no capability).
