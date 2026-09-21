---
description: Run the end-to-end synthetic transaction and check every slice invariant
---

Run the vertical slice and report what it proves.

1. `cargo run -q -p duap-demo` and read the transcript.
2. `cargo test -p duap-demo` -- the golden transcript, determinism, and the
   invariant assertions.
3. Confirm, from the output rather than from memory:
   - the trial balance is zero after every stage;
   - every issued receipt verified and was anchored;
   - each of the refusals exercised a *different* control;
   - the revocation stopped training and not service;
   - the attribution sums as expected with terminated weight accounted.
4. Report anything the slice does **not** exercise. That list is the honest
   measure of the slice, and it belongs in
   `docs/reviews/vertical-slice-review.md`.

Do not change code to make the slice pass. A failing slice is information.
