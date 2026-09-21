#!/usr/bin/env python3
"""Rights gate. --block-pending fails on unresolved permissions;
--check-expiry fails on lapsed licenses."""
import sys, datetime
from _common import figures, report
block = "--block-pending" in sys.argv
expiry = "--check-expiry" in sys.argv
today = datetime.date.today()
errs, warns = [], []
for f in figures():
    fid, r = f["figure_id"], f.get("rights") or {}
    st = r.get("status")
    if st == "permission-pending":
        (errs if block else warns).append(f"{fid}: rights permission still pending")
    if st == "fair-use-claimed" and not r.get("rationale"):
        errs.append(f"{fid}: fair-use claimed without a written rationale")
    if st == "public-domain" and not r.get("basis"):
        errs.append(f"{fid}: public-domain asserted without a recorded basis")
    if expiry and r.get("expires"):
        exp = r["expires"]
        exp = exp if isinstance(exp, datetime.date) else datetime.date.fromisoformat(str(exp))
        if exp < today: errs.append(f"{fid}: license expired {exp}")
        elif (exp - today).days < 90: warns.append(f"{fid}: license expires {exp}")
report(errs, warns, "rights")
