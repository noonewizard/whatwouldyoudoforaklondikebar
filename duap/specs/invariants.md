# Protocol invariants

**Status:** SPECIFIED · 2026-09-21

Every invariant here is (a) stated precisely enough to be false, (b) mapped
to the mechanism that enforces it, and (c) mapped to the executable test or
model check that would fail if it were violated. An invariant with no
enforcement and no test is a wish, and does not belong in this file.

Notation: `E` events, `G` grants, `R` revocations, `Rc` receipts, `J`
journal entries. `sig(x)` is a verified signature over `x`.

---

## A. Authorization

### INV-A1 — Deny overrides

For every event `e` and grant `g`, if any term of `g` whose matcher applies
to `e` has effect `Deny`, then `evaluate(g, ·, e, ·).effect = Deny`,
regardless of how many `Permit` terms also apply and regardless of their
order.

- **Enforced by:** `duap_auth::evaluate`, step 2.
- **Tested by:** `deny_overrides_permit_in_both_orders`,
  `adding_a_matching_deny_never_permits` (property).
- **Model:** `formal/Authorization.tla`, `DenyOverrides`.

### INV-A2 — No unexplained permission

If `evaluate(...).effect = Permit` then either `permitting_terms` is
non-empty, or the grant's `default_effect` is `Permit` and the decision
reason is `DefaultEffect`.

- **Enforced by:** the decision type; a `Permit` always carries its reason.
- **Tested by:** `permits_are_always_explained` (property).
- **Model:** `formal/Authorization.tla`, `PermitIsExplained`.

### INV-A3 — Determinism

`evaluate` is a pure function of `(grant, revocations, event, context)`. It
reads no clock, performs no I/O, and does not depend on map iteration order.

- **Enforced by:** the signature of `evaluate`; no clock or store is in
  scope.
- **Tested by:** `evaluation_is_deterministic` (property), and by the
  conformance vectors in `spec/vectors/authorization.json`, which a second
  implementation must reproduce exactly.

### INV-A4 — Revocation binds from its effective time

If a revocation `r` for grant `g` is in scope for term `t`, class `c` and
purpose `p`, then for every event `e` with `e.occurred_at >= r.effective_from`
matching those, term `t` does not contribute a permission.

- **Enforced by:** `duap_auth::evaluate`, step 3.
- **Tested by:** `revocation_suppresses_from_its_effective_time`,
  `notice_period_delays_effect`, `scoped_revocation_only_hits_its_scope`.
- **Model:** `formal/Authorization.tla`, `RevocationIsProspective`.

### INV-A5 — Amendment cannot escape revocation

A revocation naming epoch `n` of grant `g` binds every epoch `m >= n`.

- **Enforced by:** the epoch comparison in `evaluate`.
- **Tested by:** `amendment_cannot_escape_a_revocation`.
- **Model:** `formal/Authorization.tla`, `NoEscapeByAmendment`.

### INV-A6 — Violated obligations deny

If any obligation of a matching permit term returns `Violated`, the decision
is `Deny`. There is no "permitted but non-compliant" state.

- **Enforced by:** `duap_auth::evaluate`, step 4.
- **Tested by:** `violated_obligation_denies`,
  `obligations_union_across_matching_permits`.

### INV-A7 — No authorization, no receipt

For every receipt `rc` covering events `E`, every `e` in `E` satisfies
`evaluate(g, R, e, ·).effect = Permit` where `g` is the grant `e` cites at
the epoch and digest it cites.

This is the protocol's central authorization invariant, and the reason the
clearing node evaluates before it counts rather than after.

- **Enforced by:** the ingest pipeline, stage 4 precedes stages 5-9.
- **Tested by:** `every_rejection_names_the_stage_it_failed_at` (gateway),
  and the demonstration's refusal assertions.
- **Not enforced against:** a clearing node that is itself dishonest. A node
  can issue a receipt for usage it never evaluated. Detection requires the
  subject to check receipts against their own grants, which
  `Receipt::verify_coverage` and the log anchor make possible. `THREAT_MODEL.md`
  T-17.

---

## B. Identity and encoding

### INV-E1 — One value, one encoding

`encode` is injective, and `decode` accepts only bytes that `encode`
produces: `decode(b) = Ok(v)` implies `encode(v) = b`.

