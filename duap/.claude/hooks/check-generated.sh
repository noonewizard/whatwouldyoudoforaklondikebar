#!/usr/bin/env bash
# Fail if a generated artefact is out of sync with its source.
#
# Enforces the "generated files are regenerated, not hand-edited" clause of
# .claude/rules/06-incremental-validation.md. The taxonomy is generated into
# four languages from one ontology; a hand-edit in one of them is an
# interoperability break that no test in that language would catch.
set -uo pipefail
cd "$(git rev-parse --show-toplevel 2>/dev/null || echo .)/duap" || exit 0

fail=0

if [ -f ontology/build_ontology.py ]; then
  if ! python3 ontology/build_ontology.py --check >/dev/null 2>&1; then
    echo "generated: ontology/duap-ontology-v1.json is stale; run ontology/build_ontology.py" >&2
    fail=1
  fi
fi

if [ -f ontology/gen_code.py ]; then
  if ! python3 ontology/gen_code.py --check >/dev/null 2>&1; then
    echo "generated: taxonomy code is stale; run ontology/gen_code.py" >&2
    fail=1
  fi
fi

exit $fail
