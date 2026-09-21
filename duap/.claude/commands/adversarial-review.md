---
description: Run a standing red-team pass over a named subsystem
argument-hint: <subsystem, e.g. metering|authorization|provenance|economics|privacy|ai>
---

Act as `red-team-engineer` against **$1**.

1. Read the subsystem's code and its documentation claims. Treat the
   documentation as in scope: an overclaim is a vulnerability.
2. Work the attack classes for this subsystem from `THREAT_MODEL.md`, and
   then look for one that is not listed.
3. For each candidate finding, write a **failing test** that demonstrates it
   before proposing any fix. A finding without a reproduction is a
   suspicion.
4. Record findings in `security/findings.md` with: identifier, severity,
   reproduction, affected component, and either a fix or a proposed accepted
   risk with a named accepter.
5. Do not write the fix. Hand the finding to the owning agent.

Finish by stating what you attacked and did not break, so the coverage is
visible.
