# PRE-DRAFTING AUDITS — BOOK III

Run against the dossier before any chapter is written. Per §XLVIII, the full audit
suite runs again on the completed manuscript.

---

## AUDIT 1 — ANACHRONISM (§XXXVIII)

**Test:** does any claim require a later system to be present earlier?

Three chronological facts are load-bearing and each is stated in
`research/chronology.md`:

- The Lost Word appears in the **1720s**; Hebrew name-traditions are **pre-Christian**.
  Sixteen centuries, no carrier. Any derivation claim must bridge this and none does.
- Hermetic Qabalah is **1854 onward**. Nothing before that date can be explained by it.
- The number 33 is **1801**. The 32 paths are ancient. The resemblance is coincidence
  between independently arising numbers.

**Result: pass.** No dossier claim projects a later system backward. The one place
this could go wrong in drafting is the Royal Arch (E4), where the temptation is to
resolve an open question in either direction; the ledger records it as disputed.

## AUDIT 2 — FABRICATION

**Test:** any invented quotation, citation, page, document, ritual, organisation or date?

- Bibliography: 138 entries, each a real work. Verification marks ✔ / ~ / **[SVR]**
  applied per entry. **[SVR]** count across the dossier: **17**.
- **No ritual wording is invented** (§XXXVII). The catechism pattern reproduced in
  `words/masonic_word.md` is described as representative, not authoritative, and is
  drawn from the published exposures.
- **One error caught and logged**, not silently fixed: the dossier originally
  labelled the prospectus "3,180 words" when the section ran to 1,999. Corrected by
  writing the prospectus to length (3,238), not by changing the label.

**Result: pass, with the word-count error recorded.**

## AUDIT 3 — LINGUISTIC (§XLVIII)

**Test:** Hebrew, transliteration, alphabet order, letter names, divine names.

- Alphabet order and the 22 letters verified programmatically in `atbash.py`
  (`assert len(HEB) == 22`).
- Letter names checked against the standard sequence in `gematria.py`.
- Final (sofit) forms handled explicitly after a bug that silently dropped them.
- Divine names kept distinct in `research/hebrew_sources.md` with forms given.
- **Stated limit (gap 11):** this study's Hebrew is machine-checked against
  published figures, not independent. Where a reading turns on philology the book
  names the scholar it follows.

**Result: pass, with the limit declared.**

## AUDIT 4 — KABBALAH (§VIII)

**Test:** are Jewish, Christian, Hermetic and modern occult Kabbalah kept apart?

Four-column table in `research/kabbalah.md`; the eight questions applied to four
named claims returning **E5, E4, E6, E6**. The Lévi tarot attribution — the single
most consequential dating fact — is stated at the head.

**Result: pass.**

## AUDIT 5 — CIPHER (§XLVIII)

**Test:** every cipher result independently recalculated. **Never trust a published
cipher claim.**

Every figure in the dossier is computed by a script in
`research/computational_verification/` and is re-runnable.

- Atbash: both biblical attestations reproduce.
- Baphomet: transformation reproduces exactly.
- Gematria: **10/10** standard figures reproduce after the sofit fix.
- Pigpen: key space and frequency preservation computed.

**Two errors caught and retained rather than deleted:**
1. `atbash_controls.py` v1 — unmatched search spaces; a failed control presented as
   a passed one.
2. `gematria.py` — final forms silently dropped; Elohim computed 46 instead of 86.

**Result: pass. Both errors are printed in the book**, because a script that
silently ignores characters it does not recognise returns confident wrong answers,
and that is a general warning for this genre.

## AUDIT 6 — MATHEMATICAL (§XLVIII, §XLIX)

**Test:** numerical correspondences, gematria, geometric and probability claims
recalculated.

- Base rate for a random Hebrew string hitting a target: analytic and Monte Carlo
  agree (length 4: 0.005123% analytic, 0.005000% over 200,000 trials).
- Gematria collision rate: **25.8%** of a 132-word corpus; 17 equal-valued pairs.
- Search-space enumeration: BAPHOMET 216 admissible spellings.
- 26! computed for the key-space comparison.

**Still to run before drafting Parts VIII and X:** geometric-ratio controls;
pre-registered acrostic and null-cipher search of the *Constitutions*.

**Result: pass for what is claimed; two computations outstanding, listed.**

## AUDIT 7 — CONSPIRACY (§XLVIII)

**Test:** any unsupported claim, or any claim whose structure is conspiratorial?

- No hidden-inner-circle claim is made or relied on.
- Level F sources are segregated in the bibliography and marked as evidence of
  belief only.
- Chapter 43 examines the online decoding literature **without ridicule** (§XXVIII),
  and the distinction it draws — interesting observation versus demonstrated
  encoding — is the book's, applied evenhandedly to its own work in
  `atbash_controls3.py`.

**Result: pass.**

## AUDIT 8 — DISCONFIRMATION (§XLIV)

**Test:** has counter-evidence been sought for the book's own theses?

Three cases where the search was run and changed the result:

1. **The Atbash control.** Expected collapse; got apparent confirmation at v2;
   required a third pass to locate the real flaw. **The intermediate stage is
   printed.**
2. **The Royal Arch.** The book's general thesis has an exception, and it is named
   at E4 rather than argued away.
3. **The friendly-society analogue** (carried from Book II): where a Masonic effect
   is claimed, the comparison class is checked.

**Result: pass.**

---

## SUMMARY

| Audit | Result |
|---|---|
| 1 Anachronism | Pass |
| 2 Fabrication | Pass; word-count error logged |
| 3 Linguistic | Pass; limit declared |
| 4 Kabbalah | Pass |
| 5 Cipher | Pass; two errors logged and retained |
| 6 Mathematical | Pass; two computations outstanding |
| 7 Conspiracy | Pass |
| 8 Disconfirmation | Pass |

**Outstanding before drafting:** the four Tier-1 gaps in
`research/research_gaps.md`, chiefly the document-level check on "letter and halve
it" and the pigpen artefacts.
