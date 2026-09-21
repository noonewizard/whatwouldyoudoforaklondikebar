# CHAPTER ELEVEN
## Hebrew Letters

Everything in Parts III and IX depends on a single technical fact about the Hebrew
writing system, and the fact is not difficult. It is simply unfamiliar to readers
whose alphabet works differently.

**Hebrew was written without vowels.**

Twenty-two signs, all of them consonants. The vowels were supplied by the reader,
from knowledge of the language, exactly as a modern reader supplies them when
reading an abbreviation or a text message stripped of them. *Rdng nglsh wtht vwls
s hrdr thn t lks bt t s pssbl* — and it is a great deal easier when the language is
your own, the subject is familiar, and you have been doing it all your life.

This one fact dissolves more confusion than anything else in the book.

---

### What the letters are

The twenty-two, in order, with the traditional names: alef, bet, gimel, dalet, he,
waw, zayin, chet, tet, yod, kaf, lamed, mem, nun, samekh, ayin, pe, tsadi, qof,
resh, shin, taw.

Five of them — kaf, mem, nun, pe, tsadi — take a different shape at the end of a
word. These are the *sofit* or final forms, and they are a scribal convenience
rather than separate letters. This matters more than it sounds: any computation
performed on Hebrew must decide what to do with them, and a program that does not
recognize them will silently drop letters and return confident wrong answers.
This book's own calculations did exactly that at one point, and Chapter 14 reports
the consequence.

Four letters — alef, he, waw, yod — do double duty. Besides representing
consonants they can mark the presence of a long vowel, and in that role they are
called *matres lectionis*, "mothers of reading." Their use expands over time, which
is itself a dating tool: fuller spelling is generally later.

**This optionality is the crucial point for Part IX.** A Hebrew word can often be
spelled more than one way — with or without a mater — and both spellings are
correct. Which means that any transformation applied to a Hebrew word has, built
into it, a choice the analyst makes before the transformation begins. Chapter 16
will show what happens when that choice is made after the answer is known.

---

### The letters are also numbers

Hebrew, like Greek, had no separate numerals. The letters served.

Alef through tet are 1 to 9. Yod through tsadi are 10 to 90 by tens. Qof through
taw are 100 to 400 by hundreds. The system is transparent and entirely practical:
you need to write numbers, you have an alphabet, you use it.

**The direction of that explanation matters and is routinely reversed.**

The standard esoteric account holds that the letters carry numerical values because
of a deep correspondence between language and quantity, and that gematria reads a
structure placed in the language by its divine author. The historical account is
that the letters carry numerical values because somebody had to write down how many
sheep there were, and the alphabet was what was available.

The second account is supported by the fact that **Greek does the same thing with
the same result.** Greek letters also served as numerals, Greek also developed
number-word play — *isopsephy* — and Greek isopsephy produces the same kinds of
striking equivalences that Hebrew gematria does. Two unconnected traditions
developed the technique independently, because both had alphabets doing double
duty. That is convergence from a shared practical constraint, not evidence of a
common mystical inheritance.

The classic instance is the number of the beast in Revelation 13, which is almost
certainly an isopsephic reference and has been read as such since antiquity — and
which requires the reader to guess which language and which spelling, with the
consequence that the candidates have multiplied for two thousand years. Chapter 14
returns to why that multiplication is guaranteed.

---

### The letters have names, and the names are words

*Alef* is connected to the word for ox; *bet* means house; *dalet* is related to
door; *ayin* means eye; *pe* means mouth; *resh* means head. The connection is not
mystical. The alphabet's distant ancestors were pictographic, and the sign for the
sound /b/ began as a picture of a house.

This is genuine and it is the foundation of an enormous interpretive literature.
When a Kabbalistic text reads significance into *bet* as the letter of creation —
Genesis begins with it — the house-association is part of what is being worked
with, and the tradition is not making the etymology up.

**What the tradition adds is the inference that the association is meaningful**,
that the divine author chose to begin the Torah with the letter of dwelling for a
reason. That inference is a theological commitment, not a philological finding, and
the two should not be run together. The etymology is evidence about the history of
writing. Everything built on it is interpretation.

---

### The Masoretes, and the invention of the vowel points

Between roughly the seventh and tenth centuries CE, scholars working principally at
Tiberias devised a system of marks — dots and small strokes placed above, below and
inside the consonants — recording the vowels, the cantillation, and a great deal
of detail about how the text was to be read.[^1]

