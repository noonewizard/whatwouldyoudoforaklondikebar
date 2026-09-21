# Privacy

**Status:** PRODUCTION (process artefact) · 2026-09-21

## The largest exposure, first

**A clearing node sees every event it clears.** It does not see the data,
but it sees, for every pseudonymous subject it handles: which classes of
data are being processed, for what purposes, by whom, how often, when, in
what volume, and what was derived. At scale that is a behavioural profile
of the subject's relationship with every participating controller.

This is stated first because it is the design's central privacy cost, and
because a document that mentions it after four pages of mechanisms is
misleading by arrangement. Everything below either mitigates it or does
not.

Structural answers exist and are not implemented: jurisdictional and
sectoral partitioning so no node sees everything; threshold or
multi-party clearing so no single operator holds the picture; local
aggregation so a node receives counters rather than events. They are in
`DEPLOYMENT.md` as topology options and in `docs/research/`.

## What the protocol does not do

- It does not transport data. No event carries a value; the strongest
  statement about content is a salted hash commitment.
- It does not put anything but digests in the transparency log.
- It does not use a global subject identifier, in any form.
- It does not log pseudonyms, commitments or authorization digests.

## Pseudonyms: what holds and what does not

A subject derives a distinct pseudonym per controller from a root secret
(`specs/protocol-v0.1.md` §9, ADR-0013).

**Holds.** Two non-colluding controllers, seeing only pseudonyms, cannot
tell whether two references belong to the same person. Distinguishing them
requires the root secret. The subject can recompute every pseudonym
offline, which is what makes one dashboard possible without a central
linkage table.

**Does not hold — content linkage.** If two controllers each hold a precise
location trace, they can link the person regardless of the identifier.
Four spatio-temporal points identify 95% of individuals in a
mobility dataset (de Montjoye, Hidalgo, Verleysen and Blondel, *Unique in
the Crowd*, Scientific Reports 3:1376, 2013). The pseudonym removes the
trivial join key; it does not make the data anonymous, and the ontology's
re-identification prior of 96 for `location.trajectory` exists to price
exactly this.

**Does not hold — payout.** To pay one person for usage recorded under many
pseudonyms, some party must learn that those pseudonyms belong together. In
the reference design the settlement agent learns it. A construction that
avoids this is research, not production:
`docs/research/0002-unlinkable-payout.md`.

**Does not hold — a controller who already knows you.** A logged-in service
knows who you are. The pseudonym stops it sharing a join key with others;
it does not hide you from it.

**Does not hold — a compromised root secret.** One secret, every
pseudonym. ADR-0013 records why that trade was taken over a stored table.

## Metadata is the residual risk

An event's own fields are revealing:

| Field | What it leaks |
|---|---|
| data class | the kind of information held about the subject |
| operation, purpose | what is being done and why |
| occurrence time | behaviour patterns, timezone, sleep schedule at volume |
| quantity | intensity of the relationship |
| jurisdiction | approximate location |
| provenance | what was derived, and what it fed |

A single event leaks little. A stream of them about one pseudonym at one
controller is a profile, and the clearing node holds it. This is T-24.

## Differential privacy

The protocol carries an epsilon for `process.dp_release` operations and
can enforce a maximum through a `max_epsilon` obligation. That is all it
does.

It does **not** verify that a release actually satisfies the claimed
epsilon, does not track a privacy budget across releases, and does not
compose epsilons. A controller that declares 0.1 and implements something
else is making a false assertion, which the protocol records and cannot
detect. Budget accounting across releases is the harder half of
differential privacy in practice and is `docs/research/0004-dp-accounting.md`.

`derived.dp_aggregate` carries a re-identification prior of 2 rather than
0, because the bound holds only for the declared epsilon and only if the
accounting is correct.

## Anonymisation

`process.anonymize` records a *claim* that a transformation is
irreversible. The protocol does not adjudicate it. Re-identification risk
depends on auxiliary information nobody enumerates, and a claim that held
when it was made can fail later when an unrelated dataset is published.

The taxonomy treats `derived.synthetic` as tier t1 with a prior of 20, not
as non-personal: generative models memorise, and synthetic records have
been shown to reproduce training examples.

## Operational exposure

Rules that are enforced rather than encouraged:

1. **No pseudonym, commitment or authorization digest in any log line or
   metric label.** The gateway has exactly one logging module and every
   call site is reviewed against this. A pseudonym in an aggregated log is
   a join key handed to whoever operates logging.
2. **No personal data in this repository**, enforced by a commit hook
   (`.claude/hooks/check-no-pii.sh`) and by
   `.claude/rules/07-no-personal-data.md`.
3. **Error messages name the field, not the value.**

Residual: metrics are labelled by pipeline stage and organisation, so a
clearing node's operator can see per-organisation volumes. That is
operational necessity, and it is a disclosure about organisations rather
than about subjects.

## The two trade-offs, stated as trade-offs

### Auditability against privacy

An accounting layer must be auditable, and audit means someone sees the
records. DUAP's position: the *log* is public and carries only digests;
the *events* are private to the controller and the clearing node; the
*receipts* are shared with the subject they concern. An auditor gets
receipts and proofs, not the event corpus, unless a dispute opens one.

The cost is that an auditor cannot independently detect under-reporting
from the log alone — under-reporting leaves no trace there. Detecting it
requires sampling against the controller's own systems, which is an audit
engagement, not a protocol feature.

### Attribution against anonymity

Paying a subject requires knowing which subject. Full anonymity and
individual compensation are incompatible, and any design claiming both is
either not paying individuals or not anonymous.

DUAP chooses pseudonymity with per-controller scoping, which gives
attribution without a cross-controller join key, and accepts that payout
reintroduces linkage at one point. The alternative designs — pay a
collective rather than individuals, or pay through a blind-signature token
scheme — are recorded in `ECONOMIC_MODEL.md` and `docs/research/`.

## Regulatory note

This document is not legal advice and makes no compliance claim. See
`COMPLIANCE.md`, which maps capabilities to specific provisions and marks
every row as technical or interpretation-required.
