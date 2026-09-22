# Part VI — Provenance, Integration, and Security

Sections 12–14: Physical State Attestation · QRADLE/Aethernet Integration ·
Security Architecture

---

## 12. Physical State Attestation

### 12.1 What cryptography can and cannot prove about physical reality

State this first, plainly, because it is the boundary the whole layer lives on.

**Cryptography can prove:**
- That a specific byte string (a specification, a plan, a calibration record, a
  measurement log) existed at a certain time and has not been altered. *(hash
  commitments, timestamping)* — **ESTABLISHED.**
- That a specific key holder authorized an action. *(digital signatures)* —
  **ESTABLISHED.**
- That a record is one of an append-only, totally ordered sequence, and that an
  auditor can detect any removal or reordering. *(Merkle trees, transparency logs)*
  — **ESTABLISHED.**
- That a computation over committed inputs produced a committed output, without
  revealing the inputs. *(zero-knowledge proofs / SNARKs)* — **ESTABLISHED**, with
  the usual caveats about trusted setup, implementation bugs, and proving cost.
- That a message originated from a device holding a key in tamper-resistant hardware
  whose boot state matches a known measurement. *(remote attestation: TPM 2.0, DICE,
  TEEs)* — **STRONGLY SUPPORTED**, degraded by a continuous record of side-channel
  and fault-injection attacks against every deployed TEE.

**Cryptography cannot prove:**
- That the object in your hand is the object the record describes. **This is the
  entire problem**, and no amount of hashing addresses it. The record and the matter
  are separate; the binding between them is *physical*, and its security rests on a
  physical unclonability assumption, not a computational one.
- That a sensor reported the truth. A signed reading from an attested sensor proves
  the *sensor* said it; if the sensor is looking at the wrong thing, is miscalibrated,
  or has been physically spoofed (a heater on the thermocouple, a reference sample
  swapped in), the signature is valid and the claim is false. **The sensor-to-signature
  gap is irreducible** and must be managed by redundancy, diversity, and physical
  security — not by better cryptography.
- Anything about entropy, degradation, or the future state of the article.

The proposal's own instruction — "do not claim that cryptography protects matter
from entropy" — is correct and should be extended: *cryptography protects claims
about matter, and the weakest link is always the claim-to-matter binding.*

### 12.2 The event record, corrected

The proposal's

```
E_i = (S_i, R_{A,i}, P_i, U_i, R̂_{T,i}, V_i),   H_i = H(E_i ‖ H_{i−1})
```

is a serviceable hash chain but has three defects.

1. **It contains `R_{A,i}` and an implied realized state.** States are not
   observable (§4.4). Replace with *claims about* states: attested input
   attestations and measurement records.
2. **A linear hash chain forces total order on events that are genuinely concurrent**
   (three machines running in parallel). Use a Merkle DAG with explicit causal
   edges; total order is imposed only where it is real.
3. **It has no identity for the article**, so nothing connects the chain to the
   object.

**Corrected event schema:**

```
E_i = ⟨
   id_i          : event identifier
   type_i        : SPEC | COMPILE | LOWER | CALIBRATE | EXECUTE | MEASURE | DECIDE | TRANSFER
   parents_i     : set of prior event hashes (causal DAG)
   article_i     : article identity commitment          ← the physical binding (§12.3)
   subject_i     : hash of the relevant payload
                    (spec source, IR, plan, control trace, raw measurement blob)
   actor_i       : machine/operator identity + attestation quote
   context_i     : calibration IDs and expiries, model bundle IDs and versions,
                    environment log digest, consumable lot IDs
   policy_i      : the policy under which the action was authorized
   sig_i         : signature(s) over the above
⟩
H_i = H( canonical_encode(E_i) )
```

Two design points with real consequences:

- **`context_i` must include the model bundle version and every model's validity-
  domain check result.** Otherwise the attestation claims more than the process
  supports. This is the attestation-layer expression of §6.10's correctness
  criterion.
- **Raw measurement blobs are committed, not just derived results.** Committing only
  the pass/fail decision makes the entire record unfalsifiable after the fact.
  Commit the raw data; store it off-chain; put the digest in the record.

### 12.3 The hard part: binding the record to the matter

This is where the framework needs an actual invention rather than a protocol
selection. Four approaches, honestly assessed:

