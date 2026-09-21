# Data Usage Accounting Protocol, version 0.1 (draft)

**Status:** SPECIFIED (draft) · 2026-09-21
**Protocol identifier:** `DUAP/1`
**Editors:** DUAP reference implementation contributors

This document specifies the DUAP protocol kernel. It is written so that an
implementation can be built from it without reading the reference source. A
conforming implementation is one that produces and accepts exactly the byte
sequences the conformance vectors in `spec/vectors/` require, at the level it
claims.

## Conventions

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD
NOT, RECOMMENDED, MAY and OPTIONAL are to be interpreted as described in
BCP 14 (RFC 2119, RFC 8174) when, and only when, they appear in all
capitals.

`H(x)` denotes SHA-256 (FIPS 180-4) unless another algorithm is named.
Byte strings are written in hexadecimal. `||` denotes concatenation.

---

## 1. Overview

DUAP is an accounting layer. It defines how a participant asserts, in a
verifiable and comparable form, that a specific operation was performed on a
specific class of data about a specific pseudonymous subject under a
specific authorization — and how those assertions are counted, priced,
receipted and settled.

DUAP does not transport data, does not determine whether an assertion is
true, and does not decide what data is worth. What it does and does not
establish is set out in §6.4 and in `docs/architecture/minimal-protocol.md`.

### 1.1 Objects

| Object | Domain label | §|
|---|---|---|
| Data Usage Event | `duap.event.v1` | 4 |
| Authorization Grant | `duap.grant.v1` | 5 |
| Revocation | `duap.revocation.v1` | 5.6 |
| Data Usage Receipt | `duap.receipt.v1` | 6 |
| Transparency log entry | `duap.log-entry.v1` | 7 |
| Signed tree head | `duap.sth.v1` | 7.3 |
| Signature input | `duap.sig-input.v1` | 3.4 |

### 1.2 Conformance levels

An implementation MUST state which level it claims. Each level includes the
levels below it.

| Level | Capability | Vectors |
|---|---|---|
| L1 | Canonical encoding, strict decoding, digests, the JSON view | `canonical-encoding.json`, `events.json`, `taxonomy.json` |
| L2 | Signature verification for at least one registered suite | `crypto.json` |
| L3 | Merkle proofs and receipt verification | `merkle.json`, `receipts.json` |
| L4 | Authorization evaluation | `authorization.json` |
| L5 | Pricing arithmetic | `pricing.json` |

An implementation MUST reject a suite, algorithm or object version it does
not implement. It MUST NOT accept an object it cannot fully verify.

---

## 2. The canonical data model

### 2.1 Value kinds

A canonical value is exactly one of: null, boolean, unsigned integer
(0 .. 2^64−1), negative integer (−2^63 .. −1), byte string, text string
(valid UTF-8), array, or map with text keys.

The following are **outside** the model and MUST be rejected wherever they
appear: floating point of any width, CBOR tags, indefinite-length items,
`undefined`, unassigned simple values, non-text map keys, duplicate map
keys, and negative integers below −2^63.

> **Why negative integers are bounded at −2^63.** Full CBOR major type 1
> reaches −2^64, which does not fit a signed 64-bit integer and forces
> arbitrary-precision arithmetic on every implementation to handle one value
> that no DUAP object uses. See `docs/adr/0002-canonical-encoding.md`.

### 2.2 Canonical CBOR

The canonical encoding is the RFC 8949 §4.2.1 core deterministic encoding,
restricted to the value kinds above:

1. Integer arguments MUST use the shortest form.
2. All lengths MUST be definite and in shortest form.
3. Map keys MUST be text strings, MUST be unique, and MUST be sorted by
   their **encoded** bytes, which for text keys means shorter keys first and
   then bytewise ascending.

A decoder MUST reject any input it would not itself produce. In particular
it MUST reject non-shortest arguments, indefinite lengths, unsorted or
duplicate keys, and trailing data after a complete item.

A decoder SHOULD impose limits and MUST document them. The reference limits
are 64 levels of nesting, 2^20 items per collection and 16 MiB of input.

### 2.3 Digests

```
Digest(alg, domain, payload) = alg( "DUAP/1" || 0x00 || domain || 0x00 || payload )
```

`domain` is an ASCII label containing no NUL byte, so the input is
injective in `(domain, payload)`. Digests are rendered `alg:hex`, for
example `sha2-256:9f86d0…`.

