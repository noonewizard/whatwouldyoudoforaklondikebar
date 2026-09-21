#!/usr/bin/env python3
"""Fail if a production crate enables an insecure-by-design feature.

VS-5: a constant subject root secret makes per-controller pseudonyms
identical across controllers, so two controllers can join their records on
the pseudonym alone -- destroying the only unlinkability the protocol
offers. `duap-model`'s `insecure-fixed-secret` feature gates the type that
allows one, so a crate that has not opted in cannot construct it.

That control is only as good as the opt-in list. This script is what stops
the list growing quietly: a crate may enable the feature as a
dev-dependency (tests), or be on the explicit allowlist below (the
demonstration and the benchmark harness, which are not libraries anyone
deploys). Anything else fails.

Run: python3 tools/check_insecure_features.py
"""
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent

GATED = ["insecure-fixed-secret"]

# Crates permitted to enable a gated feature as a *normal* dependency.
# Each needs a reason, and the reason has to be that nothing deploys it.
ALLOWED = {
    "duap-demo": "a synthetic demonstration whose output must be deterministic",
    "duap-bench": "a measurement harness; determinism keeps runs comparable",
    "duap-model": "declares the feature, and self-references it for its own tests",
}


def sections(text: str) -> list[tuple[str, str]]:
    """Split a Cargo.toml into (section name, body) pairs."""
    out, name, buf = [], "", []
    for line in text.splitlines():
        m = re.match(r"\s*\[([^\]]+)\]\s*$", line)
        if m:
            out.append((name, "\n".join(buf)))
            name, buf = m.group(1), []
        else:
            buf.append(line)
    out.append((name, "\n".join(buf)))
    return out


def main() -> int:
    problems: list[str] = []
    for toml in sorted(ROOT.glob("crates/*/Cargo.toml")):
        crate = toml.parent.name
        text = toml.read_text()
        for section, body in sections(text):
            # dev-dependencies never ship, so they are always fine.
            if "dev-dependencies" in section:
                continue
            if "dependencies" not in section and section != "features":
                continue
            for feature in GATED:
                if feature in body and crate not in ALLOWED:
                    problems.append(
                        f"{crate} enables '{feature}' in [{section or 'root'}]. "
                        f"Move it to [dev-dependencies], or add {crate} to "
                        f"ALLOWED in this script with a reason it is never "
                        f"deployed."
                    )
    if problems:
        print("insecure feature enabled by a production crate:", file=sys.stderr)
        for p in problems:
            print(f"  {p}", file=sys.stderr)
        print("See VS-5 in security/findings.md.", file=sys.stderr)
        return 1
    allowed = ", ".join(sorted(ALLOWED))
    print(f"no production crate enables a gated feature (allowed: {allowed})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
