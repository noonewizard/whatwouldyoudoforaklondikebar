---
name: benchmark-reporting
description: How to produce a benchmark result that survives scrutiny. Use before writing any performance number into the repository.
---

# Benchmark reporting

## Produce the measurement

1. Record the environment first, into the result file: CPU model and count,
   memory, kernel, toolchain versions, date, and whether the machine is
   shared or virtualised. A number without a machine is not a measurement.
2. Run the committed harness. If the harness is not committed, commit it
   before running it.
3. Capture raw output verbatim. Summaries go beneath the raw output, never
   in place of it.

## Interpret it honestly

4. Report the spread, not only the central value. Criterion gives a
   confidence interval; keep it.
5. State what the benchmark does **not** measure. A signing microbenchmark
   does not measure ingest throughput: it excludes parsing, validation,
   authorization, deduplication, storage and the log.
6. Convert to a per-event or per-byte figure where that is the useful unit,
   and show the arithmetic.
7. If you model a larger system from the measurement, label the model as a
   model and list its assumptions. A model is useful; a model presented as a
   measurement is a fabrication.

## Keep it true

8. Every prose claim citing the result links to the result file.
9. When a result regresses, publish the regression. Re-running until the
   number improves is the failure this procedure exists to prevent.
10. Results are dated. A number older than the code it describes is removed,
    not quietly reused.
