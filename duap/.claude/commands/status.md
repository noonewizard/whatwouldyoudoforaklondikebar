---
description: Propose a subsystem status transition with its evidence
argument-hint: <subsystem> <target-status>
---

Propose moving **$1** to **$2**.

1. Read the entry criteria for $2 in `.claude/rules/10-status-discipline.md`.
2. For each criterion, cite the evidence: a test name, a benchmark result
   file, a review document, an audit reference. "It looks done" is not
   evidence.
3. Check `security/findings.md` for open findings against $1. PRODUCTION is
   blocked by any that are unresolved and unaccepted.
4. If every criterion is met, update `docs/STATUS.md` **in a commit that
   changes nothing else**, with the evidence in the commit message.
5. If a criterion is not met, say which, and stop.