The work was extraordinary and conservative in spirit. The Masoretes did not alter
the consonantal text, which they regarded as fixed. They added an apparatus around
it, preserving a reading tradition that had until then been transmitted orally, and
they did so with a precision that modern textual scholarship has found remarkably
reliable.

Three consequences bear on this book.

**The pointing is late.** A thousand years or more after most of the material it
records. It preserves a reading tradition of real antiquity, but it is not itself
ancient, and any argument resting on the vowels of a biblical word is resting on
Tiberian evidence about how that word was read in the early Middle Ages.

**The pointing is a reading aid, not a phonetic transcription of the original.** It
records how the text was read in a particular community at a particular time.

**And in one case the pointing deliberately does not belong to the word it is
attached to.** That case is the Tetragrammaton, and it is the subject of the next
chapter, where it has produced a name that never existed.

---

### How the script constrains what can be claimed

Two specific consequences of consonantal writing that recur throughout Part IX and
are worth isolating.

**First: the same consonantal string can be several different words.**

Hebrew *d-b-r* can be vocalised as *davar* (word, thing), *dibber* (he spoke),
*dever* (pestilence), *dover* (speaking), and more. The consonants are identical.
**Context decides, and where context is thin, the reader decides.**

This is ordinary and native speakers handle it without effort. But it means that a
transformation producing a consonantal output has not produced a word. It has
produced a *set* of possible words, and the analyst selects among them — which is a
choice, made after seeing the output, and therefore exactly the kind of choice
Chapter 42's Test 4 was written about.

**Second: spelling is genuinely variable.**

The *matres lectionis* are optional in many words. The same word appears in the
Hebrew Bible with and without a *waw* or *yod*, and both spellings are correct —
the variation is one of the tools textual scholars use for dating.

**So a Hebrew word does not have *a* spelling to transform.** It has a small family
of admissible spellings, and the analyst picks one.

**Multiply the two.** Choice of spelling, then choice of vocalisation of the output.
Each multiplies the search space, and neither is visible in a published claim,
which typically reports a single input and a single output as though both were
given.

**This is the mechanical reason that Hebrew is such productive material for
decipherment claims**, and it has nothing to do with the language being sacred. It
is a property of consonantal writing with optional vowel-letters, and Arabic —
which shares the property — supports a similar literature for the same reason.

**A script that requires the reader to supply information is a script in which the
reader can supply what they were hoping to find.**

---

### What follows for the transformations

This chapter is technical groundwork and it pays off in Part IX, so it is worth
stating the payoffs now.

**A consonantal script makes transformation easy and ambiguous.** Fewer signs,
optional spellings, no vowels to constrain the result. A four-letter string
transformed by any rule produces another four-letter string, and the analyst then
decides how to vocalise the output — which is to say, decides what it means. **The
freedom is in the vocalisation, and the vocalisation is supplied by the person who
wants an answer.**

**The numerical values are exact and the words are not.** Gematria arithmetic is
arithmetic; there is no arguing with 86. But which spelling of a word to use, and
which of several systems, and whether to count final forms at their medial values
or in the 500–900 series, are all choices, and each choice produces different
numbers.

**And the alphabet is small.** Twenty-two letters means that any substitution
cipher on Hebrew has a tiny space of possible transformations compared with what
the same operation would have in a language with more signs, which makes chance
collisions correspondingly more likely.

None of this is an argument that the Hebrew tradition's letter-techniques are
foolish. Within their own setting they operated under constraints — a fixed canon,
a shared language, a community with standards about what counted as a reading worth
making — and those constraints did real work. Chapter 15 sets them out.

**Remove the constraints and the same techniques generate anything at all**, and
that is the finding of Part IX, computed rather than asserted.

---

### NOTES

[^1]: Geoffrey Khan, *A Short Introduction to the Tiberian Masoretic Bible and Its
Reading Tradition* (Piscataway, NJ: Gorgias Press, 2013); Israel Yeivin,
*Introduction to the Tiberian Masorah*, trans. E. J. Revell (Missoula: Scholars
Press, 1980).

[^2]: On the expansion of *matres lectionis* as a dating criterion see Emanuel Tov,
*Textual Criticism of the Hebrew Bible*, 3rd edn (Minneapolis: Fortress, 2012).

[^3]: On Greek isopsephy and its independence from Hebrew practice, see the
discussion in Chapter 14 below and the sources there cited.

[^4]: The final-form error referred to above occurred in this book's own gematria
routine and produced a value of 46 for *Elohim* where the correct figure is 86. It
is described where it belongs, in Chapter 14, and in Appendix E.
