#!/usr/bin/env python3
"""FIG-0N-NNN — <title>

Claim supported : <one sentence>
Claim NOT made  : <one sentence; must mirror does_not_show in the register>
Data source     : <path or citation>
Reproduce       : python3 build.py
"""
import pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

SEED = 20260920
FIG_ID = "FIG-0N-NNN"
SLUG = "short-slug"

HERE = pathlib.Path(__file__).resolve().parent
STYLE = HERE.parents[3] / "shared" / "styles" / "ringler-print.mplstyle"
if STYLE.exists():
    plt.style.use(str(STYLE))

# Texture is the primary identity channel; color is auxiliary (Deliverable 3).
HATCH = ["", "///", "...", "xxx"]


def load():
    """Return the data. Read from ../data/ — never inline literals."""
    raise NotImplementedError


def draw(ax, data):
    """Draw the marks. Do not add the evidence badge; Typst supplies it."""
    raise NotImplementedError


def main():
    data = load()
    fig, ax = plt.subplots()
    draw(ax, data)
    ax.set_axisbelow(True)
    out = HERE.parent / "exports"
    out.mkdir(exist_ok=True)
    for ext in ("pdf", "svg", "png"):
        fig.savefig(out / f"{FIG_ID}_{SLUG}_print.{ext}")
    print(f"wrote {FIG_ID} -> {out}")


if __name__ == "__main__":
    main()
