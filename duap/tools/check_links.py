#!/usr/bin/env python3
"""Check that every repository-relative path referenced in code or docs exists.

Enforces the documentation rule against dangling references: a citation
that cannot be followed is worse than none, because a reader assumes it was
checked.
"""
import pathlib, re, sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
SKIP_DIRS = {"target", "node_modules", ".git", "states"}
# A reference that is deliberately a template rather than a path. `NNNN` is
# the ADR naming convention's placeholder for a number that does not exist
# yet, and flagging it as dangling would train readers to ignore this
# checker -- which is the failure mode it exists to prevent.
PLACEHOLDERS = re.compile(r"NNNN|<[a-z-]+>|\{[a-z_]+\}")
PATTERN = re.compile(
    r"(?<![\w./-])((?:docs|specs|spec|schemas|formal|security|benchmarks|research|"
    r"crates|sdk|gateway|ontology|infrastructure|examples|tools|compliance|simulations)"
    r"/[A-Za-z0-9_./-]+\.(?:md|rs|go|py|ts|json|tla|cfg|toml|sh|yml|yaml))"
)
TOP_LEVEL = re.compile(r"`([A-Z_]+\.md)`")


def files():
    for p in ROOT.rglob("*"):
        if any(part in SKIP_DIRS for part in p.parts):
            continue
        if p.is_file() and p.suffix in {".md", ".rs", ".go", ".py", ".ts", ".toml", ".sh"}:
            yield p


def main() -> int:
    missing: dict[str, set[str]] = {}
    for p in files():
        try:
            text = p.read_text()
        except (OSError, UnicodeDecodeError):
            continue
        refs = set(PATTERN.findall(text)) | set(TOP_LEVEL.findall(text))
        for r in refs:
            if r.endswith("/") or PLACEHOLDERS.search(r):
                continue
            if not (ROOT / r).exists():
                missing.setdefault(r, set()).add(str(p.relative_to(ROOT)))
    if missing:
        print(f"{len(missing)} dangling reference(s):", file=sys.stderr)
        for r, where in sorted(missing.items()):
            src = ", ".join(sorted(where)[:3])
            print(f"  {r}  <- {src}", file=sys.stderr)
        return 1
    print("every referenced path exists")
    return 0


if __name__ == "__main__":
    sys.exit(main())
