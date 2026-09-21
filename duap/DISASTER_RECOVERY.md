# Disaster recovery

**Status:** SPECIFIED (design) / UNIMPLEMENTED · 2026-09-21

**No recovery procedure here has been executed, and no backup of anything
has ever been taken.** This is a design document. Its most useful content
is §1, which says which losses are recoverable and which are not, because
that distinction constrains the architecture rather than the operations.

## 1. What is recoverable and what is not

| Lost | Recoverable? | Why |
|---|---|---|
| Event store | **Yes**, if reporters retain their signed events | Events are self-authenticating: a signed envelope verifies without the store it came from. Re-ingest is idempotent through the dedup index keyed on canonical digest |
| Counters and aggregates | **Yes** | Derived from events. Recompute |
| Ledger | **Yes** | Derived from priced counters and receipts. Recompute, and check the trial balance is zero — which is a genuine verification of the recovery, not a formality |
| Receipts | **Yes**, by the issuer | A clearing node can reissue from events. The reissued receipt has a new anchor, which is visible and correct |
| Transparency log | **Partially, and this is the hard one** | See §2 |
| Clearing node signing key | **No** | Every receipt it signed remains verifiable by anyone holding the public key, but no new receipt can be signed as that identity. See §3 |
| Pseudonym root secret | **No, and the consequence is permanent** | See §4 |

The pattern is that everything derived is recoverable and everything
attested is not. That is a property of the design and it is the right way
round.

## 2. Transparency log loss

The log is append-only and its value is that its history cannot change. A
log that is lost and rebuilt is a different log, and every inclusion proof
issued against the old one stops verifying.

**Mitigation that must exist before any deployment:** signed tree heads
published to, and retained by, independent auditors. A rebuilt log can then
be checked for consistency with the last widely witnessed head, which
bounds what could have been altered to the entries after it.

**Without auditors — the current state — a lost log is unrecoverable
evidence.** Receipts remain individually verifiable by signature, but
nothing establishes when they were committed, so back-dating becomes
undetectable. This is the second place in this repository where the absence
of an auditor implementation is the binding constraint, and it is the same
absence `DEPLOYMENT.md` and `OPERATIONS.md` name.

**Never:** rebuild a log and present it as the original. A rebuilt log is a
new log with a new identity, and saying so is the whole of the honest
procedure.

## 3. Clearing node key loss

Receipts already signed stay verifiable forever by anyone with the public
key; that is what a signature is for. What is lost is the ability to issue
new receipts as that identity.

Procedure: enrol a new key, publish the transition in the log so the change
is itself evidence, and continue. Do **not** re-sign historical receipts
with the new key — that would create two differently-signed receipts for
one set of events, which is indistinguishable from equivocation and is
exactly what an adversary would do.

## 4. Pseudonym root secret loss

The permanent one, and the reason it is worth its own section.

Per-controller pseudonyms are derived from a subject's root secret. Lose
it and the subject can no longer derive the pseudonyms that link them to
their own historical receipts. The receipts remain valid; the subject can
no longer prove which are theirs.

**There is no recovery.** Escrow trades the loss risk for a disclosure
risk — an escrow holder can derive every pseudonym for every controller
and link them, destroying the property the pseudonyms exist to provide.
Which risk to prefer is a deployment decision and it must be made
explicitly rather than by omission.

**The adjacent failure, now closed (VS-5):** a *constant* root secret
would make pseudonyms linkable across controllers from the start, which is
worse than losing one. `InsecureFixedSecret` is now feature-gated, so a
crate that has not opted in cannot construct one and the compiler refuses
the mistake.

## 5. Objectives

**No RTO or RPO is stated**, because stating one for a system that has
never run would be fiction. What can be said is the structure any real
objective must respect:

- Recovery time for derived state is bounded by re-ingest throughput,
  measured at 7,829 events/s single-threaded with no concurrency measured.
- Recovery point for events is bounded by reporters' retention, which is
  outside the operator's control and must be a contractual term.
- The transparency log has no recovery point without auditors, and
  therefore no meaningful objective either.

## 6. What would have to be built

1. A log auditor, and a second run by someone else. Everything else on this
   page is secondary to it.
2. Backup and restore for the event store, with a restore that has actually
   been executed.
3. A tested recompute path for counters, ledger and receipts, ending in a
   trial-balance check.
4. A key-transition procedure published in the log.
5. A decision, recorded as an ADR, on pseudonym root-secret escrow.

Items 1 and 5 are architectural. The rest are operational and cheaper.
