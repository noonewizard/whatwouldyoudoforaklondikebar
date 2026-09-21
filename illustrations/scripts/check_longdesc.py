#!/usr/bin/env python3
"""Long descriptions required for architectural, mathematical, cipher and
institutional figures -- the types a reader cannot reconstruct from alt text."""
from _common import figures, report
NEEDS = {"floor-plan","elevation","section","reconstruction","exploded-diagram",
 "architectural-comparison","spatial-sequence","geometric-construction",
 "proportional-diagram","numerical-relationship","statistical-visualization",
 "computational-experiment","control-comparison","cipher-diagram","atbash",
 "pigpen","gematria-table","notarikon","temurah","organizational-structure",
 "hierarchy","governance","authority-flow","information-flow","secrecy-architecture"}
errs, warns = [], []
for f in figures():
    fid, t = f["figure_id"], f.get("figure_type")
    ld = (f.get("long_description") or "").strip()
    if t in NEEDS:
        if not ld or ld.startswith("REQUIRED"):
            errs.append(f"{fid}: figure_type '{t}' requires a long_description")
        elif len(ld.split()) < 30:
            warns.append(f"{fid}: long_description is only {len(ld.split())} words")
        elif len(ld.split()) > 150 and not f.get("prose_restatement"):
            warns.append(f"{fid}: long_description >150 words - the claim must also "
                         f"appear in the running prose")
report(errs, warns, "long-descriptions")
