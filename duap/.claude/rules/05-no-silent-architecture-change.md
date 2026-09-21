# No silent architecture change

An architecture decision record is required for any change that:

- alters a wire format, a domain label, or a canonical encoding rule;
- adds, removes or reverses a dependency between crates;
- changes a protocol invariant, or the algorithm that enforces one;
- changes a security boundary, a trust assumption, or a threat's mitigation;
- changes the economic semantics of a protocol object;
- adds an external dependency to a crate on the trust path;
- changes what a subsystem's maturity marker claims.

## The ADR

`docs/adr/NNNN-short-title.md`, following the template in
`docs/adr/0000-template.md`, with all seven sections filled in: Context,
Problem, Alternatives, Decision, Trade-offs, Consequences, Rejected
alternatives.

An ADR whose "Rejected alternatives" section is empty has not been thought
through: something was rejected, or there was no decision to record.

## Superseding

ADRs are never edited after acceptance except to add a supersession note.
A changed decision is a new ADR that names the one it replaces.
