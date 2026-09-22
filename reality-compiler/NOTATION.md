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
