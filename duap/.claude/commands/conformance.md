---
description: Regenerate conformance vectors and re-run every implementation against them
---

1. `cargo run -q -p duap-conformance -- generate spec/vectors`
2. `git diff --stat spec/vectors` -- any change here is a **wire-format
   change**. If it was not intended, stop and find out why.
3. `cargo run -q -p duap-conformance -- check spec/vectors` (reference
   self-check).
4. `cd gateway && go run ./cmd/duap-verify ../spec/vectors` (independent
   implementation, levels L1-L3).
5. If the independent implementation fails a vector the reference passes,
   the specification is ambiguous. Fix the specification, not the verifier.
6. Record any specification ambiguity found in
   `docs/reviews/vertical-slice-review.md`.
