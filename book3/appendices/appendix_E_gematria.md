# APPENDIX E
## Gematria Systems, and the Figures

All values in this book were computed by machine and checked against standard
published figures. The scripts are published with the book.

---

## THE THREE COMMON SYSTEMS

**Mispar hechrachi** (standard). Alef 1 through tet 9; yod 10 through tsadi 90 by
tens; qof 100 through taw 400 by hundreds. This is what "gematria" means by
default.

**Mispar siduri** (ordinal). Each letter takes its position: alef 1 through taw 22.

**Mispar katan** (reduced). Each value collapsed to a single digit by addition: yod
(10) becomes 1, resh (200) becomes 2.

Others exist — systems that square the values, that count the spelled-out names of
the letters, that add the number of letters to the total, that treat final forms as
500–900.

**Each is traditional. Each gives different numbers. Nothing in the tradition
specifies which applies to a given word.** That latitude is the search space of
Chapter 39.

The full letter-by-letter table is Appendix B.

---

## VERIFICATION AGAINST PUBLISHED FIGURES

Ten standard figures, computed and compared. All ten reproduce.

| Word | Hebrew | Computed | Expected | |
|---|---|---|---|---|
| YHWH | יהוה | 26 | 26 | ✓ |
| *echad* (one) | אחד | 13 | 13 | ✓ |
| *ahavah* (love) | אהבה | 13 | 13 | ✓ |
| *chai* (life) | חי | 18 | 18 | ✓ |
| *Elohim* | אלהים | 86 | 86 | ✓ |
| *Adonai* | אדני | 65 | 65 | ✓ |
| *Shaddai* | שדי | 314 | 314 | ✓ |
| *bereshit* | בראשית | 913 | 913 | ✓ |
| *mashiach* | משיח | 358 | 358 | ✓ |
| *nachash* (serpent) | נחש | 358 | 358 | ✓ |

---

## AN ERROR, AND WHY IT IS PRINTED

The first version of the script produced **46** for *Elohim*, not 86.

The value table was keyed on the twenty-two medial letters. *Elohim* ends in final
mem (ם), which was not in the table, so the routine silently skipped it — dropping a
letter worth 40 and returning a confident wrong answer.

The error was caught by the verification table above: nine figures passed, one
failed, and the one that failed was the only word in the set containing a final
form.

**Two things about this are worth more than the embarrassment.**

The error was **invisible in the output**. Forty-six is a plausible-looking number.
Without the check it would have gone into the book, and a reader would have had to
know the correct figure already to catch it.

And the failure mode is **general**. A procedure that silently skips characters it
does not recognize will produce confident wrong results, and this is close to the
standard failure in the literature this book examines. Published gematria claims
depend on decisions about final forms, spelling and system, and most publications do
not state them.

The corrected routine normalizes final forms to their medial equivalents.

---

## THE COLLISION BASE RATE

The computation reported in Chapter 14.

**Method, fixed in advance.** A corpus of common Biblical Hebrew words was assembled
— chosen for frequency, not for their values, and fixed before any arithmetic was
performed. Body parts, family terms, materials, times of day, ordinary furniture of
the language. Standard gematria computed for each.

**Corpus: 132 distinct words. Distinct values: 115.**

**Words sharing a value with at least one other: 34 — 25.8 per cent.**

**Equal-valued pairs available in this corpus alone: 17.**

### The pairs

| Value | Words |
|---|---|
| 14 | *zahav* (gold) = *yad* (hand) |
| 32 | *kavod* (glory) = *lev* (heart) |
| 48 | *chayil* (strength) = *kokhav* (star) |
| 50 | *tame* (impure) = *yam* (sea) |
| 52 | *ben* (son) = *kelev* (dog) |
| 70 | *yayin* (wine) = *sod* (secret) |
| 75 | *kohen* (priest) = *laylah* (night) |
| 90 | *mayim* (water) = *melekh* (king) |
| 120 | *kesil* (fool) = *mo'ed* (appointed time) |
| 126 | *sus* (horse) = *avon* (iniquity) |
| 160 | *kesef* (silver) = *etz* (tree) |
| 302 | *boker* (morning) = *kever* (grave) |
| 306 | *ishah* (woman) = *musar* (discipline) |
| 358 | *mashiach* (messiah) = *nachash* (serpent) |

*(Remaining pairs omitted for space; the full output is in the published script.)*

### What this establishes

A quarter of ordinary Hebrew words collide with another ordinary Hebrew word in a
132-item sample. A full Biblical Hebrew lexicon runs to roughly eight thousand
words, and the number of available equal-valued pairs in the language is very
large.

**Every pair above can be preached**, and several have been. *Glory equals heart.*
*Morning equals grave.* *Water equals king.*

Note *yayin* = *sod* — wine equals secret — which is a genuine rabbinic observation
(*nichnas yayin, yatza sod*, "wine goes in, the secret comes out"). **The corpus
produced it unprompted**, because both words were included for being common and the
arithmetic did the rest.

**So *mashiach* = *nachash* = 358 is arithmetically correct and, alone,
evidentially weightless.** It is one of many thousands of such pairs. Its fame comes
from somebody having noticed it and said something interesting, and the interesting
part was supplied by the interpreter.

**This is not a debunking of rabbinic gematria.** The tradition operated in a
homiletic register, under constraints of canon, community and judgment, and did
not claim to be recovering concealed information. The modern error is the category
error: treating a device for generating reflection as a device for recovering fact.

---

## ENGLISH GEMATRIA

English letters have no numerical values. Any A=1 through Z=26 scheme is a modern
invention with no tradition behind it and no historical usage to constrain it.

The preceding volume of this series tested it. Twenty words central to Masonic
symbolism and twenty control words — household objects and root vegetables — both
lists fixed in advance, the same arithmetic applied to both.

**Masonic set, three internal pairs:**
APRON = PLUMB = 64 · LEVEL = LIGHT = 56 · TYRE = PILLAR = 68
**One significant-number hit:** GAVEL = 47.

**Control set, three internal pairs:**
TESCO = PUDDLE = 62 · KETTLE = SAUSAGE = 73 · POTATO = SPANNER = 87
**One significant-number hit:** PARSNIP = 93.

**Identical yield.** And running the sets against each other — which is what a
cross-traditional correspondence argument does — gives two more: MASON = TESCO = 62,
TUBALCAIN = BISCUIT = 83.

Twenty words make 190 possible pairs, spread over a range of about eighty values.
Two or three collisions per set is what chance predicts, and three is what both
sets produced.

**The procedure is not detecting. It is generating**, and it generates at the same
rate from root vegetables as from sacred symbolism, because the words' meanings
make no difference to the arithmetic.
