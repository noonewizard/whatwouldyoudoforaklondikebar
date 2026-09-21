#!/usr/bin/env python3
"""No figure reaches print without a completed Visual Argument Audit."""
import os
from _common import figures, report, ROOT
errs = []
for f in figures():
    fid = f["figure_id"]
    if f.get("status") != "approved": continue
    p = os.path.join(ROOT, "qa", "audits", f"{fid}.md")
    if not os.path.exists(p):
        errs.append(f"{fid}: approved with no Visual Argument Audit record"); continue
    txt = open(p).read()
    if "VERDICT:" not in txt: errs.append(f"{fid}: audit record has no VERDICT")
    elif "VERDICT: FAIL" in txt: errs.append(f"{fid}: audit verdict is FAIL")
    for n in range(1, 8):
        if f"{n}." not in txt: errs.append(f"{fid}: audit missing question {n}")
report(errs, [], "visual-argument-audits")
