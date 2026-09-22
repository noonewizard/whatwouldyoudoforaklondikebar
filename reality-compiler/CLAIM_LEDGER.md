# Claim Ledger

Every major claim in the monograph, with its label and where it is argued. Labels
are defined in [`README.md`](README.md). Claims originating in the source proposal
are marked **[P]**; claims originating in this analysis are marked **[A]**.

---

## Foundational claims

| # | Claim | Label | Where | Note |
|---|---|---|---|---|
| 1 **[P]** | Matter cannot be created from nothing; the compiler transforms a supplied substrate | **ESTABLISHED** | §2.1 | Conservation laws |
| 2 **[P]** | Physical synthesis can be formalized as compilation | **STRONGLY SUPPORTED** | §1.2(A) | EDA, CAM, retrosynthesis, DNA origami are working instances |
| 3 **[A]** | The compilation target must be an acceptance region, not a state | **ESTABLISHED** | §4.4 | Three independent arguments: measure-theoretic, metrological, engineering practice |
| 4 **[P]** | `R = (x,p,c,f,q)` is an adequate state representation | **MATHEMATICALLY INCORRECT** | §4.1 | Mixes ontic, derived, and epistemic quantities |
| 5 **[A]** | Properties are functionals of (state × protocol), not state components | **ESTABLISHED** | §4.1, §5.1 | Standard metrological practice |
| 6 **[A]** | Coarse-graining maps are information-destroying and non-invertible | **ESTABLISHED** | §4.3 | Why inverse design is underdetermined |
| 7 **[A]** | Model error is not uniformly bounded and is largest where it matters most | **STRONGLY SUPPORTED** | §4.3 | Out-of-distribution silence in MLIPs, CALPHAD extrapolation, continuum models at interfaces |
| 8 **[A]** | The binding layer of the permission stack is kinetic/control, not thermodynamic | **STRONGLY SUPPORTED** | §3.2, §15.3 | Diamond at STP; catalysis as the central industrial technology |
| 9 **[A]** | Compositionality of physical guarantees requires a sufficient, restoring, contractive intermediate variable | **PLAUSIBLE** as a general rule; **ESTABLISHED** as the explanation for digital electronics | §3.4 | Open Problem 1 |

## Complexity claims

| # | Claim | Label | Where |
|---|---|---|---|
| 10 **[P]** | "Exact physical compilation is NP-hard" | **CONTRADICTED** as a characterization | §16.1 |
| 11 **[A]** | General hybrid-system reachability is undecidable | **ESTABLISHED** | §16.2 |
| 12 **[A]** | Spectral gap determination is undecidable in the thermodynamic limit | **ESTABLISHED** (with scope caveat) | §16.2 |
| 13 **[A]** | Chemical reaction network reachability is Ackermann-complete | **ESTABLISHED** | §16.3 |
| 14 **[A]** | k-local Hamiltonian ground-energy estimation is QMA-complete, including on realistic 2D lattices | **ESTABLISHED** | §16.5 |
| 15 **[A]** | Ising ground state, lattice protein folding, minimum tile set: NP-complete/NP-hard | **ESTABLISHED** | §16.4 |
| 16 **[A]** | Motion planning with movable objects is PSPACE-hard | **ESTABLISHED** | §16.4 |
| 17 **[P]** | A substrate-cut taxonomy PCC_{classical/quantum/analog/biological/hybrid} is meaningful | **UNSUPPORTED** | §16.6 |
| 18 **[A]** | A multidimensional complexity vector is the appropriate formulation | **PLAUSIBLE**, and the correct research direction | §16.7 |
| 19 **[A]** | Exchange relation E1 (simulation ⇄ verification) | **PLAUSIBLE** — test F-2 | §16.7 |
| 20 **[A]** | Exchange relation E2 (fabrication ⇄ verification; design-for-testability) | **STRONGLY SUPPORTED** in VLSI; **SPECULATIVE** elsewhere — test F-4 | §16.7 |
| 21 **[A]** | Exchange relation E3 (control ⇄ search; landscape engineering amortizes over volume) | **PLAUSIBLE** | §16.7 |
| 22 **[A]** | Tractability comes from bounded horizon, coarse-graining, tolerance, structure, amortization, and physical evaluation | **STRONGLY SUPPORTED** | §16.8 |

## Thermodynamic claims

