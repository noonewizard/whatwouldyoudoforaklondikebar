# ADR-0015: Align the maturity-marker vocabulary with the status vocabulary

**Status:** accepted · 2026-09-21
**Decided by:** `chief-architect`

## Context

Two vocabularies describe maturity in this repository.

Rule 01 defines per-item **markers** for module documentation:
`PRODUCTION`, `PRODUCTION-CANDIDATE`, `REFERENCE`, `EXPERIMENTAL`, `STUB`,
`UNIMPLEMENTED`.

Rule 10 defines per-subsystem **statuses** for `docs/STATUS.md`:
`CONCEPT`, `RESEARCH`, `SPECIFIED`, `PROTOTYPE`, `REFERENCE`,
`EXPERIMENTAL`, `PRODUCTION_CANDIDATE`, `PRODUCTION`, `DEPRECATED`,
`REJECTED`.

The sets overlap but are not the same. Rule 10 has `PROTOTYPE`; rule 01
does not.

This was not noticed until the VS-8 status commit lowered four crates to
`PROTOTYPE` in `docs/STATUS.md` and wrote the same word into their module
headers. `.claude/hooks/check-markers.sh` greps for rule 01's set, so those
four crates read as having no marker at all. The hook did not catch it at
commit time because it only inspected staged files in `crates/*/src/` and
its exit code was advisory; it surfaced only when the hook was given an
`--all` mode and run over the whole tree in CI.

## Problem

Should the two vocabularies be reconciled, and if so in which direction?

## Alternatives

**A. Add `PROTOTYPE` to rule 01's marker set.** The vocabularies converge
on the states they share. A module that is implemented and tested but has
no conformance vectors has an honest word available, and the word means the
same thing in both places.

**B. Keep the sets distinct and forbid `PROTOTYPE` in module markers.**
The two vocabularies answer different questions — rule 01 asks "is this
code finished?", rule 10 asks "where is this subsystem in its lifecycle?" —
and conflating them loses that distinction. Crates currently marked
`PROTOTYPE` would move to `REFERENCE` with a caveat, or to `EXPERIMENTAL`.

**C. Collapse rule 01 into rule 10 entirely.** One vocabulary, used at both
granularities.

## Decision

Take **A**. Add `PROTOTYPE` to rule 01's marker set, defined as
*implemented, compiles, has tests, but does not meet REFERENCE's criteria*
— which for a normative subsystem means no conformance vectors exist.

Rule 01 keeps `STUB` and `UNIMPLEMENTED`, which have no rule 10 equivalent
and describe item-level states a subsystem status cannot express. Rule 10
keeps `CONCEPT`, `RESEARCH`, `SPECIFIED`, `DEPRECATED` and `REJECTED`,
which describe subsystems rather than code and would be meaningless on a
module.

Normalise the spelling: rule 01 writes `PRODUCTION-CANDIDATE` and rule 10
writes `PRODUCTION_CANDIDATE`. The hook accepts both; the documents keep
their existing spellings rather than churning every file for a hyphen.

## Trade-offs

The distinction alternative B defends is real and this decision blurs it.
A module marked `REFERENCE` inside a subsystem at `PROTOTYPE` is now
possible to write without an obvious contradiction, and a reader could
take the module marker as the subsystem's status.

That cost is accepted because the alternative is worse in practice: two
vocabularies, differing in one word, is precisely the shape of
inconsistency that produces an unnoticed defect — which it already did.
`docs/STATUS.md` remains the single authority for subsystem status, and
that is where the distinction is preserved.

## Consequences

- `.claude/rules/01-no-hallucinated-implementation.md` gains a `PROTOTYPE`
  row.
- `.claude/hooks/check-markers.sh` accepts `PROTOTYPE`, gains an `--all`
  mode, and **fails** rather than warns in that mode. The advisory posture
  stays right for a commit hook, where a false block trains people to
  bypass it; in CI the file set is stable and a missing marker is a defect.
- Files with no marker at all — several `error.rs`, `main.rs` and helper
  modules — get one. They are genuinely unmarked rather than
  mis-vocabularied, and the hook not having seen them is the same gap.
- CI runs both hooks in `--all` mode.

## Rejected alternatives

**B, keeping the sets distinct.** Defensible in principle: the two
vocabularies do answer different questions, and someone who cares about
that distinction is not wrong. Rejected because the practical cost landed
first — the mismatch silently disabled a check across four crates, and no
reader had noticed the two lists differed. A distinction that produces
undetected defects is not paying for itself, and `docs/STATUS.md` carries
the part of it that matters.

**C, collapsing rule 01 into rule 10.** Rejected because `STUB` and
`UNIMPLEMENTED` have no subsystem meaning and are the two markers that do
the most work: rule 01 requires them to fail loudly when reached, which is
an item-level property a lifecycle vocabulary cannot express. Collapsing
would lose the markers that prevent the specific failure rule 01 exists
for.
