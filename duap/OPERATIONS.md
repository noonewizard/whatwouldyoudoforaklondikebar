# Operations

**Status:** SPECIFIED (design) / UNIMPLEMENTED · 2026-09-21

**Nothing here has been operated.** No DUAP component has run outside a
test process on a developer machine or CI runner. These are the runbooks
that would be needed, written from the design and the failure modes the
vertical slice exposed — not from operational experience, of which there
is none. Treat every procedure as untested.

## What is observable today

`duap-gateway::observe` exposes Prometheus text-format metrics and
structured JSON logs, and the nine pipeline stages are individually timed.
Structured logs deliberately exclude pseudonyms: an operational log that
accumulates subject pseudonyms is a re-identification surface, and it would
undermine the only unlinkability property the protocol provides.

| Signal | Why it matters |
|---|---|
| Stage timings | Locates a regression to a pipeline stage rather than to "ingest" |
| Rejection counts by reason | A spike in `DefaultEffect` means grants and traffic have diverged |
| Dedup hits | A spike means a reporter is retrying or replaying |
| Sequence gaps | **The suppression signal.** A reporter omitting events shows here and nowhere else |
| Double-count detections | Two reporters claiming one operation |
| Period-close duration | Grows with counter count, not event count |
| Log append rate and tree size | Capacity and, with PERF-01 open, exposure |

## Alerts worth waking someone for

Ordered by how bad the underlying condition is, not by how often it fires.

1. **Trial balance non-zero.** The ledger has lost or invented money.
   INV-L1 is violated, and `Ledger::post` is supposed to make it
   impossible. Stop settlement, do not correct by hand, and treat it as a
   critical defect.
2. **A receipt fails to verify against its own anchor.** Either the log or
   the receipt path is broken; this is the failure VS-1 was.
3. **Log consistency proof fails.** Either the log operator equivocated or
   the log is corrupt. Both are critical, and distinguishing them requires
   the auditors that are not implemented.
4. **Sequence gaps rising on a stream.** A reporter is suppressing,
   crashing, or its clock has moved. Benign and malicious look identical
   from here, which is a limitation of the mechanism rather than of the
   alert.
5. **Rejections rising.** Usually grants and traffic have diverged;
   occasionally an agent is misconfigured and generating garbage.
6. **Proof-endpoint latency.** While PERF-01 is open, this is a
   denial-of-service indicator rather than a performance one.

## Runbooks

Each says what to do and what not to do. The "do not" lines are the ones
that matter, because most of these failures have a tempting wrong fix.

### Trial balance non-zero

1. Stop settlement immediately. Do not net, do not pay out.
2. Identify the first period whose closing balance is non-zero.
3. Inspect the journal entries for that period. `Ledger::post` rejects
   unbalanced entries before mutating balances, so a non-zero balance means
   either a defect in that check or mutation outside it.
4. **Do not post a correcting entry to make the balance zero.** That
   destroys the evidence of the defect and leaves a system that is wrong
   and looks right.
5. Corrections, once the cause is understood, are reversing entries.
   History is never edited (INV-L5).

### A reporter's sequence has gaps

1. Confirm the gap is in the reporter's sequence and not in receipt order.
2. Ask the reporter for the missing sequence numbers. Legitimate causes —
   a crash between assigning a sequence and sending — are common.
3. Record the gap in the period's evidence regardless of the explanation.
   A gap that was explained is still a gap, and a later dispute will want
   to know it happened.
4. **Do not renumber.** The sequence is the evidence.

### A key is compromised

1. Revoke it in the key registry with reason `Compromise`. This is not
   cosmetic: `preserves_earlier_signatures()` is false for `Compromise` and
   `PolicyViolation`, so earlier signatures stop being honoured too.
2. Understand what that means before doing it — every event that key ever
   signed becomes unusable as evidence. That is the correct default for a
   compromised key and it is destructive.
3. Enrol a replacement and re-establish the reporting path.
4. Events signed during the compromise window cannot be rehabilitated.
   Whatever they were evidence of has to be established another way.

### Period close fails partway

`ClearingNode::close_period` prices counters, issues receipts, anchors
them, builds invoices and posts the ledger. A partial close is the worst
available state.

1. Do not retry blindly. Determine which stages completed.
2. Receipts already anchored are in the log permanently; the log is
   append-only and there is no un-anchoring.
3. Re-close must be idempotent with respect to already-issued receipts.
   **Whether the current implementation is idempotent here has not been
   tested**, and that is a gap worth a test before any deployment.

### The log operator may have equivocated

1. Collect signed tree heads from every auditor.
2. Check consistency between them.
3. If two heads are inconsistent, the log operator served different
   histories and the evidence of it is the two signed heads. Preserve both.
4. **This procedure requires auditors, and none is implemented.** Today
   there is no way to detect equivocation operationally, only in design.
   `DEPLOYMENT.md` says the same thing and it is the most important unbuilt
   component in the system.

## Capacity

Measured single-threaded ingest is 7,829 events/s
(`benchmarks/results/2026-09-21-ci-runner.md`). **No concurrency, network,
or durable-storage behaviour has been measured**, so this document states
no system capacity and no operator should plan from one. Rule 02 forbids
the multiplication that would produce a comfortable number.

## The honest summary

This file describes procedures for a system that has never been operated,
against failure modes inferred from its design and from four defects found
by running it once end to end. The runbooks most likely to be wrong are the
ones for failures that have never occurred — which is all of them.