| # | Claim | Label | Where |
|---|---|---|---|
| 23 **[A]** | Landauer bound, finite-time excess work, optimal-transport bound, QSL, TUR are each established | **ESTABLISHED** | §15.2 |
| 24 **[A]** | Thermodynamic bounds sit 2–6 orders below practice for matter, ~20 for computation | **ESTABLISHED** by calculation | §15.3 |
| 25 **[A]** | Therefore thermodynamics is not the binding constraint on physical compilation | **STRONGLY SUPPORTED** | §15.3 |
| 26 **[A]** | Proposition 15.1 (generalized compilation cost with irreversibility fraction χ) | **PLAUSIBLE** (components ESTABLISHED individually) | §15.4 |
| 27 **[A]** | TUR implies tolerance costs dissipation | **ESTABLISHED** for steady-state currents; **SPECULATIVE** generalized to manufacturing tolerance | §15.2, E5 |

## Quantum claims

| # | Claim | Label | Where |
|---|---|---|---|
| 28 **[P]** | "Constraint-Stabilized Quantum Substrate" as stated | **UNSUPPORTED** (underspecified) | §9.1 |
| 29 **[A]** | Decoherence-free subspaces, dynamical decoupling, engineered dissipation are established protection mechanisms | **ESTABLISHED** | §9.1 |
| 30 **[A]** | Topological qubits | **SPECULATIVE** — none demonstrated | §9.1, §9.5 |
| 31 **[A]** | Many-body localization as a protection mechanism | **CONTESTED** | §9.5 |
| 32 **[P]** | Room-temperature *macroscopic* quantum coherence | **CONTRADICTED** | §9.2 |
| 33 **[A]** | Room-temperature coherence in single registers (NV, molecular spins, photons) | **ESTABLISHED** | §9.2, §9.5 |
| 34 **[A]** | Biological systems perform useful quantum computation | **CONTRADICTED** in the strong form | §9.2 |
| 35 **[A]** | Exponential quantum advantage for generic ground-state chemistry | **CONTRADICTED** (as generic claim) | §9.3 |
| 36 **[A]** | Below-threshold surface-code memory has been demonstrated | **ESTABLISHED** (2024, single platform) | §9.4 |
| 37 **[A]** | Quantum sensing is the near-term quantum contribution to the framework | **STRONGLY SUPPORTED** | §9.3 |
| 38 **[A]** | Attractor engineering unifies classical self-assembly and quantum dissipative preparation | **PLAUSIBLE**, and the framework's strongest novel synthesis | §9.7, §8 |

## Landscape-engineering claims

| # | Claim | Label | Where |
|---|---|---|---|
| 39 **[P]** | Landscape engineering is a viable control strategy | **ESTABLISHED** | §8.2(I) |
| 40 **[P]** | Landscape engineering provides generic computational speedup | **CONTRADICTED** in the strong form | §8.2(II), §8.3 Case B |
| 41 **[A]** | Quantum simulation of quantum dynamics gives genuine advantage | **STRONGLY SUPPORTED** | §8.3 Case A |
| 42 **[A]** | Analog computation exceeds Turing computability | **CONTRADICTED** under realistic noise | §8.3 Case D |
| 43 **[A]** | The five-condition test for when landscape engineering pays | **PLAUSIBLE** — proposed here | §8.5 |
| 44 **[A]** | Shortcuts to adiabaticity are not a free lunch (exact counterdiabatic terms are typically nonlocal) | **ESTABLISHED** | §8.4 |

## Assembly and verification claims

