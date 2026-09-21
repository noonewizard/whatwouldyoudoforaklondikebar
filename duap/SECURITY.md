# Security

**Status:** PRODUCTION (process artefact) · 2026-09-21

This document holds the *arguments*. `THREAT_MODEL.md` holds the
adversaries; `security/findings.md` holds open findings.

## Reporting a vulnerability

This is a research repository with no production deployment. Open a
GitHub issue, or contact the maintainers privately if the finding affects
a third party. There is no bug bounty and no service-level commitment on
response.

## What has and has not been reviewed

| | |
|---|---|
| Independent security audit | **None.** No part of this repository has been audited. |
| Independent cryptographic review | **None.** |
| Internal adversarial testing | Yes: `security/findings.md`, and adversarial tests throughout the suites. |
| Second implementation | Partial: `gateway/` implements L1–L3 and passes all 77 vectors in range. |
| Formal model checking | Yes, of the algorithms, not the implementation: `formal/README.md`. |

Nothing in this repository should be relied on to protect real money or
real personal data until at least the first two rows change.

## Dependency surface

Keeping this small is a security property, so it is counted.

| Crate | Non-workspace dependencies | Why |
|---|---|---|
| `duap-canon` | serde, serde_json, ciborium, sha2, blake3, hex, base64, thiserror | Codec and digests |
| `duap-crypto` | ed25519-dalek, fips204, sha2, getrandom, hex, thiserror, serde | Signature suites |
| `duap-model` | serde, time, getrandom, hex, thiserror | Object model |
| `duap-auth`, `duap-provenance`, `duap-meter`, `duap-valuation`, `duap-ledger`, `duap-receipt` | serde, thiserror only | Pure logic |
| `duap-gateway` | serde, serde_json, hex, clap | Hand-written HTTP stack, ADR-0011 |

No crate on the trust path depends on an async runtime, a web framework, a
TLS stack or a database driver. `duap-clearing` optionally depends on
`rusqlite` behind a feature flag that is off by default.

`ed25519-dalek` and `fips204` are the two dependencies whose correctness the
protocol's security rests on. Neither has been audited by us. `fips204` is a
young implementation of a young standard, which is one reason the hybrid
suite exists: a flaw in it does not by itself forge a hybrid signature.

## Security arguments

Each argument states the property, the adversary, the assumption, what it
does not cover, and the test.

### S-1 Object integrity

**Property.** An object cannot be altered after signing without detection.

**Adversary.** Anyone who can modify bytes in transit or at rest.

**Assumption.** SHA-256 is collision and second-preimage resistant; the
signature suite is existentially unforgeable under chosen-message attack.

**Argument.** The signature covers the canonical encoding of a structure
containing `Digest(sha2-256, domain, payload)`. Altering the payload
changes the digest, which changes the signed structure, which invalidates
the signature. The canonical encoding is injective (INV-E1), so no two
payloads share bytes.

**Does not cover.** Whether the payload's contents are true.

**Tests.** `payload_tampering_detected` flips every byte of a payload and
requires each to fail.

### S-2 No type confusion

**Property.** A signature over one object type never verifies as another.

**Adversary.** Anyone holding a legitimately signed object.

**Assumption.** Domain labels contain no NUL byte (asserted in code), so
`prefix || 0 || domain || 0 || payload` is injective in `(domain,
payload)`.

**Argument.** The payload domain appears both in the digest input and as a
separate field of the signature input. Changing it changes both.

**Does not cover.** Confusion between two objects of the *same* type; that
is the job of the content binding in each object (grant digests, event-set
roots).

**Tests.** `domain_substitution_detected`, `domain_separation` (property).

### S-3 No algorithm substitution or downgrade

**Property.** An attacker cannot present a signature as though it came
from a different suite or key, nor strip the post-quantum half of a hybrid.

**Adversary.** Anyone holding a signed envelope.

**Assumption.** The key identifier is a hash of the key material, checked
on every verification.

**Argument.** The suite label and key identifier are inside the signed
structure. A verifier recomputes the key identifier from the registered
public key and rejects a mismatch. Stripping the ML-DSA half of a hybrid
changes the signature length, which is checked before any verification is
attempted.

**Tests.** `suite_and_key_substitution_detected`, `hybrid_requires_both_halves`.

### S-4 Hybrid unforgeability

**Property.** `ed25519+ml-dsa-44` is existentially unforgeable if *either*
component is.

**Adversary.** Including one with a cryptanalytically relevant quantum
computer (breaks Ed25519) or a break of ML-DSA.

**Argument.** The signature is the concatenation and both halves must
verify over the same message. A forgery therefore contains a forgery of
each component, so producing one requires breaking both.