Registered algorithms: `sha2-256` (REQUIRED), `blake3-256` (OPTIONAL). A
verifier MUST compare the algorithm as well as the bytes.

### 2.4 The JSON view

The JSON view is a lossless rendering for systems that cannot process CBOR.
**It is never the hashing input.** An implementation MUST convert to
canonical CBOR before computing or verifying any digest or signature.

| Canonical value | JSON |
|---|---|
| null | `null` |
| boolean | `true` / `false` |
| unsigned, ≤ 2^53−1 | number |
| unsigned, > 2^53−1 | `{"$u64": "<decimal>"}` |
| negative, ≥ −(2^53−1) | number |
| negative, < −(2^53−1) | `{"$n64": "<decimal>"}` |
| byte string | `{"$b64": "<base64url, unpadded>"}` |
| text string | string |
| array | array |
| map | object; a key beginning `$` is escaped by doubling it |

A parser MUST reject a `$`-prefixed key that is neither a known marker nor
a `$$` escape, a marker used inside the JSON-safe range, and base64url that
is not in canonical unpadded form.

---

## 3. Cryptography

### 3.1 Suites

| Label | Public key | Signature | Post-quantum |
|---|---:|---:|---|
| `ed25519` | 32 | 64 | no |
| `ml-dsa-44` | 1312 | 2420 | yes |
| `ml-dsa-65` | 1952 | 3309 | yes |
| `ed25519+ml-dsa-44` | 1344 | 2484 | yes |

`ed25519` is REQUIRED to implement for L2. Ed25519 verification MUST use
the strict, cofactorless, non-malleable equation (RFC 8032 with the
additional checks in the "verify_strict" sense): a verifier MUST reject
small-order and non-canonical public keys and signatures.

The hybrid suite is the concatenation `ed25519_sig || ml_dsa_44_sig`, and
both halves MUST verify. Forging the hybrid therefore requires forging
both, so it is unforgeable if either component is.

### 3.2 Key identifiers

```
KeyId = "kid1:" || hex( first 16 bytes of Digest(sha2-256, "duap.key-id.v1",
                          suite_label || 0x00 || public_key) )
```

A key identifier is a function of the key material alone. A registry MUST
recompute it on enrolment and MUST reject a record whose identifier does
not match its key.

### 3.3 Key derivation (informative)

The reference implementation derives all component private keys from one
256-bit seed by domain-separated hashing, so that backup and escrow handle
32 bytes regardless of suite. This is an implementation convenience and is
not required for interoperability; only the public key and the signature
are on the wire.

### 3.4 Signature input

A signature is computed over the canonical encoding of the signature input
structure, which is a map with these keys:

| Key | Type | Meaning |
|---|---|---|
| `c` | text | Fixed: `duap.sig.v1` |
| `s` | text | Suite label |
| `k` | text | Signing key identifier |
| `d` | text | Domain label of the payload |
| `h` | text | `Digest(sha2-256, d, payload)` rendered as `alg:hex` |
| `t` | uint | Creation time, microseconds since the Unix epoch, UTC |
| `n` | bytes | OPTIONAL nonce; omitted entirely when absent |

The message passed to the suite is that canonical encoding. For suites with
no native context parameter (Ed25519), the context string is bound by
prefixing:

```
message = len_be64(ctx) || ctx || signature_input
```

where `ctx` is the ASCII string `DUAP/1 signature`. For ML-DSA the same
string is passed as the FIPS 204 context parameter.

Binding the suite, key identifier and payload domain inside the signed
structure is what prevents algorithm substitution and type confusion: a
signature over a grant can never verify as a signature over a receipt.

### 3.5 Envelopes

An envelope is a map with keys `domain` (text), `payload` (bytes, the
canonical encoding of the object) and `signatures` (array). Each signature
is a map with `suite`, `kid`, `created`, optional `nonce` and `sig`.

The payload travels as opaque bytes. A verifier MUST NOT re-encode the
payload before verifying: verification depends on the exact bytes, and
re-encoding is how implementations disagree.

Multiple signatures are independent. A verifier reports which verified; an
envelope with no policy-acceptable verified signature is unauthenticated.

### 3.6 Key validity and revocation

A key record carries `not_before`, optional `not_after`, a status and an
optional revocation. A revocation carries a reason, a declaration time and
an **effective time**.

- For an orderly retirement (`superseded`, `retired`) signatures created
  before the effective time remain valid.