| # | Claim | Label | Where |
|---|---|---|---|
| 45 **[P]** | A universal assembler could exist | **CONTRADICTED** by throughput arithmetic | §10.1 |
| 46 **[A]** | Serial atomic assembly cannot produce bulk matter at any plausible rate | **ESTABLISHED** by arithmetic | §10.1 |
| 47 **[A]** | The four compilability criteria for a substrate | **PLAUSIBLE**, derived from the EDA precedent | §10.3 |
| 48 **[A]** | Substrate discipline matters more than algorithmic power | **STRONGLY SUPPORTED** | §1.2(B), §10.3 |
| 49 **[A]** | Full quantum state tomography requires Θ(d²/ε²) copies | **ESTABLISHED** | §11.2 |
| 50 **[A]** | Classical shadows give O(log m/ε²) for m local observables | **ESTABLISHED** | §11.2 |
| 51 **[A]** | Proposition 11.1: verification cost is linear in specified properties, log in confidence, independent of object complexity | **ESTABLISHED** under its hypotheses; hypotheses may fail in practice | §11.2 |
| 52 **[A]** | Over-specification is quantifiably expensive | **ESTABLISHED** (corollary of 51) | §11.2 |
| 53 **[A]** | Sampling cannot certify unspecified properties, rare localized defects, or time-dependent failure | **ESTABLISHED** | §11.3 |
| 54 **[A]** | Damage-tolerance doctrine (certify against the worst article consistent with the measurements) is the correct response | **ESTABLISHED** as aerospace practice | §11.3 |
| 55 **[A]** | Co-design so a cheap functional measurement is a sufficient statistic for an expensive structural spec | **PLAUSIBLE**, framework's most valuable concrete technique | §11.6 |

## Provenance and security claims

| # | Claim | Label | Where |
|---|---|---|---|
| 56 **[P]** | Cryptography protects matter from entropy | **CONTRADICTED** | §12.1 |
| 57 **[A]** | Cryptography binds records; record↔matter binding rests on a physical unclonability assumption | **ESTABLISHED** | §12.1, §12.3 |
| 58 **[A]** | The sensor-to-signature gap is irreducible | **ESTABLISHED** | §12.1 |
| 59 **[A]** | A linear hash chain is the wrong structure; use a causal Merkle DAG | **STRONGLY SUPPORTED** | §12.2 |
| 60 **[A]** | Hash-based signatures (SLH-DSA) are the conservative choice for decades-long attestations | **PLAUSIBLE** (engineering judgment on established primitives) | §12.4 |
| 61 **[A]** | ZK conformance proofs solve a real commercial problem (proving conformance without revealing process) | **PLAUSIBLE**; proving cost over sensor time-series is open | §12.4 |
| 62 **[A]** | Reuse in-toto/SLSA/Sigstore/CT rather than inventing a parallel stack | **STRONGLY SUPPORTED** | §12.4, §13.4 |
| 63 **[A]** | Do not put physical execution behind consensus | **STRONGLY SUPPORTED** | §13.4 |
| 64 **[A]** | "Rollback" for physical actions must mean compensating action plus attested disposition | **ESTABLISHED** | §13.3 |
| 65 **[A]** | Process substitution detection is an open empirical question, not an assumption | **SPECULATIVE** until F-5 | §14.2 T8, §25 F-5 |
| 66 **[A]** | An organization cannot securely certify its own output | **ESTABLISHED** by precedent across every high-consequence industry | §14.3, §23.2 |
| 67 **[A]** | Dual-use screening must occur at the substrate chokepoint and cover intermediates | **PLAUSIBLE**, consistent with nucleic-acid-synthesis screening consensus | §14.4 |

## Universality claims

| # | Claim | Label | Where |
|---|---|---|---|
| 68 **[A]** | No universal representation | **STRONGLY SUPPORTED** | §17.2 |
| 69 **[A]** | A universal *interface* (language + evidence calculus + attestation) is achievable | **PLAUSIBLE**, by the LLVM precedent | §17.2 |
| 70 **[A]** | No universal optimizer | **ESTABLISHED** | §17.3 |
| 71 **[A]** | Universal quantum control holds for bilinear systems via LARC; universal Hamiltonian families exist | **ESTABLISHED** | §17.4 |
| 72 **[A]** | Control universality does not transfer to bulk matter | **STRONGLY SUPPORTED** | §17.4 |
| 73 **[A]** | No universal fabrication; no universal verification | **STRONGLY SUPPORTED** | §17.5–17.6 |

## Domain claims

