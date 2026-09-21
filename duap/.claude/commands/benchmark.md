---
description: Run the benchmark suite and record a reproducible result file
---

1. Record the environment: `nproc`, `free -m`, `cargo --version`, `uname -a`.
2. Run `cargo bench --workspace -- --output-format bencher` (or the named
   subset) and capture the raw output.
3. Write `benchmarks/results/<ISO-date>-<short-machine>.md` containing the
   environment block, the exact command, the raw output, and a short
   interpretation.
4. Update any performance claim that the result changes. If a claim is no
   longer supported, remove it in the same commit.
5. Never re-run to get a better number. If the result is noisy, say so and
   report the spread.

Rule: `.claude/rules/02-no-fake-benchmarks.md`.
