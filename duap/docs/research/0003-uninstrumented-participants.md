# 0003 — What happens when an organisation refuses to participate?

**Status:** analysed; no technical answer exists · 2026-09-21

## Problem

DUAP accounts for what it is told. An organisation that declines to
instrument, or that copies data out of an instrumented system and processes
it elsewhere, generates no events. No amount of cryptography detects
processing that happens entirely outside the protocol.

The question is what, if anything, closes that gap, and what is honestly
outside software's reach.

## Known work

The analogous problems are well studied outside computing: tax
self-assessment, emissions reporting, financial audit, and royalty
statements in music and publishing. In every case the reporting mechanism
is not what produces compliance; audit, penalty and the cost of being
caught are. The technical literature on remote attestation addresses a
narrower question (is *this* machine running *that* code) and does not
generalise to "is this organisation reporting everything".

## Hypothesis

There is no purely technical mechanism that compels an organisation to
report activity occurring outside the protocol. The available levers are
detection at the margin, and non-technical enforcement.

## Experiment

Enumerate the proposed levers and classify each as technical enforcement,
detection, or non-technical enforcement. Implement the detection ones.

| Lever | Class | Status here |
|---|---|---|
| Per-stream sequence numbers | detection (partial) | Implemented; detects gaps within a started stream, not a stream never started |
| Cross-checking a subject's receipts against their own records | detection | Possible: `Receipt::verify_coverage` and the subject dashboard |
| Audit sampling against the controller's systems | non-technical | Out of scope; an engagement, not a feature |
| Contractual requirement in procurement | non-technical | Out of scope; the protocol makes the obligation checkable |
| Browser or OS-level enforcement | partial technical | Would cover client-side collection only; server-side processing is invisible to it |
| App-store policy | non-technical, with technical leverage | Same limitation |
| Regulation | non-technical | The protocol supplies evidence, not compulsion |
| Data-access gateways that mediate every read | technical | Would work, and requires custody — rejected in ADR-0001 |
| Industry consortium requirement | non-technical | Governance question |

## Result

Only three levers are technical, and each covers a proper subset:
sequence gaps (partial suppression within an instrumented stream),
subject-side cross-checking (usage the subject can observe independently),
and a mediating gateway (which requires the custody model ADR-0001
rejects).

## Conclusion

The protocol must state this boundary rather than obscure it. Accepted as
RISK-02 in `security/findings.md`, recorded as T-13 in `THREAT_MODEL.md`,
and stated in `docs/architecture/minimal-protocol.md` §8.

The useful framing is that DUAP changes what non-participation *costs*,
not whether it is possible. An organisation that cannot produce receipts
cannot demonstrate authorised use; whether that matters depends on whether
anyone requires the demonstration, which is a procurement and regulatory
question rather than a protocol one.

## Remaining uncertainty

Whether a partial-adoption equilibrium is stable. If instrumented
organisations bear a cost that uninstrumented ones avoid, adoption may
select against participation — the classic problem of voluntary reporting
regimes. Modelling that is `simulations/` work that has not been done.
