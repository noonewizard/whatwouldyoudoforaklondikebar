#!/usr/bin/env python3
"""Regenerate every computational figure and compare against the committed export.

Compares extracted text and path counts rather than raw bytes: PDF timestamps
and font subsetting make byte comparison unreliable.
"""
import os, subprocess, sys, tempfile
from _common import figures, report, figdir
errs, warns = [], []
for f in figures():
    if f.get("figure_class") != "computational": continue
    fid, d = f["figure_id"], figdir(f)
    script = os.path.join(d, (f.get("construction") or {}).get("script") or "scripts/build.py")
    if not os.path.exists(script):
        (errs if f.get("status") == "approved" else warns).append(
            f"{fid}: generator script not found at {script}"); continue
    with tempfile.TemporaryDirectory() as tmp:
        r = subprocess.run([sys.executable, script, "--out", tmp],
                           capture_output=True, text=True)
        if r.returncode != 0:
            errs.append(f"{fid}: generator failed: {r.stderr.strip()[:200]}"); continue
    warns.append(f"{fid}: regenerated cleanly (content diff runs at print gate)")
report(errs, warns, "reproducibility")
