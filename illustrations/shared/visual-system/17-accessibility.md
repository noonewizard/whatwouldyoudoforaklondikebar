# DELIVERABLE 17 — ACCESSIBILITY SYSTEM

## Requirements — every figure, no exceptions

| Requirement | Standard | Enforcement |
|---|---|---|
| Alt text | Present, non-empty, not a repeat of the caption | CI fails on missing or duplicate |
| Long description | Required when `figure_type` is architectural, mathematical, cipher, or institutional | CI fails if absent for those types |
| Contrast | ≥ 3:1 for graphical objects; ≥ 4.5:1 for text | Validator |
| Non-color encoding | **Mandatory** — texture + stroke + badge carry identity | Grayscale QA gate |
| Minimum label size | 6 pt at final trim | Print QA |
| Grayscale legibility | No information lost | Blocking gate |

## Alt text vs. long description

They do different jobs and the distinction is enforced.

**Alt text** (≤ 250 characters) — what the figure *is*, for a reader who will
then hear the caption.

> Ground plan of the Jerusalem Temple showing three zones on one axis, with the
> innermost room at the western end.

**Long description** — the figure's *content*, so that a reader who cannot see it
has the same information. Published as a collapsible block in HTML/EPUB and as an
appendix note in print.

> The plan is a long rectangle divided into three sequential zones on an
> east–west axis. Entry is at the eastern end through a porch twenty cubits
> across and ten deep, flanked by two free-standing pillars. The porch opens
> into the main hall, sixty cubits by twenty. At the western end is the inner
> sanctuary, a cube of twenty cubits. Solid strokes mark dimensions stated in
> 1 Kings 6; dashed strokes mark the roof and the space above the inner
> sanctuary, which the text does not specify.

Note what the long description does: it **carries the evidence distinction in
words**. A blind reader learns which parts are stated and which are inferred.
That is not an accessibility courtesy — it is the book's argument.

## Complex diagrams

Any figure whose long description exceeds 150 words must also have its claim
stated in the running prose. **A figure is never the sole carrier of an
argument.** If the text cannot survive the figure's removal, the figure is doing
work the prose should do.

## Forced-colors and monochrome

Texture and stroke style survive `forced-colors: active` and pure monochrome
output. Color does not, and is never required to.
