---
name: protocol-change
description: The procedure for changing anything that appears on the wire - canonical encodings, object schemas, domain labels, field names or signature inputs. Use before editing any encoded structure.
---

# Changing the protocol

A wire change is not a refactor. Every deployed implementation, every stored
receipt and every signature over the old form is affected.

## Before writing code

1. **Is it a wire change?** It is, if it alters: a field name, a field's
   presence or type, a domain label, the canonical encoding rules, a
   signature input, or the meaning of an existing field. Changing a comment
   is not; changing a validation rule usually is, because it changes which
   objects are acceptable.

2. **Which compatibility class?**

   | Class | Example | Requires |
   |---|---|---|
   | Additive | A new optional field with a new wire name | Minor version bump, and a version negotiation note, because strict readers reject unknown fields |
   | Semantic | An existing field means something new | New major version and new domain label |
   | Breaking | A field removed, renamed, or made required | New major version and new domain label |

   There is no silent semantic change. If old and new readers would disagree
   about an object's meaning, the domain label changes so signatures cannot
   cross versions.

3. **Write the ADR** (`.claude/rules/05-no-silent-architecture-change.md`).

## Making the change

4. Change the Rust model.
5. Regenerate the taxonomy if the ontology moved: `python3 ontology/gen_code.py`.
6. Regenerate vectors: `cargo run -p duap-conformance -- generate spec/vectors`.
7. **Read the vector diff.** Every changed byte should be explicable. An
   unexpected change means the edit did more than intended.
8. Update `specs/protocol-v0.1.md` and the field table in `PROTOCOL.md`.
9. Run the independent Go verifier. If it now fails, decide whether the
   verifier is behind or the specification is ambiguous. Prefer the second
   explanation: it has been right before.

## After the change

10. Add a migration note to `CHANGELOG.md` saying what an existing deployment
    must do.
11. If any stored object becomes unverifiable, say so explicitly, in the
    changelog and in the release notes. Do not let a deployment discover it.
