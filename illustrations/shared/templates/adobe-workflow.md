# DELIVERABLE 9 & 32 — ADOBE WORKFLOW

## Division of labor

| Tool | Use | Never use for |
|---|---|---|
| **Illustrator** | Final vector: maps, architectural plans, precise line work, type setting in figures | Anything reproducible by script — that belongs in Python |
| **Photoshop** | Archival image restoration, photographic correction, compositing, cleanup | Creating content that is not in the source |
| **InDesign** | Not used — page composition is Typst | — |

## Illustrator template — `ringler-figure.ait`

**Layers, locked in this order (bottom to top):**

```
09  Evidence gutter          ← never edited by hand; placed from library
08  Annotation
07  Labels
06  Dimension lines
05  Content — CONJECTURAL    ← dashed/dotted strokes only
04  Content — INFERRED
03  Content — DOCUMENTED     ← solid strokes only
02  Measured field (grid)
01  Reference underlay       ← source scan; ALWAYS non-printing
```

**The layer separation is the enforcement mechanism.** Evidence level is a layer,
not a stroke choice made per object. A reviewer can toggle layer 05 off and see
exactly what the sources actually support. That toggle is a required step in the
Visual Argument Audit.

**Graphic styles** (one per evidence status, matching Deliverable 3) and
**character styles** (one per type role) ship with the template. Ad-hoc strokes
fail the preflight script.

**Document setup:** CMYK, 300 ppi effective, no spot colors, no transparency
flattening until export, artboard = live area, 3 mm bleed on full-page plates.

## Photoshop rules for archival material

**Permitted, logged in `provenance.yml` under `modifications`:**
dust and scratch removal · straightening · cropping (with the crop recorded) ·
global levels/curves for legibility · resolution normalization

**Forbidden:**
- Removing or adding any content
- Reconstructing damaged or missing areas
- "Enhancing" an inscription so it reads more clearly than it does
- Compositing two source images into one apparent object
- Any adjustment that changes what the object appears to show

**The test:** could a scholar who examined the original object accuse this image
of showing something the object does not? If yes, the edit is forbidden.

Every archival file keeps its **unmodified original** in `source/`. The edited
version lives in `working/`. Both are committed.

## Preflight — `scripts/preflight_ai.jsx`

Run before export. Fails on: RGB objects, missing links, text not converted to
outlines in the print export, strokes below 0.25 pt, type below 6 pt, objects on
the reference underlay layer set to print, or any swatch outside the validated
palette.