| # | Claim | Label | Where |
|---|---|---|---|
| 74 **[P]** | Self-assembly is computation | **ESTABLISHED** | §18.1 |
| 75 **[A]** | Programmed assembly is exponentially more compact than explicit assembly (Θ(log N/log log N) tiles for N×N) | **ESTABLISHED** | §18.1 |
| 76 **[A]** | The history of algorithmic self-assembly is the history of physical error correction | **STRONGLY SUPPORTED** | §18.2 |
| 77 **[A]** | Hierarchical error compounding is the blocker for scaling self-assembly | **STRONGLY SUPPORTED** | §18.3 |
| 78 **[P]** | Compiling a specification to a target biological morphology | **SPECULATIVE** for arbitrary targets; **PLAUSIBLE** for constrained targets within ~10 years | §19.1, §19.4 |
| 79 **[A]** | Protein design is the working biological reality compiler today | **STRONGLY SUPPORTED** | §19.2 |
| 80 **[A]** | Absence of a forward model of tissue geometry is the primary morphogenesis bottleneck | **STRONGLY SUPPORTED** | §19.3 |
| 81 **[A]** | Synthesizability, not stability, is the materials-discovery bottleneck | **STRONGLY SUPPORTED** | §20.3 |
| 82 **[A]** | The documented failure point of autonomous laboratories was characterization, not synthesis | **STRONGLY SUPPORTED** (published critique of a flagship result) | §20.3 |
| 83 **[A]** | Nanophotonic inverse design is a working reality compiler | **ESTABLISHED** | §20.3 |

## Economic and organizational claims

| # | Claim | Label | Where |
|---|---|---|---|
| 84 **[P]** | Quintillion-dollar valuation | **CONTRADICTED** (≈476× the capitalized output of civilization) | §22.1 |
| 85 **[A]** | The EDA capture ratio (~2.5%) is the right anchor | **PLAUSIBLE**; listed in Final Answer F as one of the three claims most likely wrong | §22.2 |
| 86 **[A]** | The verification market (~$250B TIC) is ~15× the design-tool market | **ESTABLISHED** (market data, approximate) | §22.2 |
| 87 **[A]** | The framework's largest near-term revenue is verification, not design | **PLAUSIBLE**, follows from 86 | §22.2 |
| 88 **[A]** | Best case ~$100–250B enterprise value; extraordinary case ~$1T | **PLAUSIBLE** scenario modeling, not forecast | §22.5 |
| 89 **[A]** | Qualification cost, not technical capability, is the dominant adoption barrier | **STRONGLY SUPPORTED** | §22.6 |
| 90 **[A]** | Six separate companies is the wrong structure; one operating company + open standard + independent verifier | **PLAUSIBLE** (engineering/organizational judgment with strong precedent) | §23 |
| 91 **[A]** | A proprietary specification language will not be adopted by regulated primes | **PLAUSIBLE**, by analogy to LLVM/RISC-V adoption patterns | §23.3 |

## IP claims

| # | Claim | Label | Where |
|---|---|---|---|
| 92 **[A]** | The theory is unpatentable and should be published | **ESTABLISHED** (subject-matter exclusions; not legal advice) | §24.1 |
| 93 **[A]** | Control methods bound to apparatus, metrology, processes, compositions, and fingerprinting are the plausible patent territory | **PLAUSIBLE** (not legal advice) | §24.2 |
| 94 **[A]** | The compiler's *outputs* may be worth more in IP terms than the compiler | **PLAUSIBLE** | §24.3 |
| 95 **[A]** | AI inventorship remains unsettled | **ESTABLISHED** as a statement about legal uncertainty | §24.3 |

---

# Draft 2 Addendum — Quantum Matter, Emergence, and the Modality Corrections

Claims from Parts XI–XIII. These use the modality vocabulary (FORBIDDEN / ALLOWED /
DEMONSTRATED / PLAUSIBLE / SPECULATIVE / UNSUPPORTED) rather than Draft 1's labels,
per §29's preamble. **[C]** marks a claim that corrects Draft 1.

## Corrections to Draft 1

