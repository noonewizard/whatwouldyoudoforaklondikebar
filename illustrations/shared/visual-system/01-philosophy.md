# DELIVERABLE 1 — VISUAL PHILOSOPHY

## The five principles, with enforcement

Each principle is paired with the mechanism that makes it operative. A principle
without an enforcement point is decoration.

### Principle 1 — Evidence before aesthetics

*A beautiful illustration must never make weak evidence appear stronger.*

**Enforced by:** the Visual Argument Audit (Deliverable 28), which is a blocking
QA gate. A figure cannot reach `qa_status: approved` without it.

### Principle 2 — Reconstruction must look like reconstruction

*Never present a reconstruction as a photograph of something known to have
existed.*

**Enforced by:** the evidence-encoding system. Line weight, stroke style, and fill
are bound to `evidence_status`; a speculative element **cannot** be drawn in the
solid stroke reserved for documented remains, because the Illustrator and Figma
libraries do not contain that combination as a usable style.

### Principle 3 — Diagrammatic precision

*When an image makes a factual claim, the underlying data must be traceable.*

**Enforced by:** `provenance.yml` is required for every figure whose
`figure_class` is `evidence-bearing`. CI fails the build if it is missing or
incomplete.

### Principle 4 — No visual hallucination

*Never invent manuscripts, artifacts, inscriptions, architectural features,
rituals, quotations, symbols, portraits, provenance, or source documents.*

**Enforced by:** every figure declares `sources:`. A figure with
`evidence_status` other than `conceptual` and an empty `sources` list fails CI.
AI-generated imagery is confined to `figure_class: atmospheric` and carries a
mandatory `ai_generation` block.

### Principle 5 — Separate evidence from interpretation

**Enforced by:** the five-value evidence scale below, which is a controlled
vocabulary. No free text is permitted in that field.

---

## THE EVIDENCE SCALE

This is the spine of the whole system. It is deliberately **not** numeric — the
brief forbids "truth scores," and a number invites false precision.

| Value | Means | Visual encoding | Caption obligation |
|---|---|---|---|
| `documented` | Physically attested, or stated in a datable primary source | Solid stroke, full opacity, full-value fill | State the holding institution or the source |
| `supported` | Strong scholarly consensus from converging evidence | Solid stroke, 85% opacity | Name the scholarship |
| `disputed` | Specialists actively disagree | Dual/parallel stroke, split fill | **Must show the competing readings, not one** |
| `speculative` | A proposal with reasoning but thin evidence | Dashed stroke, 20% tint, no fill | State whose proposal, and what is missing |
| `unsupported` | Claim shown to fail; drawn only to be examined | Dotted stroke, hatched fill, `✕` badge | State the failure explicitly |

A sixth class exists outside the scale:

| `conceptual` | Author's own analytical model; makes no historical claim | Distinct geometric register (see identity) | Must say "analytical model, not a historical claim" |

### The binding rule

**A single figure may mix evidence levels, and when it does the mixing must be
visible within the frame.** A Temple reconstruction showing Herodian retaining
walls (`documented`) and a conjectured roof (`speculative`) draws them in
different strokes and says so in the caption. This is the most common case in this
series and the system is built around it.

---

## THE PROGRESSION THE VISUALS MUST CARRY

| Book | Move | Dominant visual mode |
|---|---|---|
| I — Magic | Where did the ideas come from? | **Transmission**: chains, chronologies, stemmata |
| II — Ritual | What does the experience do? | **Sequence**: process, blocking, embodied stages |
| III — Word | How is knowledge encoded? | **Transformation**: input → rule → output, with controls |
| IV — Temple | How does meaning become space? | **Plan**: orthographic architecture, measured |
| V — Builder | How does a person construct himself? | **Material**: process from rough to finished *(provisional)* |
| VI — Architect | Who constructs the systems? | **Structure**: organizational and authority topology *(provisional)* |

The series identity persists (grid, palette, type, evidence encoding). The
**dominant diagram type changes**, so the books do not look repetitive and the
visual argument advances with the intellectual one.
