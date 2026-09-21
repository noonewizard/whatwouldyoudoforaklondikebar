# Contributing

**Status:** REFERENCE · 2026-09-21

## Before anything else

Read `.claude/rules/00-index.md`. Ten rules bind every change, and each
exists because it prevents a failure characteristic of this kind of
project — infrastructure making strong claims about cryptography, privacy,
economics and regulation, where an unearned claim is worse than a missing
feature.

A rule is not advice. If a change cannot satisfy a rule, the change does
not land. If the rule is wrong, it is changed by an ADR, not by exception.

## The loop

```
implement -> compile -> test -> attack -> benchmark -> document -> integrate
```

`attack` comes before `benchmark` deliberately: optimising code that is
wrong wastes the optimisation, and a security fix usually changes the
performance profile.

## What every change must satisfy

1. **`cargo test --workspace` passes.** A commit that leaves it failing is
   a defect regardless of what follows.
2. **Tests land with the code**, in the same commit. "Tests to follow"
   means the design has not been validated.
3. **Every public item carries a maturity marker** from rule 01's set, and
   a marker is never raised in the same change that implements the
   functionality.
4. **Performance claims cite a result file** in `benchmarks/results/`.
   "Fast", "scalable" and "low-overhead" are performance claims; either
   cite a measurement or delete the word.
5. **Security claims carry the five-part argument** from rule 03: the
   property stated precisely enough to be false, the adversary, the
   assumption, what it does not cover, and the test or proof.
6. **Regulatory statements cite a provision**, not an instrument, and are
   marked `technical` or `interpretation-required`.
7. **No real personal data**, anywhere, including commit messages. Fixtures
   are synthetic and recognisably so; stand-ins are prefixed `synthetic:`.
8. **An ADR for anything in rule 05's list** — wire format, canonical
   encoding, crate dependencies, invariants, security boundaries, economic
   semantics, maturity claims. An ADR whose "Rejected alternatives" section
   is empty has not been thought through.
9. **Status changes go in their own commit** with the evidence in the
   message (rule 10).

## Changing the protocol

The wire format is not frozen, and it is still the most expensive thing to
get wrong.

1. Write the ADR first.
2. Change the specification in `specs/protocol-v0.1.md`.
3. Change the implementation.
4. Regenerate the vectors:
   `cargo run -p duap-conformance -- generate spec/vectors`.
   **Never hand-edit a vector.**
5. Update the Go implementation in `gateway/` if the change is in L1–L3.
   If you cannot, say so in the ADR — a change one implementation cannot
   follow is a warning about the change.
6. Update `specs/invariants.md` if an invariant moved, and
   `tools/check_invariant_tests.py` will fail if a cited test does not
   exist.

`crates/duap-conformance/tests/vectors_test.rs` fails the build if a
committed vector stops matching the generator, which is the intended
friction.

## Architecture authority

No agent or contributor changes an interface another owns. Propose; the
owner decides; an unresolved disagreement becomes an ADR. Editing another
subsystem directly to get around this produces two architectures that both
work locally and cannot be reconciled, which is the specific failure rule
09 exists to prevent.

The red team may write a failing test anywhere to demonstrate a finding. It
may not write the fix.

## Running the checks

```
cargo test --workspace                      # 264 tests
cargo run -p duap-conformance -- check      # 93 vectors
cargo run --release -p duap-bench           # benchmarks; writes nothing
./formal/check.sh                           # 5 TLA+ configurations
python3 tools/check_links.py                # intra-repository references
python3 tools/check_invariant_tests.py      # invariants cite real tests
cd gateway && go test ./...                 # the independent implementation
```

`formal/check.sh` expects three configurations to pass and two to **fail**:
the non-vacuity checks assert that a desirable outcome is unreachable, and
their counterexamples are the evidence that the real invariants are not
holding vacuously.

## Writing

The audience is cryptographers, distributed-systems engineers, economists,
privacy engineers, regulators, standards bodies, security auditors and
open-source maintainers. Write for the most sceptical of them.

State the limitation next to the capability, in the same paragraph — not in
an appendix nobody reads. Explain why an alternative was rejected, not only
what was chosen. Cite primary sources by number and provision. Mark
uncertainty as uncertainty: "we have not measured this" and "this has not
been audited" are expected. No marketing register.

A citation that cannot be verified is deleted, not approximated. Where a
source could not be retrieved, say so next to the claim.

## Reporting a security finding

Open an entry in `security/findings.md` with a reproduction, a severity,
and an owner. A finding is never closed because it is "documented as a
limitation" unless the documentation is accurate, prominent, and in the
artefact a reader would actually consult.

An inflated maturity marker is a finding. VS-8 was exactly that.
