# CHAPTER FOURTEEN
## Gematria

*Echad*, the Hebrew word for one, has the value 13. *Ahavah*, the word for love,
also has the value 13. Together they make 26, which is the value of the
Tetragrammaton.

Unity plus love equals the Name.

I have run these figures and they are exact. Alef 1, chet 8, dalet 4: thirteen.
Alef 1, he 5, bet 2, he 5: thirteen. Yod 10, he 5, waw 6, he 5: twenty-six.[^1]

It is a lovely thing and it has been treasured for centuries, and this chapter is
about what it is and what it is not.

---

### The systems

Gematria is not one procedure. It is a family, and which member you use changes
every answer.

**Mispar hechrachi**, the standard reckoning, is the one everyone means by default:
alef 1 through tet 9, yod 10 through tsadi 90 by tens, qof 100 through taw 400 by
hundreds. The full table is in Appendix E.

**Mispar siduri**, the ordinal reckoning, gives each letter its position: alef 1,
bet 2, on to taw 22.

**Mispar katan**, the reduced reckoning, collapses each value to a single digit by
adding: yod, normally 10, becomes 1; resh, normally 200, becomes 2.

And there are more — systems that square the values, that count the spelled-out
names of the letters, that add the number of letters to the total, that treat final
forms as 500 through 900 rather than at their medial values.

**Each of these is a legitimate traditional system. Each produces different
numbers. Nothing in the tradition specifies which to use for a given word.**

That last sentence is the whole methodological problem in one line, and Part IX
will return to it under the name *search space*.

---

### An error, reported where it happened

This book's gematria calculations were done by machine, and the first version of
the program was wrong.

It held a table of the twenty-two letters and their values, looked up each
character of a word, and summed. Clean, simple, and it silently ignored any
character it did not recognise — which included all five final forms.

So *Elohim*, אלהים, ending in final mem, computed as 46. The correct value is 86.
The program had dropped a letter worth 40 and returned a confident answer.

I caught it because I had written a check: ten standard figures from the published
literature, computed and compared. Nine passed and one did not, and the one that
failed was the only word in the test set with a final form.

**Two things about this are worth more than the embarrassment.**

The first is that the error was invisible in the output. Forty-six is a perfectly
plausible-looking number. Had the check not been there, it would have gone into the
book, and a reader would have had to know the correct figure already to catch it.

The second is general. **A procedure that silently skips what it does not
understand will produce confident wrong results**, and this is not a rare failure
mode in the literature this book examines — it is close to the standard one. Every
published gematria claim depends on decisions about final forms, about spelling,
about which system, and most publications do not state them. A reader cannot check
what is not disclosed.

The corrected program reproduces all ten standard figures. The script is published
with this book. The error is in Appendix E as well as here.

---

### What gematria is for

Now the substantive question, and the answer is not what either the enthusiasts or
the debunkers expect.

Within rabbinic tradition, gematria is a **homiletic device**. It belongs to
*derash* — the interpretive mode that draws lessons from a text — and its function
is to propose a connection worth reflecting on. A preacher notes that two words
share a value; the sharing becomes an occasion for saying something about how the
two ideas relate; the congregation takes the point or does not.

**It is not, and was not offered as, a method for recovering concealed
information.**

This distinction is the one that the modern literature destroys. Gematria in its
home tradition is closer to a pun than to a cipher — a device for generating
insight, operating in a register where nobody supposes that the arithmetic *proves*
anything. Nobody in the rabbinic literature claims that God encoded a message
retrievable by addition. The claim is that a numerical coincidence can be an
occasion for thought, which is a considerably more modest and more defensible
position.

And the technique operated under constraints. A fixed canon: the text was not
open-ended. A shared language, with agreed spellings. And a community with
standards about what counted as a reading worth making, in which a proposal that
struck everyone as forced simply did not survive.

**Remove canon, community and judgement, and the technique becomes a machine for
producing anything.** That is what happened to it, and the rest of this chapter
measures the damage.

---

### The base rate

Here is the computation that this chapter exists to report.

I assembled a corpus of common Biblical Hebrew words — *chosen for frequency, not
for their values*, and fixed before any arithmetic was done. Body parts, family
terms, materials, times of day, ordinary furniture of the language. 132 distinct
words.

Then I computed the standard gematria of each and asked how many share a value with
at least one other word in the corpus.

**Thirty-four of the 132 — 25.8 per cent — collide.** The corpus yields
**seventeen** equal-valued pairs.[^2]

A sample, and I am taking these in numerical order rather than selecting the
striking ones:

- 14 — *zahav* (gold) = *yad* (hand)
- 32 — *kavod* (glory) = *lev* (heart)
- 50 — *tame* (impure) = *yam* (sea)
- 70 — *yayin* (wine) = *sod* (secret)
- 90 — *mayim* (water) = *melekh* (king)
- 302 — *boker* (morning) = *kever* (grave)
- 358 — *mashiach* (messiah) = *nachash* (serpent)

