#!/usr/bin/env python3
"""Every registered figure declares export files; approved ones must have them."""
import os
from _common import figures, report, figdir
errs, warns = [], []
for f in figures():
    fid, d = f["figure_id"], figdir(f)
    for key in ("export_print", "export_web"):
        rel = (f.get("files") or {}).get(key)
        if not rel:
            errs.append(f"{fid}: files.{key} not declared"); continue
        path = os.path.join(d, rel)
        if f.get("status") == "approved" and not os.path.exists(path):
            errs.append(f"{fid}: approved but {key} missing at {rel}")
        elif not os.path.exists(path):
            warns.append(f"{fid}: {key} not yet produced ({f.get('status')})")
report(errs, warns, "files")