| Mechanism | How it binds | Strength | Failure mode |
|---|---|---|---|
| **Intrinsic physical fingerprint (a physical unclonable function)** — speckle patterns, paper/fiber microstructure, surface topography at µm scale, grain structure, dopant fluctuations | Measure an inherent, high-entropy, hard-to-clone feature of *this* article; commit its digest | Best available. Requires no added material. | Fingerprint must be stable through service life and re-measurable with the same instrument class; ageing and wear destroy it; some PUFs have been modeled and cloned |
| **Added taggants** — rare-earth markers, DNA taggants, engineered isotope ratios, microprinted codes | Add entropy deliberately | High entropy, controllable | Adds material (may be forbidden); can be copied if the taggant is obtainable; supply chain for the taggant becomes a target |
| **Embedded secure element** | A chip holding a key, bonded to the article | Strong cryptographically | Binding of chip to article is the weak point — chips can be transplanted; only works for assemblies |
| **Process-intrinsic evidence** — unique in-process signatures (melt-pool traces, layer images) that a forger would have to reproduce | Correlate the article's measurable microstructure with the recorded process trace | Elegant; no added material; leverages data already collected | Requires that the process trace → microstructure map be *sensitive and hard to fake*; this is an open research question |

**Recommendation and honest statement of the security model.**
The framework's attestation should rest on an explicit, named assumption:

> **Assumption PUF-1.** For substrate class X and fingerprint protocol F, an adversary
> with capability bound C cannot produce a distinct article whose F-measurement is
> within the matching threshold of a target article's, at cost below B.

Everything above the assumption is standard, well-understood cryptography.
Everything below it is materials science. **Making this assumption explicit, with a
per-substrate quantitative bound, is the correct contribution — and it is a real,
publishable, falsifiable research problem** (Falsification Experiment F-6, §25).

### 12.4 Primitive selection

Straightforward, and should be boring by design:

- **Hash:** SHA-3/SHAKE (FIPS 202) for interoperability and for the sponge's
  flexibility; BLAKE3 where throughput on large measurement blobs matters (it is
  fast and parallel but is not a NIST-approved primitive, which matters in regulated
  industries). Use SHA-256/SHA-3 for anything that must survive an audit; BLAKE3 for
  internal content addressing.
- **Signatures:** dual-sign with a classical scheme (Ed25519 or ECDSA P-256) and a
  post-quantum scheme. NIST standardized ML-DSA (FIPS 204, lattice-based) and
  SLH-DSA (FIPS 205, hash-based) in August 2024, with ML-KEM (FIPS 203) for key
  establishment. For attestations that must be verifiable for decades — aerospace,
  nuclear, structural, pharmaceutical — **SLH-DSA is the conservative choice**: it
  rests only on hash-function security, has the smallest assumption footprint, and
  its large signatures are irrelevant at manufacturing event rates. This is a case
  where the usual "signatures are too big" objection does not apply.
- **Transparency:** an append-only Merkle log with published signed tree heads and
  independent witnesses/auditors — the Certificate Transparency architecture,
  which is deployed, attacked, and survived at internet scale. Do not invent a new
  one.
- **Zero-knowledge:** genuinely useful here for a specific, real problem —
  *proving conformance without revealing the process*. A contract manufacturer can
  prove "the executed recipe is one of the approved set, all in-process readings
  stayed within the approved envelope, and the calibration was current" without
  disclosing the recipe. That is a real commercial need (IP protection in contract
  manufacturing) that ZK solves and nothing else does. **PLAUSIBLE and
  high-value**; proving cost over long sensor time-series is the open engineering
  question.
- **Supply chain:** reuse in-toto (attested step-by-step supply chain layouts),
  SLSA (provenance levels), and Sigstore (keyless signing with transparency). These
  were built for software supply chains and map onto physical ones with modest
  changes. **Reusing them, rather than inventing a parallel stack, is worth more than
  any novel protocol the framework could produce.**

### 12.5 What an attestation actually asserts

The honest statement that should appear on every certificate the framework issues:

