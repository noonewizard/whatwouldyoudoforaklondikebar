#!/usr/bin/env bash
# Block anything that looks like real personal data.
#
# Enforces .claude/rules/07-no-personal-data.md. A coarse filter by design:
# it catches the accidental paste, not a determined author. Reviewers and
# red-team-engineer carry the rest of the rule.
#
# Exit 0 to allow, 2 to block.
set -uo pipefail
cd "$(git rev-parse --show-toplevel 2>/dev/null || echo .)" || exit 0

# Two modes. With no argument it scans the staged change, which is what the
# pre-commit hook wants. With --all it scans every tracked file, which is
# what CI wants: a hook that only ever sees diffs never notices what landed
# before it existed.
if [ "${1:-}" = "--all" ]; then
  files=$(git ls-files 2>/dev/null || true)
else
  files=$(git diff --cached --name-only --diff-filter=ACM 2>/dev/null || true)
fi
[ -z "$files" ] && exit 0

# Only inspect this project's files.
files=$(printf '%s\n' "$files" | grep -E '^duap/|^\.claude/' || true)
[ -z "$files" ] && exit 0

fail=0
report() { echo "no-personal-data: $1" >&2; fail=1; }

for f in $files; do
  [ -f "$f" ] || continue
  case "$f" in
    *spec/vectors/*|*/golden/*|*.lock) continue ;;
  esac

  # Email addresses that are not obviously synthetic.
  # The optional `synthetic:` prefix is captured so that a synthetic address
  # on the same line as a real one does not mask the real one.
  if grep -oE '(synthetic:)?[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}' "$f" 2>/dev/null \
      | grep -v '^synthetic:' \
      | grep -vE '@(example\.(com|org|net)|duap\.dev|invalid|localhost|test|noreply\.anthropic\.com|anthropic\.com)' \
      | head -3 | grep -q .; then
    report "$f contains an email address that is not in a reserved example domain"
  fi

  # National identifier shapes.
  if grep -nE '\b[0-9]{3}-[0-9]{2}-[0-9]{4}\b' "$f" 2>/dev/null | head -1 | grep -q .; then
    report "$f contains something shaped like a US social security number"
  fi

  # Payment card shapes. The lookarounds exclude digit runs that are part
  # of a longer number: `\b` matches after a decimal point, so a float like
  # 2.5560241107087633 in a results file reads as a Mastercard prefix. That
  # false positive was real -- found by running this hook over the whole
  # tree rather than over a staged diff -- and a filter that cries wolf on
  # research output is a filter people learn to bypass.
  if grep -nP '(?<![\d.])(4\d{12}(\d{3})?|5[1-5]\d{14}|3[47]\d{13})(?![\d.])' "$f" 2>/dev/null | head -1 | grep -q .; then
    report "$f contains something shaped like a payment card number"
  fi

  # Public IPv4 addresses (loopback, RFC 1918 and documentation ranges are fine).
  if grep -nEo '\b([0-9]{1,3}\.){3}[0-9]{1,3}\b' "$f" 2>/dev/null \
      | grep -vE '\b(0\.0\.0\.0|127\.|10\.|192\.168\.|172\.(1[6-9]|2[0-9]|3[01])\.|169\.254\.|255\.|224\.|192\.0\.2\.|198\.51\.100\.|203\.0\.113\.|1\.2\.3\.4)' \
      | head -1 | grep -q .; then
    report "$f contains a public IPv4 address; use a documentation range"
  fi
done

if [ "$fail" -ne 0 ]; then
  echo "" >&2
  echo "Blocked by .claude/rules/07-no-personal-data.md." >&2
  echo "Use synthetic fixtures. If a value is genuinely synthetic, prefix it 'synthetic:'" >&2
  echo "or use a reserved example domain." >&2
  exit 2
fi
exit 0
