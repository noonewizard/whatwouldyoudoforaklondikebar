# The Reality Compiler

### A Computational Theory of Physically Permitted State Transformation

**Status:** research monograph, first complete draft
**Discipline proposed:** *Synthesis Complexity* (see §27 and Final Answer J)
**Recommended technical name for the artifact:** *Physical Synthesis Compiler* (PSC).
"Reality Compiler" is retained as a program name only; it should not appear in a
peer-reviewed title, for reasons given in §2.4.

---

## What this document is

This is an adversarial development of the Reality Compiler proposal. The brief was
explicit: do not elaborate the premise, determine which parts of it survive contact
with known physics, complexity theory, and the experimental record.

The result is that **the central thesis survives in a substantially narrowed and
substantially sharper form**, and that several of its most rhetorically attractive
components do not survive at all.

The three most important findings:

1. **The complexity claim in the original proposal is wrong in the direction of
   optimism.** "Exact physical compilation is NP-hard" is not an overstatement; it
   is an *understatement* so severe that it misrepresents the problem. General
   physical reachability over hybrid continuous/discrete dynamics is **undecidable**,
   not NP-hard. Several natural sub-problems are QMA-complete, Ackermann-complete,
   or undecidable in the thermodynamic limit. NP-hardness is a floor that the
   easiest interesting cases already exceed. See §16.

2. **The representational core of the proposal is a category error, and fixing it
   is the single most productive move in the entire framework.** A target state
   `R_B` cannot be a point in a physical state space; no physical process reaches a
   point, and no measurement certifies one. The target must be an **acceptance
   region** — the preimage of a tolerance set under a finite vector of measurable
   functionals. Once this substitution is made, the compiler's output type changes,
   the verification layer becomes tractable rather than exponential, and the
   thermodynamic accounting closes. See §4 and §6.

3. **Thermodynamics is not the binding constraint, and claiming that it is, is a
   distraction.** Landauer-type and finite-time bounds on physical compilation are
   real and computable, but they sit two to six orders of magnitude below the energy
   that real fabrication processes consume, and twenty orders below the energy that
   real computation consumes. The binding constraints are
   *kinetic* (barrier crossing times), *epistemic* (metrology sample complexity),
   and *combinatorial* (search over synthesis routes). A research program that
   optimizes against Landauer is optimizing against a constraint that will not bind
   this century. See §15.

The quintillion-dollar valuation in the original framing is rejected on arithmetic
grounds: it exceeds the gross product of the planet by roughly four orders of
magnitude. A bottom-up model anchored on the EDA-to-semiconductor capture ratio
(~2.5%) gives a very different, still very large, and actually defensible number.
See §22.

---

## Reading order

| # | File | Sections |
|---|------|----------|
| 1 | [`01-foundations.md`](01-foundations.md) | 1 Executive Thesis · 2 Definition · 3 Mathematical Foundations · 4 Physical State Spaces |
| 2 | [`02-reality-dsl.md`](02-reality-dsl.md) | 5 Reality DSL · 6 Compilation Semantics |
| 3 | [`03-engine-and-hamiltonian.md`](03-engine-and-hamiltonian.md) | 7 Physical State Transition Engine · 8 Hamiltonian Engineering |
| 4 | [`04-quantum-substrate.md`](04-quantum-substrate.md) | 9 Quantum Substrate |
| 5 | [`05-assembly-and-verification.md`](05-assembly-and-verification.md) | 10 Assembly Engine · 11 Metrology and Verification |
| 6 | [`06-attestation-and-security.md`](06-attestation-and-security.md) | 12 Physical State Attestation · 13 QRADLE/Aethernet Integration · 14 Security Architecture |
| 7 | [`07-limits-and-complexity.md`](07-limits-and-complexity.md) | 15 Thermodynamic Limits · 16 Complexity Theory · 17 Universality |
| 8 | [`08-assembly-bio-materials.md`](08-assembly-bio-materials.md) | 18 Self-Assembly · 19 Biological Morphogenesis · 20 Materials Discovery |
| 9 | [`09-roadmap-economics-org-ip.md`](09-roadmap-economics-org-ip.md) | 21 Industrial Roadmap · 22 Economic Model · 23 Corporate Architecture · 24 Intellectual Property |
| 10 | [`10-falsification-and-conclusions.md`](10-falsification-and-conclusions.md) | 25 Falsification · 26 Research Priorities · 27 Open Problems · 28 Conclusions · Final Answers A–J |
| — | [`CLAIM_LEDGER.md`](CLAIM_LEDGER.md) | Every major claim, labeled |
| — | [`CITATIONS.md`](CITATIONS.md) | Literature audit with recall-confidence annotations |
| — | [`NOTATION.md`](NOTATION.md) | The proposed notation system, in one place |

---

## Claim labels used throughout

| Label | Meaning |
|---|---|
| **ESTABLISHED** | Textbook or theorem. Not seriously disputed. |
| **STRONGLY SUPPORTED** | Multiple independent experimental or proof lines; small chance of revision. |
| **PLAUSIBLE** | Consistent with known physics; partial evidence; no demonstration at the claimed scale. |
| **SPECULATIVE** | Not excluded, but no evidence either way. A research hypothesis. |
| **UNSUPPORTED** | Asserted in the source proposal with no evidential or mathematical basis found. |
| **CONTRADICTED** | The claim, as stated, conflicts with established results or measured data. |

Labels are applied to claims, not to sections, and a section may contain claims at
every level. Where the original proposal's claim is **CONTRADICTED** or
**UNSUPPORTED**, the text states the strongest nearby claim that is defensible and
labels that separately.

---

## One-paragraph summary of the defensible thesis

> Physical synthesis can be formalized as compilation, but not as compilation to a
> state — only as compilation to a *certified acceptance region* over a finite
> vector of measurable functionals, under a substrate-specific action algebra, with
> an explicit epistemic budget. The resulting problem has no single complexity
> class; it has a **complexity vector** whose components (search, simulation,
> control, verification, fabrication, energy, time) are independently hard and
> independently reducible. There is no universal physical compiler, but there is a
> universal *interface*: a specification language and an attestation format can be
> substrate-independent even though every compiler back end is substrate-specific.
> That interface — not the assembler — is the defensible core of the program, and it
> is buildable now.
