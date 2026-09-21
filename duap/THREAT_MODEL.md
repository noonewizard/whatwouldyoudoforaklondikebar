# Threat model

**Status:** PRODUCTION (process artefact) · 2026-09-21 · protocol `DUAP/1`

This document enumerates adversaries and threats, and for each records the
control and — crucially — whether the attack is **prevented**, **detected**,
**deterred** or **accepted**. Most transparency-log properties are
detection, not prevention, and writing "prevented" where detection is meant
is the commonest error in this field.

Open findings live in `security/findings.md`. Security arguments live in
`SECURITY.md`. The procedure for adding to this document is
`.claude/skills/threat-modelling/SKILL.md`.

## Trust boundaries

```
  subject agent          controller / agent            clearing node
  ───────────────        ─────────────────────         ──────────────────
  root secret            data (never leaves)           event store
  grant signing          event signing                 registry, log, ledger
        │                        │                            │
        └──── signed grant ──────┴──── signed events ─────────┤
                                                              │
                              signed receipts, invoices ──────┤
                                                              │
          auditors ◀── log heads, proofs, receipts ───────────┘
```

Four boundaries: agent→node (events), subject→node (grants and
revocations), node→relying party (receipts and heads), node→rail
(settlement). Each is crossed only by signed, canonically encoded objects.

## Adversaries

| Adversary | Capability | Motive |
|---|---|---|
| A-1 Dishonest controller | Signs anything with its own key; controls its own instrumentation | Under-report usage, over-state authorization |
| A-2 Dishonest processor | As A-1, acting for a controller | Claim usage it did not perform; double-report |
| A-3 Dishonest subject | Signs grants and revocations; controls its own agent | Claim usage that did not occur; deny usage that did |
| A-4 Dishonest clearing node | Signs receipts and log heads; sees all events | Fabricate, omit, equivocate, misprice |
| A-5 Network adversary | Observes, replays, delays, drops | Replay, censor, correlate |
| A-6 Colluding organisations | Two or more of the above | Inflate or suppress jointly |
| A-7 Curious observer | Sees the log, the registry, published schedules | Re-identify, infer commercial behaviour |
| A-8 Compromised key holder | Holds a private key it should not | Forge as the legitimate holder |
| A-9 Future quantum adversary | Breaks discrete-log in polynomial time | Forge historical signatures |

---

## Threats

### T-01 Forged event (A-5, A-8) — **prevented**

Signature over a domain-separated digest of canonical bytes.
Test: `payload_tampering_detected`, `signature_bitflips_rejected`.

### T-02 Event replay (A-5) — **prevented within the window, then rejected**

Deduplication on the canonical digest inside the replay window; outside it,
events are rejected on age. Note the boundary: a replay after the window is
*rejected as stale*, not *detected as a replay*. Unbounded replay detection
needs unbounded state.
Tests: `retransmission_is_idempotent`, `replay_outside_the_window_is_rejected`.

### T-03 Forged receipt (A-8) — **prevented**

Receipts verify against a registry key holding the receipt-signer role.
Test: `verification_requires_a_receipt_signer_key`.

### T-04 Type confusion between objects (A-5, A-8) — **prevented**

The signature input binds the payload domain. A grant signature cannot
verify as a receipt signature.
Tests: `domain_substitution_detected`, `signing_input_is_domain_separated_from_raw_payload`.

### T-05 Algorithm substitution (A-5) — **prevented**

The signature input binds the suite and the key identifier; the key
identifier is a hash of the key. Downgrading a hybrid to its classical half
changes the length and fails.
Tests: `suite_and_key_substitution_detected`, `hybrid_requires_both_halves`.

### T-06 Grant substitution (A-1) — **prevented**

An event cites the grant's digest. A controller cannot claim to have relied
on a text other than the one that hashes to that value.
Test: `event_must_cite_the_exact_grant_document`.

### T-07 Escaping a revocation by amendment (A-1) — **prevented**

A revocation naming epoch *n* binds every epoch ≥ *n*.
Test: `amendment_cannot_escape_a_revocation`. Model: `NoEscapeByAmendment`.

### T-08 Purpose escalation (A-1) — **prevented**

Purposes are matched through the ontology lattice, and deny-overrides makes
a prohibition undefeatable by adding permissions.
Tests: `purpose_lattice_narrows_and_widens_correctly`,
`deny_overrides_permit_in_both_orders`. Model: `DenyOverrides`.

### T-09 Stale authorization (A-1) — **prevented**

Grant validity windows and revocation effective times are checked at the
event's occurrence time.
Test: `outside_the_window_denies`.

### T-10 Double counting by two reporters (A-2, A-6) — **detected, then resolved by policy**

