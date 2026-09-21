# Formal models

**Status:** REFERENCE · 2025-09-21

Two TLA+ specifications, both model-checked exhaustively in CI by
`check.sh`.

| Model | Checks | Invariants |
|---|---|---|
| `Authorization.tla` | the combining algorithm and revocation semantics | INV-A1, A2, A4, A5 |
| `Accounting.tla` | double entry and the rounding boundary | INV-L1, L2, L3 |

## What is modelled, and what is not

This is the part of a formal-methods claim that is usually left out, so it
is stated first.

### Authorization

**Modelled:** deny-overrides combining; the default-deny fallback; grant
epochs and amendment; revocation with an effective time and a purpose
scope; the rule that a revocation against epoch *n* binds every epoch from
*n* onwards.

**Not modelled:** the matcher language (classes and purposes are flat sets,
not the selector algebra with namespaces, tiers and lattice descent);
obligations of any kind; signature verification; the grant-digest binding;
pricing selection; time beyond a three-tick clock.

**Bounds:** two data classes, two purposes, three epochs, three clock ticks,
at most one outstanding revocation. Exhaustive within those bounds: 15,237
distinct states, zero left on the queue.

### Accounting

**Modelled:** balanced postings; reversal as a new entry; accrual at the
computation scale; settlement into whole units with the remainder carried.

**Not modelled:** multiple currencies; invoice structure; tax; disputes;
netting; the account taxonomy beyond four accounts.

**Bounds:** amounts in −3..3, three entries, accruals to 25. Exhaustive:
1,920 distinct states.

## Non-vacuity

`NonVacuity.cfg` asserts `NoPermitIsEverReachable`, which the model
**violates**. That violation is the point: it demonstrates that the
authorization model can reach a Permit, so the four real invariants are not
true merely because nothing happens. `check.sh` fails if this check ever
*passes*.

## Mirroring

Every model invariant has a Rust test of the same name, in
`crates/duap-auth/tests/model_mirror.rs` and
`crates/duap-ledger/tests/model_mirror.rs`. The mirror tests exist so that a
change to the implementation that breaks a modelled property fails the
ordinary test suite, not only the model check that a contributor might not
run. `tools/check_invariant_tests.py` fails if `specs/invariants.md` cites a
test or a model invariant that does not exist.

## What this does and does not establish

It establishes that the *algorithms as modelled* have the stated properties
within the stated bounds.

It does **not** establish that the Rust implements the model. Nothing here
is extracted from or verified against the source. The mirror tests are
evidence of agreement on specific cases, not a refinement proof. Anyone
reading a claim of "formally verified" into this is reading more than is
written: the honest statement is *"the authorization and accounting
algorithms are model-checked; the implementation is tested against the same
invariants"*.

## Running

```
./check.sh                 # fetches a pinned tla2tools.jar and runs all three
TLA_JAR=/path/to.jar ./check.sh
```
