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
