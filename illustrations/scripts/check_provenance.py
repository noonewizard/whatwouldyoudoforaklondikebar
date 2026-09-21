#!/usr/bin/env python3
"""Every figure directory carries a provenance.yml with the required fields."""
import os, yaml
from _common import figures, report, figdir
REQ = ["figure_id", "sources", "construction", "evidence_status",
       "interpretation_status", "claims", "does_not_claim", "rights"]
errs, warns = [], []
for f in figures():
    fid, d = f["figure_id"], figdir(f)
    p = os.path.join(d, "provenance.yml")
    if not os.path.isdir(d):
        warns.append(f"{fid}: directory not created yet"); continue
    if not os.path.exists(p):
        errs.append(f"{fid}: provenance.yml missing"); continue
    try: rec = yaml.safe_load(open(p)) or {}
    except Exception as e: errs.append(f"{fid}: provenance.yml unparseable: {e}"); continue
    for k in REQ:
        if rec.get(k) in (None, "", [], {}):
            errs.append(f"{fid}: provenance.{k} empty")
    if rec.get("evidence_status") == "disputed" and not rec.get("competing_reconstructions"):
        errs.append(f"{fid}: disputed figure must list competing reconstructions")
report(errs, warns, "provenance")
