# Status discipline

Every subsystem holds exactly one status in `docs/STATUS.md`:

```
CONCEPT -> RESEARCH -> SPECIFIED -> PROTOTYPE -> REFERENCE
        -> EXPERIMENTAL
        -> PRODUCTION_CANDIDATE -> PRODUCTION
        -> DEPRECATED
        -> REJECTED
```

## Requirements

1. A status changes only in a commit that changes nothing else, with the
   evidence for the transition in the commit message.
2. `chief-architect` approves every transition.
3. Transitions have entry criteria, and a transition that does not meet them
   does not happen:

   | Target | Entry criteria |
   |---|---|
   | SPECIFIED | A written specification another team could implement from |
   | PROTOTYPE | Implemented, compiles, has tests |
   | REFERENCE | Tests cover the specified behaviour; conformance vectors exist |
   | EXPERIMENTAL | Results published with methods, seeds and uncertainty |
   | PRODUCTION_CANDIDATE | Red-teamed, benchmarked, documented, operable |
   | PRODUCTION | Independently reviewed; for cryptographic subsystems, independently audited |

4. No subsystem reaches PRODUCTION while any open red-team finding against it
   is unresolved and unaccepted.
5. A status may go down. A regression that invalidates an entry criterion
   lowers the status in the same commit that discovers it.
