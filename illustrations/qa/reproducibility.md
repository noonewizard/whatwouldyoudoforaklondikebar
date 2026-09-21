# DELIVERABLE 30 — REPRODUCIBILITY SPECIFICATION

## The standard

> A reader with the repository must be able to regenerate any computational
> figure and get a byte-identical result.

This is not aspirational. CI enforces it on every push.

## Required for every `figure_class: computational`

| Artifact | Location | Why |
|---|---|---|
| Input data | `data/` | The figure's claim rests on it |
| Generator script | `scripts/build.py` | The transformation must be inspectable |
| Pinned dependencies | `scripts/requirements.txt` | Silent library changes alter output |
| Random seed | `provenance.yml: construction.random_seed` | Stochastic results must be fixed |
| Environment | `provenance.yml: construction.environment` | Python and OS version |
| Output | `exports/` | The committed artifact CI compares against |
| README | `README.md` | What the figure claims, and does **not** claim |

## The CI check

```bash
python3 scripts/build.py --out /tmp/regen
diff <(pdftotext /tmp/regen/FIG.pdf -) <(pdftotext exports/FIG.pdf -)
```

PDF byte comparison is unreliable (timestamps, font subsetting), so the check
compares **extracted text and vector path counts**, not raw bytes. A drift fails
the build and names the figure.

## Applies especially to

The series' existing 16 scripts. Book III's cipher work and Book IV's
golden-ratio and hexagram controls are **already** reproducible at the data
level; the figures derived from them inherit that property rather than
re-establishing it. A figure showing the golden-ratio null distribution is
regenerated from `golden_ratio_controls.py`, not traced from its printed output.

## Reproducibility of non-computational figures

Cannot be byte-identical, but must be **reconstructible**:

- Working files (`.ai`, `.fig` link, `.qgz`) committed
- Source scans committed, unmodified, in `source/`
- Every modification logged in `provenance.yml: modifications`
- Methodology stated in prose: what was measured, from what, at what scale

**The test:** could a competent person, given only this directory, produce a
figure that makes the same claim on the same basis? If not, the record is
incomplete.
