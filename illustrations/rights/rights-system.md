# DELIVERABLE 15 — RIGHTS & PROVENANCE SYSTEM

## Status vocabulary

| Status | Meaning | May go to print? |
|---|---|---|
| `public-domain` | Verified PD in the publication territory | Yes |
| `own-work` | Created for this series | Yes |
| `licensed` | License obtained, terms recorded | Yes, within terms |
| `permission-granted` | Written permission on file in `rights/correspondence/` | Yes |
| `permission-pending` | Requested, not received | **No** — CI warns, print QA blocks |
| `fair-use-claimed` | Asserted, with a written rationale on file | Publisher counsel decides |

## Public domain is a claim, not a default

A "public domain" status requires a recorded basis, not an assumption:

```yaml
rights:
  status: public-domain
  basis: "Published 1604; author d. 1608; PD in US and EU (life+70)."
  verified_by: ""
  verified_date: ""
```

**Two traps this series will hit repeatedly:**

1. **A photograph of a public-domain object may itself be in copyright** in some
   jurisdictions. The object's date does not settle the image's status. Where a
   museum asserts rights in a reproduction, that assertion is recorded even if
   the publisher's counsel ultimately disputes it.

2. **Institutional terms of use are not copyright**, but they are contractual and
   they bind. Both are recorded separately: `license` (copyright) and
   `institution_terms` (contract).

## Archival request workflow

```
rights/
├── requests/         one file per institution, with the request as sent
├── correspondence/   replies, verbatim, dated
├── licenses/         license texts and invoices
└── ledger.csv        institution · object · requested · status · expires · cost
```

Requests are sent **at research stage, not production stage.** Institutional
turnaround of three to six months is normal, and the illustration program cannot
absorb that at the end.

## Attribution

Every archival figure carries its credit in the evidence gutter, in the form the
institution requires. Where an institution specifies wording, that wording is
used **verbatim** and stored in `rights/licenses/`.

## Expiry

Time-limited licenses carry `expires`. CI warns at 90 days and fails the print
build after expiry. A second or third printing is a new rights event.
