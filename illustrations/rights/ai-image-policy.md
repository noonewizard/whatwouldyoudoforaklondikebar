# DELIVERABLE 16 — AI-GENERATED ARTWORK POLICY

## The boundary

| Permitted | Forbidden — without exception |
|---|---|
| Chapter-opener atmosphere | Historical evidence of any kind |
| Abstract conceptual visualization | Archaeological evidence |
| Clearly labeled artistic interpretation | Historical documents or manuscripts |
| Decorative texture and rule work | Authentic portraits |
| | Ritual scenes presented as historical |
| | Artifacts, inscriptions, architectural features |
| | "Discovered" manuscripts of any description |
| | Anything a reader could mistake for a record |

**Structural enforcement:** AI imagery is permitted **only** where
`figure_class: atmospheric`, and `atmospheric` is permitted **only**
`evidence_status: conceptual`. The validator rejects any other combination, and
`ai_generation.role: evidence` is rejected outright. It is not possible to
register an AI image as evidence.

## Why this series in particular

These books demonstrate, at length and with controls, how images and
reconstructions acquire unearned authority — Villalpando's engravings being the
governing case. Chapter 66 of Book IV states the mechanism directly: *a drawing
removes the uncertainty that the text contains.* A generative model removes it
more completely and more persuasively than an engraver can.

A book that documents that mechanism and then deploys it would forfeit its
standing. This is not caution. It is consistency.

## Mandatory metadata

```yaml
ai_generation:
  used: true
  tool: ""
  model_version: ""
  prompt: ""                    # verbatim, in full
  negative_prompt: ""
  date: ""
  human_edits: ""               # itemized
  source_references: []         # anything the prompt or image conditioning drew on
  historically_constrained: false
  role: atmosphere              # atmosphere | illustration. NEVER evidence
```

## Mandatory labeling

Every AI-generated image carries, in the evidence gutter:

> **ARTISTIC VISUALIZATION — generated image, not a historical source.**

Not in a colophon. Not in the back matter. **On the figure.** A reader who sees
only the page must know.

## Where an AI image depicts a reconstruction

Additionally labeled *artistic reconstruction*, with the sources the
reconstruction rests on named in the caption — and if there are none, the
caption says so and the figure is almost certainly not worth printing.

## The standing test

> Could this image be mistaken, by a reader who does not read the caption, for a
> photograph, a document, or an artifact?

If yes, it does not go in the book, whatever the caption says.