| # | Claim | Label | Where |
|---|---|---|---|
| 96 **[C]** | Serial atomic assembly cannot make bulk matter | **True, and does not generalize.** The arithmetic bounds serial assembly only | §29.2, E1 |
| 97 **[C]** | Massively parallel programmable positional assembly at kg/day | **DEMONSTRATED** — ribosomal synthesis, ~10¹⁹ parallel sites | §29.2 |
| 98 **[C]** | A general-palette artificial parallel molecular assembler | **ALLOWED / INACCESSIBLE** — no prohibition; undemonstrated | §29.2, F-15 |
| 99 **[C]** | Positioned mechanochemical event rate ceiling ~10¹²–10¹³ /s/site | **FORBIDDEN above it** (vibrational timescale); permissive — biology runs 11 orders below | §29.2 |
| 100 **[C]** | Room-temperature macroscopic quantum coherence | Splits four ways: ODLRO at 300 K **DEMONSTRATED**; macroscopic superposition at 300 K **ALLOWED / INACCESSIBLE**; embedded RT processor **ALLOWED / INACCESSIBLE**; biological quantum computation **CONTRADICTED** | §29.8, E2 |
| 101 **[C]** | Matter-wave interference mass frontier | **DEMONSTRATED** at >7,000 atoms / >170 kDa; levitated spheres ~10⁸ amu ground-state cooled. The frontier moves | §29.8 |
| 102 **[C]** | Decoherence is a rate set by engineering variables, not a law | **ESTABLISHED** as a statement about the structure of decoherence theory | §29.8, F-16 |
| 103 **[C]** | Topological qubits are SPECULATIVE | **True for Majorana hardware qubits only.** Engineered Abelian and non-Abelian topological order: **DEMONSTRATED** with braiding verified (2023–2024) | §29.6, E4 |
| 104 **[C]** | Landscape engineering gives no computational advantage | **Over-broad.** True for glassy optimization; **exponential advantage** for many-body state preparation (conditional on BQP ⊄ BPP); **proven depth separation** for adaptive preparation | §32.3, E5 |
| 105 **[C]** | No universal physical compiler | Holds for bulk matter. **Universal Hamiltonian families are proved to exist**, so a universal compiler exists for spin-Hamiltonian physics | §35.10, E6 |
| 106 **[C]** | Verification is uniformly costly | **Topological targets are the cheapest verification problem in physics** — the estimand is an integer | Prop. 29.1, E7 |
| 107 **[C]** | Undecidability/QMA results bound physical achievability | **No.** They bound algorithms answering questions about arbitrary instances. Matter is not constrained by our ability to decide questions about it | §35.2, E9 |
| 108 **[C]** | The environment envelope is a constraint | **It is a costed free variable** | §34, E10 |
| 109 **[C]** | The action algebra is an instruction set of placements | **It is an algebra of conditions**; placement is the degenerate case | §33.4, E8 |

## Physical boundaries affirmed (theorem-grade)

| # | Claim | Label | Where |
|---|---|---|---|
| 110 | Constant-depth *unitary* local preparation of long-range entanglement | **FORBIDDEN** (Lieb–Robinson) | §29.5 |
| 111 | Adiabatic passage through a critical point in finite time | **FORBIDDEN**; Kibble–Zurek gives the defect scaling | §29.5 |
| 112 | Continuous-symmetry breaking at finite T in d ≤ 2, short-range | **FORBIDDEN** (Mermin–Wagner); finite-size systems evade the asymptotics | §29.5 |
| 113 | Trivial gapped symmetric ground state at half-odd-integer spin per cell | **FORBIDDEN** (Lieb–Schultz–Mattis) — a compile-time static check | §29.5 |
| 114 | Single chiral fermion on a local Hermitian lattice | **FORBIDDEN** (Nielsen–Ninomiya) | §29.5 |
| 115 | Passive 2D topological quantum memory at finite T | **FORBIDDEN** (thermal instability of the 2D toric code). 3D self-correction **OPEN**; 4D proved stable | §29.5 |
| 116 | Universal transversal gate set | **FORBIDDEN** (Eastin–Knill) | §29.5 |
| 117 | Copying a prepared state for verification | **FORBIDDEN** (no-cloning) | §29.5 |

## The escapes

| # | Claim | Label | Where |
|---|---|---|---|
| 118 | Measurement + feedforward prepares long-range-entangled states in constant depth, beating the Lieb–Robinson floor | **DEMONSTRATED** — D₄ non-Abelian order on 27 trapped-ion qubits, >98.4% per-site fidelity, non-Abelian braiding detected by anyon interferometry | §29.5 |
| 119 | Dissipative preparation is gap-agnostic and self-correcting | **DEMONSTRATED** | §29.5 |
| 120 | Effective-field engineering can exceed brute-force extremes | **DEMONSTRATED** — strain-induced pseudo-magnetic fields >300 T, above any laboratory magnet | §34.2 |
| 121 | Nonequilibrium phases with no equilibrium counterpart are compilable | **DEMONSTRATED** — discrete time-crystalline order on multiple platforms | §29.6 |

