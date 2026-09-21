#!/usr/bin/env python3
"""Validate every committed conformance vector against the JSON Schemas.

A schema that nothing validates against drifts from the implementation
within a release or two and then actively misleads, which is worse than
having none. So the schemas are checked against the same vectors the
implementations are checked against.

What this proves: the schemas accept what the reference implementation
produces. What it does not prove: that they reject everything they should.
Each schema sets "additionalProperties": false and constrains its value
domains for that reason, and the negative cases below exercise a few of
them directly.

Run: python3 tools/check_schemas.py
"""
import copy
import json
import pathlib
import sys

try:
    from jsonschema import Draft202012Validator
    from referencing import Registry, Resource
except ImportError:
    print(
        "check_schemas: jsonschema is not installed; skipping.\n"
        "  pip install jsonschema",
        file=sys.stderr,
    )
    sys.exit(0)

ROOT = pathlib.Path(__file__).resolve().parent.parent
SCHEMA_DIR = ROOT / "schemas"
VECTOR_DIR = ROOT / "spec" / "vectors"

# Which schema validates which vector. A vector not listed here carries no
# JSON view of a kernel object (encoding, crypto and Merkle vectors).
VECTOR_SCHEMA = {
    "event/canonical": "event.schema.json",
    "authz/grant-canonical": "grant.schema.json",
    "receipt/canonical": "receipt.schema.json",
}


def registry() -> Registry:
    """Resolve the relative $refs between the schema files."""
    resources = []
    for path in sorted(SCHEMA_DIR.glob("*.schema.json")):
        schema = json.loads(path.read_text())
        resources.append((path.name, Resource.from_contents(schema)))
    return Registry().with_resources(resources)


def validator_for(name: str, reg: Registry) -> Draft202012Validator:
    schema = json.loads((SCHEMA_DIR / name).read_text())
    Draft202012Validator.check_schema(schema)
    return Draft202012Validator(schema, registry=reg)


def json_views() -> list[tuple[str, str, dict]]:
    """Every vector that carries a JSON view, with its id and file."""
    out = []
    for path in sorted(VECTOR_DIR.glob("*.json")):
        data = json.loads(path.read_text())
        for vector in data.get("vectors", []):
            inp = vector.get("input")
            if isinstance(inp, dict) and isinstance(inp.get("json_view"), dict):
                out.append((path.name, vector["id"], inp["json_view"]))
    return out


def negative_cases(views: dict[str, dict]) -> list[tuple[str, str, dict]]:
    """Mutations that every schema must reject.

    Without these, "the schemas validate the vectors" would be satisfied by
    a schema that accepts everything.
    """
    cases = []
    event = views.get("event/canonical")
    if event:
        unknown = copy.deepcopy(event)
        unknown["zz"] = 1
        cases.append(("event.schema.json", "unknown field", unknown))

        missing = copy.deepcopy(event)
        missing.pop("az")
        cases.append(("event.schema.json", "missing authorization", missing))

        bad_digest = copy.deepcopy(event)
        bad_digest["az"]["d"] = "not-a-digest"
        cases.append(("event.schema.json", "malformed digest", bad_digest))

        reserved = copy.deepcopy(event)
        reserved["xt"] = {"duap.reserved": 1}
        cases.append(("event.schema.json", "reserved extension key", reserved))

    receipt = views.get("receipt/canonical")
    if receipt:
        float_money = copy.deepcopy(receipt)
        float_money["ch"]["minor"] = 0.18
        cases.append(("receipt.schema.json", "money as a number", float_money))

    grant = views.get("authz/grant-canonical")
    if grant:
        long_notice = copy.deepcopy(grant)
        long_notice["rv"] = {"k": "after_notice", "h": 10_000}
        cases.append(("grant.schema.json", "notice beyond 30 days", long_notice))

    return cases


def main() -> int:
    reg = registry()
    validators = {name: validator_for(name, reg) for name in
                  sorted({v for v in VECTOR_SCHEMA.values()})}

    failures: list[str] = []
    checked = 0
    views: dict[str, dict] = {}

    for file, vid, view in json_views():
        views[vid] = view
        schema_name = VECTOR_SCHEMA.get(vid)
        if schema_name is None:
            continue
        errors = sorted(validators[schema_name].iter_errors(view), key=str)
        checked += 1
        for e in errors:
            where = "/".join(str(p) for p in e.absolute_path) or "(root)"
            failures.append(f"{file}:{vid} against {schema_name} at {where}: {e.message}")

    if checked == 0:
        failures.append("no vector was validated; the mapping in VECTOR_SCHEMA is stale")

    rejected = 0
    for schema_name, label, doc in negative_cases(views):
        if validators[schema_name].is_valid(doc):
            failures.append(
                f"{schema_name} ACCEPTED a document it must reject: {label}"
            )
        else:
            rejected += 1

    if failures:
        print("schema check failed:", file=sys.stderr)
        for f in failures:
            print(f"  {f}", file=sys.stderr)
        return 1

    print(
        f"schemas validate {checked} vector JSON view(s) and reject "
        f"{rejected} malformed variant(s)"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