- **Enforced by:** `duap_canon::codec`, strict decoding.
- **Tested by:** `roundtrip_value`, `roundtrip_bytes`, `injective`,
  `accepted_input_is_canonical` (all property tests), plus 34 conformance
  vectors.

### INV-E2 — One event, one canonical identity

An event's identity is the digest of its canonical encoding under the event
domain. Two byte-identical events have the same identity; two events
differing in any field do not.

- **Enforced by:** `DataUsageEvent::digest`.
- **Tested by:** `events_round_trip_canonically_and_digest_is_stable`.

### INV-E3 — Domain separation

For domains `d1 != d2` and any payload `p`,
`Digest::of(alg, d1, p) != Digest::of(alg, d2, p)` (except with negligible
probability). A signature over an object of one type never verifies as
another.

- **Enforced by:** the NUL-separated, length-injective digest input.
- **Tested by:** `domain_separation` (property),
  `domain_substitution_detected`, `signing_input_is_domain_separated_from_raw_payload`.

### INV-E4 — Self-certifying key identity

A key identifier is a function of the key material alone. A registry cannot
bind an identifier to a different key.

- **Enforced by:** `KeyId::derive`, checked on enrolment.
- **Tested by:** `key_ids_are_self_certifying`, `distinct_seeds_distinct_kids`.

---

## C. Metering

### INV-M1 — Idempotent accounting

Offering the same event any number of times counts it once.

- **Enforced by:** `DedupIndex` keyed on the canonical digest.
- **Tested by:** `counting_is_idempotent` (property),
  `retransmission_is_idempotent`, `pipeline_counts_once_per_event`.

### INV-M2 — Replay resistance is bounded, and the bound is stated

Deduplication holds within the replay window. Outside it, events are
rejected on age rather than deduplicated, which bounds the index.

- **Enforced by:** `WindowConfig` and `DedupIndex::evict_before`.
- **Tested by:** `replay_outside_the_window_is_rejected`,
  `eviction_bounds_the_index`.
- **Limitation:** an event replayed after the window closes is rejected, not
  detected as a replay. A system that needs unbounded replay detection needs
  unbounded state; DUAP chooses the bound and says so.

### INV-M3 — Monotone sequences expose omission

For a stream with indices `0..n`, any index never delivered appears in
`gaps()`.

- **Enforced by:** `StreamState` in `duap_meter::dedup`.
- **Tested by:** `sequence_gaps_are_detected`, `a_late_arrival_closes_its_gap`.
- **Limitation:** an agent that never starts a stream reports nothing, and no
  gap appears. Sequences detect *partial* suppression.

### INV-M4 — One operation, one count

Two distinct events describing the same operation collide on the operation
fingerprint and are resolved by an explicit policy; they are never both
counted.

- **Enforced by:** `DoubleCountDetector`.
- **Tested by:** `two_reporters_of_one_operation_collide`,
  `pipeline_reverses_on_controller_wins`.

### INV-M5 — Aggregation is lossless for pricing

Two events that differ in any dimension that can change their price fall
into different counters.

- **Enforced by:** the `UsageKey` field set.
- **Tested by:** `aggregation_groups_by_pricing_dimensions`.

---

## D. Provenance

### INV-P1 — The log is append-only, verifiably

For sizes `m <= n`, a consistency proof verifies iff the tree of size `m` is
a prefix of the tree of size `n`.

- **Enforced by:** RFC 6962 construction.
- **Tested by:** `every_consistency_proof_verifies` (property),
  `consistency_proof_catches_a_rewritten_history`.

### INV-P2 — Inclusion is provable and exclusive

An inclusion proof verifies for the leaf it was generated for and for no
other, and not against an earlier root.

- **Tested by:** `every_inclusion_proof_verifies` (property),
  `inclusion_proofs_reject_wrong_leaf_and_root`,
  `later_entries_are_not_in_earlier_trees` (property).

### INV-P3 — Attribution conserves weight

For any node, the sum of the attributed shares plus the terminated weight is
exactly the input weight. Nothing is silently lost or created.

- **Enforced by:** exact rational arithmetic and the `dropped` accumulator.
- **Tested by:** `uniform_attribution_totals_one` (property),
  `tiny_shares_are_dropped_and_accounted`.

