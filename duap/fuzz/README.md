# Fuzz targets

**Status:** REFERENCE · 2026-09-21

Coverage-guided fuzzing of every place DUAP parses input it did not
produce. The vertical-slice review called this the highest-value missing
test in the repository, on the grounds that `duap-canon` parses
adversary-supplied bytes on a network path and nothing had ever attacked
it with a coverage-guided fuzzer.

That judgement was correct. The first target found a denial-of-service
defect within two minutes of its first run (FUZZ-01).

## Targets

| Target | Input | Property asserted |
|---|---|---|
| `canon_decode` | Arbitrary bytes | Anything the strict decoder accepts must re-encode to identical bytes. Canonical form is injective or signatures do not work |
| `canon_roundtrip` | A structured `Value` built from the bytes | Encode-then-decode preserves the value, and encoding is deterministic. Reaches values the byte-first target cannot, because arbitrary bytes rarely decode |
| `event_decode` | Arbitrary bytes as a `DataUsageEvent` | No panic; accepted events re-encode identically; the digest is stable across a round trip, since it is what signatures commit to |
| `json_view` | Both directions | The JSON view is a second parser on the trust path and therefore a second place canonical form can break. VS-4 was found here by hand |
| `envelope_verify` | Arbitrary bytes as a signed envelope | Verification never panics **and never succeeds** on fuzzer-supplied input. A success is a forgery |
| `inclusion_proof_verify` | Arbitrary bytes as a Merkle proof | Verification terminates on any claimed path length and never verifies against an unrelated root |

Two of the six assert a *negative*: `envelope_verify` and
`inclusion_proof_verify` fail if the fuzzer ever produces something that
verifies. Those are the targets where a finding would be a break rather
than a crash.

## Running

```
rustup toolchain install nightly
cargo install cargo-fuzz

cd fuzz
cargo +nightly fuzz run canon_decode -- -max_total_time=300
cargo +nightly fuzz list
```

The crate is deliberately outside the workspace: the targets build with a
nightly toolchain and sanitizer flags, and pulling them in would impose
that on every ordinary `cargo build`.

## What is committed, and what is not

Corpora and crash artefacts are **not** committed. A corpus is a cache
that grows without bound — 23 MB after fifteen minutes — and a crash blob
is not evidence anybody can read.

A crash becomes a **named regression test in the crate it belongs to**,
with the input written out in source and the finding cited. FUZZ-01 is
`a_64_bit_length_header_cannot_overflow_the_bounds_check` in
`crates/duap-canon/tests/regressions.rs`, and it generalises the one input
libFuzzer found to all four length-carrying major types.

## Results

`benchmarks/results/2026-09-21-fuzzing.md` records the campaign: what was
run, for how long, how many executions, and what was found. As with every
other measurement here, it is a record of one session on one machine and
not a claim that the code is free of defects. Fuzzing finds bugs; it never
shows their absence.