## New propositions

| # | Claim | Label | Where |
|---|---|---|---|
| 122 | Quantum matter escapes the Avogadro problem: a phase is a correlation structure, not a quantity | **ESTABLISHED** by the definition of a phase | §29.1 |
| 123 | Quantum matter beats structural alloys on 3 of Draft 1's 4 compilability criteria | **PLAUSIBLE**, argued from the criteria | §29.3 |
| 124 | Prop. 29.1 — topological targets have O(1) verification margin because the estimand is an integer | **ESTABLISHED**; quantized Hall conductance is measured to 1 part in 10⁹ and defines the SI ohm | §29.7 |
| 125 | Prop. 31.1 — κ = K(H*)/K(target) predicts compilability; κ ≈ 1 means no emergence leverage | **PLAUSIBLE** — test F-14 | §31.1 |
| 126 | Prop. 32.1 — advantage class is determined by relaxation time alone | **PLAUSIBLE**, with each row independently supported | §32.3 |
| 127 | Controlled emergence is the general primitive; instruction execution is its degenerate case | **PLAUSIBLE**, argued from scaling, containment, and the industrial record | §33.2 |
| 128 | Specification layers stack deterministically upward and are degenerate downward; independent control exists only at the nonequilibrium handles | **ESTABLISHED** (Born–Oppenheimer and the definition of a phase) | §30 |
| 129 | Extreme regimes are cheap at small volume and unavailable at large volume, so quantum-matter and bulk back ends have opposite environmental economics | **ESTABLISHED** by the cost table | §34.3 |
| 130 | Ambient-pressure room-temperature superconductivity | **ALLOWED / undiscovered** — no known theorem bounds Tc; the field's failure mode has been verification integrity, not physics | §29.6, §29.9 Target C |
| 131 | Track B (quantum matter) should be funded before Track A | **PLAUSIBLE**, argued against Draft 1's own criteria | §37 |

---

# Draft 3 Addendum — Physical State Compilation and the Theorem Program

**[R]** marks a claim that revises the framing rather than correcting a fact.

## Framing

| # | Claim | Label | Where |
|---|---|---|---|
| 132 **[R]** | The hierarchy classical ≫ quantum was an artifact of where compilers exist today, not of the physics | **Bias, removed** | Part I revision note |
| 133 **[R]** | No level of the macro→collective hierarchy is privileged; the hardest and most valuable targets are not necessarily classical | **PLAUSIBLE**, argued from §29.3 | §1.2 |
| 134 | Three target classes (A material synthesis, B state preparation, C quantum-matter synthesis) are not interchangeable and must be typed separately | **ESTABLISHED** by their different invariants and reachable sets | §2.2 |
| 135 | Target class C is the deepest form of physical compilation: the compiler emits the law the system obeys | **PLAUSIBLE**, argued structurally | §2.2 |
| 136 | H* is designed, not discovered — the compiler solves an inverse problem over a low-dimensional Λ_Ω | **ESTABLISHED** for the demonstrated platforms | §2.4, §31.3 |
| 137 | Landscape Compilation is a core architectural object with five dynamical conditions (attraction, robustness, kinetics, selectivity, realizability) | **PLAUSIBLE** — proposed here | Def. 2.2 |
| 138 | Physical computational leverage ℒ = I_verified/(K(P*)+C_ver); ℒ ≫ 1 iff dynamics carry the computational load | **PLAUSIBLE** — proposed here | Def. 2.3, §42.2 |
| 139 | Optimizing only 𝔍 systematically under-invests in the substrates where the paradigm's leverage lives | **PLAUSIBLE** | §42.2 |

## Physics

| # | Claim | Label | Where |
|---|---|---|---|
| 140 | **Proposition 3.1** — unitary control preserves the spectrum of ρ; cooling and purification are dissipation/measurement problems, not control problems | **ESTABLISHED** (elementary), and belongs in the compile-time screen | §3.5 |
| 141 | This explains why entropy, not control fidelity, binds cold-atom many-body preparation | **STRONGLY SUPPORTED** | §3.5, §29.9 |
| 142 | Quantum error correction is the only known *constructed* instance of the digital abstraction outside classical electronics | **ESTABLISHED**; the existence proof for Open Problem 1 | §3.6 |
| 143 | Specification layers are deterministic upward and degenerate downward; independent control exists only at the nonequilibrium handles | **ESTABLISHED** | §4.2, §30 |

