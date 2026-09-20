# FINAL QUALITY CONTROL — BOOK III

Per §XLVIII, run against the completed manuscript. Every figure is the output of a
command run against the files. Commands are given so the audits can be repeated.

**Manuscript at audit: 90,983 words of chapter text; 104,865 assembled including
appendices, bibliography, illustration list and index framework.** Within the
brief's 90,000–110,000.

---

## AUDIT 1 — HISTORICAL

**Every date, person, organisation, ritual and relationship checked against the
chronology in Appendix A.**

The three load-bearing chronological facts, each stated in the text where it
matters:

- **The Lost Word is not attested before the 1720s.** The catechisms of 1696–1714
  have a word and no loss. Sixteen centuries separate the Masonic material from the
  Hebrew name-traditions proposed as its source, with no documented carrier.
- **Hermetic Qabalah is 1854 onward.** Nothing before that date is explicable by it.
- **Thirty-three is 1801**; the 32 paths are ancient; the numbers arose
  independently.

No claim in the manuscript requires a later system to be present earlier.

**Pass.**

## AUDIT 2 — LINGUISTIC

**Hebrew, Greek, Latin, transliteration, alphabet order, letter names, divine names,
grammatical claims.**

- Alphabet order and the count of twenty-two verified programmatically
  (`assert len(HEB) == 22` in `atbash.py`).
- Letter names checked against the standard sequence in `gematria.py`.
- Final (sofit) forms handled explicitly after the bug logged in Audit 5.
- Divine names kept distinct in Appendix G with forms, senses and values given.
- *Logos* separated into four concepts (Chapter 7); *davar* and *Memra* treated with
  the Barr caution against arguing from lexical range to national psychology.

**Declared limit, stated in the front matter and in Chapter 11's notes:** this
study's Hebrew is machine-checked against published figures, not independent. Where
a reading turns on philology the text names the scholar it follows.

**Pass, with the limit declared.**

## AUDIT 3 — KABBALAH

**Jewish, Christian, Hermetic and modern occult Kabbalah kept apart.**

Four-column table in Chapter 35. The eight questions of the protocol applied to four
named claims, returning **E5, E4, E6, E6** (Appendix H rows 22–25).

The single most consequential dating fact — that the tarot–Hebrew letter
correspondence is Lévi's, from the 1850s — is stated in Chapters 32 and 35 and in
the timeline.

**The Royal Arch is reported at E4 rather than resolved.** An argument with no
exceptions would be a suspicious argument, and this is the exception.

**Pass.**

## AUDIT 4 — CIPHER

**Every cipher result independently recalculated. No published cipher claim trusted
without checking.**

```
python3 research/computational_verification/atbash.py
python3 research/computational_verification/atbash_controls2.py
python3 research/computational_verification/atbash_controls3.py
python3 research/computational_verification/pigpen.py
```

- Atbash table built and verified; **both biblical attestations reproduce**
  (Sheshach → Babel; Leb Qamai → Kasdim).
- **The Baphomet transformation reproduces exactly**: בפומת → שופיא.
- Pigpen key space (1) and frequency preservation computed.

**Pass.**

## AUDIT 5 — MATHEMATICAL

**Numerical correspondences, gematria, geometric claims and probability claims
recalculated.**

- Gematria: **10/10 standard figures reproduce.**
- Collision base rate: **25.8%** of a 132-word frequency-chosen Hebrew corpus;
  17 equal-valued pairs.
- Atbash base rate: analytic **0.000116%** for a five-letter string, confirmed by
  200,000 seeded Monte Carlo trials.
- Search space: BAPHOMET **216** admissible spellings; controls 777 across nine
  matched names.
- Pentagram control: max vertex deviation **0.20 units = 5.7%** of circumradius on a
  96-point jittered lattice, 400,000 placements.
- 26! computed for the key-space comparison.

**Two errors caught and retained rather than deleted:**

1. **`atbash_controls.py` v1 — unmatched search spaces.** BAPHOMET searched 216
   spellings against 4–72 for the controls. A failed control presented as a passed
   one. Superseded by v2/v3; the file is kept and the failure is printed in
   Chapter 16 in the order it occurred.