- For a suspected compromise the holder MUST set the effective time to the
  earliest instant the key could have been exposed; if that is unknown it
  MUST be set to the key's `not_before`, invalidating everything the key
  ever signed. A verifier cannot distinguish the holder from a thief who
  held the key at the same time, so there is no honest middle option.

Because `created` is asserted by the signer, revocation ordering is only as
good as the evidence of time. An implementation SHOULD require that
receipts be anchored (§7) and SHOULD reject a signature whose `created` is
earlier than its anchoring allows by more than a configured skew.

---

## 4. The Data Usage Event

### 4.1 Purpose

An event asserts that one operation was performed. It carries no data: at
most a salted commitment (§4.6) stands in for a value.

### 4.2 Fields

Wire names are two or three characters. At the protocol's target volume the
difference is a double-digit fraction of total storage; the mapping is
fixed, and generated into every SDK.

| Wire | Name | Type | Presence |
|---|---|---|---|
| `v` | schema | uint | REQUIRED, `1` |
| `id` | event id | text `evt1:<32 hex>` | REQUIRED |
| `ts` | occurred at | uint µs | REQUIRED |
| `rt` | recorded at | uint µs | REQUIRED, ≥ `ts` |
| `sq` | sequence | map `{s: stream cid, i: uint}` | OPTIONAL, RECOMMENDED |
| `ag` | agent | map `{name, version, build?}` | REQUIRED |
| `ct` | controller | text `org:<authority>/<local>` | REQUIRED |
| `pr` | processor | org id | OPTIONAL |
| `sb` | subject scope | tagged map, §4.3 | REQUIRED |
| `ju` | jurisdiction | map `{c, s?, r?}` | REQUIRED |
| `dc` | data class | taxonomy code | REQUIRED |
| `sn` | sensitivity | tier code | REQUIRED, ≥ class default |
| `cm` | collection method | taxonomy code | OPTIONAL |
| `op` | operation | taxonomy code | REQUIRED |
| `pp` | purpose | taxonomy code | REQUIRED |
| `lb` | lawful basis | code | OPTIONAL |
| `az` | authorization ref | map `{g, d, e}`, §4.4 | REQUIRED |
| `qy` | quantity | map `{u: unit, n: uint}` | REQUIRED |
| `rp` | retention | map `{b, d?, u?}` | OPTIONAL |
| `pv` | provenance | map `{i?, o?, t?}` | OPTIONAL |
| `ec` | economics | map `{s?, cp?, rv?, ex?}` | OPTIONAL |
| `cx` | commitment | digest | OPTIONAL |
| `xt` | extensions | map | OPTIONAL |

An implementation MUST reject an event carrying a field not in this table.
Silently ignoring an unknown field risks dropping one that changes the
event's meaning.

### 4.3 Subject scope

A tagged union on key `k`:

- `{"k":"s","r":<subject ref>}` — one subject, under the pseudonym they
  present to this controller.
- `{"k":"c","h":<cohort cid>,"n":<uint>}` — a cohort of at least `n`
  subjects. `n` MUST be ≥ 2.
- `{"k":"n"}` — not about a natural person.

A subject reference is `sub1:<32 hex>`. How it is derived is an
implementation matter; it MUST be stable for a (subject, controller) pair
and MUST NOT be derivable from the subject's references to other
controllers. The reference derivation is in §9.

### 4.4 Authorization reference

`{"g": <grant id>, "d": <grant digest>, "e": <epoch uint>}`.

The digest binds the event to the *exact* grant document relied on. An
evaluator MUST recompute the grant's digest and MUST reject a mismatch.

### 4.5 Validation

An implementation MUST reject an event unless all of the following hold:

1. `v` is 1.
2. `rt` ≥ `ts`.
3. The sensitivity tier's rank is ≥ the data class's default tier rank.
4. `qy.u` is the operation's metering unit in the ontology.
5. `qy.n` > 0.
6. If the operation derives (per the ontology), `pv.o` is present.
7. If the operation is in the transfer family and is not `transfer.internal`,
   `ec.cp` is present.
8. If the subject scope is a cohort, its size is ≥ 2.
9. If the subject scope is non-personal, the data class's default tier rank
   is ≤ 1.
10. Every extension key contains a `.` and does not begin `duap.`.
11. If retention basis is `fixed_period`, a day count is present.

### 4.6 Value commitments

`C = Digest(sha2-256, "duap.commitment.v1", len_be64(salt) || salt || value)`
with a fresh uniform 256-bit salt.