## The theorem program

| # | Claim | Label | Where |
|---|---|---|---|
| 144 | **Q1 fails strictly**: phase membership is not a function of finite-size expectation values, since a phase is an equivalence class under finite-depth local unitaries | **ESTABLISHED** | §39.2 |
| 145 | **Q1′ holds**: a phase-target is a nested family of acceptance regions plus a finite-size-scaling hypothesis against a named alternative set | **ESTABLISHED** as a description of what condensed-matter practice already is | §39.2 |
| 146 | The acceptance-region reformulation is *forced* for quantum matter, not merely convenient | **ESTABLISHED**, and independent corroboration of Draft 1's central correction | §4.4, §39.2 |
| 147 | A phase specification without an alternative hypothesis set is ill-formed and should be refused | **PLAUSIBLE** — proposed; supported by the retraction record | §39.2, §41.4 |
| 148 | **Q2 holds constructively** for tensor-network targets (parent Hamiltonians) and via encoding for spin Hamiltonians (universal families) | **ESTABLISHED** | §39.3 |
| 149 | **Q2 fails generically**: a generic state has no local parent Hamiltonian (poly parameters vs 2^N dimensions) | **ESTABLISHED** by counting | §39.3 |
| 150 | **Q3 holds** on three sub-classes: gapped adiabatic paths, measurement-preparable states (constant depth), rapidly mixing engineered Liouvillians | **ESTABLISHED** | §39.4 |
| 151 | **Q3 fails** in the worst case (QMA-hardness of arbitrary local-Hamiltonian ground states) | **ESTABLISHED** | §39.4 |
| 152 | If quantum PCP holds, approximate compilation is no easier than exact in the worst case, and no general approximation guarantee can exist | **OPEN** — the single external result most likely to change this program's foundations | §39.5 |
| 153 | **Q4 holds** on structured targets; quantized/topological estimands are the cheapest | **ESTABLISHED** | §39.6 |
| 154 | Verifying a quantum simulator in the regime where classical simulation fails | **OPEN** | §39.6 |
| 155 | **Proposition Q is FALSE in general, TRUE on CQM(Ω)** | **ESTABLISHED** (disproof by counting; positive class characterized) | §39.7–39.8 |
| 156 | The general disproof applies equally to classical matter — almost no arrangement of 10²³ atoms is specifiable | **ESTABLISHED**; shows the restriction is not a quantum-specific weakness | §39.8 |
| 157 | **Compilability of quantum matter rests on the area law** — proved in 1D, conjectured in higher d | **ESTABLISHED** as a derivation; the area law itself is **proved in 1D / OPEN in general** | §39.9 |
| 158 | The framework's scope is co-extensive with how broadly the area law holds | **PLAUSIBLE**, follows from 157 | §39.9, §43.1 |

## The IR

| # | Claim | Label | Where |
|---|---|---|---|
| 159 | A layered Quantum Physical IR (L0 spec → L1 algebraic → L2 encoded → L3 native → L4 signals) is well-defined, and the L1→L2 passes are real named transformations with quantified overhead | **ESTABLISHED** — the passes all exist | §40.2–40.3 |
| 160 | Portability is cheap within a substrate class and expensive across classes, chiefly from perturbative-gadget coupling-ratio blowup | **STRONGLY SUPPORTED** | §40.3 |
| 161 | The LLVM analogy holds architecturally and fails on cost uniformity: some cross-class compilations are infeasible, not merely slow | **PLAUSIBLE** | §40.3 |
| 162 | Moiré/solid-state is a structurally different back end — λ is fixed at fabrication, making it the bridge between target classes C and A, and the route to engineered quantum matter outside a cryostat | **PLAUSIBLE**, and strategically important | §40.4 |
| 163 | Universal: L0 and L1; universal as transformations but not as costs: L1→L2; not universal: L2→L3 | **ESTABLISHED** | §40.5 |

## Verification

