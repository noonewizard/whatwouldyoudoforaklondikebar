# COMPUTATIONAL VERIFICATION — BOOK III

Per §XLIX and §L. Every cipher, gematria and probability figure in the manuscript
is produced by a script in this directory and can be re-run. Nothing is asserted
that was not computed.

Format for each: input · transformation · algorithm · output · controls · interpretation.

| Script | Establishes | Status |
|---|---|---|
| `atbash.py` | Atbash table; reproduces both biblical attestations; reproduces the Baphomet transformation | Verified |
| `atbash_controls.py` | v1 control — **confounded**, retained as a record of the error | Superseded |
| `atbash_controls2.py` | v2 matched-shape controls + random-string base rate | Verified |
| `atbash_controls3.py` | v3 target-list contamination — the decisive test | Verified |
| `gematria.py` | Three gematria systems; 10/10 standard figures reproduce | Verified |
| `gematria_baserate.py` | Collision rate in a frequency-chosen Hebrew corpus | Verified |
| `pigpen.py` | Pigpen key space and frequency preservation | Verified |

---

## THE ATBASH SEQUENCE — WHY THERE ARE THREE VERSIONS

This is the book's central methodological demonstration and the versions are kept
deliberately, because the sequence *is* the lesson.

**v1** compared BAPHOMET against control names and found the controls produced no
hits. The comparison was invalid: BAPHOMET's spelling pattern carried three
optional *mater lectionis* positions and the controls did not, so BAPHOMET
searched 216 candidate spellings against 12–72 for the controls. A larger search
space finds more by chance. **This was a failed control presented as a passed one**,
and it is exactly the error Part IX exists to teach.

**v2** matched the skeletons. Result: controls 0 hits in 777 spellings; BAPHOMET
1 hit in 216. Random-string base rate for a 5-letter string landing on a target:
about 1 in 10⁶ (analytic 0.000116%, confirmed by 200,000 trials). On this showing
the Baphomet result looks genuinely unusual, and an honest analyst would have to
say so.

**v3** asked where the target list came from. I wrote it. I wrote it knowing the
answer, and *Sophia* — a Greek word, defectively transliterated — is on it only
because Schonfield's claim put it there. Remove that one entry and BAPHOMET
returns **0 hits, identical to every control.**

**The finding:** the transformation is real and exactly reproducible — בפומת maps
to שופיא under Atbash, and the arithmetic is in `atbash.py`. What carries no
evidential weight is the *interpretation*, because the criterion for "a meaningful
output" was fixed after the output was known. A test whose success condition is
written by someone who already has the answer is not a test.

**What would carry weight:** specify the target lexicon in advance from a source
independent of the claim — a standard Biblical Hebrew dictionary — then run the
transformation. Sophia is not in such a lexicon.

---

## ERRORS CAUGHT AND CORRECTED

Logged here rather than silently fixed, per §XLVIII.

1. **`atbash_controls.py` (v1): unmatched search spaces.** Described above.
   Superseded by v2/v3; the file is retained.
2. **`gematria.py`: final forms silently dropped.** The value table was keyed on
   medial letters only, so final *mem* in אלהים contributed nothing and Elohim
   computed as 46 instead of 86 — a wrong figure that looked like a result.
   Fixed by normalising sofit forms to their medial equivalents. After the fix,
   10/10 standard figures reproduce. **A cipher script that silently ignores
   characters it does not recognise will produce confident wrong answers, which
   is a general warning for this whole genre.**

---

## KEY FIGURES FOR THE MANUSCRIPT

- Atbash reproduces **Sheshach → Babel** (Jer 25:26, 51:41) and **Leb Qamai →
  Kasdim** (Jer 51:1). These are the only undisputed attestations, both biblical,
  both Hebrew-to-Hebrew, both proper nouns.
- **בפומת → שופיא** under Atbash. Real; see v3 for what it does and does not show.
- Standard gematria: YHWH 26 · echad 13 · ahavah 13 · chai 18 · Elohim 86 ·
  Adonai 65 · Shaddai 314 · bereshit 913 · mashiach 358 · nachash 358.
- **25.8%** of words in a 132-word frequency-chosen Hebrew corpus share a gematria
  value with another word in the same corpus; that corpus alone yields **17**
  equal-valued pairs, among them יין = סוד = 70 (wine = secret), which the corpus
  produced without being asked and which the tradition itself noticed.
- Pigpen key space: **1**. Not keyed, monoalphabetic, frequency-preserving,
  word-division-preserving.