### INV-P4 — No content in the log

A log entry carries a kind, a digest, a submitter, a timestamp and an
optional shard tag. Nothing else.

- **Enforced by:** the `LogEntry` type with `deny_unknown_fields`.
- **Tested by:** the type's shape and `receipt/canonical` vectors.

---

## E. Accounting

### INV-L1 — Every entry balances, per currency

For every posted journal entry and every currency, the signed posting
amounts sum to zero.

- **Enforced by:** `Ledger::post`, checked before any balance is mutated.
- **Tested by:** `unbalanced_entries_are_refused`,
  `a_mixed_currency_entry_cannot_balance_by_accident`.
- **Model:** `formal/Accounting.tla`, `EntryBalances`.

### INV-L2 — The trial balance is zero

Over the whole ledger, per currency, the sum of all balances is zero at
every point in time.

- **Tested by:** `ledger_always_balances` (property),
  `balanced_entries_post_and_the_trial_balance_stays_zero`.
- **Model:** `formal/Accounting.tla`, `TrialBalanceZero`.

### INV-L3 — Rounding conserves value

For every precise amount `p` and rounding mode `m`,
`money * 10^9 + residue = p.nmu`, and the residue is posted.

- **Tested by:** `rounding_conserves_value`, `rounding_always_conserves`
  (property), `invoice_residue_always_reconciles` (property),
  `value_conserved`, `remainder_bounded`.
- **Model:** `formal/Accounting.tla`, `ValueConserved`.

### INV-L4 — Corrections are entries

No posted entry is ever mutated or removed. A correction is a new balanced
entry that references the one it reverses.

- **Enforced by:** `JournalEntry` has no mutating API; `Ledger` exposes no
  removal.
- **Tested by:** `reversal_is_an_entry_not_an_edit`.

### INV-L5 — Netting conserves positions

After applying the transfers netting produces, every participant's net
position is zero, and the number of transfers is at most `n-1`.

- **Tested by:** `netting_always_settles` (property),
  `netting_conserves_positions_and_bounds_transfers`.

### INV-L6 — The accounting identity

Over any period, per currency:

```
sum(credits) = sum(debits)
```

and, decomposed for a clearing node,

```
sum(charges) = sum(subject shares) + sum(taxes) + sum(clearing margin)
             + sum(rounding residue swept)
```

subject to the adjustments recorded as reversing entries.

- **Tested by:** `invoice_posts_a_balanced_entry_with_subject_shares`,
  `debits_and_credits` in `balanced_entries_post_and_the_trial_balance_stays_zero`.

### INV-L7 — Payout conserves value

`sum(paid) + sum(outstanding) = sum(accrued)` for every recipient, always.

- **Tested by:** `payout_accumulator_conserves_value`,
  `accumulator_conserves` (property).

---

## F. Receipts

### INV-R1 — Coverage is exact

A receipt's `events_root` is the Merkle root of exactly the event digests it
covers, in acceptance order. Any substitution, omission, addition or
reordering changes it.

- **Tested by:** `coverage_is_checkable_against_the_event_set`.

### INV-R2 — A receipt states its own limits

`Receipt::claims()` returns both what the receipt establishes and what it
does not, and the not-established list is never empty.

- **Tested by:** `claims_distinguish_what_is_and_is_not_established`,
  `receipts_publish_their_own_limits`.

### INV-R3 — The anchor is detachable

A receipt's digest is over its anchor-free core, so the log can commit to a
receipt before the inclusion proof exists, and a holder can upgrade to a
fresher proof without changing the receipt's identity.

- **Tested by:** `an_anchored_receipt_verifies_its_log_position`.

---

## G. What is deliberately not invariant

Stating these prevents a reader inferring them:

- **Completeness.** Nothing guarantees that all usage was reported.
- **Truth.** Nothing guarantees a reported operation occurred.
- **Fair price.** Nothing guarantees the charge is right, only that it
  follows from the agreed rule.
- **Deletion.** Nothing guarantees data was deleted when a deferred
  obligation said it would be.
- **Unlinkability against content.** Pseudonyms remove a join key, not the
  information in the data.
- **Non-equivocation.** The log makes equivocation *detectable* given
  gossip between relying parties; it does not prevent it.