> This attestation asserts that: a process whose recorded plan hashes to *h_P* was
> executed on machine *M* (attested boot state *q_M*, calibration *c* valid on the
> execution date), consuming inputs bearing attestations *A₁…A_k*; that the recorded
> in-process and acceptance measurements hash to *h_Y*; that the decision procedure
> *D* applied to those measurements returns PASS at confidence 1−δ under measurement
> model *m*; and that the article presented bears physical fingerprint *f* matching
> the commitment made at time *t*.
>
> It does **not** assert that the article conforms to any property not in the
> specification, that the models used are correct, that the sensors reported
> physical truth, or that conformance persists after time *t*.

An attestation layer that says this is trustworthy. One that says "verified" is not.

---

## 13. Deterministic Provenance Substrate Integration (QRADLE / Aethernet)

### 13.1 A necessary disclaimer

QRADLE, Aethernet, QRATUM, QuASIM, QuNimbus, and REVULTRA are not systems this
author has documentation for; they appear to be internal or proposed architectures.
What follows therefore evaluates the *architectural role* the proposal assigns them —
a deterministic, ordered, immutable event substrate governing physical compilation —
rather than any specific implementation, and states what any such substrate must
provide. Where the proposal asks "identify what must change," the answer is given as
requirements that an implementation must meet, which can then be checked against the
real system.

### 13.2 The proposed flow, audited

```
Reality DSL → Compilation Request → Physical State Model → Transition Plan
→ Execution → Metrology → Verification → Attestation → Immutable Event
```

The flow is broadly right and has four gaps:

1. **No authorization gate before Execution.** Execution of a physical process is
   irreversible and potentially hazardous. There must be an explicit
   policy-evaluation and human-authorization step between plan and execution, and
   its result must itself be an attested event.
2. **No branch for INCONCLUSIVE.** Verification is three-valued (§11.1); the flow has
   no path for "measure more," which will be the most common outcome on hard
   specifications.
3. **No rework/scrap/quarantine path.** FAIL must lead somewhere, and where it leads
   is a regulated question in most industries.
4. **Metrology precedes Verification but nothing binds the *article* through the
   flow.** Per §12.3, an article identity commitment must be created at the first
   point the article physically exists and carried thereafter.

**Corrected flow:**

```
   Spec ──► Compile ──► Plan ──► [POLICY + AUTHORIZATION] ──► Execute ──► Article-ID commit
                ▲                        │                       │              │
                │                        │ deny                  │ abort        ▼
                │                        ▼                       ▼           Metrology
                │                  Attested denial        Attested abort         │
                │                                                                ▼
                └──────── refine spec / plan ◄── INCONCLUSIVE ◄──────────── Verification
                                                       │  FAIL                   │ PASS
                                                       ▼                         ▼
                                              Quarantine / rework           Attestation
                                              (attested disposition)             │
                                                                                 ▼
                                                                          Immutable event
```

### 13.3 Requirements on the provenance substrate

Any substrate filling this role must provide:

| Requirement | Why | Note |
|---|---|---|
| **Append-only with independent auditability** | An operator who can rewrite history can launder a failed lot | Requires *external* witnesses; self-attested immutability is not immutability |
| **Causal (partial) ordering, not global total ordering** | Physical processes are concurrent; forcing total order either lies or serializes throughput | Merkle DAG + per-machine sequence numbers |
| **Bounded, known finality latency** | Execution cannot wait on distributed consensus; a machine must be able to act and log | Design: act locally with a local signed sequence; anchor to the global log asynchronously; the attestation's strength grows with anchoring depth |
| **Offline/air-gapped operation** | Many fabs, defense facilities, and pharmaceutical plants are network-isolated by policy | Requires store-and-forward attestation with local roots of trust |
| **Key management and revocation with long horizons** | A 30-year airframe outlives every key | Hash-based signatures; timestamping; re-attestation ceremonies |
| **Data volume realism** | A single LPBF build generates 10²–10³ GB of melt-pool imagery; a fab generates petabytes/day | Records hold *digests*; payloads live in content-addressed object storage; the ledger must never hold bulk data |
| **Policy evaluation with a human-in-the-loop gate for hazard classes** | §14 | Policy must be versioned and attested like everything else |
| **Rollback semantics that are honest** | Physical actions cannot be rolled back | "Rollback" must mean *compensating action plus attested disposition*, never erasure. This is a genuine semantic difference from database transactions and must be designed in, not retrofitted |

### 13.4 What must change relative to a generic blockchain-style architecture

