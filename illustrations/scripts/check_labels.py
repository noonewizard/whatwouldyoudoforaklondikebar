#!/usr/bin/env python3
"""Every @fig- label used in the manuscripts exists in the register, and vice versa."""
import re, glob, os
from _common import figures, report, REPO
reg = {f["quarto_label"]: f for f in figures()}
used = {}
for p in glob.glob(os.path.join(REPO, "book*/chapters/*.md")) + \
         glob.glob(os.path.join(REPO, "book*/appendices/*.md")):
    for m in re.finditer(r"@fig-[a-z0-9-]+", open(p).read()):
        used.setdefault(m.group(0), set()).add(os.path.relpath(p, REPO))
errs, warns = [], []
for lbl, files in used.items():
    if lbl not in reg:
        errs.append(f"{lbl} referenced in {sorted(files)[0]} but absent from the register")
    elif reg[lbl].get("status") != "approved":
        warns.append(f"{lbl} referenced but status={reg[lbl].get('status')}")
for lbl, f in reg.items():
    if lbl not in used and f.get("status") == "approved":
        warns.append(f"{lbl} approved but never referenced in prose")
report(errs, warns, "labels")
