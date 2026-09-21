# Schemas

**Status:** REFERENCE · 2026-09-21

Machine-readable descriptions of the DUAP wire objects, in two forms:

| File | Describes |
|---|---|
| `duap-v1.cddl` | The **normative** CBOR grammar (RFC 8610), matching `specs/protocol-v0.1.md` |
| `*.schema.json` | JSON Schema (Draft 2020-12) for the **JSON view** of each object |

## Which is normative

**CDDL is.** DUAP objects are canonical CBOR; the JSON view exists so an
implementation without a CBOR library can read an object, and is described
in `specs/protocol-v0.1.md`. Where the two disagree the CDDL wins, and the
disagreement is a defect in this directory.

The JSON view is not merely a convenience: it is a second parser on the
trust path, which is why it has its own fuzz target and why VS-4 — an
unnecessary arbitrary-precision requirement — was found there.

## Why these are checked rather than published

A schema that nothing validates against drifts from the implementation
within a release or two and then actively misleads, which is worse than
having none. So `tools/check_schemas.py` validates **every committed
conformance vector's JSON view** against the schema for its object type,
and CI runs it. A schema that stops matching the vectors fails the build.

That check has a bound worth stating: it proves the schemas accept what the
reference implementation produces. It does not prove they *reject*
everything they should. Each schema therefore sets
`"additionalProperties": false` and constrains its value domains, so an
object carrying an unknown field fails — mirroring the strict decoder's
`deny_unknown_fields`, which is the behaviour an implementer needs to
reproduce.

## Regenerating

The CDDL and JSON Schemas are written by hand against
`specs/protocol-v0.1.md`, not generated from the Rust. That is deliberate:
generating them from the implementation would make them a description of
the code rather than a specification the code can be wrong about. The
check is what keeps them honest in the other direction.