The salt makes the commitment hiding even for low-entropy values; an
unsalted commitment to a postcode is recoverable by brute force. The length
prefix makes `(salt, value)` injective.

---

## 5. Authorization

### 5.1 Grant

| Wire | Name | Presence |
|---|---|---|
| `v` | schema, `1` | REQUIRED |
| `id` | grant id `gr1:<32 hex>` | REQUIRED |
| `ep` | epoch, starting at 1 | REQUIRED |
| `pv` | digest of the previous epoch | REQUIRED iff `ep` > 1 |
| `sb` | subject reference | REQUIRED |
| `sk` | subject's signing key id | REQUIRED |
| `ct` | controller | REQUIRED |
| `is` | issued at | REQUIRED |
| `nb` | not before | REQUIRED |
| `ex` | expires at | OPTIONAL |
| `tm` | terms | REQUIRED |
| `df` | default effect | REQUIRED |
| `pc` | default pricing rule | OPTIONAL |
| `rv` | revocation policy | REQUIRED |
| `cu` | currency | REQUIRED |
| `xt` | extensions | OPTIONAL |

`df` MUST be encoded explicitly. A grant that does not mention an operation
does not authorise it, and that fact is never left to a default in the
reader.

Grants are immutable. An amendment is a new document with the same `id`,
`ep + 1`, and `pv` set to the previous epoch's digest, forming a hash chain.

### 5.2 Terms

A term is `{i: id, e: effect, m: matcher, ob: obligations, pr: pricing, l: label}`.
A `Deny` term MUST NOT carry obligations.

A matcher is a conjunction of independent dimension selectors: data classes
(by list, namespace, or maximum tier), operations, operation families,
purposes (by lattice descent, exact match, negation, or commercial flag),
countries, counterparties, processors, collection methods, a maximum
sensitivity, and a time window. Every dimension defaults to "any", so
adding one only ever narrows.

Purposes MUST be matched through the ontology's lattice, never by string
equality: `marketing` covers `marketing.advertising.behavioral`;
`marketing.advertising.contextual` does not.

### 5.3 Combining algorithm

Deny overrides; obligations union.

1. Collect every term whose matcher applies.
2. If any is `Deny`, the decision is `Deny`. Term order is irrelevant.
3. Otherwise remove permits suppressed by a revocation in force (§5.6). If
   none remains, apply `df`.
4. Union the obligations of all remaining permits and check each. A single
   violated obligation makes the decision `Deny`.

Deny-overrides is REQUIRED rather than first-applicable so that term order
carries no meaning: order-dependent policy makes an authoring mistake, or a
malicious re-ordering during amendment, into a breach. Obligation union is
REQUIRED so that a broad permissive term cannot silently erase a narrow
term's safeguard.

Evaluation MUST be a pure function of `(grant, revocations, event,
context)`. It MUST NOT consult a clock or any mutable store.

### 5.4 Obligations

Each obligation is classified by what a verifier can do with it:

- **Checkable** — decidable from the event; a violation forces `Deny`.
  `max_retention`, `no_derivative`, `max_derivation_depth`,
  `no_onward_transfer`, `transfer_allowlist`, `no_ai_training`,
  `ai_allowlist`, `residency`, `min_cohort`, `max_epsilon`,
  `require_commitment`, `require_processor`.
- **Deferred** — a promise about the future, recorded and monitored, not
  enforced. `delete_by`, `notify`.
- **Economic** — handed to the valuation engine. `min_price`.

An implementation MUST NOT describe a deferred obligation as enforced.

### 5.5 Pricing selection

Among permitting terms that state a pricing rule, an implementation MUST
prefer one whose rule can price the event's metering unit, then the most
specific matcher, then the lowest term identifier. The ordering is fully
determined so that two implementations agree.

### 5.6 Revocation

A revocation carries the grant id, the epoch, the grant digest, a scope
(all, listed terms, listed purposes, or listed classes), a declaration
time, an effective time and a retroactive request.

The effective time MUST be at least the time implied by the grant's own
revocation policy. A notice period beyond 30 days MUST be rejected: a
withdrawal that takes effect after an arbitrary delay is a withdrawal in
name only.

A revocation naming epoch *n* binds every epoch ≥ *n*.

**Revocation is prospective.** It stops future authorised operations and
starts the clock on deletion obligations. It does not undo processing, and
it does not reach into a derived object or a trained model. A retroactive
request is recorded as a request; the controller's response is itself an
event, and the gap between them is what an auditor examines.

