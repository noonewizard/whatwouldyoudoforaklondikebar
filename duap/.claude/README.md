# Engineering configuration

This directory configures Claude Code (ECC) for work on the Data Usage
Accounting Protocol. It is part of the repository because the way this
project is built is itself a design decision: a protocol that aspires to be
independently implementable must be developed under rules that stop
implementation detail leaking into the specification, and stop claims
outrunning evidence.

If DUAP is extracted into its own repository -- which `GOVERNANCE.md`
recommends once the protocol reaches SPECIFIED status -- `duap/` becomes the
root and this directory is already in the right place.

## What was adopted, and why

| Component | Adopted | Reasoning |
|---|---|---|
| `agents/` | Yes | The work spans cryptography, distributed systems, economics, privacy law mapping and adversarial testing. These demand genuinely different judgement. Splitting them into scoped agents with **forbidden responsibilities** is what stops a cryptographer quietly redesigning the accounting model, or an economist relaxing a security boundary to make a pricing model work. |
| `rules/` | Yes | Every rule here exists because the failure it prevents is the characteristic failure of this *kind* of project: fake benchmarks, unearned cryptographic claims, invented compliance, silent architecture drift. |
| `hooks/` | Yes, three | Hooks run deterministically; a rule that can be enforced mechanically should not rely on an agent remembering it. We enforce: no personal data in fixtures, generated files stay in sync with their source, and the workspace still compiles before a commit is proposed. |
| `commands/` | Yes, six | The recurring workflows -- vertical-slice run, adversarial review, benchmark, status transition, conformance regeneration, ADR creation -- are long enough to be got wrong from memory. |
| `skills/` | Yes, three | Protocol change, threat modelling and benchmark reporting each have a house procedure that is easy to short-cut. |
| Output styles | No | The documentation style is specified in `rules/documentation.md` and enforced in review; a style file would duplicate it. |
| MCP servers | No | The project has no external service dependencies by design. Adding one would contradict `docs/adr/0001-scope.md`. |
| Subagent auto-delegation | Deliberately constrained | `rules/architecture-authority.md` requires the chief architect to approve cross-subsystem changes. Unconstrained delegation is exactly how incompatible architectures appear in parallel. |

## The authority model

```
                      chief-architect
                    (ADRs, interfaces, invariants)
                             |
        +--------------------+---------------------+
        |                    |                     |
   protocol layer      economic layer        assurance layer
   (crypto, identity,  (valuation,           (security, formal-methods,
    authorization,      accounting,           red-team, benchmark,
    provenance,         clearing,             compliance)
    metering)           settlement)
        |                    |                     |
        +--------------------+---------------------+
                             |
                    delivery (sdk, cloud, database)
```

Two rules make the hierarchy real rather than decorative:

1. **No agent may change an interface another agent owns.** It proposes; the
   owner decides; a disagreement goes to the chief architect as an ADR.
2. **The red team has standing authority to challenge any claim**, including
   the chief architect's, and its findings are tracked in
   `security/findings.md` until closed or explicitly accepted.
