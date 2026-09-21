#!/usr/bin/env python3
"""Grayscale gate: a print figure must lose no information in black and white.

Because identity in this system is carried by texture rather than luminance
(Deliverable 3), this check asserts the *declaration* -- that every figure with
two or more categorical series declares a texture encoding -- and flags any
figure relying on color alone for review by eye.
"""
from _common import figures, report
errs, warns = [], []
for f in figures():
    fid = f["figure_id"]
    if f.get("figure_class") not in ("computational", "evidence-bearing"): continue
    enc = (f.get("encoding") or {})
    series = enc.get("series_count", 0)
    if series >= 2 and not enc.get("texture"):
        errs.append(f"{fid}: {series} series but no texture encoding declared - "
                    f"would lose identity in grayscale")
    if enc.get("color_only"):
        errs.append(f"{fid}: declares color-only encoding, which this system forbids")
report(errs, warns, "grayscale")