---

## 6. The Data Usage Receipt

### 6.1 Fields

| Wire | Name | Presence |
|---|---|---|
| `v` | schema, `1` | REQUIRED |
| `pr` | protocol, `DUAP/1` | REQUIRED |
| `is` | issuer | REQUIRED |
| `ct` | controller | REQUIRED |
| `px` | processor | OPTIONAL |
| `sb` | subject scope | REQUIRED |
| `pd` | period | REQUIRED |
| `cv` | coverage | REQUIRED |
| `az` | authorization reference | REQUIRED |
| `ds` | decision summary | REQUIRED |
| `pv` | provenance output | REQUIRED iff the operation derives |
| `ch` | charge | REQUIRED |
| `ss` | subject share | OPTIONAL |
| `pb` | price breakdown digest | REQUIRED |
| `an` | log anchor | OPTIONAL |
| `at` | issued at | REQUIRED |

Coverage is `{dc, op, pp, ju, qy, n, r}` where `n` is the event count and
`r` is the Merkle root over the covered events' digests in acceptance order.

### 6.2 Identity and the anchor

A receipt's digest is computed over the receipt **with the anchor removed**.
The log entry commits to that digest, and the resulting inclusion proof is
then attached. This makes the anchor a detachable proof about the receipt
rather than part of it, so a holder can upgrade to a fresher proof without
changing the receipt's identity.

### 6.3 Verification

A verifier MUST, in order: verify the envelope signature against a key the
registry accepts in the receipt-signer role; confirm the sealed payload is
the receipt; apply the structural rules; and, if an anchor is present,
recompute the log entry, recompute the leaf hash, and verify the inclusion
proof against the signed tree head.

### 6.4 What a receipt establishes

**Established:**

1. Some holder of the issuer's private key produced this exact statement.
2. It has not been altered since.
3. It is bound to one specific grant document at one specific epoch.
4. It covers one specific, fixed set of events.
5. If anchored: it existed no later than the anchored tree head, and the
   log's history has not been rewritten since.

**Not established:**

1. That the described operations took place.
2. That no other operations went unreported.
3. That the charge is a fair price.
4. That deferred obligations will be honoured.
5. That the named pseudonym corresponds to a particular legal person.

An implementation that displays a receipt to a person SHOULD display both
lists. The reference implementation returns them programmatically so that
the interface cannot drift from the specification.

---

## 7. Transparency log

### 7.1 Construction

RFC 6962 §2, with DUAP domain separation:

```
MTH({})     = Digest(alg, "duap.log.empty.v1", "")
LeafHash(d) = Digest(alg, "duap.log.leaf.v1",  0x00 || d)
NodeHash(l,r)= Digest(alg, "duap.log.node.v1", 0x01 || l || r)
MTH(D[0:n]) = NodeHash(MTH(D[0:k]), MTH(D[k:n])), k the largest power of
              two strictly below n
```

The `0x00`/`0x01` prefixes are REQUIRED: without them a subtree root can be
presented as a leaf, producing a second preimage for the root.

### 7.2 Entries

`{k: kind, d: object digest, s: submitter, t: sequenced at, sh?: shard}`.
An entry MUST NOT carry any other field. The log is a commitment device,
not a database: no personal data, no event bodies, no grant contents.

### 7.3 Tree heads

`{l: log id, n: size, r: root, t: issued at, a: algorithm}`, signed by a key
the registry accepts in the log-signer role.

A relying party MUST retain the most recent verified head and MUST require a
consistency proof before accepting a larger one. A head of the same size
with a different root, or a smaller size, is a fork and MUST be refused.

### 7.4 What the log provides

Existence by a time, append-only history, and **detection** of equivocation
given gossip between relying parties. It does not *prevent* equivocation, and
an implementation MUST NOT describe it as doing so.

---

## 8. Metering and accounting (L5)

### 8.1 Deduplication

Two events are the same event iff their canonical digests are equal.
Offering the same event repeatedly MUST count it once and MUST NOT be an
error: retransmission and replay are indistinguishable, so idempotence is
the defence.

Deduplication state MAY be bounded by a replay window. Outside the window
events are rejected on age. The window MUST be documented.

### 8.2 Sequences

Where an event carries a sequence, an implementation MUST detect gaps and
MUST expose them. A reused index with different content is a conflict and
MUST be rejected.

### 8.3 Double counting