Stated bluntly, because this is where such proposals usually go wrong:

- **Do not put physical execution behind consensus.** Consensus latency and
  liveness failures become safety hazards. Local authority, asynchronous anchoring.
- **Do not use a permissionless ledger.** The participants are identified legal
  entities under regulatory obligation; the trust model is a PKI with transparency,
  not proof-of-work.
- **Do not treat immutability as the security property.** The security property is
  *detectability of tampering by an independent auditor*. Immutability without
  auditors is a claim, not a guarantee.
- **Do not store measurement data on-ledger.** See the volume row above.
- **Do encode the safety interlocks outside the ledger.** A ledger is a record, not a
  safety system. Safety-instrumented functions must be independent (IEC 61511) and
  must fail safe when the ledger is unavailable.

The honest summary: **the provenance substrate should look like Certificate
Transparency plus in-toto plus a PKI, operating alongside a conventional
industrial control and safety architecture — not like a blockchain, and not like a
controller.**

---

## 14. Security Architecture

### 14.1 Why the threat model is unusual

Physical compilation has a property that ordinary software supply chains do not:
**the adversary's payload is matter.** A compromised compiler does not exfiltrate
data; it causes a bracket to fail at 30,000 feet, a drug lot to carry an undeclared
impurity, or a reactor to run outside its safety envelope. Consequences are
physical, delayed, and often non-attributable.

Three consequences for the architecture:

1. **Detection latency matters more than prevention completeness.** A defect
   introduced today may surface in five years. The architecture must optimize for
   *forensic reconstructability*, which is what an honest attestation layer buys.
2. **Independence beats strength.** Two diverse, independent checks beat one strong
   check, because model error and compromise are correlated within a stack.
3. **Safety and security share machinery here, and they conflict.** A security
   response (halt everything) can be a safety hazard (an exotherm that must be
   quenched, a furnace that must be ramped down). Safe-state transitions must be
   designed per process.

### 14.2 Threat model

Organized by the layer attacked, with the framework-specific mitigations. Threat
categories are those the proposal lists, with the assessment added.

| # | Threat | Layer | Realism | Primary mitigation |
|---|---|---|---|---|
| T1 | **Malicious specification** — a spec that is conformant but designed to produce a hazardous article (e.g. a compliant-looking energetic formulation, a pathogen sequence) | Digital | **High** | Policy evaluation at compile time: hazard screening on the spec and on every intermediate; export-control and biosecurity screens; human authorization gate. **Screening must run on intermediates, not just targets** — decomposition into innocuous sub-specs across suppliers is the obvious evasion |
| T2 | **Unauthorized physical execution** | Control | High | Machine-side policy enforcement: the machine refuses plans without a valid authorization token; enforcement in the machine's own trusted element, not in the orchestrator |
| T3 | **Malicious firmware / compromised controller** | Control | **High; demonstrated in the wild** (industrial control system attacks have caused physical damage) | Measured boot + remote attestation; signed firmware with rollback protection; independent safety instrumented system; out-of-band process monitoring |
| T4 | **Compromised sensors / falsified metrology** | Measurement | **High and under-defended** | Sensor attestation; *diverse redundancy* (different physical principles); periodic blind reference samples ("known-answer" articles) inserted into the stream — this is the single most effective countermeasure and is borrowed from proficiency testing |
| T5 | **Adversarial materials** — feedstock that meets incoming spec but behaves differently (substituted supplier, recycled content, counterfeit alloy) | Physical | **High; routine in real supply chains** | Incoming attestation + independent verification of a sampled fraction; fingerprinting at receipt; PMI (positive material identification) |
| T6 | **Supply-chain attack on the toolchain** (compiler, model weights, libraries) | Digital | High | Reproducible builds; in-toto/SLSA provenance for the compiler itself; independent reimplementation of the acceptance decision procedure |
| T7 | **Model poisoning** — corrupting the learned potentials/property models so a route that appears conformant is not | Digital | **High, and specific to this framework** | Held-out physical validation sets that the model provider never sees; model provenance and versioning in every attestation; requiring that safety-critical properties be `VERIFY`, never `DERIVE`; ensemble disagreement as an alarm |
| T8 | **Process substitution** — executing a cheaper/different process while claiming the approved one | Control/Physical | **High; the classic manufacturing fraud** | This is the flagship use case for physical provenance. Detection requires that the substituted process leave a *measurable* trace: process-intrinsic microstructural signatures, in-line data with attested timestamps, and energy/consumable reconciliation. **See F-5 (§25) — whether substitution is reliably detectable is an empirical question and the framework should not assume it** |
| T9 | **Unauthorized replication / IP extraction** | Digital/Physical | High | ZK conformance proofs (§12.4); need-to-know partitioning of the recipe; machine-bound encrypted recipes decrypted only inside an attested controller. Note honestly: an adversary with physical access to the machine and the article can always reverse-engineer the process eventually; the goal is cost, not impossibility |
| T10 | **Compiler manipulation** — subtle miscompilation producing conformant-looking but defective plans | Digital | Medium-high | Translation validation: independently check the emitted plan against the spec with a separate checker (the "verified checker" pattern from certifying compilers); N-version compilation for critical routes |
| T11 | **Physics-model corruption via the literature/data pipeline** | Digital | **Medium and novel** | Data provenance for training sets; replication requirements; anomaly detection on parameter shifts between model versions |
| T12 | **Actuator compromise** (setpoints honored in the log, not in the metal) | Physical | Medium-high | Independent measurement of the actuated variable by a separate sensor chain; energy balance reconciliation; physical seals and tamper evidence |
| T13 | **Calibration attack** — subverting the reference standards rather than the instrument | Measurement | **Medium; very high impact; almost never modeled** | Traceability chains to national metrology institutes with attested certificates; multiple independent reference sources; blind proficiency testing |
| T14 | **Denial of service / availability** | All | Medium | Local autonomy (§13.3); graceful degradation to attested-offline mode |

