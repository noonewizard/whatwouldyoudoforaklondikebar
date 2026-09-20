# POST-DRAFTING AUDITS — BOOK II

Run against the completed manuscript (50 chapters + prologue + epilogue, 8 appendices).
Every figure below is the output of a command run against the files, not an estimate.
Commands are given so the audits can be repeated.

**Manuscript at audit:** 92,770 words of chapter text; 108,004 words assembled including
appendices, bibliography, illustration list and index framework. Within the brief's
90,000–110,000.

---

## AUDIT 1 — FAKE SCHOLARSHIP

**Rule (§XXXI):** never write "scholars generally agree" or equivalent without identifying
the scholarship.

```
grep -rniE "scholars (generally )?agree|it is (widely|generally) (accepted|agreed)|most scholars|historians agree" chapters/ appendices/
```

**Result: 0 occurrences.** Where the manuscript reports a scholarly position it names the
scholar (Stevenson, Bell, Whitehouse, Hanegraaff, Prescott, Snoek, Strube, McKay et al.).
Where specialists disagree the chapter says who takes which side.

**Pass.**

---

## AUDIT 2 — FABRICATION

**Rule:** never fabricate quotations, citations, page numbers, archival documents, rituals,
organisations, dates or manuscripts.

Two findings during drafting, both self-caught and both corrected:

1. **Bibliography placeholders.** Two entries carried invented-looking author names. Replaced
   with real works (Meyer, *The Ancient Mysteries*, 1987; Clauss, *The Roman Cult of
   Mithras*, 2000). Logged at the time.
2. **A fabricated building.** A draft passage in Chapter 38 claimed the author had fitted a
   pentagram to the car park of a named supermarket "built in 1987 by a contractor working to
   a standard layout." No such exercise had been performed and the building was invented.
   **The passage was removed and replaced with a computation that was actually run**
   (§Audit 6). This was a genuine violation caught before assembly, and it is recorded here
   rather than quietly fixed.

**Unverified material is marked, not asserted.** 75 `[SOURCE VERIFICATION REQUIRED]` /
`[PAGE VERIFICATION REQUIRED]` markers across 41 files.

```
grep -ro "SOURCE VERIFICATION REQUIRED\|PAGE VERIFICATION REQUIRED" chapters/ appendices/ | wc -l
```

**Pass, with the Chapter 38 violation recorded.**

---

## AUDIT 3 — FRATERNAL OBLIGATION

**Rule (§XXXII):** do not reproduce or invent private ritual material; state jurisdictional
variation; never present one working as universal.

**No verbatim obligation or penalty text appears anywhere in the manuscript.**

```
grep -rniE "I do (hereby and )?(solemnly )?(and sincerely )?swear|penalty of having my" chapters/ appendices/
→ 0
```

Ritual material is handled at the level of structure and function, drawn from the published
eighteenth-century exposures which have been in print since 1730, and described rather than
performed. The one extended quotation of catechism form (Chapter 9) is given explicitly as
"a representative pattern rather than any single authoritative text."

**Jurisdictional variation is stated in 29 of the manuscript's files**, including a standing
caution at the head of Appendix A and a paragraph in Appendix B §B.7.

```
grep -rli "jurisdiction" chapters/ appendices/ | wc -l → 29
```

**Pass.**

---

## AUDIT 4 — THE METAPHYSICAL QUESTION

**Rule (§VI):** do not assume supernatural magic exists; do not assume it does not.

Type E claims are reported, never asserted or denied. Chapter 39 declines the question
formally and explains the decline as a methodological position. Appendix E §E.5 row 19
carries **no evidence tag by design** and says so. Eight files carry explicit hedging
language of this kind.

Chapter 49 resolves the "is it magic?" question as a question about vocabulary rather than
about the world, and states that both answers are facts about the word.

**Pass.**

---

## AUDIT 5 — PARALLEL IS NOT DESCENT

**Rule:** structural resemblance is not evidence of derivation absent documented transmission.

The rule is invoked or applied in 23 chapters. Chapter 48 states the textual-criticism form
of the test (shared *arbitrary* features indicate descent; shared *entailed* features
indicate convergence) and applies it to the five transformations. Chapter 8's
"half-of-everything rule" generalises it to low-cardinality features. Chapter 45 runs a
control on the book's own strongest positive finding (the friendly societies) and weakens the
inference accordingly.

**Pass.**

---

## AUDIT 6 — COMPUTATIONS ACTUALLY RUN

**Rule:** a book that teaches control tests must run its own.

Two computations appear in the manuscript as the author's own work. Both were executed and
their scripts are archived under `computations/` so the figures can be checked.

**Gematria control test (Chapter 38, Appendix F).** `computations/gem.py`. Simple English
gematria on two fixed twenty-word sets. Output transcribed unaltered: three internal pairs
and one significant-number hit on each side. Masonic — LEVEL=LIGHT=56, APRON=PLUMB=64,
TYRE=PILLAR=68, GAVEL=47. Control — TESCO=PUDDLE=62, KETTLE=SAUSAGE=73, POTATO=SPANNER=87,
PARSNIP=93. Appendix F also reports the two cross-set matches (MASON=TESCO=62,
TUBALCAIN=BISCUIT=83) and the arithmetic of why the yield is unremarkable.

**Pentagram fit (Chapter 38).** `computations/penta.py`. A 12 × 8 lattice of 96 points with
±0.03 jitter; 400,000 random placements of a regular pentagram; best result refined by local
optimisation. **Maximum vertex deviation 0.20 grid units, 5.7 per cent of circumradius.**
Procedure and tolerance fixed before the search. Recorded in note 4a.

**Pass.**

---

## AUDIT 7 — CONTESTED FINDINGS CARRY THEIR CONTESTATION

**Rule:** two findings heavily cited in the popular Masonic literature are weaker than their
reputation and must never be leaned on.

**Aronson & Mills (1959), severity of initiation.** Appears in Chapters 10, 32, 44. Every
occurrence is accompanied by a statement of the mixed replication record. Chapter 44: *"A
book that spends a chapter teaching readers to distrust a striking result cannot then lean on
one because it is convenient."*

**Meyers-Levy & Zhu (2007), ceiling height.** Appears in Chapter 46 and the Epilogue. Both
occurrences identify it as a priming finding of the kind the replication crisis has treated
most harshly. Chapter 46 concludes against the architectural explanation on independent
grounds.

Neither finding supports any load-bearing claim in Appendix E §E.5.

**Pass.**

---

## SUMMARY

| Audit | Result |
|---|---|
| 1 — Fake scholarship | Pass (0 occurrences) |
| 2 — Fabrication | Pass; one violation caught, corrected and recorded |
| 3 — Fraternal obligation | Pass |
| 4 — Metaphysical question | Pass |
| 5 — Parallel is not descent | Pass |
| 6 — Computations actually run | Pass; scripts archived |
| 7 — Contested findings | Pass |

**Outstanding before publication**, carried from the research files and not resolved by
drafting:

- The 75 verification markers, chiefly page numbers and the document-level evidence for
  lodge attrition (Chapter 44 note 1), which is the book's central disconfirming claim and
  currently rests on published summaries rather than archival work.
- Prescott & Sommers on the 1717/1721 dating: the *AQC* printing was not accessible to this
  study and the citation carries a marker.
- Rights clearance for the illustration programme (Appendix / illustration list).