Implementations SHOULD compute a reporter-independent operation
fingerprint over controller, subject scope, data class, operation, purpose,
a rounded occurrence time, quantity and provenance output. Two distinct
events with the same fingerprint MUST NOT both be counted; which one counts
is a documented policy. An implementation MUST NOT silently drop both.

### 8.4 Money

Amounts are exact integers. Settlement scale is the currency's minor unit;
computation scale is 10^−9 of a minor unit. Amounts are encoded as
canonical decimal strings, because the data model caps integers at 64 bits
and aggregate sums exceed it.

Rounding happens once per invoice line, and the residue MUST be posted, not
discarded:

```
money * 10^9 + residue = precise_amount
```

### 8.5 Double entry

Every accounting entry MUST balance per currency. An implementation MUST
reject an unbalanced entry rather than posting and reconciling later. A
correction MUST be a new balanced entry; a posted entry MUST NOT be edited
or removed.

---

## 9. Subject pseudonyms (RECOMMENDED)

A subject holds a 256-bit root secret and derives, per controller:

```
SubjectRef(org)  = first 16 bytes of Digest(sha2-256, "duap.pseudonym.v1",
                     root || 0x00 || org_id)
SubjectKey(org)  = seed Digest(sha2-256, "duap.subject-key.v1",
                     root || 0x00 || org_id)
```

**Holds:** two non-colluding controllers cannot tell whether two references
belong to the same person; the subject can recompute all their pseudonyms
offline.

**Does not hold:** unlinkability against content; unlinkability at payout
time, where some party must learn that pseudonyms belong together;
protection against a controller that already knows who the subject is.

---

## 10. Extension and versioning

- Within a major version, only OPTIONAL fields with new wire names may be
  added. Receivers reject unknown fields, so an addition requires version
  negotiation.
- Any change to the meaning of an existing field, or any removal or
  requirement change, is a new major version **and a new domain label**, so
  that signatures cannot cross versions.
- Extension keys MUST be namespaced with a `.` and MUST NOT use the `duap.`
  namespace.
- Relays that do not understand a version can still forward and verify
  signatures, because signatures cover exact bytes rather than parsed
  structures.

---

## 11. IANA-style registries

These registries are maintained in this repository pending any standards
process. Additions require an ADR.

**Domain labels.** `duap.event.v1`, `duap.grant.v1`, `duap.revocation.v1`,
`duap.receipt.v1`, `duap.log-entry.v1`, `duap.sth.v1`, `duap.sig-input.v1`,
`duap.key-id.v1`, `duap.commitment.v1`, `duap.pseudonym.v1`,
`duap.subject-key.v1`, `duap.log.leaf.v1`, `duap.log.node.v1`,
`duap.log.empty.v1`, `duap.op-fingerprint.v1`, `duap.key-registry.v1`,
`duap.pricing-schedule.v1`, `duap.invoice.v1`, `duap.dispute.v1`,
`duap.settlement.v1`, `duap.journal-entry.v1`, `duap.bid-commitment.v1`,
`duap.negotiation.v1`, `duap.ack.v1`, `duap.object.v1`, `duap.stream.v1`,
`duap.event-id.v1`, `duap.test.v1`.

**Hash algorithms.** `sha2-256` (required), `blake3-256` (optional).

**Signature suites.** As §3.1.

**Taxonomy.** Data classes, operations, purposes, units, collection
methods, sensitivity tiers, regimes and lawful bases are defined by
`ontology/duap-ontology-v1.json`, whose SHA-256 is published in the
`taxonomy.json` vector. An implementation MUST agree on every code and its
attributes.

---

## 12. Security considerations

See `SECURITY.md` for the arguments and `THREAT_MODEL.md` for the
adversaries. In summary, this specification depends on:

- the collision and preimage resistance of SHA-256;
- the existential unforgeability of the signature suites in use;
- correct, constant-time implementations of those suites;
- an honest key registry, or gossip sufficient to detect a dishonest one;
- relying parties that actually check consistency proofs.

It does **not** depend on, and does not provide: confidentiality of data
(none is transported), correctness of assertions, completeness of
reporting, or prevention of equivocation.

## 13. Privacy considerations

See `PRIVACY.md`. In summary: the protocol transports no personal data and
the log commits only to digests; pseudonyms are per-controller; the largest
residual exposures are the metadata in an event (class, operation, purpose,
timing, volume), which is itself revealing at scale, and the linkage that
becomes necessary at payout time.
