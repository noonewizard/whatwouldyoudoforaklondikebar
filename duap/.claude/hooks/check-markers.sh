#!/usr/bin/env bash
# Warn when a Rust source file declaring a public module has no maturity
# marker in its module documentation.
#
# Enforces .claude/rules/01-no-hallucinated-implementation.md. Advisory: it
# prints and returns 0, because a marker's honesty cannot be checked
# mechanically and a false block would train people to bypass hooks.
set -uo pipefail
cd "$(git rev-parse --show-toplevel 2>/dev/null || echo .)" || exit 0

# With --all, scan every tracked source file rather than the staged change,
# and fail rather than warn. The advisory posture is right for a commit
# hook, where a false block trains people to bypass it; in CI the file set
# is stable and a missing marker is simply a defect.
strict=0
if [ "${1:-}" = "--all" ]; then
  strict=1
  list=$(git ls-files 2>/dev/null | grep -E '^duap/crates/.*/src/.*\.rs$' || true)
else
  list=$(git diff --cached --name-only --diff-filter=ACM 2>/dev/null | grep -E '^duap/crates/.*/src/.*\.rs$' || true)
fi

missing=0
for f in $list; do
  [ -f "$f" ] || continue
  head -40 "$f" | grep -qE 'STATUS: (PRODUCTION|PRODUCTION[-_]CANDIDATE|PROTOTYPE|REFERENCE|EXPERIMENTAL|STUB|UNIMPLEMENTED)' && continue
  echo "markers: $f has no 'STATUS:' line in its module documentation" >&2
  missing=1
done
if [ "$missing" -ne 0 ]; then
  echo "See .claude/rules/01-no-hallucinated-implementation.md" >&2
  [ "$strict" -eq 1 ] && exit 1
fi
exit 0
