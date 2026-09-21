# DELIVERABLE 18 — PRINT PRODUCTION SPECIFICATION

## Targets

| Edition | Trim | Live area | Figure max |
|---|---|---|---|
| Trade paperback | 156 × 234 mm | 118 × 190 mm | 118 × 190 mm |
| Hardcover | 162 × 240 mm | 122 × 196 mm | 122 × 196 mm |
| Digital PDF | matches paperback | — | — |
| EPUB | reflowable | — | 1600 px wide |
| HTML | responsive | — | SVG, unbounded |

Single-column figure measure: **78 mm**. Full-measure: **118 mm**.

## Formats

| Content | Print | Web | EPUB |
|---|---|---|---|
| Diagrams, maps, plans, charts | **PDF/X-4 vector** | SVG | PNG @ 300 ppi |
| Archival photography | TIFF, 300 ppi at final size, LZW | WebP + JPEG | JPEG q85 |
| Manuscript/engraving scans | TIFF, **600 ppi** (fine line work) | WebP | JPEG q90 |
| Type in figures | Live text, fonts embedded | Live text | Outlined |

**Vector wherever possible.** A diagram delivered as a raster has failed.

## Resolution floors

| Material | Floor | Note |
|---|---|---|
| Continuous tone | 300 ppi at final size | |
| Line art / engravings | 600 ppi at final size | 1200 ppi for dense hatching |
| Bitonal scans | 1200 ppi | |

A supplied file below the floor is not upsampled. The figure is re-requested at
higher resolution, reduced in size, or dropped — **never interpolated**.

## Color

- Interior body: **black plate only**, unless a color signature is budgeted
- Figures are designed so that **the black-only rendering loses no information** —
  this is why identity is carried by texture (Deliverable 3)
- Where a color signature exists: CMYK, **US Web Coated (SWOP) v2**, total ink ≤ 300%
- No spot colors; no RGB in any print-bound file
- Rich black `60/40/40/100` for large solids; plain `K100` for type and rules

## The reduction test

**Every figure is proofed at final printed size, in grayscale, at 100%.**

| Fails if | |
|---|---|
| Any label reduces below 6 pt | |
| Any rule reduces below 0.25 pt | It will drop out on press |
| Any adjacent fills become indistinguishable in grayscale | |
| Any hatch pattern moirés at the screen ruling | |

A figure that fails is redrawn, promoted to a larger placement, or split across a
spread. **It is never simply shrunk.**

## Placement

| Type | Handling |
|---|---|
| Inline | Top or bottom of the text block, on the page of first reference or the following spread |
| Full-page plate | Recto preferred; caption on the plate page |
| Facing-pair | Verso/recto together — **required** for target/control pairs, which must be visible simultaneously |
| Landscape | Rotated 90° counter-clockwise; caption rotates with the figure |
| Spread figure | Content kept 12 mm clear of the gutter; a registration mark on each half |

**Target/control pairs never break across a spread.** The comparison is the
argument; separating them destroys it.

## Bleed and safety

Full-bleed plates: 3 mm bleed, 5 mm safety from trim, 14 mm from the gutter on a
perfect-bound spine.

## Deliverables to the printer

```
print-package/
├── interior.pdf              PDF/X-4, fonts embedded, no transparency
├── figures/                  linked source PDFs at full resolution
├── fonts/                    licenses for every embedded face
├── figure-manifest.csv       ID · page · size · format · resolution · rights
├── rights-clearance.pdf      signed permissions for every archival figure
└── grayscale-proof.pdf       every figure at final size, black only
```
