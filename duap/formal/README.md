# Formal models

**Status:** REFERENCE · 2026-09-21

Three TLA+ specifications, all model-checked exhaustively in CI by
`check.sh`.

| Model | Checks | Invariants |
|---|---|---|
| `Authorization.tla` | the combining algorithm and revocation semantics | INV-A1, A2, A4, A5 |
| `Accounting.tla` | double entry and the rounding boundary | INV-L1, L2, L3 |
| `Negotiation.tla` | the authorization negotiation state machine | INV-N1 .. N6 |

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

### Negotiation

**Modelled:** the six states and every transition; the digest chain, so an
offer names the request it answers and an acceptance names the offer it
accepts; counter-offers superseding the live offer; replay rejection;
expiry, with rejection exempt because a party may always refuse.

**Not modelled:** signatures and envelopes — every message is assumed
authenticated, so the model says nothing about forgery; the content of
terms, prices and matchers, so it cannot say whether an offer is sensible,
only whether the exchange is well-formed; transport, retransmission and
message loss; concurrent negotiations.

**Bounds:** two counter-offers, a three-tick clock. Exhaustive: 244
distinct states, zero left on the queue.

**Deadlock checking is off, deliberately.** A negotiation that has reached
Granted or Closed with the clock exhausted has no enabled action, which TLC
reports as deadlock. Here termination is the intended outcome and
`TerminalIsAbsorbing` is the invariant that asserts it on purpose.

## Non-vacuity

Two configurations assert that a desirable outcome is unreachable, and both
must **fail**:

| Config | Asserts | Demonstrates when it fails |
|---|---|---|
| `NonVacuity.cfg` | no Permit is ever reachable | the authorization model can reach a Permit |
| `Negotiation_NonVacuity.cfg` | no Grant is ever reachable | the negotiation model can reach a Grant |

The violations are the point: without them the real invariants could be
true merely because nothing happens. `check.sh` fails if either check ever
*passes*.

## Mirroring

Every model invariant has a Rust test of the same name, in
`crates/duap-auth/tests/model_mirror.rs`,
`crates/duap-auth/tests/negotiation_model_mirror.rs` and
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
./check.sh                 # fetches a pinned tla2tools.jar and runs all five
TLA_JAR=/path/to.jar ./check.sh
```
