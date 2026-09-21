# No hallucinated implementation

Every module, type and function carries an explicit maturity marker in its
documentation comment, from this set:

| Marker | Meaning |
|---|---|
| `PRODUCTION` | Implemented, tested, and intended for real use |
| `PRODUCTION-CANDIDATE` | Implemented and tested, not yet reviewed or audited to the standard its role requires |
| `REFERENCE` | Implemented and correct, but not built for production characteristics (throughput, hardening, operability) |
| `EXPERIMENTAL` | Implemented to answer a research question; results are provisional |
| `STUB` | Present so callers can compile; does nothing useful |
| `UNIMPLEMENTED` | Named in an interface, deliberately absent |

## Requirements

1. A public item without a marker is a defect.
2. `STUB` and `UNIMPLEMENTED` items must fail loudly when reached -- return an
   error naming themselves, never a plausible default. A stub that returns
   `Ok(())` is the specific failure this rule exists to prevent.
3. A marker may not be raised in the same change that implements the
   functionality. Implementation and promotion are separate reviews.
4. Prose describing an `UNIMPLEMENTED` capability must say so in the same
   paragraph, not in a footnote.

## Checking

`.claude/hooks/check-markers.sh` fails when a new public module lacks a
marker. It cannot judge whether a marker is honest; that is a review
responsibility, and `red-team-engineer` treats an inflated marker as a
finding.
