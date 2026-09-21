# DELIVERABLE 3 — COLOR & TYPOGRAPHY SYSTEM

Every palette value below was **computed and validated**, not chosen by eye. The
validator output is reproduced verbatim so the author can re-run it.

---

## PART 1 — THE THREE-WAY CONSTRAINT, AND HOW IT WAS RESOLVED

A scholarly print series imposes four demands that **cannot all be satisfied at
once** for a large categorical palette:

1. an archival, restrained register (no coral, lilac, lime);
2. grayscale separation, for black-and-white reproduction;
3. colorblind separation (deutan/protan/tritan);
4. adequate contrast against a light paper surface.

**This was tested, not assumed.** Three candidate palettes were built and run
through the validator:

| Attempt | Result |
|---|---|
| 7 archival hues, chosen for register | **FAILED** — indigo outside the lightness band, three hues below the chroma floor (read as gray), plum↔slate ΔE 13.0 |
| 6 hues re-spaced for **grayscale** separation | Grayscale PASSED, but hues drifted to `#F26B59` coral, `#D195ED` lilac, `#9AD61B` lime — **violates the archival register** |
| 4 hues, saturation capped for register | Grayscale PASSED, but ochre fell outside the lightness band and two hues dropped below 3:1 against paper |

### The resolution

**Grayscale separation is not carried by hue. It is carried by texture.**

Attempting to encode identity in luminance forces the palette out of its register
and out of the contrast band. So the system inverts the usual priority:

> **Texture is the primary identity channel. Color is auxiliary.**

This satisfies the brief's own rule (§10, "do not rely on color alone") by
construction rather than by good intentions, and it is *more* robust — texture
survives photocopying, fax-quality scans, forced-colors mode, and the grayscale
plates that most trade printings actually use.

---

## PART 2 — THE VALIDATED PALETTES

### Print / light — the primary target

Surface: `#FAF8F4` (warm paper white)

| Slot | Name | Hex | CMYK target |
|---|---|---|---|
| 1 | Indigo | `#3A5FA8` | 78 / 55 / 0 / 20 |
| 2 | Ochre | `#C08420` | 10 / 40 / 95 / 12 |
| 3 | Verdigris | `#0E8C72` | 85 / 12 / 55 / 15 |
| 4 | Oxblood | `#A83A33` | 20 / 88 / 85 / 12 |

```
node scripts/validate_palette.js "#3A5FA8,#C08420,#0E8C72,#A83A33" \
     --mode light --surface "#FAF8F4" --pairs all

  [PASS] Lightness band       all 4 inside L 0.43–0.77
  [PASS] Chroma floor         all 4 >= 0.1
  [PASS] CVD separation       worst all-pairs #A83A33↔#0E8C72 ΔE 9.0 (deutan) · tritan 9.1
  [PASS] Normal-vision floor  worst all-pairs #0E8C72↔#3A5FA8 ΔE 18.0 (normal)
  [PASS] Contrast vs surface  all 4 >= 3:1
  → ALL CHECKS PASS
```

**Four hues, clean at the strictest all-pairs setting.** Slot order is fixed and
never cycled. A fifth category does not get a new hue — it gets a texture, a
facet, or folds into "other."

### Dark / screen — HTML and EPUB

Surface: `#1C1B19`

| Slot | Name | Hex |
|---|---|---|
| 1 | Oxblood | `#C2635C` |
| 2 | Indigo | `#6A8CD4` |

```
node scripts/validate_palette.js "#C2635C,#6A8CD4" --mode dark \
     --surface "#1C1B19" --pairs all
  → ALL CHECKS PASS  (ΔE 18.1 deutan, 21.1 normal)
```

**Dark mode carries only two hues.** This is a measured limit, not a preference:
at three, `#2A9D80`↔`#C2635C` falls to ΔE 5.5 deutan under all-pairs, and at four
the two warm hues collapse to ΔE 10.3 even for normal vision. The dark lightness
band (0.48–0.67) is too narrow to hold four separable archival hues.

Beyond two categories on a dark surface: texture plus direct labels.

### Sequential (magnitude)

One hue, light → dark. Verdigris ramp, monotonic in lightness:

`#E4F0EC · #B8DAD1 · #7FBFAF · #3F9E88 · #0E8C72 · #0A6B57 · #064A3C`

### Diverging (polarity — e.g. *supports* ↔ *undermines* a claim)

Two poles, **neutral gray midpoint**, never a hue at the center:

`#A83A33 · #C98079 · #E3BEBA · #E8E6E1 · #A9C4CE · #5E93A8 · #2A6379`

### Reserved — never reused as a series color

| Role | Hex | Use |
|---|---|---|
| Ink primary | `#1A1917` | Body rules, documented strokes |
| Ink secondary | `#55524C` | Labels |
| Ink muted | `#8A857C` | Grid, axes, the measured field |
| Paper | `#FAF8F4` | Print surface |
| Grid tint | `#1A1917` @ 6% | The measured field |

---

## PART 3 — EVIDENCE ENCODING

**Color is the last channel, never the only one.** Each evidence status is
identified by stroke, texture, and badge before color is considered.

| Status | Stroke | Texture | Badge | Color |
|---|---|---|---|---|
| `documented` | Solid, 1.0 pt | Solid fill | ■ | Ink primary |
| `supported` | Solid, 0.75 pt | 15% tint | ▣ | Ink primary |
| `disputed` | **Two parallel strokes**, 0.5 pt | Split fill, 45°/135° | ⧗ | Slots 1 & 4 |
| `speculative` | Dashed 3-2, 0.5 pt | 20% tint, no outline fill | ◌ | Ink muted |
| `unsupported` | Dotted 1-2, 0.5 pt | Cross-hatch 45°+135° | ✕ | Ink muted |
| `conceptual` | Solid, 0.75 pt, **rounded joins** | None | ◇ | Slot 2 |

A figure printed in pure black and white loses **no** information. That is the
test, and it is a blocking QA check.

---

## PART 4 — TYPOGRAPHY

| Role | Face | Size (print) | Notes |
|---|---|---|---|
| Figure title | Serif, small caps | 9 pt / 11 pt lead | Matches the book's running head |
| Figure subtitle | Serif italic | 8.5 pt | Optional |
| Axis & data labels | Humanist sans | 7 pt | Lining figures |
| In-figure annotation | Humanist sans | 7 pt | |
| Dimension figures | Humanist sans, **tabular lining** | 6.5 pt | Tabular is mandatory so columns align |
| Caption | Serif | 8.5 pt / 11 pt lead | Set to the measure, not the figure width |
| Evidence gutter | Humanist sans, small caps | 6 pt | Letterspaced +40/1000 |
| Figure ID | Humanist sans mono | 6 pt | `FIG-04-037` |
| Original-language | Serif with full Hebrew/Greek coverage | 9 pt | Never substituted |

**Minimum printed size: 6 pt.** Any label that would reduce below 6 pt at final
trim fails QA — the figure is redrawn or promoted to a larger placement.

Recommended open families: **EB Garamond** (serif), **Source Sans 3** (sans),
**Source Code Pro** (mono), with **SBL Hebrew** and **SBL Greek** for
original-language text. All are licensed for embedding.

---

## PART 5 — MATPLOTLIB STYLE

Committed as `shared/styles/ringler-print.mplstyle`. Python figures load it and
therefore cannot silently drift from the validated palette.
