#!/usr/bin/env python3
"""Negative controls for validate_register.py.

A validator that has never rejected anything is not known to work. Each case
below is a specific misrepresentation the system exists to prevent; the test
asserts the validator catches it.
"""
import subprocess, sys, tempfile, os, shutil, yaml

BASE = yaml.safe_load(open(os.path.join(os.path.dirname(__file__),"..","register","illustrations.yml")))

CASES = [
  ("analytical model claiming documented evidence",
   lambda f: f[0].update(evidence_status="documented")),
  ("evidence-bearing figure with no sources",
   lambda f: (f[1].update(figure_class="evidence-bearing", evidence_status="supported"), f[1].update(sources=[]))),
  ("missing does_not_show",
   lambda f: f[0].pop("does_not_show")),
  ("AI image used for an evidence-bearing figure",
   lambda f: f[0].update(figure_class="evidence-bearing", evidence_status="documented",
        ai_generation={"used":True,"tool":"X","prompt":"p","date":"2026-01-01",
                       "human_edits":"none","role":"illustration"})),
  ("ai_generation.role set to 'evidence'",
   lambda f: f[0].update(figure_class="atmospheric", evidence_status="conceptual",
        ai_generation={"used":True,"tool":"X","prompt":"p","date":"2026-01-01",
                       "human_edits":"none","role":"evidence"})),
  ("computational figure with no seed",
   lambda f: f[1]["construction"].pop("random_seed")),
  ("alt text duplicating the caption",
   lambda f: f[0].update(alt_text=f[0]["caption"])),
  ("duplicate figure_id",
   lambda f: f[1].update(figure_id=f[0]["figure_id"])),
  ("rights holder missing",
   lambda f: f[0]["rights"].pop("holder")),
  ("approved figure still carrying TBD creator",
   lambda f: f[0].update(status="approved")),
]

tmp = tempfile.mkdtemp()
os.makedirs(os.path.join(tmp,"register"))
shutil.copy(os.path.join(os.path.dirname(__file__),"validate_register.py"), tmp)
passed = failed = 0
for name, mutate in CASES:
    import copy
    data = copy.deepcopy(BASE)
    try: mutate(data["figures"])
    except Exception as e: print(f"  SETUP FAIL {name}: {e}"); failed += 1; continue
    with open(os.path.join(tmp,"register","illustrations.yml"),"w") as fh:
        yaml.safe_dump(data, fh)
    r = subprocess.run([sys.executable, os.path.join(tmp,"validate_register.py")],
                       capture_output=True, text=True)
    if r.returncode != 0:
        print(f"  CAUGHT   {name}"); passed += 1
    else:
        print(f"  MISSED   {name}  <-- validator gap"); failed += 1

# positive control: the real register must still pass
r = subprocess.run([sys.executable, os.path.join(os.path.dirname(__file__),"validate_register.py")],
                   capture_output=True, text=True)
if r.returncode == 0: print("  PASSES   unmodified register (positive control)"); passed += 1
else: print("  BROKEN   unmodified register fails <-- false positive"); failed += 1

print(f"\n{passed} passed, {failed} failed")
shutil.rmtree(tmp)
sys.exit(1 if failed else 0)
