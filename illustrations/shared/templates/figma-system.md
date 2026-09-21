# DELIVERABLE 8 & 31 — FIGMA COMPONENT SYSTEM

## Library structure

```
Ringler Series — Figure System   (published library)
│
├── 00 Foundations
│   ├── Color/            ← the four validated slots + ink + paper, as styles
│   ├── Type/             ← the nine type roles from Deliverable 3
│   ├── Stroke/           ← the six evidence stroke styles
│   └── Texture/          ← solid · 15% · split · 20% · cross-hatch
│
├── 01 Frame
│   ├── Frame/Single-Column       (78mm live)
│   ├── Frame/Full-Measure        (118mm live)
│   ├── Frame/Full-Page           (128 × 190mm)
│   └── Frame/Spread              (2 × 128mm + gutter allowance)
│
├── 02 Evidence
│   ├── Evidence/Badge/*          (six variants, one per status)
│   ├── Evidence/Gutter           ← ID · badge · source line
│   ├── Evidence/Key              ← inline legend
│   └── Evidence/Uncertainty-Edge ← the "unfinished edge" terminator
│
├── 03 Diagram
│   ├── Diagram/Node/*            (entity · text · institution · person · place)
│   ├── Diagram/Connector/*       (attested · inferred · disputed · refuted)
│   ├── Diagram/Arrowhead/*
│   └── Diagram/Threshold-Rule
│
├── 04 Timeline
│   ├── Timeline/Axis · Tick · Band · Event · Range · Uncertainty-Bracket
│
├── 05 Architecture
│   ├── Architecture/Wall/Documented · Inferred · Conjectural
│   ├── Architecture/Dimension-Line · North-Point · Scale-Bar · Section-Mark
│
├── 06 Map
│   ├── Map/Frame · Graticule · Scale-Bar · North-Point · Site-Marker
│
├── 07 Cipher
│   ├── Cipher/Alphabet-Cell · Transformation-Arrow · Rule-Box
│   ├── Cipher/Control-Panel      ← target vs control, side by side
│   └── Cipher/Search-Space-Meter ← denominator made visible
│
├── 08 Comparison
│   ├── Comparison/Matrix-Cell/*  (present · absent · disputed · not-assessed)
│   └── Comparison/Panel-Pair
│
└── 09 Annotation
    ├── Annotation/Label · Leader · Callout · Source-Note · Footnote-Marker
```

## Naming convention

`FIG/<Category>/<Subcategory>/<Name>` — matches Deliverable 5 so a component can
be traced from a published figure back to its source component.

## Component properties

Every diagram component exposes an `evidence` variant property bound to the six
statuses. **Changing the property changes stroke, texture, and badge together.**
It is not possible in this library to draw a speculative wall in a documented
stroke — the combination does not exist.

## The Control-Panel component

Specific to this series and used across Books II, III, and IV. A fixed two-pane
frame:

| Left pane | Right pane |
|---|---|
| TARGET — the claimed material | CONTROL — matched material with no claim attached |
| Result | Result |

with a shared footer carrying **search space**, **tolerance**, and **fixed in
advance: yes/no**. The component cannot be placed without those three fields
filled. This is Deliverable 29's visual controls, built into the tooling.

## Export

Figma → SVG (web) and PDF (print) via the Figma REST API, driven by
`scripts/figma_export.py`, keyed on the figure ID in the frame name. No manual
export; no hand-named files.
