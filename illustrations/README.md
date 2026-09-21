# ILLUSTRATION, DIAGRAM & VISUAL-PUBLISHING SYSTEM
### Six-Book Research Series — Robert E. L. Ringler

**Status: SYSTEM DESIGN. No book illustrations have been produced.**
Per the brief's §36, the infrastructure is established first.

**Scope note.** Books I–IV exist as complete manuscripts. Books V and VI are not
yet written; their directories and plans are forward-provisioned from their stated
central questions, and are marked **PROVISIONAL** throughout.

---

## THE GOVERNING QUESTION

> How can visual information help the reader distinguish what happened, what was
> believed, what was imagined, what was reconstructed, what was inferred, what was
> encoded, what was later interpreted, and what remains unknown?

Every element below exists to answer that. The system's single organizing device
is the **Evidence Status** field: it is required on every figure, it drives the
color, the line treatment, the caption template, and the QA gate.

---

## MASTER PRINCIPLE

> **The illustration should make the evidence easier to see, not make the evidence
> look stronger than it is.**
>
> **When uncertainty is part of the historical truth, uncertainty must become
> visible.**

---

## DIRECTORY MAP

| Path | Contents |
|---|---|
| `register/` | `illustrations.yml` (master), `illustrations.csv` (export), `schema.yml` |
| `shared/visual-system/` | Color, typography, line, evidence-encoding specs |
| `shared/templates/` | Figma, Illustrator, QGIS, Python, caption templates |
| `shared/styles/` | `figures.typ`, `figures.scss`, matplotlib style |
| `book-0N/` | One directory per figure, `FIG-0N-NNN/` |
| `provenance/` | Per-figure `provenance.yml` records |
| `rights/` | Licenses, permissions correspondence, archival requests |
| `exports/` | Build outputs by target (print, web, epub) |
| `qa/` | Checklists, audit records, CI reports |
| `scripts/` | Validators and generators |

## PER-FIGURE STRUCTURE

```
book-04/FIG-04-037/
├── source/          # acquired originals, unmodified
├── research/        # notes, measurements, source scans
├── working/         # .fig links, .ai, .psd, .qgz
├── data/            # input data, CSV/JSON
├── scripts/         # generator + seed + environment
├── exports/         # figure.svg, figure.pdf, figure.png
├── provenance.yml   # REQUIRED
└── README.md        # what this figure claims and does not claim
```

---

## THE DELIVERABLES

| # | Deliverable | File |
|---|---|---|
| 1 | Visual philosophy | `shared/visual-system/01-philosophy.md` |
| 2 | Series visual identity | `shared/visual-system/02-identity.md` |
| 3 | Color & typography | `shared/visual-system/03-color-typography.md` |
| 4 | Figure taxonomy | `shared/visual-system/04-taxonomy.md` |
| 5 | Naming convention | `shared/visual-system/05-naming.md` |
| 6 | Register schema | `register/schema.yml` |
| 7 | Directory architecture | this file |
| 8 | Figma architecture | `shared/templates/figma-system.md` |
| 9 | Adobe workflow | `shared/templates/adobe-workflow.md` |
| 10 | QGIS workflow | `shared/templates/qgis-workflow.md` |
| 11 | Python workflow | `shared/templates/python-workflow.md` |
| 12 | Quarto integration | `shared/templates/quarto-integration.md` |
| 13 | Typst integration | `shared/templates/typst-integration.md` |
| 14 | CI/CD | `.github/workflows/figures.yml` + `qa/cicd.md` |
| 15 | Rights & provenance | `rights/rights-system.md` |
| 16 | AI-image policy | `rights/ai-image-policy.md` |
| 17 | Accessibility | `shared/visual-system/17-accessibility.md` |
| 18 | Print production | `shared/visual-system/18-print.md` |
| 19–24 | Book I–VI plans | `book-0N/PLAN.md` |
| 25 | Register template | `register/illustrations.yml` |
| 26 | Caption templates | `shared/templates/captions.md` |
| 27 | Visual QA checklist | `qa/visual-qa-checklist.md` |
| 28 | Visual Argument Audit | `qa/visual-argument-audit.md` |
| 29 | Provenance schema | `provenance/schema.yml` |
| 30 | Reproducibility spec | `qa/reproducibility.md` |
| 31–34 | Tool templates | `shared/templates/` |
| 35 | Quarto example | `shared/templates/quarto-integration.md` §example |
| 36 | Typst example | `shared/styles/figures.typ` |
| 37 | Actions example | `.github/workflows/figures.yml` |
| 38 | Production checklist | `qa/production-checklist.md` |