**Does not cover.** Implementation flaws that leak the key from either
component — a side channel in one half exposes that half's key, and if the
same seed derives both, an attacker who recovers the seed has both. The
seed is the single point of failure and is stated as such in ADR-0004.

### S-5 Idempotent accounting under replay

**Property.** Replaying an event cannot inflate a count.

**Adversary.** A network adversary with captured traffic.

**Argument.** Dedup is keyed on the canonical digest, which is the event's
identity. A replay is byte-identical and is recognised.

**Does not cover.** Replay after the window closes, which is rejected as
stale rather than recognised as a replay; and a *different* event
describing the same operation, which is the double-count problem and has
its own control.

**Tests.** `counting_is_idempotent` (property), `pipeline_counts_once_per_event`.

### S-6 Append-only history

**Property.** A log cannot remove or alter an entry without detection by a
party that saw an earlier head.

**Adversary.** The log operator.

**Assumption.** RFC 6962 construction with domain-separated leaf and node
hashing; relying parties actually check consistency proofs.

**Argument.** A consistency proof between sizes *m* and *n* verifies only
if the tree of size *m* is a prefix of the tree of size *n*. Any change to
an entry ≤ *m* changes the root at *m*.

**Does not cover.** Equivocation: showing two consistent-but-different
histories to two parties. That is detected only by comparing heads, which
requires gossip. **Detection, not prevention.**

**Tests.** `consistency_proof_catches_a_rewritten_history`,
`log_monitor_rejects_regression_and_forks`,
`every_consistency_proof_verifies` (property).

### S-7 Authorization soundness

**Property.** No receipt covers an operation the cited grant did not
permit.

**Adversary.** A controller trying to widen its permission.

**Argument.** The clearing node evaluates before it counts (pipeline stage
4 precedes 5–9). Evaluation is deterministic, binds the grant's digest,
and is deny-overriding. The algorithm is model-checked
(`formal/Authorization.tla`) and mirrored in tests.

**Does not cover.** A dishonest clearing node that skips evaluation. That
is T-14, accepted, and the mitigation is the coverage root a subject can
check.

### S-8 Commitment hiding and binding

**Property.** A published commitment reveals nothing about the value and
cannot be opened two ways.

**Assumption.** SHA-256 is collision resistant; the salt is 256 bits of
uniform randomness.

**Argument.** Binding follows from collision resistance of the
length-prefixed input. Hiding follows from the salt: even for a value from
a small domain, the commitment is indistinguishable from random without
the salt.

**Does not cover.** A committer who reuses a salt, or generates it from a
predictable source. The demonstration uses a fixed salt for
reproducibility and says, at the call site, that a real agent must not.

**Tests.** `commitments_bind_and_hide`, `commitment_concatenation_is_injective`.

### S-9 Input hardening

**Property.** Malformed or hostile input cannot crash a decoder or cause
unbounded allocation.

**Argument.** The decoder bounds nesting depth, collection length and total
input; a length header larger than the remaining input is rejected before
any allocation; every decode path returns a typed error rather than
panicking.

**Tests.** `decode_never_panics` (property, arbitrary bytes),
`hostile_length_header_does_not_allocate`, `depth_limit_enforced`.

**Not done.** No coverage-guided fuzzing has been run. The property tests
generate arbitrary byte strings, which is weaker. Recorded as a gap.

## Supply chain

Present:

- A committed `Cargo.lock`, `go.sum` and pinned tool versions.
- A pinned SHA-256 for `tla2tools.jar`, checked by `formal/check.sh`.
- A dependency surface small enough to enumerate in this document.
- CI runs `cargo audit` and `cargo deny` where available.

Absent, and it matters:

- No signed releases; there are no releases.
- No SLSA provenance and no reproducible-build verification.
- No SBOM published with an artefact, because no artefact is published.
- The agent build attestation field in an event is optional and unverified
  by the protocol (T-31).

## Cryptographic hygiene notes

- Ed25519 verification uses the strict, cofactorless equation, rejecting
  small-order points and non-canonical encodings.
- Signing is deterministic so that conformance vectors are reproducible.
  The known cost is reduced resistance to fault-injection attacks on the
  signing device; a deployment that cares should supply a hedged `Signer`
  over an HSM. Stated in `key.rs` at the point of the trade-off.
- Secret seeds are scrubbed on drop with a volatile write. This does not
  prevent copies the optimiser or the allocator already made, and the code
  says so rather than implying a guarantee.
- Digest comparison is not constant time. Digests are public; no secret
  depends on the comparison.
