# DELIVERABLE 27 — VISUAL QA CHECKLIST

Six gates, in order. A figure may not skip a gate. `qa_status` advances only on
a pass; a failure returns the figure to `in-progress`.

---

## GATE 1 — CONTENT  *(human)*

- [ ] The figure is the one the manuscript refers to
- [ ] Every label is spelled correctly and matches the manuscript's usage
- [ ] Every date, dimension, and name is correct against the cited source
- [ ] Chronological order is correct
- [ ] Original-language text is correct, and checked by someone who reads it
- [ ] Transliterations match the series convention
- [ ] Jurisdiction is named wherever Masonic practice is shown

## GATE 2 — EVIDENCE  *(human, blocking)*

- [ ] Every source in `provenance.yml` exists and was actually consulted
- [ ] `consulted:` states direct / facsimile / reproduction — no guessing
- [ ] No page or folio number is inferred
- [ ] `evidence_status` matches what the sources support
- [ ] `interpretation_status` matches what the drawing does
- [ ] Inferred elements are visually distinct from documented ones
- [ ] Omitted elements are omitted, not completed for tidiness
- [ ] Where disputed: **competing reconstructions are shown, not one chosen**
- [ ] `does_not_show` is filled and is reflected in the caption

## GATE 3 — DESIGN  *(human)*

- [ ] Legible at final printed size
- [ ] Consistent with the series visual system
- [ ] Palette drawn only from the validated slots
- [ ] Texture present on every filled series
- [ ] Evidence gutter present and correct
- [ ] No decorative element that could be mistaken for data
- [ ] Visual hierarchy matches argumentative importance

## GATE 4 — TECHNICAL  *(automated)*

- [ ] Vector where vector is possible
- [ ] Resolution at or above the floor for its material
- [ ] CMYK, no RGB, no spot color, ink ≤ 300%
- [ ] Fonts embedded and licensed
- [ ] PDF/X-4 compliant
- [ ] No rule below 0.25 pt, no type below 6 pt
- [ ] Filename and location match the naming convention
- [ ] Computational figures regenerate identically

## GATE 5 — ACCESSIBILITY  *(automated + human)*

- [ ] Alt text present, ≤ 250 chars, not a caption copy
- [ ] Long description present where required, and it carries the evidence distinction in words
- [ ] Contrast ≥ 3:1 graphical, ≥ 4.5:1 text
- [ ] **Grayscale test: no information lost**
- [ ] CVD simulation: deutan, protan, tritan all legible
- [ ] Identity never color-alone
- [ ] Claim also stated in running prose where the long description exceeds 150 words

## GATE 6 — VISUAL ARGUMENT AUDIT  *(human, blocking)*

- [ ] All seven questions answered in writing
- [ ] The layer test performed
- [ ] Reconstruction / resemblance / pattern traps checked as applicable
- [ ] Verdict recorded at `qa/audits/FIG-0N-NNN.md`

---

## Automated coverage

Gates 4 and 5 are largely automated by `scripts/check_*.py` and the CI workflow.
**Gates 1, 2, 3 and 6 are irreducibly human.** No tool can tell you whether a
source was really consulted or whether a drawing overstates what it shows.
