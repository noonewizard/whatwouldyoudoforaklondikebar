# No fake benchmarks

Every performance claim anywhere in the repository -- documentation, code
comments, commit messages, specifications -- must cite a result file in
`benchmarks/results/` produced by a committed harness.

## Requirements

1. A result file records: the harness, the machine (CPU model, core count,
   memory), the toolchain version, the date, the exact command, and the raw
   output.
2. No extrapolation is presented as a measurement. "1.2 million events per
   second per core, therefore 1.2 billion across a thousand cores" is a
   model, and must be labelled as one with its assumptions stated.
3. A microbenchmark result may not be described as a system throughput.
4. Numbers in prose are rounded consistently and never rounded in the
   flattering direction.
5. If a benchmark regressed, the regression is reported, not re-run until it
   passes.

## What counts as a performance claim

"Fast", "scalable", "high-throughput", "low-overhead", "negligible cost" and
their synonyms are performance claims. Either cite a measurement or delete
the word.
