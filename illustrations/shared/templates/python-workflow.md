# DELIVERABLE 11 & 34 — PYTHON VISUALIZATION WORKFLOW

## The rule

**If a figure can be generated from data, it must be.** Hand-drawing a chart in
Illustrator breaks the chain between the claim and its evidence. Illustrator is
for things scripts cannot do: architectural line work, map cartography, layout.

This matters more in this series than in most, because Books II–IV already ship
16 reproducible scripts whose outputs are the argument. Those figures must be
regenerated from the same scripts, not redrawn from their printed output.

## Every computational figure ships

```
FIG-04-031/
├── data/         input, committed
├── scripts/
│   ├── build.py         reads ../data, writes ../exports
│   └── requirements.txt pinned
├── exports/      figure.pdf, figure.svg, figure.png
└── provenance.yml   with script, random_seed, environment
```

CI regenerates every computational figure and **fails if the output differs**
from the committed export. A figure cannot drift from its data.

## Template — `shared/templates/python/build_template.py`

```python
#!/usr/bin/env python3
"""FIG-0N-NNN — <title>

Claim supported : <one sentence>
Claim NOT made  : <one sentence — mirrors does_not_show in the register>
Data source     : <path or citation>
Reproduce       : python3 build.py   (seed fixed below)
"""
import pathlib, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

SEED = 20260920
HERE = pathlib.Path(__file__).resolve().parent
STYLE = HERE.parents[3] / "shared/styles/ringler-print.mplstyle"
plt.style.use(str(STYLE))          # validated palette; never override colors

FIG_ID = "FIG-0N-NNN"
EVIDENCE = "documented"

def load():
    ...

def draw(ax, data):
    ...

def main():
    data = load()
    fig, ax = plt.subplots()
    draw(ax, data)
    # evidence gutter is added by Typst, not here -- the figure must not
    # carry its own badge or it will duplicate at typeset
    out = HERE.parent / "exports"
    out.mkdir(exist_ok=True)
    for ext in ("pdf", "svg", "png"):
        fig.savefig(out / f"{FIG_ID}_{ext_slug}.{ext}")

if __name__ == "__main__":
    main()
```

## Hard rules

| Rule | Reason |
|---|---|
| Load the shared `.mplstyle`; never set colors inline | Palette drift |
| Never use a colormap for categorical data | Rainbow encoding |
| Never dual-axis | Two scales, one frame — the commonest chart lie |
| Texture (`hatch=`) on every filled series | Grayscale and CVD |
| Direct-label ≤ 4 series; legend always present for ≥ 2 | Identity never color-alone |
| Fix and record the seed | Reproducibility |
| No y-axis truncation on magnitude charts | Exaggerates difference |
| Annotate n and the search space on any result chart | Deliverable 29 |

## The control-comparison template

For Books II–IV's pattern claims, `build_control_comparison.py` produces the
standard two-panel figure: target result and matched-control result on a **shared
axis**, with `n`, tolerance, and *fixed in advance: yes/no* printed in the
footer. The two panels always share scale — that is the whole point.
