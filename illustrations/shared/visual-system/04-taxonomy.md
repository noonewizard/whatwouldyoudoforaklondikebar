# DELIVERABLE 4 — FIGURE TAXONOMY

Two orthogonal axes. Every figure carries **one value from each**, plus an
evidence status. Together these three fields determine the template, the caption
form, and the QA gate.

## AXIS 1 — `figure_class` (what kind of object it is)

| Class | Definition | Provenance required? | May be AI-generated? |
|---|---|---|---|
| `evidence-bearing` | Makes or supports a factual claim | **YES — CI-enforced** | Never |
| `analytical` | Author's own model or framework | Sources for inputs | Never |
| `computational` | Generated from data by a script | **YES + script + seed** | Never |
| `atmospheric` | Chapter opener, texture, mood | No | Yes, with mandatory `ai_generation` block |

## AXIS 2 — `figure_type` (what form it takes)

### HISTORICAL
`archival-image` · `manuscript-page` · `engraving` · `photograph` · `portrait`
· `artifact` · `inscription`

### ARCHITECTURAL
`floor-plan` · `elevation` · `section` · `reconstruction` · `exploded-diagram`
· `architectural-comparison` · `spatial-sequence`

### GEOGRAPHIC
`historical-map` · `modern-map` · `archaeological-map` · `migration-map`
· `transmission-map` · `route-map` · `institutional-geography`

### TEMPORAL
`chronology` · `timeline` · `genealogy` · `intellectual-transmission`
· `development-sequence`

### SYMBOLIC
`symbol-system` · `iconographic-comparison` · `ritual-sequence`
· `symbolic-transformation` · `emblem-analysis`

### LINGUISTIC
`alphabet-table` · `word-transformation` · `etymological-tree`
· `manuscript-transmission` · `cipher-diagram` · `atbash` · `pigpen`
· `gematria-table` · `notarikon` · `temurah`

### MATHEMATICAL
`geometric-construction` · `proportional-diagram` · `numerical-relationship`
· `statistical-visualization` · `computational-experiment` · `control-comparison`

### COGNITIVE
`ritual-process` · `attention-model` · `memory-model` · `embodied-cognition`
· `identity-transformation`

### INSTITUTIONAL
`organizational-structure` · `hierarchy` · `governance` · `authority-flow`
· `membership` · `information-flow` · `secrecy-architecture`

### CONCEPTUAL
`philosophical-model` · `systems-diagram` · `causal-model`
· `epistemological-framework` · `transformation-model`

## THE COMBINATION RULE

`figure_class` **constrains** `evidence_status`:

| Class | Permitted evidence statuses |
|---|---|
| `evidence-bearing` | documented · supported · disputed · speculative · unsupported |
| `analytical` | conceptual only |
| `computational` | documented (the computation ran) — the *claim* it bears is rated separately in `interpretation_status` |
| `atmospheric` | conceptual only |

An `analytical` figure carrying `evidence_status: documented` is a category error
and **fails CI**. This is the single most useful validation in the system: it
prevents the author's own models from acquiring the visual authority of evidence.
