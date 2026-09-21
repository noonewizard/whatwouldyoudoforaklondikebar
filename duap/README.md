# DUAP — Data Usage Accounting Protocol

**Status:** PROTOTYPE, working toward SPECIFIED · protocol `DUAP/1` ·
2026-09-21

DUAP is an accounting layer for data usage. It lets an organisation assert,
in a verifiable and comparable form, that a specific operation was
performed on a specific class of data about a specific pseudonymous subject
under a specific authorization — and lets those assertions be counted,
priced, receipted, invoiced and settled.

It does not transport data, does not decide whether an assertion is true,
and does not decide what data is worth. Those boundaries are the design,
not omissions; `docs/architecture/minimal-protocol.md` explains why each
one is where it is.

**Start with [`ASSESSMENT.md`](ASSESSMENT.md)** if you want the verdict
before the machinery: what survived a deliberate attempt to kill the
thesis, what did not, and the two open questions that are not technical.

## Run it

Everything below runs locally, with no network and no services.

```bash
cd duap

# The whole transaction, end to end: authorization, collection, processing,
# derivation, licensing, AI training, four distinct refusals, a detected
# double count, a scoped revocation, period close, invoicing, independent
# receipt verification, provenance attribution, a dispute and settlement.
cargo run -p duap-demo

# The test suite: 250+ tests including property tests and adversarial cases.
cargo test --workspace

# The conformance vectors, checked by the reference implementation...
cargo run -p duap-conformance -- check spec/vectors

# ...and by an independent Go implementation written from the specification.
cd gateway && go run ./cmd/duap-verify ../spec/vectors && cd ..

# The formal models, exhaustively model-checked.
./formal/check.sh

# The benchmark harness.
cargo run --release -p duap-bench
```

## What it looks like

```rust
use duap_sdk::prelude::*;

// A subject authorises a controller, priced, with prohibitions.
let (grant, signed) = subject.authorize(&controller, grant_id, vec![
    Term::permit(1, Matcher::any()
        .classes(ClassSelector::Namespace { namespaces: vec!["location".into()] })
        .purposes(PurposeSelector::Under { roots: vec![Purpose::Service] }))
        .with_pricing(PricingRule::per_unit(
            Unit::Query,
            Precise::new(Currency::EUR, 2_000_000),
        )),
    Term::deny(2, Matcher::any()
        .purposes(PurposeSelector::Commercial { value: true })),
], now, Currency::EUR)?;

// The controller records a use. The agent refuses to emit anything the
// clearing node would reject.
let (event, digest, envelope) = agent.record_usage(
    UsageRecord::new(subject_scope, DataClass::LocationCoarse,
                     Operation::AccessQuery, Purpose::ServiceCore, 1, now),
    now,
)?;

// Anyone can verify the resulting receipt, and is handed its limits.
let (verified, claims) = verify_receipt(&receipt_envelope, &registry, &policy, now)?;
for c in claims.iter().filter(|c| !c.established) {
    println!("this receipt does NOT establish: {}", c.statement);
}
```

## The shape of it

```
subject ──grant──▶ controller ──event──▶ clearing node ──receipt──▶ subject
   │                   │                      │                        │
   │                   │                 transparency log              │
   │                   │                      │                        │
   └──revocation───────┘                   invoice ──▶ ledger ──▶ settlement
```

| Layer | Crate | What it owns |
|---|---|---|
| Canonical encoding | `duap-canon` | One encoding per value; domain-separated digests; the JSON view |
| Cryptography | `duap-crypto` | Suites, self-certifying key ids, signed envelopes, key registry |
| Object model | `duap-model` | Events, identifiers, pseudonyms, exact money, the taxonomy |
| Authorization | `duap-auth` | Grants, matchers, obligations, evaluation, revocation, negotiation |
| Provenance | `duap-provenance` | Transparency log, RFC 6962 proofs, derivation graph |
| Metering | `duap-meter` | Dedup, replay windows, gap detection, double counting, aggregation |
| Valuation | `duap-valuation` | Pricing rules, multipliers, schedules, auctions, distribution |
| Accounting | `duap-ledger` | Double entry, invoices, disputes, netting, settlement instructions |
| Receipts | `duap-receipt` | The Data Usage Receipt and what it does and does not prove |
| Clearing | `duap-clearing` | Nine-stage ingest, batching, period close |
| SDK | `duap-sdk` | Eight operations, one per protocol primitive |
| Gateway | `duap-gateway` | An observable HTTP ingest endpoint |
| CLI | `duap-cli` | `duax` |

Every crate is usable on its own. An organisation can meter without
clearing, verify receipts without metering, or evaluate authorizations
without any of it. That separability is what makes independent
implementation of parts possible, and independent implementation is the
test this project is written against.

## Reading order

1. `docs/architecture/minimal-protocol.md` — what the kernel is and what is
   deliberately excluded.
2. `specs/protocol-v0.1.md` — the normative specification.
3. `specs/invariants.md` — the 25 invariants, each mapped to the test or
   model check that enforces it.
4. `THREAT_MODEL.md` — the adversaries, and what is prevented, detected,
   deterred or accepted.
5. `docs/reviews/vertical-slice-review.md` — what the first working
   transaction proved and what it did not.

## Honest status

| Claim | Evidence |
|---|---|
| The kernel can be implemented twice | `gateway/` passes 77 of 77 vectors in its range, written from the specification |
| The algorithms hold their invariants | `formal/` model-checks 4 authorization and 3 accounting invariants exhaustively |
| The implementation agrees with the models | mirror tests of the same names in `crates/*/tests/model_mirror.rs` |
| The transaction works end to end | `cargo run -p duap-demo`, with a golden transcript CI diffs |
| Performance | `benchmarks/results/`: 7,829 events/s single-threaded, with the decomposition |

| Not claimed | Why |
|---|---|
| Audited | No independent security audit has been performed |
| Verified | Models are checked; the implementation is not proven to refine them |
| Production-ready | No subsystem is above PROTOTYPE or REFERENCE; see `docs/STATUS.md` |
| Standards-compliant | Compatibility with W3C, IETF, ISO and NIST work is analysed in `INTEROPERABILITY.md`, not certified |
| Legally compliant | `COMPLIANCE.md` maps capabilities to provisions; it is not legal advice |

## Licence

Apache-2.0. See `LICENSE`.
