# No hallucinated implementation

Every module, type and function carries an explicit maturity marker in its
documentation comment, from this set:

| Marker | Meaning |
|---|---|
| `PRODUCTION` | Implemented, tested, and intended for real use |
| `PRODUCTION-CANDIDATE` | Implemented and tested, not yet reviewed or audited to the standard its role requires |
| `REFERENCE` | Implemented and correct, but not built for production characteristics (throughput, hardening, operability) |
| `PROTOTYPE` | Implemented, compiles, has tests, but does not meet `REFERENCE`'s bar -- for a normative subsystem, no conformance vectors exist |
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

## Relationship to rule 10

This set and rule 10's status set share four words -- `PROTOTYPE`,
`REFERENCE`, `EXPERIMENTAL`, `PRODUCTION` (and `PRODUCTION-CANDIDATE`,
spelled with an underscore there) -- and they mean the same thing in both
places. ADR-0015 records why they were aligned: they previously differed
by one word, which silently disabled the marker check across four crates.

`STUB` and `UNIMPLEMENTED` are item-level only. `CONCEPT`, `RESEARCH`,
`SPECIFIED`, `DEPRECATED` and `REJECTED` are subsystem-level only.
`docs/STATUS.md` remains the single authority for a subsystem's status; a
module marker describes the module.

## Checking

`.claude/hooks/check-markers.sh` fails when a new public module lacks a
marker. With `--all` it scans every tracked source file and exits non-zero,
which is how CI runs it; without it, it inspects the staged change and only
warns, because a false block at commit time trains people to bypass hooks. It cannot judge whether a marker is honest; that is a review
responsibility, and `red-team-engineer` treats an inflated marker as a
finding.