Every one of those can be preached. *Glory equals heart.* *Morning equals grave.*
*Water equals king.* They arrive ready-made, and a sufficiently fluent homilist
could build something on any of them within a minute.

Note particularly *yayin* = *sod*, wine equals secret. **That one is a genuine
rabbinic observation** — *nichnas yayin, yatza sod*, "wine goes in, the secret comes
out" — and my corpus produced it without being asked, because I had put both words
in for being common and the arithmetic did the rest.[^3]

---

### What the base rate means

A quarter of ordinary Hebrew words collide with another ordinary Hebrew word in a
132-item sample. A full Biblical Hebrew lexicon runs to something like eight
thousand words. The number of available equal-valued pairs in the language is very
large.

**So *mashiach* = *nachash* = 358 is arithmetically correct and, taken by itself,
evidentially weightless.** It is one of many thousands of such pairs. The reason it
is famous is that somebody noticed it and said something interesting about it, and
the interesting thing was supplied by the interpreter, not found in the numbers.

The same applies to *echad* + *ahavah* = YHWH. The arithmetic is exact. The
question is how many other pairs of theologically usable Hebrew words sum to 26,
and the answer is: enough that finding one is not a discovery.

**This is not a debunking of rabbinic gematria**, and I want to be explicit
because the argument is easy to misread. The tradition was not claiming to have
found proof. It was proposing connections in a homiletic register, in full awareness
that the register was homiletic. A preacher who says *notice that glory and heart
share a value* has said something a congregation can think about, and has not
claimed to have decoded anything.

**The error is entirely modern**, and it consists in taking a device for generating
reflection and treating it as a device for recovering fact — then presenting the
output as evidence.

---

### The Greek control

A test available for gematria that nobody in the esoteric literature seems to run,
and which settles a good deal.

**Greek does the same thing, and nobody thinks Greek is a sacred language.**

Greek letters served as numerals: alpha 1 through theta 9, iota 10 through koppa
90, rho 100 through sampi 900. The system is structurally identical to Hebrew's, and
for the same practical reason.

**And Greek developed the same technique.** *Isopsephy* — from *isos*, equal, and
*psephos*, a counting-pebble — is number-word interpretation, attested in graffiti,
in oracles, in riddles, and in at least one very famous passage of scripture.

**Revelation 13:18** gives the number of the beast, and tells the reader that it
requires wisdom to calculate. This is an isopsephic puzzle, understood as such since
antiquity, and the proposed solutions have multiplied for two thousand years —
Nero Caesar in Hebrew transliteration being the most widely accepted, which requires
choosing a language, a spelling and a transliteration, each of which is a fork.

**Now the control question.** If gematria's striking results indicate something
special about Hebrew, then Greek isopsephy should produce fewer of them.

**It does not.** It produces exactly the same density of coincidence, because the
phenomenon is a property of alphabets doing double duty as numerals, not of the
languages they write.

**And English, which never had letter-numerals, produces the same density again**
once a scheme is invented for it — as the preceding volume demonstrated with
household objects.

**Three languages. Three completely different histories. Identical yield.**

That is a control test in the proper sense, and it points where all the controls in
this book point: **the procedure is not detecting a property of the material. It is
generating a property of the arithmetic.**

---

### And in English

One last observation, needed for Part V.

Everything above concerns Hebrew, where the letters really do have numerical values
and really were used as numerals. English letters have no such values. Any scheme
assigning A=1 through Z=26 is an invention of the modern period, with no tradition
behind it and no historical usage to constrain it.

Applying gematria to English Masonic vocabulary is therefore not a use of an
ancient technique. It is a new procedure with an old name, and the preceding volume
of this series tested it: twenty Masonic words and twenty household objects and
root vegetables, the same arithmetic on both, and **the identical yield** — three
internal pairs and one "significant" number on each side.

The parsnips did as well as the pillars. That test is reproduced in Appendix E of
this volume for readers who want the figures.

---

### NOTES

[^1]: All gematria in this book was computed by the author; the script, the
procedure and the verification against ten standard published figures are given in
Appendix E and published with the book.

[^2]: The corpus, the collision figures and the full pair list are in Appendix E.
The corpus was fixed before any value was computed, which is the condition Chapter
42 requires of everyone else.

[^3]: On the rabbinic *yayin/sod* observation see the standard collections; it is
widely quoted and the attribution varies. **[SOURCE VERIFICATION REQUIRED — earliest
occurrence.]** The point here is not the provenance of the saying but that a
frequency-chosen corpus reproduced the numerical fact unprompted.

[^4]: On gematria within rabbinic interpretation see Ephraim E. Urbach, *The Sages*
(Jerusalem: Magnes, 1975); Joseph Dan, *Kabbalah: A Very Short Introduction* (New
York: Oxford University Press, 2006); and, for the linguistic tradition more
broadly, Moshe Idel, *Language, Torah, and Hermeneutics in Abraham Abulafia*
(Albany: SUNY Press, 1989).
