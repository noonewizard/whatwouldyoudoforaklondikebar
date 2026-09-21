#!/usr/bin/env bash
# Model-check every TLA+ specification and assert the expected outcome.
#
# STATUS: PRODUCTION (CI gate).
#
# Two of the three checks must pass. The third -- NonVacuity -- must FAIL:
# it asserts that no Permit is ever reachable, and a counterexample is the
# evidence that the real invariants are not holding vacuously. A model whose
# invariants are true because nothing interesting happens reassures nobody.
set -uo pipefail
cd "$(dirname "$0")"

TLA_VERSION="v1.7.4"
TLA_SHA256="936a262061c914694dfd669a543be24573c45d5aa0ff20a8b96b23d01e050e88"
JAR="${TLA_JAR:-tla2tools.jar}"

if [ ! -f "$JAR" ]; then
  echo "fetching tla2tools $TLA_VERSION"
  curl -sSL -o "$JAR" \
    "https://github.com/tlaplus/tlaplus/releases/download/$TLA_VERSION/tla2tools.jar" || {
      echo "formal: could not fetch tla2tools; set TLA_JAR to a local copy" >&2
      exit 2
    }
fi

have=$(sha256sum "$JAR" | cut -d' ' -f1)
if [ "$have" != "$TLA_SHA256" ]; then
  echo "formal: tla2tools.jar digest $have does not match the pinned $TLA_SHA256" >&2
  exit 2
fi

fail=0
run() { # name, config, expect_pass(0|1)
  local name="$1" cfg="$2" expect="$3"
  rm -rf states
  local out
  out=$(timeout 600 java -XX:+UseParallelGC -cp "$JAR" tlc2.TLC \
        -workers 2 -config "$cfg" -cleanup "$name.tla" 2>&1)
  local code=$?
  local states
  states=$(printf '%s' "$out" | grep -oE '[0-9,]+ distinct states found' | tail -1)
  if [ "$expect" -eq 0 ]; then
    if [ $code -ne 0 ]; then
      echo "FAIL $name ($cfg): expected no violation, TLC exited $code" >&2
      printf '%s\n' "$out" | grep -E "Error|Invariant" | head -5 >&2
      fail=1
    else
      echo "ok   $name ($cfg): no violation, $states"
    fi
  else
    if [ $code -eq 0 ]; then
      echo "FAIL $name ($cfg): expected a counterexample and found none" >&2
      fail=1
    else
      echo "ok   $name ($cfg): counterexample found as required"
    fi
  fi
}

run Authorization Authorization.cfg 0
run Authorization NonVacuity.cfg 1
run Accounting Accounting.cfg 0

rm -rf states
exit $fail
