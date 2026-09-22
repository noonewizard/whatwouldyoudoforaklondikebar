# Part IV — The Quantum Substrate

Section 9

---

## 9. Quantum Substrate

### 9.1 The proposed object, and why it cannot be assessed as stated

The proposal introduces a **Constraint-Stabilized Quantum Substrate**: "localized
quantum coherence capable of supporting useful computation or state engineering
under comparatively accessible environmental conditions."

As written this is **UNSUPPORTED**, not because it is refuted but because it is not
a physical proposal. It names no degree of freedom, no interaction, no protection
mechanism, and no figure of merit. "Constraint-stabilized" could mean four
physically distinct things, and they have different prospects:

| Reading | Mechanism | Status |
|---|---|---|
| **(a) Symmetry-protected** | Decoherence-free subspaces: states in a subspace on which a collective noise operator acts trivially | **ESTABLISHED**; demonstrated in ions, photons, NMR, superconducting circuits. Protects against *correlated* noise only. |
| **(b) Gap-protected** | An energy gap suppresses transitions out of the code space; topological order gives exponential suppression in system size | **Theoretically ESTABLISHED; experimentally unrealized for topological qubits.** Non-Abelian anyon qubits remain undemonstrated; the Majorana-based claims of the last decade have a poor replication record, including a high-profile retraction. Treat as **SPECULATIVE hardware**. |
| **(c) Dissipation-protected** | Engineered coupling to a reservoir whose steady state *is* the code space (autonomous error correction; two-photon dissipation stabilizing cat states) | **STRONGLY SUPPORTED and the most promising reading.** Bit-flip lifetimes in dissipatively stabilized cat qubits have been extended by orders of magnitude; the mechanism is exactly "constraints stabilize the state." Still cryogenic. |
| **(d) Dynamically protected** | Dynamical decoupling / continuous driving averaging out low-frequency noise | **ESTABLISHED**; routine. Buys 10–1000× on T2 against 1/f noise. Does not address Markovian decay (T1). |

**Recommendation: retire the phrase and replace it with an explicit choice among
(a)–(d), or a named combination.** The framework loses nothing and gains
falsifiability. If the intended claim was (c) — dissipative stabilization — that is
a good and defensible bet, and it should be said.

### 9.2 The claim that must be refused

**"Room-temperature macroscopic quantum coherence is achievable."**
**CONTRADICTED** in the macroscopic form. The scaling argument is elementary and
robust: decoherence rates from coupling to a thermal environment grow with the
number of coupled modes and with temperature, and for a superposition of states
separated by a macroscopic distance or mass the decoherence time falls off faster
than any engineering margin can compensate. No mechanism is known that suppresses
environmental coupling for a macroscopic object at 300 K, and the burden of proof is
on any claim that one exists.

Two adjacent claims that are often bundled with it, and must be separated:

- **"Room-temperature quantum coherence exists in useful systems."** **ESTABLISHED
  and important.** Nitrogen-vacancy centers in diamond maintain electron-spin
  coherence for ~0.1–2 ms at 300 K under dynamical decoupling, and coupled ¹³C
  nuclear spins reach seconds. Molecular nuclear spins in liquids reach seconds.
  Photons do not decohere in flight at all. The room-temperature resource is real —
  it is just *small*, *few-qubit*, and *hard to scale*.
- **"Biological systems perform useful quantum computation at room temperature."**
  **CONTRADICTED in the strong form.** The long-lived oscillations observed in
  photosynthetic light-harvesting complexes, once read as long-lived electronic
  coherence, are now generally attributed to vibrational (vibronic) coherence with
  lifetimes of tens to a few hundred femtoseconds at physiological temperature —
  far too short to support computation. Avian magnetoreception via a radical-pair
  mechanism remains a live and interesting hypothesis with real supporting evidence,
  but it is a *sensing* mechanism, not a computational one. Nothing in quantum
  biology currently supports a warm-wet quantum processor.

### 9.3 What the framework actually needs quantum resources *for*

This is the section's most useful clarification, because the proposal never
distinguishes two completely different uses:

**Use 1 — Quantum computation at *compile time*, to predict physics.**
Electronic-structure calculation for reaction barriers, catalysts, correlated
materials. The workpiece is classical; the quantum machine sits in the data center.

**Use 2 — Coherent quantum control of the *workpiece*, at run time.**
The article being manufactured has quantum degrees of freedom that are deliberately
prepared — spin defects, qubit arrays, single-photon emitters, quantum sensors.

**Use 3 — Quantum-enhanced *metrology* of the workpiece.**
Squeezed-light interferometry, NV magnetometry, entanglement-enhanced clocks.