### 14.3 The defense-in-depth stack

```
DIGITAL     spec hazard screening · type system · reproducible builds · toolchain
            provenance · translation validation · model provenance + held-out validation
  ──────────────────────────────────────────────────────────────────────────────
CONTROL     authorization tokens enforced machine-side · measured boot + attestation ·
            signed firmware with anti-rollback · INDEPENDENT safety instrumented system
  ──────────────────────────────────────────────────────────────────────────────
PHYSICAL    tamper-evident enclosures · consumable lot attestation + incoming verification ·
            energy/mass reconciliation · article fingerprint commitment at first existence
  ──────────────────────────────────────────────────────────────────────────────
MEASUREMENT diverse redundant sensing · sensor attestation · blind reference articles ·
            NMI-traceable calibration with attested certificates · independent
            re-verification by a party that did not manufacture
```

The bottom row is the one the framework should treat as load-bearing, for a
structural reason developed in §23: **an organization cannot securely certify its
own output.** Every mature high-consequence industry has independent verification —
financial audit, ISO/IEC 17025 accredited laboratories, notified bodies, FAA
designated engineering representatives, pharmacopoeial testing. A Reality Compiler
that both manufactures and attests has a conflict of interest that no cryptography
can remove.

### 14.4 The dual-use problem, stated rather than avoided

A general specification-to-substrate compiler lowers the barrier to producing
regulated and hazardous artifacts: energetics, chemical weapons precursors,
controlled pathogens, uncontrolled radiological sources, and untraceable weapons.
This is not hypothetical; it is the predictable consequence of making synthesis
easier, and it is the same problem that DNA synthesis screening and chemical
precursor controls already address imperfectly.

The framework's honest position should be:

- **Screening belongs at the substrate, not only at the compiler.** Software can be
  forked; a synthesizer, a reactor, or a fab is a physical chokepoint. The
  enforcement point of last resort is the machine, and it should refuse unauthorized
  plans regardless of what compiler produced them. This mirrors the consensus
  direction in nucleic-acid synthesis screening.
- **Screening on intermediates and on capability, not only on named targets.**
  Blacklists of end products are trivially evaded by decomposition.
- **Attestation is a biosecurity and chemical-security asset**, not only an IP asset:
  a world in which every synthesis leaves an attested, auditable trace is materially
  safer than one in which it does not. This is one of the framework's strongest
  public-interest arguments and is underplayed in the proposal.
- **The framework should not claim to solve this.** It should claim to make
  enforcement *possible at the chokepoints*, and should be built so that a regulator
  can specify screening policy as a first-class, versioned, attested artifact.
