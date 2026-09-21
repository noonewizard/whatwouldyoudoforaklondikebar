# DELIVERABLE 38 — MASTER PRODUCTION CHECKLIST

From manuscript to press, per figure. Phases 1–2 happen at **research** stage,
not production stage — this is the commonest scheduling failure in illustrated
scholarly books.

---

## PHASE 1 — SPECIFICATION *(with the manuscript)*

- [ ] Figure identified by the argument that needs it, not by page attractiveness
- [ ] Priority assigned (A/B/C/D) — **prefer A and B; C selectively; D almost never**
- [ ] `figure_class`, `figure_type`, `evidence_status`, `interpretation_status` set
- [ ] `purpose` and `does_not_show` written **before** any drawing
- [ ] Register entry created; `status: proposed`
- [ ] Quarto label assigned and used in the manuscript
- [ ] Sources identified, or marked **RESEARCH REQUIRED**

## PHASE 2 — RIGHTS *(begins immediately; 3–6 month lead)*

- [ ] Holding institution identified
- [ ] Reproduction request sent and logged in `rights/requests/`
- [ ] Fee agreed; budget recorded
- [ ] Permission received, filed verbatim in `rights/correspondence/`
- [ ] Attribution wording captured exactly as the institution requires
- [ ] `rights.status` updated; expiry recorded if time-limited

## PHASE 3 — RESEARCH

- [ ] Every source consulted and `consulted:` recorded honestly
- [ ] Competing reconstructions identified — **all of them, not the convenient one**
- [ ] Measurements taken from the source, not from another reconstruction
- [ ] Uncertainty catalogued: what is inferred, what is conjectural, what is absent
- [ ] `provenance.yml` drafted
- [ ] Anything unverifiable marked `NOT_VERIFIED` — never guessed

## PHASE 4 — PRODUCTION

- [ ] Correct tool chosen (script over hand-drawing wherever possible)
- [ ] Series template used; ad-hoc styling avoided
- [ ] Evidence encoding applied via layer or component property, not per object
- [ ] Working files committed
- [ ] Computational figures: seed fixed, environment recorded, script committed
- [ ] Caption drafted to the template, answering all four questions
- [ ] Alt text and long description written

## PHASE 5 — QA

- [ ] Gate 1 Content
- [ ] Gate 2 Evidence *(blocking)*
- [ ] Gate 3 Design
- [ ] Gate 4 Technical
- [ ] Gate 5 Accessibility
- [ ] Gate 6 **Visual Argument Audit** *(blocking)*
- [ ] `status: approved`; `qa_status: passed`

## PHASE 6 — INTEGRATION

- [ ] Exports produced for print, web, EPUB
- [ ] Register `files` updated; CI green
- [ ] Figure renders in all three formats
- [ ] Cross-references resolve
- [ ] Placement reviewed on the actual spread
- [ ] Target/control pairs confirmed to face each other

## PHASE 7 — PRINT

- [ ] Proofed at final size, grayscale, 100%
- [ ] Reduction test passed
- [ ] Included in the figure manifest
- [ ] Rights clearance in the bundle
- [ ] Printer's proof checked against the file

---

## THE FINAL QUESTION

Before any figure goes to press:

> **Does this image make the evidence easier to see, or does it make the evidence
> look stronger than it is?**

If the honest answer is the second, the figure does not go in the book — however
good it looks, however much work it took, and however much the page wants it.
