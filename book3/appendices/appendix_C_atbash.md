# APPENDIX C
## The Atbash Table

Atbash pairs the first letter of the alphabet with the last, the second with the
second-last, and so on. The name is itself an instance: **A-T-B-SH** — alef, taw,
bet, shin.

It is its own inverse: applying it twice returns the original. There is no key.

---

## THE ELEVEN PAIRS

| | Letter | | Letter | |
|---|---|---|---|---|
| 1 | **א** alef | ↔ | **ת** taw | 22 |
| 2 | **ב** bet | ↔ | **ש** shin | 21 |
| 3 | **ג** gimel | ↔ | **ר** resh | 20 |
| 4 | **ד** dalet | ↔ | **ק** qof | 19 |
| 5 | **ה** he | ↔ | **צ** tsadi | 18 |
| 6 | **ו** waw | ↔ | **פ** pe | 17 |
| 7 | **ז** zayin | ↔ | **ע** ayin | 16 |
| 8 | **ח** chet | ↔ | **ס** samekh | 15 |
| 9 | **ט** tet | ↔ | **נ** nun | 14 |
| 10 | **י** yod | ↔ | **מ** mem | 13 |
| 11 | **כ** kaf | ↔ | **ל** lamed | 12 |

Final (sofit) forms are normalized to their medial equivalents before
transformation: ך→כ, ם→מ, ן→נ, ף→פ, ץ→צ.

---

## THE TWO BIBLICAL ATTESTATIONS

These are the only undisputed occurrences of Atbash in the Hebrew Bible. Both were
computed for this book and both reproduce exactly.

**Jeremiah 25:26 and 51:41**

| | Hebrew | Transliteration |
|---|---|---|
| Written | ששך | sh-sh-k |
| Atbash | בבל | b-b-l |

**Sheshach → Babel** (Babylon).

**Jeremiah 51:1**

| | Hebrew | Transliteration |
|---|---|---|
| Written | לב קמי | l-b-q-m-y |
| Atbash | כשדימ | k-sh-d-y-m |

***Lev Qamai* → *Kasdim*** (the Chaldeans).

### What these attestations constrain

**Both are Hebrew to Hebrew.** Input and output are both Hebrew strings. No
translation, no transliteration from another language, no crossing between
alphabets.

**Both are proper nouns**, and both name a hostile power in a prophetic text.

**And in both cases the plain name appears elsewhere in the same book.** Jeremiah
names Babylon openly dozens of times. This is not a cipher protecting a secret; it
is a literary device used occasionally for effect.

Whether the device is protective or rhetorical is disputed in the commentaries and
this book does not resolve it.

---

## THE BAPHOMET TRANSFORMATION

Reproduced here because Chapter 16 turns on it and the reader is entitled to check
the arithmetic.

| Position | Hebrew | Atbash | |
|---|---|---|---|
| 1 | ב bet | ש shin | |
| 2 | פ pe | ו waw | |
| 3 | ו waw | פ pe | |
| 4 | מ mem | י yod | |
| 5 | ת taw | א alef | |

**בפומת → שופיא**

b-p-w-m-t → sh-w-p-y-'

**The transformation is exact and reproducible.** What it does not establish is set
out in Chapter 16 and in Appendix I, Example 2.

---

## OTHER TEMURAH SCHEMES

Atbash is one substitution scheme among several that the tradition treats as
available. The existence of alternatives matters for the search-space analysis of
Chapter 39, and they are listed here for completeness.

**Albam** — the alphabet is divided in half and the first letter paired with the
twelfth: alef↔lamed, bet↔mem, and so on.

**Atbach** — pairs letters whose numerical values combine to a round figure.

**Achbi**, **Ayaq Bakar** and others are attested in the Kabbalistic literature.

**There is no rule in the tradition specifying which scheme applies to which word.**
An interpreter free to choose among them has several chances to find a usable
output, and Chapter 39 counts what that does to the evidential situation.

---

## COMPUTATION

The table, both biblical attestations and the Baphomet transformation were computed
by the author. The script (`atbash.py`) is published with this book and includes an
assertion that the alphabet contains exactly twenty-two letters, which is the sort
of check that catches the error described in Appendix E.
