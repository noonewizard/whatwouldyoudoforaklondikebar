# ADR-0004: Key custody is a deployment decision

**Status:** accepted · 2026-09-21

## Context

Signing keys in DUAP protect accounting records that may be disputed years
later. The reference implementation holds a 256-bit seed in process memory
and derives component keys from it. That is appropriate for a reference
implementation and inappropriate for a clearing node handling real money.

## Problem

Does the protocol mandate a custody model?

## Alternatives

1. **Mandate hardware custody** for specified roles.
2. **Ship a software signer only** and say nothing.
3. **Define a signing interface** and ship a software implementation,
   leaving custody to deployment.

## Decision

Define `Signer` as a trait taking an already-assembled signing input and
returning a signature. Ship `SoftwareSigner`. Write every component that
needs to sign against the trait, not against the concrete key type.

Custody is a deployment decision, recorded in `OPERATIONS.md`, not a
protocol requirement.

## Trade-offs

The protocol cannot require a security property it has no way to verify. A
relying party cannot tell from a signature whether the key lived in an HSM
or in a container's environment. So the protocol gains nothing from
mandating custody, and would gain a requirement nobody can check.

What is lost: a deployment can be careless, and the protocol will not
notice.

## Consequences

- No component takes a `SecretKey` where a `Signer` would do, except the
  reference implementations that construct one.
- The seed-based derivation is an implementation convenience, explicitly
  not required for interoperability: only public keys and signatures are on
  the wire.
- `OPERATIONS.md` states the custody expectations per role, and
  `GOVERNANCE.md` makes them an audit item rather than a protocol rule.
- An HSM-backed `Signer` is UNIMPLEMENTED in this repository and is marked
  so.

## Rejected alternatives

**Mandating hardware custody** is rejected because it is unverifiable from
outside and would therefore be a claim rather than a control. It would also
exclude the deployments most likely to adopt an accounting layer first:
small participants for whom an HSM is a disproportionate cost.

**A software signer with no interface** is rejected because it makes the
custody upgrade a fork rather than a configuration change.
