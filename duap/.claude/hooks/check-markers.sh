#!/usr/bin/env bash
# Warn when a Rust source file declaring a public module has no maturity
# marker in its module documentation.
#
# Enforces .claude/rules/01-no-hallucinated-implementation.md. Advisory: it
# prints and returns 0, because a marker's honesty cannot be checked
# mechanically and a false block would train people to bypass hooks.
set -uo pipefail
cd "$(git rev-parse --show-toplevel 2>/dev/null || echo .)" || exit 0

missing=0
for f in $(git diff --cached --name-only --diff-filter=ACM 2>/dev/null | grep -E '^duap/crates/.*/src/.*\.rs$' || true); do
  [ -f "$f" ] || continue
  head -40 "$f" | grep -qE 'STATUS: (PRODUCTION|PRODUCTION-CANDIDATE|REFERENCE|EXPERIMENTAL|STUB|UNIMPLEMENTED)' && continue
  echo "markers: $f has no 'STATUS:' line in its module documentation" >&2
  missing=1
done
[ "$missing" -ne 0 ] && echo "See .claude/rules/01-no-hallucinated-implementation.md" >&2
exit 0