| # | Claim | Label | Where |
|---|---|---|---|
| 164 | The minimum sufficient measurement set problem is a test-cover problem: NP-hard, submodular, greedily (1−1/e)-approximable | **ESTABLISHED** | §41.1 |
| 165 | The cost-weighted cover gives a computable preference ordering over witnesses, favoring quantized observables | **PLAUSIBLE** — test F-13 | §41.3 |
| 166 | Minimality is relative to the declared alternative set, and unconsidered alternatives are the mechanism behind essentially every retracted phase claim | **STRONGLY SUPPORTED** by the retraction record | §41.4 |
| 167 | The compiler must optimize preparability + stability + measurability + verifiability jointly; fidelity alone is one of four | **PLAUSIBLE** — proposed here | §3.3 |
| 168 | S_stability is not optional for target classes B and C; its absence is the most common reason optimal control sequences fail in the laboratory | **STRONGLY SUPPORTED** | §42.1 |

---

# Draft 4 Addendum — Parts III–IV

The full Parts III–IV claim audit, with assumptions, evidence, and falsifiers for each
claim, is in [`15-audits-parts-III-IV.md`](15-audits-parts-III-IV.md) §A. Recorded here
are only the **withdrawals and retractions**, since those correct earlier ledger rows.

| # | Earlier claim | Ledger row corrected | Status in Draft 4 |
|---|---|---|---|
| W-1 | "Even verifying condition (a) [that the target is the ground state] is QMA-hard" (Draft 1 §8.6) | — | **WITHDRAWN as imprecise.** The exact decision problem is APX-SIM — estimating a local observable on the ground state — which is **P^QMA[log]-complete**, a class believed to strictly contain QMA. Draft 1 understated the hardness while appearing to overstate it. §8.22 |
| W-2 | "The minimum sufficient measurement set is a test-cover problem: NP-hard, submodular, greedily (1−1/e)-approximable" (Drafts 1–3, §41.1, §16.4) | 164, 165 | **PARTIALLY WITHDRAWN.** True for the **coverage** formulation (monotone submodular, guarantee holds). **False in general for the information formulation** — mutual information over measurement sets is submodular only under conditional independence, and otherwise weakly submodular with a (1−e^{−γ}) guarantee that degrades with γ. §9.8.3 |
| W-3 | "The criterion is the relaxation time, and nothing else" (Draft 2 §32.3) | 126 | **RETRACTED.** Sub-exponential τ_relax is neither necessary (metastable targets — steel, glass, diamond, every Floquet phase — are reached by *avoiding* relaxation) nor sufficient (fast relaxation to the wrong attractor, or to an unverifiable state, fails). Replaced by the total-verified-cost criterion C_PSE/T_PSE with amortized setup. §8.7 |
| W-4 | "Quantum error correction is the only known constructed instance of the digital abstraction outside classical electronics" (Draft 3 §3.6) | 142 | **SOFTENED.** No comprehensive literature review supports "only." Classical error correction in digital logic, kinetic proofreading and DNA mismatch repair in biology, and multistable mechanical metamaterials are candidate instances. QEC remains the clearest *deliberately engineered* quantum instance |
| W-5 | Attractor/landscape compilation presented without prior art | 137 | **PRIOR ART STATED.** Reservoir engineering (Poyatos–Cirac–Zoller 1996), inverse statistical mechanics for targeted self-assembly (Torquato), dissipative state preparation, control-Lyapunov design, and "energy landscape engineering" as established terminology. The contribution is the unification and the six-axis figure of merit, not the mechanism |
| W-6 | Physical Design-for-Verifiability presented as novel | 55, 20 | **PRIOR ART STATED.** Design for testability (scan chains, BIST, ~1973) is ~50-year prior art in one substrate; design for inspection and design for metrology are established. New: verification cost inside an automated cross-substrate synthesis objective, and the quantum-matter application |
| W-7 | VERIFY/ASSUME/DERIVE presented as a new epistemic type system | — | **PRIOR ART STATED.** The GUM's Type A / Type B uncertainty distinction is direct prior art and is an international metrology standard. Citing it strengthens the contribution |
| W-8 | δ_total ≤ δ₁ + δ₂ + ε with ε unspecified (Draft 1 Desideratum 3.1) | 9 | **SHARPENED.** ε = 0 when the downstream guarantee is **uniform** on 𝒦₁ (Theorem D.1, elementary union bound). ε ≤ L·diam(𝒦₁) is the price of nominal-point validation. **The binding condition is not Lipschitz continuity but sufficiency**: P₂'s outcome must factor through Φ₁ as a Markov kernel, or no bound follows at all. §D |
