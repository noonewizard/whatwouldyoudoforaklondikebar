# DUAP protocol

**Status:** SPECIFIED · 2026-09-21
**Normative text:** `specs/protocol-v0.1.md`. This file is a map, not a
specification; where the two differ, the specification wins.

## What DUAP standardises

Three objects, and nothing else:

| Object | Answers |
|---|---|
| **Data Usage Event** | What was done with which data, under which authorization, by whom, how much |
| **Data Usage Receipt** | A clearing node's signed statement about a set of events, with its limits attached as data |
| **Economic obligation** | What is owed as a result, computed reproducibly from the metered quantity and the agreed term |

Plus two supporting objects the three depend on: the **Grant** (the
authorization being exercised) and the **Log entry** (the transparency-log
commitment that timestamps a receipt).

Everything else — identity, catalogues, rights expression, negotiation
transport, provenance serialisation, telemetry, settlement — is bound to an
existing standard rather than defined here. `docs/standards/prior-art.md`
records nine such bindings against these three definitions, and states that
if the ratio ever inverts the project has lost its argument.

## The kernel

```
canonical encoding  ->  object schemas  ->  signature construction  ->  evaluation semantics
```

Four documents, five objects. `docs/architecture/minimal-protocol.md`
derives them and states the defining architectural test: the kernel must be
small enough that an independent organisation could implement it without
adopting this software stack.

**Evidence for that test, and its limit:** `gateway/` is a Go
implementation of conformance levels L1–L3 written against the
specification rather than ported from the Rust. It passes all 77 vectors in
its range and found a real specification defect (VS-4). The same author
wrote both, and L4–L5 have no second implementation, so this is evidence of
independent implementability rather than proof of it.

## Conformance levels

| Level | Capability | Who needs it |
|---|---|---|
| L1 | Canonical encoding, decoding, digests | Everyone |
| L2 | L1 + signature verification | Anyone checking authenticity |
| L3 | L2 + Merkle proofs and receipt verification | A browser extension checking receipts |
| L4 | L3 + authorization evaluation | A gateway deciding whether to permit |
| L5 | L4 + pricing arithmetic | A clearing node |

93 vectors across 8 files in `spec/vectors/`, checked by `cargo test`
(`crates/duap-conformance/tests/vectors_test.rs`) and by the independent Go
verifier. An implementation states the level it claims; a changed vector is
a wire-format change and fails the build.

## The properties the protocol provides

Stated precisely, because rule 03 forbids the loose version.

| Property | Holds against | Rests on | Does not cover |
|---|---|---|---|
| Canonical form is injective | A party choosing an alternative encoding of the same value | Length-first deterministic CBOR; strict decoding rejects anything the encoder would not emit | Semantic equivalence — two different values that mean the same thing |
| Domain separation | Presenting one object's digest where another's is expected | `H("DUAP/1" ‖ 0x00 ‖ domain ‖ 0x00 ‖ payload)` being injective in (domain, payload) | Anything outside the digest |
| Non-repudiation of a report | A party later denying or altering what it said | Signature unforgeability, plus key hygiene at the signer | Whether what it said was true |
| Detection of equivocation | A log operator serving different histories | RFC 6962 consistency proofs, **conditional on auditors gossiping** | Prevention. Nothing stops equivocation; gossip detects it |
| Detection of partial suppression | A reporter omitting events from a stream | Per-stream monotonic sequence numbers | A reporter that never starts a stream |
| Detection of double counting | Two reporters claiming the same operation | A reporter-independent operation fingerprint | Two reporters colluding on one report |
| Conservation of value | Rounding, apportionment, netting and payout losing or inventing money | Exact integer arithmetic at two scales with residue posting; INV-L1 | Whether the amounts were right in the first place |

## What the protocol does not provide

Every item here is accepted, documented and reachable from the artefact a
reader would consult.

- **That the described operations occurred.** The clearing node signs, so
  the clearing node could fabricate or omit. RISK-01, accepted.
  `Receipt::claims()` returns this as one of five not-established claims,
  in data, so a verifier cannot read a receipt without reading its limits.
- **That unreported usage did not occur.** RISK-02, accepted. Copying data
  outside instrumentation generates no events and no cryptography detects
  it.
- **Erasure.** Revocation binds future use. It does not un-train a model,
  and the demonstration prints `already_trained_model=NOT_UNLEARNED`.
- **Unlinkability against content.** Per-controller pseudonyms prevent
  joining on the identifier and do nothing against joining on payload,
  timing or quasi-identifiers.
- **Attribution of value.** Refused on evidence; see `AI_ATTRIBUTION.md`.
- **That the implementation matches the formal models.** Three models are
  checked exhaustively within bounds; none is extracted from the source.

## Versioning

The protocol version is in the digest domain (`DUAP/1`). A change to
canonical encoding, digest construction or the signature input is a new
major version, and a mismatched implementation fails to verify rather than
silently disagreeing — which is the intended behaviour for a protocol whose
objects are evidence. Strict decoding with `deny_unknown_fields` means even
an added optional field requires a negotiated version. `GOVERNANCE.md` §4
has the rules.

## Reading order

1. `docs/architecture/minimal-protocol.md` — why these objects and no others
2. `specs/protocol-v0.1.md` — the normative text
3. `specs/invariants.md` — 25 invariants, each mapped to an enforcement point and a test or model
4. `SECURITY.md` and `THREAT_MODEL.md` — the arguments and the adversaries
5. `spec/vectors/` — what an implementation must reproduce
6. `docs/reviews/vertical-slice-review.md` — what broke when it was first run end to end
