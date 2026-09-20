# COMPUTATIONAL VERIFICATION — BOOK IV

Per the master prompt's Computational Verification and Sacred Geometry/Numerology
standards. Every geometric, numerical and cryptographic claim in the manuscript is
produced by a script here and is re-runnable. Nothing is asserted that was not
computed.

| Script | Establishes | Status |
|---|---|---|
| `temple_dimensions.py` | Raw MT ratios (1 Kings 6–7); the Holy-of-Holies cube; the Molten Sea "π = 3" crux and its rim/bore resolution | Verified |
| `golden_ratio_test.py` | Golden-ratio claim tested against ALL pairwise ratios of stated Temple dimensions, with a matched control building | Verified — **control reproduces the same hit rate** |
| `temple_gematria.py` | Standard gematria and Atbash on 18 Temple/Solomon/Hiram terms against a fixed 62-word independent lexicon | Verified — **0/18 false hits, as the base rate predicts**; **one input-sanitisation bug caught and fixed on first run** |
| `dc_geometry_test.py` | Hexagram-fitting control on a plain jittered street lattice, for the "Washington D.C. Masonic/Solomonic geometry" genre of claim | Verified — **the control finds a hexagram to a tighter tolerance than the online literature typically reports** |

---

## WHAT THE FOUR RUNS ESTABLISH TOGETHER

**1. The Temple's stated dimensions are simple small-integer ratios** (3:1, 2:1,
2:3) that require no numerology to explain — they are the kind of ratio a
Bronze/Iron Age builder reaches for without any symbolic intention, and the text
gives no indication that more is meant.

**2. The "hidden golden ratio" claim fails its own control.** Run identically
against a structurally ordinary building with no golden-ratio mythology attached,
the control set produces a hit at the same rate as the Temple set. This is not
evidence that no ancient building was ever proportioned to φ; it is evidence that
*this method* — searching all pairwise ratios of a small dimension list with an
unstated tolerance — cannot distinguish a designed building from an arbitrary one.

**3. Gematria and Atbash on Temple vocabulary return nothing, as the base rate
predicts.** Eighteen Temple/Solomon/Hiram words tested against Book III's
independent 62-word lexicon return zero hits — which is the expected null result,
not a finding, and matters here because it sets the baseline against which any
*positive* claim in this literature must be judged (per Book III's three-stage
Atbash discipline, reused rather than re-derived).

**4. The Washington D.C. "sacred geometry" genre is reproduced by an inert control
at a *tighter* tolerance than most published claims achieve.** A plain jittered
street grid — containing nothing by construction — yields a Seal-of-Solomon
hexagram at 8.8% of its own circumradius. That is the pentagram-in-a-lattice
demonstration from Books II and III, extended to the hexagram for this book's
Solomonic material.

---

## THE BUG, LOGGED RATHER THAN QUIETLY FIXED

`temple_gematria.py`'s first draft crashed on `קדש הקדשים` ("Holy of Holies,"
written as two words with a space) because `tr()` and `atbash()` lacked the
input-filtering guard that `g()` already had. This is the same failure class as
Book III's dropped-final-form bug: **a routine that does not defensively filter its
own input will fail loudly on some inputs and silently on others**, and the fact
that this one failed loudly (a crash) rather than silently (a wrong number) is luck,
not design. Both functions were patched to filter on `if c in TABLE`, matching `g()`,
and the run was repeated in full. The uncorrected traceback is preserved in this
session's record rather than deleted from it.

---

## STILL TO RUN BEFORE DRAFTING

- Molten Sea rim/bore resolution checked against the Talmudic discussion (Eruvin
  14a) at primary-source level — currently summarized from secondary literature.
  **[SVR]**
- A systematic (not illustrative) run of the golden-ratio test against 5–10
  additional control buildings, to report a distribution rather than one control.
- Astronomical-alignment claims for the Temple Mount's orientation, if any
  are made in the secondary literature worth testing — not yet scoped.