2. **`gematria.py` — final forms silently dropped.** *Elohim* computed as 46 instead
   of 86. Fixed by normalising sofit forms; described in Chapter 14 and Appendix E.
   **A routine that silently skips what it does not recognise returns confident
   wrong answers**, which is a general warning for this genre.

**Pass. Both errors are printed in the book.**

## AUDIT 6 — SOURCE

**Every citation verified or marked.**

Bibliography: **138 numbered entries**, each a real work, with ✔ / ~ / **[SVR]**
marks per entry. Brief requires 125+.

**30 verification markers** across the manuscript:

```
grep -ro "SOURCE VERIFICATION REQUIRED\|PAGE VERIFICATION REQUIRED" chapters/ appendices/ | wc -l
→ 30
```

The two that matter most, both declared in Chapter 26 and the research gaps:
**specific pigpen artefacts** with dates and holdings, and the **earliest printed
occurrence of "letter and halve it"** with edition and page.

**Level F sources are segregated and used only to document belief**, never to
establish fact.

**Pass, with 30 markers outstanding.**

## AUDIT 7 — ANACHRONISM

**No later system projected backward.**

Checked against the three columns of Appendix A. The manuscript's standing rule —
**availability is not transmission** — is applied explicitly to the one genuine
opportunity (Knorr, 1677) in Chapters 19, 34 and 35.

**Pass.**

## AUDIT 8 — CONSPIRACY

**No unsupported claim; no claim of conspiratorial structure.**

```
grep -rniE "scholars (generally )?agree|it is (widely|generally) (accepted|agreed)" chapters/ appendices/
→ 0
```

Chapter 43 examines the modern decoding literature **without ridicule**, and applies
the same tests to this book's own work in Chapter 16. Where the book disagrees, it
says which test fails and why.

**Pass.**

## AUDIT 9 — FRATERNAL OBLIGATION (§XXXVII)

**No private ritual reproduced; none invented; jurisdictional variation stated.**

```
grep -rniE "I do (hereby and )?(solemnly )?swear|penalty of having my" chapters/ appendices/
→ 0
```

**No verbatim obligation or penalty text anywhere in the manuscript.** Ritual
material is handled at the level of structure and function, drawn from the
published exposures in print since 1730, and described rather than performed. The
one extended quotation of catechism form (Chapter 5) is given as "a representative
pattern rather than any single authoritative text."

**Jurisdictional variation stated in 19 files**, with a dedicated treatment in
Chapter 19 on what the variation implies for every claim in the book.

**Pass.**

## AUDIT 10 — NARRATIVE

**Does the book remain compelling?**

The descent-and-return structure holds: twelve parts, Part IX as the floor, Part X
beginning the ascent. Recurring imagery (door, word, key, stone, light, darkness,
voice, silence, name, memory, architecture) is distributed rather than clustered.

**The prologue's candidate at the door returns in Chapter 52 and in the epilogue**,
and the questions §XL specifies are asked in sequence across the parts.

**Pass.**

---

## SUMMARY

| Audit | Result |
|---|---|
| 1 Historical | Pass |
| 2 Linguistic | Pass; limit declared |
| 3 Kabbalah | Pass; Royal Arch left at E4 |
| 4 Cipher | Pass; all results recomputed |
| 5 Mathematical | Pass; two errors logged and retained |
| 6 Source | Pass; 30 markers outstanding |
| 7 Anachronism | Pass |
| 8 Conspiracy | Pass |
| 9 Fraternal obligation | Pass |
| 10 Narrative | Pass |

**Outstanding before publication:**

- **The pigpen artefacts.** That Masons used the cipher is not in doubt; the book
  must cite objects rather than assertions.
- **"Letter and halve it" at document level.** The reading offered is E3 pending
  that check.
- **An independent machine-readable Hebrew lexicon** for the Atbash control. The
  book asks this of others and has not fully done it; the gap is printed.
- **Systematic sampling of the online decoding corpus** with archived URLs and
  capture dates for Chapter 43.
- **Rights clearance** for the illustration programme.