These have wholly different maturity, and conflating them inflates the framework's
apparent quantum dependence.

#### Use 1: honest status

**The case for exponential quantum advantage in ground-state chemistry has been
substantially weakened.** A careful 2023 analysis found that, for generic chemical
systems, evidence of exponential advantage is absent: the bottleneck is preparing a
good initial state with non-vanishing overlap with the true ground state, and
classical heuristics (DMRG, coupled cluster, selected CI, quantum Monte Carlo) scale
better in practice than the naive comparison suggests. Advantage may survive for
specific strongly correlated targets, and *polynomial* advantage for dynamics
(Hamiltonian simulation) remains solid.

Resource estimates for a canonical hard target (the FeMo cofactor of nitrogenase)
have fallen by several orders of magnitude over the last decade through better
algorithms — from ~10¹⁵ T gates to ~10¹⁰–10¹¹ — but still imply millions of physical
qubits at current error rates and hours-to-days of runtime. **Verdict: Use 1 is real
but is a 2035+ contributor, not a near-term compiler dependency.** Machine-learned
interatomic potentials trained on classical DFT are the near-term workhorse, and
they are already changing materials practice.

#### Use 2: honest status

Genuinely relevant, but for a narrow and valuable product class: deterministic
placement of single spin defects (NV, SiV, SiC divacancies), single-dopant devices
in silicon, and engineered qubit arrays. Deterministic single-ion implantation and
atomically precise hydrogen-depassivation lithography have both demonstrated
single-dopant placement. This is exactly the regime where a compiler
(spec → placement plan → control → verification) is appropriate, and where the
verification burden is brutal (§11.6). **A defensible flagship demonstration for the
framework.**

#### Use 3: honest status

**STRONGLY SUPPORTED and undersold in the proposal.** NV magnetometry and
scanning-NV imaging give nanoscale magnetic-field maps; squeezed light gives
sub-shot-noise displacement sensing (deployed in gravitational-wave detectors);
NV-based NMR reaches single-cell and even single-molecule regimes. Quantum sensing
is the part of quantum technology that is *already* useful for the framework's
verification layer, and it should be in the roadmap earlier than quantum computing.

### 9.4 Quantitative requirements

For a quantum resource to be *useful* to the framework, state the requirement
properly. Let a task require N qubits and circuit depth D with gate time t_g.

**Unencoded (NISQ-style) operation** requires roughly
```
ε_gate ≲ 1 / (N · D)        and        T₂ ≳ D · t_g
```
For N = 50, D = 100: ε_gate ≲ 2×10⁻⁴ — beyond current two-qubit fidelities on most
platforms, and the reason unencoded quantum advantage for useful tasks has not
arrived.

**Encoded (fault-tolerant) operation** requires
```
ε_phys < ε_threshold  (≈ 10⁻² for the surface code under circuit-level depolarizing noise)
```
with overhead: physical qubits per logical qubit ≈ 2d² for a distance-d surface
code, where d is set by the target logical error rate:
ε_L ≈ A (ε_phys/ε_th)^{(d+1)/2}. Reaching ε_L = 10⁻¹⁰ from ε_phys = 10⁻³ needs
d ≈ 17–21, i.e. ~600–900 physical qubits per logical qubit before accounting for
magic-state distillation, routing, and the factory overhead — which typically
dominates. Practical estimates for chemistry-scale algorithms land at 10⁵–10⁷
physical qubits.

**The threshold has now been crossed experimentally.** A 2024 superconducting
demonstration achieved below-threshold surface-code memory: a distance-7 code on
101 qubits with 0.143% ± 0.003% logical error per cycle, error suppression
Λ = 2.14 ± 0.02 per distance step of 2, and real-time decoding at distance 5 with
63 µs average latency over a million cycles. This is a genuine inflection point and
should be recorded as such — **ESTABLISHED, single-platform, 2024** — while noting
that it demonstrates a *memory*, not a computation, and that the road from
below-threshold memory to 10⁶ physical qubits is one of engineering scale-up,
cryogenic wiring, and control electronics, not of physics.

### 9.5 Feasibility matrix

Values are order-of-magnitude representative figures from the published record as of
2026; they are platform bests, not typical, and specific numbers move quickly.
Confidence annotations are in `CITATIONS.md`.

