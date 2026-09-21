# Governance

**Status:** REFERENCE (a proposal, not an operating structure) · 2026-09-21

**Read this first:** nothing described here exists. There is no
foundation, no steering committee, no membership, no trademark and no
certification programme. This document is a design for a structure that
would have to be created, and `docs/reviews/falsification.md` kill attempt
9 rates the viability of *any* governance structure for this protocol as
**unresolved** — one of two findings that could end the project. Writing a
governance design does not resolve it.

## 1. The problem this must solve

Two failure modes, each fatal, each the other's mitigation:

- **Company-controlled.** A protocol owned by one company is not neutral,
  and no competitor builds accounting infrastructure on a competitor's
  standard. The specification drifts toward whatever the funded
  implementation does.
- **Foundation-controlled with no funding.** A protocol governed by a body
  with no engineering capacity publishes documents and waits. Gaia-X is
  the current worked example, and its own community treats adoption rather
  than specification as the open question **[search-summary]**
  ([Forrester](https://www.forrester.com/blogs/gaia-x-is-back-but-only-adoption-will-signal-success)).

The structure below is the standard split. It is proposed because it is
the one with precedent, not because this project has evidence it works
here.

## 2. The split

| Under foundation governance | Commercial |
|---|---|
| The protocol specification | Hosted clearing infrastructure |
| Canonical encoding rules and domain registry | Managed gateway operation |
| Object schemas and wire format | Analytics over a customer's own usage |
| Conformance vectors and the conformance suite | Compliance and audit tooling |
| The verifier | Support, integration, professional services |
| The reference SDK | Certification administration (fees to the foundation) |
| Trademark and certification marks | — |

**The test applied:** anything an independent implementation needs in
order to interoperate is foundation-governed. Anything that is a service
around a working protocol is commercial. Put another way — if withholding
it would let one party break another's implementation, it cannot be
commercial.

**The specific commitment this implies** is that the conformance vectors
and the verifier are never commercial. A certification programme whose
test suite is proprietary certifies nothing except a relationship with the
certifier.

## 3. What would have to be true

Stated as conditions rather than plans, because none is satisfied.

| Condition | Status |
|---|---|
| An existing standards body willing to host the work | **Unmet.** Candidates: IDSA, the Linux Foundation (which hosts the FinOps Foundation), W3C community group, RSL Collective. None approached |
| At least two independent implementers with a commercial interest | **Unmet.** One implementation exists; its second was written by the same author |
| A funding model that does not depend on protocol rents | **Designed** (`docs/business/business-model.md`), untested |
| Trademark held by the neutral body from the start | **Unmet.** No mark exists, and this gets harder the longer it waits |
| Patent non-assertion covering the specification | **Unmet.** See §6 |

**The honest reading:** creating a new foundation is almost certainly the
wrong answer. The cost of standing one up is high, the neutrality is
nominal until there are members with divergent interests, and the
precedents for a single-vendor foundation are poor. Contributing the
specification to a body that already has convening power is more likely to
work and is the recommended path — and `docs/market/go-to-market.md`
stage 4 makes "a work item accepted somewhere that is not this repository"
an explicit gate rather than an aspiration.

## 4. Versioning and compatibility

These rules are enforceable today and are the part of this document that
is real.

- The protocol version is in the digest domain (`DUAP/1`). A change to
  canonical encoding, digest construction or the signature input is a new
  major version and is not backward compatible by construction — a
  mismatched implementation fails to verify rather than silently
  disagreeing. This is deliberate: silent disagreement about a digest is
  the worst failure available to a protocol whose objects are evidence.
- Adding an optional field to an object is a minor version. Strict
  decoding with `deny_unknown_fields` means an old implementation
  *rejects* an object carrying a new field rather than ignoring it, so
  even minor additions require a negotiated version.
- Conformance vectors are append-only. A changed vector is a wire-format
  change requiring an ADR, and
  `crates/duap-conformance/tests/vectors_test.rs` fails the build if a
  committed vector stops matching the generator or if the count falls.
- An accepted ADR is never edited except to add a supersession note.

## 5. Conformance and certification

Levels L1–L5 are defined in `specs/protocol-v0.1.md` and implemented as 93
vectors. An implementation states the level it claims.

**What a certification mark could honestly assert:** that an
implementation passed the published vectors for a stated level on a stated
date. Nothing else. Not that it is secure, not that it is correct beyond
the vectors, not that its operator is trustworthy.

**What it must not become:** a revenue mechanism that gates
interoperability. The vectors are public and anyone may run them; the mark
is a claim about having done so under observation.

## 6. Intellectual property

- **Specification and schemas:** should be published under terms
  permitting independent implementation without permission or fee. Not yet
  licensed; `LICENSE` covers the code only.
- **Patents:** no patent application has been filed and no prior-art
  search has been conducted. No claim of patentability is made anywhere in
  this repository, and none should be — asserting patentability without a
  search is exactly the unsupported claim the project rules forbid. The
  defensible options are defensive publication (this repository, with its
  commit history, is already one) and a non-assertion covenant. Whether
  anything here is novel is **UNKNOWN**.
- **Trademark:** none registered. If a certification mark is ever to mean
  anything it must be held by the neutral body, not the company, and
  acquired before adoption rather than after.

## 7. Decision-making, if a body existed

Recorded so the structure is complete, and marked as what it is.

- **Technical decisions** by rough consensus among implementers, with an
  ADR for anything in rule 05's list.
- **Two independent implementations** required before a feature enters the
  specification. This project already applies a weaker version of the rule
  and states the weakness: the Go verifier and the Rust reference share an
  author, which is recorded as a limit in
  `docs/architecture/minimal-protocol.md` §10.
- **Conflict of interest** declared on any decision affecting a member's
  commercial position, which for an accounting protocol is most of them.
- **Fork right** preserved: permissive licensing, public vectors and a
  published specification mean a captured protocol can be forked. This is
  the only governance guarantee that does not depend on the good behaviour
  of the governing body, and therefore the only one worth relying on.

## 8. Capture risks

| Risk | Mitigation | Honest assessment |
|---|---|---|
| One vendor's implementation becomes the de facto specification | Conformance vectors; an independent verifier | Weak while both were written by one author |
| A large adopter drives the specification toward its own needs | Two-implementation rule; ADRs | Untested. A sufficiently large adopter is usually accommodated |
| The foundation is funded by one company | Diversified membership | Unmet, and the most likely actual failure |
| Certification becomes a toll | Public vectors; anyone may self-test | Structurally sound |
| The protocol is forked and fragments | Nothing prevents it | Acceptable. Fragmentation is a better failure than capture |
