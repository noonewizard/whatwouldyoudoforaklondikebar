#!/usr/bin/env python3
"""Check that every test named in specs/invariants.md actually exists.

Enforces the rule in specs/invariants.md that an invariant with no test does
not belong there. Run in CI; exits non-zero on a dangling reference.
"""
import pathlib, re, subprocess, sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
SPEC = ROOT / "specs" / "invariants.md"


def cited_tests(text: str) -> list[tuple[str, str]]:
    """Return (invariant_id, test_name) pairs from 'Tested by' lines."""
    out = []
    current = "?"
    lines = text.splitlines()
    for i, line in enumerate(lines):
        m = re.match(r"^### (INV-[A-Z0-9]+)", line)
        if m:
            current = m.group(1)
        if "**Tested by:**" in line or "**Model:**" in line:
            block = line
            # Continuation lines are indented.
            j = i + 1
            while j < len(lines) and lines[j].startswith("  ") and "**" not in lines[j]:
                block += " " + lines[j].strip()
                j += 1
            if "**Model:**" in block:
                continue
            for name in re.findall(r"`([a-z0-9_]{4,})`", block):
                out.append((current, name))
    return out


def rust_test_names() -> set[str]:
    names: set[str] = set()
    for p in ROOT.rglob("*.rs"):
        if "/target/" in str(p):
            continue
        try:
            text = p.read_text()
        except OSError:
            continue
        names.update(re.findall(r"\bfn\s+([a-z0-9_]+)\s*\(", text))
    return names


def model_invariants() -> set[str]:
    names: set[str] = set()
    for p in (ROOT / "formal").rglob("*.tla"):
        try:
            text = p.read_text()
        except OSError:
            continue
        names.update(re.findall(r"^([A-Z][A-Za-z0-9_]*)\s*==", text, re.M))
    return names


def cited_models(text: str) -> list[tuple[str, str]]:
    out = []
    current = "?"
    for line in text.splitlines():
        m = re.match(r"^### (INV-[A-Z0-9]+)", line)
        if m:
            current = m.group(1)
        if "**Model:**" in line:
            for name in re.findall(r"`([A-Za-z][A-Za-z0-9_]+)`", line):
                if name.endswith(".tla"):
                    continue
                out.append((current, name))
    return out


def main() -> int:
    text = SPEC.read_text()
    tests = cited_tests(text)
    have = rust_test_names()
    missing = [(inv, t) for inv, t in tests if t not in have]

    models = cited_models(text)
    have_models = model_invariants()
    missing_models = [(inv, m) for inv, m in models if m not in have_models]

    print(f"invariants.md cites {len(tests)} tests and {len(models)} model invariants")
    ok = True
    for inv, t in missing:
        print(f"  MISSING TEST  {inv}: no `fn {t}` in the repository", file=sys.stderr)
        ok = False
    for inv, m in missing_models:
        print(f"  MISSING MODEL {inv}: no `{m} ==` in formal/", file=sys.stderr)
        ok = False
    if ok:
        print("  every cited test and model invariant exists")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
