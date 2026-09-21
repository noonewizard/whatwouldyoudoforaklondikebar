#!/usr/bin/env python3
"""Validate the illustration register against the research-integrity rules.

These are not style checks. Each one blocks a specific way a figure can
misrepresent evidence.

Usage:  python3 scripts/validate_register.py [--strict]
Exit 1 on any ERROR. --strict also fails on WARN.
"""
import sys, re, os
try:
    import yaml
except ImportError:
    sys.exit("pyyaml required: pip install pyyaml")

REG = os.path.join(os.path.dirname(__file__), "..", "register", "illustrations.yml")

CLASS_ALLOWS = {
    "evidence-bearing": {"documented","supported","disputed","speculative","unsupported"},
    "analytical":       {"conceptual"},
    "computational":    {"documented"},
    "atmospheric":      {"conceptual"},
}
LONGDESC_TYPES = {
    "floor-plan","elevation","section","reconstruction","exploded-diagram",
    "architectural-comparison","spatial-sequence","geometric-construction",
    "proportional-diagram","numerical-relationship","statistical-visualization",
    "computational-experiment","control-comparison","cipher-diagram","atbash",
    "pigpen","gematria-table","notarikon","temurah","organizational-structure",
    "hierarchy","governance","authority-flow","information-flow",
    "secrecy-architecture",
}
PLACEHOLDERS = {"UNKNOWN","NOT_VERIFIED","NOT_APPLICABLE","TBD"}

errors, warns = [], []
def err(fid, msg):  errors.append(f"ERROR  {fid}: {msg}")
def warn(fid, msg): warns.append(f"WARN   {fid}: {msg}")

figs = yaml.safe_load(open(REG))["figures"]
seen_ids, seen_labels = set(), set()

for f in figs:
    fid = f.get("figure_id", "<no id>")

    # 1. Identity
    if not re.match(r"^FIG-0[1-6]-\d{3}$", fid): err(fid, "malformed figure_id")
    if fid in seen_ids: err(fid, "duplicate figure_id")
    seen_ids.add(fid)
    lbl = f.get("quarto_label","")
    if not re.match(r"^@fig-[a-z0-9-]+$", lbl): err(fid, f"malformed quarto_label {lbl!r}")
    if lbl in seen_labels: err(fid, f"duplicate quarto_label {lbl}")
    seen_labels.add(lbl)

    cls, ev = f.get("figure_class"), f.get("evidence_status")

    # 2. THE CATEGORY-ERROR GATE
    # An analytical model must never wear the visual authority of evidence.
    if cls in CLASS_ALLOWS and ev not in CLASS_ALLOWS[cls]:
        err(fid, f"figure_class '{cls}' may not carry evidence_status '{ev}' "
                 f"(allowed: {sorted(CLASS_ALLOWS[cls])})")

    # 3. NO UNSOURCED CLAIMS
    srcs = f.get("sources") or []
    if cls != "atmospheric" and not srcs:
        err(fid, "non-atmospheric figure has no sources")
    for s in srcs:
        if not s.get("citation"): err(fid, "source entry without a citation")
        if not s.get("role"):     err(fid, "source entry without a role")

    # 4. THE CAPTION MUST CONSTRAIN INTERPRETATION
    if not f.get("does_not_show"):
        err(fid, "missing 'does_not_show' - every figure must state what the "
                 "reader must NOT infer from it")

    # 5. RECONSTRUCTION MUST LOOK LIKE RECONSTRUCTION
    if f.get("interpretation_status") in ("reconstructive","speculative"):
        cap = (f.get("caption") or "").lower()
        if not any(w in cap for w in ("reconstruct","conjectur","inferred",
                                      "not specified","speculative","analytical")):
            err(fid, "reconstructive/speculative figure whose caption does not "
                     "say so")

    # 6. REPRODUCIBILITY
    if cls == "computational":
        c = f.get("construction") or {}
        for k in ("script","random_seed","environment"):
            if c.get(k) in (None,"",*PLACEHOLDERS):
                err(fid, f"computational figure missing construction.{k}")
        if not (f.get("data") or {}).get("source"):
            err(fid, "computational figure missing data.source")

    # 7. AI POLICY
    sw = [s.lower() for s in (f.get("construction") or {}).get("software") or []]
    ai_named = any(t in " ".join(sw) for t in
                   ("midjourney","dall","stable diffusion","firefly","imagen","flux"))
    ai = f.get("ai_generation")
    if ai_named and not ai:
        err(fid, "AI tool in software list without an ai_generation block")
    if ai and ai.get("used"):
        if cls != "atmospheric":
            err(fid, "AI-generated imagery is permitted only for "
                     "figure_class: atmospheric")
        if ai.get("role") == "evidence":
            err(fid, "ai_generation.role may never be 'evidence'")
        for k in ("tool","prompt","date","human_edits"):
            if not ai.get(k): err(fid, f"ai_generation.{k} missing")

    # 8. RIGHTS
    r = f.get("rights") or {}
    for k in ("holder","license","status","attribution"):
        if not r.get(k): err(fid, f"rights.{k} missing")
    if r.get("status") == "permission-pending":
        warn(fid, "rights permission still pending - cannot go to print")

    # 9. ACCESSIBILITY
    alt = f.get("alt_text") or ""
    if not alt: err(fid, "alt_text missing")
    if len(alt) > 250: err(fid, f"alt_text too long ({len(alt)} chars)")
    if alt.strip() and alt.strip() == (f.get("caption") or "").strip():
        err(fid, "alt_text duplicates the caption")
    if f.get("figure_type") in LONGDESC_TYPES and not f.get("long_description"):
        err(fid, f"figure_type '{f.get('figure_type')}' requires a long_description")

    # 10. FILES
    files = f.get("files") or {}
    for k in ("export_print","export_web"):
        if not files.get(k): err(fid, f"files.{k} missing")

    # 11. NO FABRICATED FIELDS
    if f.get("status") == "approved":
        c = f.get("construction") or {}
        if c.get("creator") in PLACEHOLDERS or c.get("date") in PLACEHOLDERS:
            err(fid, "approved figure still carries placeholder construction data")

print(f"Validated {len(figs)} register entries.\n")
for e in errors: print(e)
for w in warns:  print(w)
print(f"\n{len(errors)} error(s), {len(warns)} warning(s)")
if errors or ("--strict" in sys.argv and warns): sys.exit(1)
print("REGISTER OK")