| Platform | Operating T | T₁ | T₂ (best, with DD) | 1Q / 2Q fidelity | Gate time | Qubits demonstrated (single device) | Relevance to the framework |
|---|---|---|---|---|---|---|---|
| **Superconducting (transmon)** | 10–20 mK | 0.1–1 ms | 0.1–0.5 ms | ~99.9% / 99.5–99.9% | 10–50 ns | ~10²–10³ | Use 1 (compile-time). Below-threshold QEC demonstrated. Cryogenic — never in the workpiece. |
| **Trapped ions (hyperfine)** | µK ions, RT–cryo apparatus | very long | seconds to >1 h (clock qubits) | >99.9% / ~99.9% | µs–ms | ~10–10² | Highest fidelity; slow; excellent for Use 1 and for metrology standards. |
| **Neutral atoms (Rydberg)** | µK | seconds | ~1–10 s (nuclear-spin encodings) | ~99.9% / ~99.5% | 0.1–1 µs | 10²–10³ | Best scaling path for analog simulation (Use 1) and for programmable many-body physics. |
| **Si spin qubits (Si/SiGe, donors)** | 0.1–1.5 K | ms–s | ms–s (isotopically purified ²⁸Si) | >99.9% / ~99% | 10–100 ns | ~10⁰–10¹ | **Directly relevant to Use 2**: fabricated by semiconductor processes; a spec→fab→verify compiler target. |
| **NV centers in diamond** | **300 K** | ~ms (e⁻), hours (¹³C at low T) | 0.1–2 ms (e⁻, DD); seconds (nuclear) | ~99.9% / ~99% (local register) | ns–µs | 1–10 per register | **The room-temperature resource.** Use 2 and Use 3. Small registers, not scalable processors. |
| **Photonic (dual-rail / CV)** | **300 K** | n/a (loss-limited) | n/a | high single-qubit; probabilistic 2Q | ps–ns | 10¹–10² modes | RT operation, no thermal decoherence; loss and non-determinism are the costs. Strong for Use 3 and for networking attestation channels. |
| **Molecular spin qubits** | 4–300 K | µs–ms | µs (RT) – ms (low T) | research-grade | ns–µs | 1–few | Chemically tunable; **the natural meeting point of the framework's chemistry and quantum layers**; far from application. |
| **Nuclear spins (liquid NMR ensembles)** | **300 K** | s–min | ~1 s | high (ensemble) | ms | ~10 (not scalable) | Historical; no individual addressability; useful as a control-theory testbed only. |
| **Excitons / polaritons (organic)** | **300 K** | fs–ps | fs–ps | n/a | fs | n/a | RT condensation demonstrated; coherence far too short for computation; possible for ultrafast analog operations. |
| **Phononic / optomechanical** | mK–K | µs–ms | µs–ms | research-grade | ns–µs | 1–few | Transduction between microwave and optical domains; interesting for hybrid links. |
| **Topological (non-Abelian)** | mK | — | — | — | — | **0 demonstrated** | **SPECULATIVE.** Would be transformative if realized; no working qubit exists. |
| **Many-body localization as protection** | — | — | — | — | — | — | **CONTESTED.** Numerical and theoretical work (avalanche instability arguments) casts doubt on true MBL in the thermodynamic limit in ≥1D with generic baths. Do not build on it. |

### 9.6 What the matrix implies for the framework

1. **No platform gives a room-temperature, many-qubit, long-coherence substrate that
   could be embedded in a manufactured article.** The framework must not require
   one. **STRONGLY SUPPORTED.**
2. **Room-temperature coherence is real at the level of single registers (NV, SiV,
   molecular spins, photons).** The right framing is *quantum components in a
   classical article*, not a quantum article. That is a legitimate and large product
   category (quantum sensors, single-photon sources, memories) and a compiler for it
   is a credible flagship.
3. **The quantum layer's near-term contribution to the framework is metrology, not
   computation.** Reordering the roadmap accordingly is one of this document's
   concrete recommendations.
4. **Cryogenic quantum computation contributes at compile time, through better
   Hamiltonians and thermochemistry, with a realistic horizon in the 2030s and with
   the exponential-advantage claim for generic chemistry retracted.**

### 9.7 The one quantum claim in the proposal that should be strengthened

The proposal's instinct that *dissipative/constraint stabilization* is the right
protection strategy is correct and can be stated more strongly than it is. The
general result — that engineered dissipation is a universal resource for state
preparation and computation, and that dissipatively prepared states are *attractors*
and therefore self-correcting without measurement — is the cleanest formal
connection between the proposal's §VIII (landscape engineering) and §IX (quantum
substrate). It says: the same idea (make the target an attractor of engineered
dynamics) works classically for self-assembly and quantum-mechanically for state
preparation, and in both cases the figure of merit is the *basin/gap*, not the
energy.

That unification — **"attractor engineering" as the common mechanism across
classical self-assembly and quantum dissipative state preparation** — is, in this
author's assessment, the strongest genuinely novel theoretical synthesis available
in the proposal. It is developed as Research Priority 2 in §26.