A reporter-independent operation fingerprint collides. The detector flags;
policy resolves; it never silently drops both, because silent dropping
under-counts in the reporting organisation's favour.
Tests: `two_reporters_of_one_operation_collide`,
`pipeline_reverses_on_controller_wins`.
**Residual:** a false positive suppresses a genuine second operation in the
same collision window. The policy is documented and the collision is
recorded, so the suppression is visible rather than silent.

### T-11 Inflated counters (A-1) — **deterred**

An inflated count costs the controller money, because the controller pays.
Inflation is therefore only rational for a party paid per event, which in
DUAP is the subject. A subject's agent cannot inflate, because events are
signed by the controller.
**Residual:** a controller colluding with a subject can inflate to move
money between themselves; that is a transaction between consenting parties
and outside what the protocol polices.

### T-12 Backdated signatures (A-1, A-4) — **detected, conditionally**

`created` is asserted by the signer. Transparency-log anchoring bounds
existence from above: an object in a tree head dated T existed by T. A
receipt whose `created` is much earlier than its anchor allows is
suspicious and a node should reject it.
**Residual:** an unanchored object proves authorship only. The receipt's
own claims list says so.

### T-13 Suppressed events (A-1) — **partially detected**

Per-stream monotonic sequences make holes visible; gaps are exposed on
`/v1/stats`.
Test: `sequence_gaps_are_detected`.
**Residual, and it is large:** an agent that never starts a stream reports
nothing and leaves no gap. Complete non-participation is invisible. See
RISK-02 in `security/findings.md`.

### T-14 A dishonest clearing node fabricates usage (A-4) — **accepted**

The node signs receipts; nothing stops it signing one for usage no event
described.
**Mitigations:** every receipt commits to an event-set root the subject can
demand and check; anchoring prevents back-dating.
**Residual:** fabrication and omission are detected only by cross-checking
against a party with independent records. RISK-01.

### T-15 Log equivocation (A-4) — **detected given gossip**

A node that shows different histories to different parties is caught when
they compare tree heads. The reference `LogMonitor` refuses a regressed or
forked head.
Test: `log_monitor_rejects_regression_and_forks`.
**Residual:** between equivocation and comparison, a relying party can be
shown a history that omits an entry. Gossip between auditors is therefore a
protocol-adjacent requirement, and `GOVERNANCE.md` makes independent
auditors a governance role.

### T-16 History rewriting (A-4) — **detected**

Consistency proofs between tree heads fail if any earlier entry changed.
Test: `consistency_proof_catches_a_rewritten_history`.

### T-17 Sybil subjects (A-3) — **not addressed by the protocol**

Nothing stops one person creating a thousand subject root secrets and
presenting a thousand pseudonyms to one controller.
**Why it matters less than it appears:** the subject is not paid for
existing, only for usage a *controller* reports, and a controller has no
reason to report usage about a person it has no relationship with.
**Residual:** in a design where subjects are paid per enrolment rather than
per use, this would be fatal. DUAP's economics do not have that shape, and
`ECONOMIC_MODEL.md` explains why that was a deliberate constraint on the
design rather than an accident.

### T-18 Sybil organisations (A-6) — **deterred by the registry**

An `OrgId` names the authority that vouches for it. The protocol does not
verify the binding; the authority does, and a clearing node's admission
policy is an operational control.
**Residual:** a permissive authority admits shells. ADR-0006.

### T-19 Pricing manipulation by a schedule edit (A-1) — **prevented**

A grant pins the schedule's digest. An edited schedule hashes differently
and simply is not found.
Test: `a_schedule_cannot_be_edited_behind_a_pinned_digest`.

### T-20 Rounding attacks (A-1, A-4) — **prevented**

Rounding happens once per invoice line, and the residue is posted to a
rounding account, so repeated rounding cannot skim. Half-even is the
default because half-up drifts.
Tests: `rounding_conserves_value`, `invoice_residue_always_reconciles`,
`half_even_is_unbiased_where_half_up_is_not`.

### T-21 Transaction splitting to exploit tiers (A-1) — **not prevented**

A controller that splits usage across periods pays the first tier twice
where tiers are volume-descending. Aggregation windows bound this, and a
schedule can define tiers over a longer horizon.
**Residual:** a schedule with steep volume discounts creates an incentive
to consolidate, and one with steep volume *premiums* creates an incentive
to split. Recorded as an economic design constraint in `VALUATION.md`.

### T-22 Settlement fraud (A-4) — **out of scope, bounded**

The protocol produces instructions, not payments. A dishonest node can
produce a correct instruction and not execute it.
**Mitigation:** the ledger's liability remains on the books, and the
subject's balance is a claim.
**Residual:** enforcement is contractual and legal. Recorded plainly.

