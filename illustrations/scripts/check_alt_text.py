#!/usr/bin/env python3
"""Alt text: present, <=250 chars, not a copy of the caption, not boilerplate."""
from _common import figures, report
BAD = ("image of", "picture of", "figure showing", "diagram", "photo")
errs, warns = [], []
for f in figures():
    fid = f["figure_id"]; alt = (f.get("alt_text") or "").strip()
    cap = (f.get("caption") or "").strip()
    if not alt: errs.append(f"{fid}: alt_text missing"); continue
    if len(alt) > 250: errs.append(f"{fid}: alt_text {len(alt)} chars (max 250)")
    if alt == cap: errs.append(f"{fid}: alt_text duplicates the caption")
    if alt.lower().startswith(BAD):
        warns.append(f"{fid}: alt_text opens with boilerplate ({alt[:30]!r})")
    if len(alt.split()) < 6: warns.append(f"{fid}: alt_text may be too terse")
report(errs, warns, "alt-text")
