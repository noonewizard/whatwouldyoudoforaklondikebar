---
description: Create an architecture decision record
argument-hint: <short-title>
---

Create the next ADR in `docs/adr/` for **$1**, using
`docs/adr/0000-template.md`.

Fill every section. In particular:

- **Alternatives** lists at least two that were genuinely considered.
- **Rejected alternatives** explains why each was rejected, in terms someone
  who preferred it would recognise as a fair statement of their position.
- **Trade-offs** names what the decision costs, not only what it buys.
- **Consequences** names what must now change elsewhere.

Then update `docs/adr/README.md`'s index and, if the decision changes a
subsystem's maturity, follow `.claude/rules/10-status-discipline.md`.