### T-23 Shill bidding in an auction (A-4, A-6) — **deterred**

Commit-reveal stops the auctioneer's last look. It does not stop a
colluding sham bidder raising the second price.
**Mitigation:** bidders are enrolled organisations, and commitments are
logged, so a pattern of losing shill bids is visible to auditors.
**Residual:** single-round mechanisms do not prevent rings.

### T-24 Re-identification from event metadata (A-7) — **partially mitigated**

Events carry no data, but class, operation, purpose, timing and volume are
themselves revealing at scale: a stream of `health.inferred` inferences
about one pseudonym at one controller is a profile.
**Mitigations:** per-controller pseudonyms remove the cross-controller join
key; the log carries no event fields at all.
**Residual:** a clearing node sees everything about every subject it
clears. That is the largest privacy exposure in the design and
`PRIVACY.md` states it first, not last.

### T-25 Linkage at payout (A-4) — **accepted**

To pay one person for usage under many pseudonyms, some party must learn
that those pseudonyms belong together.
**Residual:** the settlement agent learns the linkage. A construction that
avoids it (blind signatures over per-pseudonym payout tokens) is research,
not production. `PRIVACY.md`, `docs/research/0002-unlinkable-payout.md`.

### T-26 Metadata leakage through logs (A-7, insider) — **prevented by rule and by review**

No pseudonym, commitment or authorization digest appears in any log line or
metric label. Enforced by review of the single logging module, and by
`.claude/agents/privacy-engineer.md`'s standing obligation.
**Residual:** a new log line added carelessly. This is a process control,
not a technical one, and is recorded as such.

### T-27 Key compromise (A-8) — **detected, and bounded by revocation**

Revocation with an explicit effective time, and the honest requirement that
an unknown compromise time invalidates everything the key signed.
Tests: `revocation_for_compromise_invalidates_later_signatures`,
`unknown_compromise_time_invalidates_everything`.

### T-28 Quantum forgery of historical signatures (A-9) — **mitigated for hybrid-signed objects**

Records signed with `ed25519+ml-dsa-44` remain unforgeable if either
component holds. Records signed with Ed25519 alone do not.
**Residual:** every Ed25519-only signature in existence becomes forgeable
if and when Ed25519 falls. Re-signing historical records with a
post-quantum suite before that happens is an operational task, not a
protocol one, and `OPERATIONS.md` records it.

### T-29 Denial of service on the proof endpoint — **open finding**

Inclusion-proof generation is O(n): 14.99 ms in a 100,000-entry tree
against 2.5 µs to verify. An attacker spends 15 ms of server time per cheap
request.
`security/findings.md` PERF-01. Until fixed, the endpoint must be
rate-limited separately.

### T-30 Deferred obligations not honoured (A-1) — **detected after the fact**

A `delete_by` obligation is a promise. The protocol records it, timestamps
it, and makes the later breach provable; it cannot prevent it.
**Residual:** the gap between the deadline and its discovery. A clearing
node should raise an exception when a deadline passes with no matching
`lifecycle.delete` event.

### T-31 Malicious SDK or compromised agent build (supply chain) — **partially mitigated**

Events carry the agent's name, version and an optional build attestation
digest, so a compromised agent version can be quarantined without
disabling the organisation.
**Residual:** the attestation is optional and unverified by the protocol.
Signed releases and SBOMs are in `SECURITY.md`; SLSA-level provenance is
not yet produced.

### T-32 Compromised clearing node (A-4, insider) — **partially detected**

Everything a node signs is anchored, and its registry state has a digest
that can itself be anchored, so a node showing different registries to
different parties is detectable.
**Residual:** a node that is compromised can misprice, mis-attribute and
leak. Multi-node cross-clearing is the structural answer and is
UNIMPLEMENTED.

---

## Summary by disposition

| Disposition | Threats |
|---|---|
| Prevented | T-01, T-03, T-04, T-05, T-06, T-07, T-08, T-09, T-19, T-20, T-26 |
| Prevented within a stated bound | T-02 |
| Detected | T-12, T-15, T-16, T-27, T-30, T-32 |
| Detected then resolved by policy | T-10 |
| Partially detected | T-13, T-24, T-31 |
| Deterred | T-11, T-18, T-23 |
| Accepted | T-14, T-17, T-22, T-25 |
| Not prevented, recorded | T-21 |
| Open finding | T-29 |

Eleven of thirty-two threats are prevented. That proportion is the honest
shape of an accounting layer over systems it does not control, and any
design in this space claiming substantially better should be asked which
of these it has quietly reclassified.
