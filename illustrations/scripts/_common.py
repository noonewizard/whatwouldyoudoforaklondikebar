import os, sys, yaml
ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
REPO = os.path.abspath(os.path.join(ROOT, ".."))
def figures():
    return yaml.safe_load(open(os.path.join(ROOT, "register", "illustrations.yml")))["figures"]
def report(errs, warns=(), label=""):
    for e in errs: print(f"ERROR  {e}")
    for w in warns: print(f"WARN   {w}")
    print(f"{label}: {len(errs)} error(s), {len(warns)} warning(s)")
    sys.exit(1 if errs else 0)
def figdir(f):
    return os.path.join(ROOT, f"book-{f['book']:02d}", f["figure_id"])
