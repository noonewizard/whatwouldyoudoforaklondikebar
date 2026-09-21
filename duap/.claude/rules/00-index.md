# Project rules

These rules bind every agent and every contributor. They exist because each
one prevents a failure that is characteristic of this kind of project --
infrastructure that makes strong claims about cryptography, privacy,
economics and regulation, where an unearned claim is worse than a missing
feature.

A rule is not advice. If a change cannot satisfy a rule, the change does not
land; if the rule is wrong, it is changed by an ADR, not by exception.

| Rule | Prevents |
|---|---|
| `01-no-hallucinated-implementation.md` | Code that looks finished and is not |
| `02-no-fake-benchmarks.md` | Performance claims without measurement |
| `03-no-unearned-crypto-claims.md` | Security claims without an argument |
| `04-no-fake-compliance.md` | Regulatory claims without a provision |
| `05-no-silent-architecture-change.md` | Architecture drift without a record |
| `06-incremental-validation.md` | Large untested commits |
| `07-no-personal-data.md` | Real personal data in the repository |
| `08-documentation.md` | Marketing prose in a technical record |
| `09-architecture-authority.md` | Parallel incompatible architectures |
| `10-status-discipline.md` | Subsystems maturing by assertion |
