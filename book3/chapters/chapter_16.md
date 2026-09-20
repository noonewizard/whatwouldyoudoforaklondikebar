# PART IV — THE MASONIC WORD

# CHAPTER SIXTEEN
## Atbash

This chapter is an account of being wrong, and then wrong again, and then arriving
somewhere I did not expect. I am telling it in the order it happened, because the
order is the argument.

---

### The cipher

Atbash is the simplest possible substitution on an alphabet. First letter for last,
second for second-last, and so on to the middle.

In Hebrew: alef ↔ taw, bet ↔ shin, gimel ↔ resh, dalet ↔ qof, he ↔ tsadi, waw ↔
pe, zayin ↔ ayin, chet ↔ samekh, tet ↔ nun, yod ↔ mem, kaf ↔ lamed. Eleven pairs,
twenty-two letters. The name is itself an example: **A-T-B-SH**, the first and last
and second and second-last letters, spelling out the rule.

The full table is Appendix C.

It is its own inverse — apply it twice and you get back what you started with —
which makes it convenient and cryptographically worthless. There is no key. There
is one transformation, and anyone who knows the alphabet knows it.

---

### The two attestations

Here is the entire undisputed evidential base for Atbash in antiquity. It consists
of two places in the book of Jeremiah.

**Jeremiah 25:26 and 51:41** contain the word *Sheshach*, ששך. Under Atbash: shin →
bet, shin → bet, kaf → lamed. The result is בבל — **Babel**, Babylon.

**Jeremiah 51:1** has the phrase *Lev Qamai*, לב קמי, "the heart of those who rise
against me." Under Atbash: lamed → kaf, bet → shin, qof → dalet, mem → yod, yod →
mem. The result is כשדים — **Kasdim**, the Chaldeans.

I ran both. Both reproduce exactly.[^1]

Three features of these attestations deserve attention, because they constrain
everything built on the technique afterwards.

**Both are Hebrew to Hebrew.** The input is a Hebrew string and the output is a
Hebrew word. No translation, no transliteration, no crossing between languages.

**Both are proper nouns.** Specifically, both are the names of a hostile power, in
a prophetic text, at a moment where naming Babylon directly might have been
imprudent or where the concealment is a literary flourish — scholars differ on
which.

**And in both cases the text preserves the plain name elsewhere.** Jeremiah names
Babylon openly dozens of times. This is not a cipher protecting a secret. It is a
device, used occasionally, for effect, in a text that does not otherwise conceal
the thing.

**Atbash in the Hebrew Bible is a figure of speech, not a security system.**

---

### The claim

Now the famous application.

In 1984 Hugh Schonfield proposed that *Baphomet* — the name attached to an object
the Knights Templar were accused of venerating, in testimony extracted during the
proceedings against the order from 1307 — is Atbash for something.

Write Baphomet in Hebrew consonants: בפומת, b-p-w-m-t. Apply Atbash: bet → shin, pe
→ waw, waw → pe, mem → yod, taw → alef. The result is שופיא, sh-w-p-y-a.

Which is *Sophia* — the Greek word for wisdom.[^2]

I ran it. **It reproduces exactly.** בפומת maps to שופיא and there is no arithmetic
error anywhere in the operation.

This is the point at which I expected to write the debunking, because I have spent
two volumes on claims of this shape and they generally fall over at the first
control.

---

### First attempt, and what was wrong with it

The obvious test: take names that nobody claims are encoded, run them through the
same procedure, and see whether they produce anything comparable. If control names
also yield meaningful outputs, the procedure is generating rather than detecting.

So I built a list. For each name I enumerated the plausible Hebrew spellings —
since a foreign word transliterated into Hebrew has more than one admissible form —
and applied Atbash to all of them, checking the outputs against a list of Hebrew
and Greek words a motivated interpreter would accept as meaningful.

Controls: Melchior, Balthazar, Pantagruel, Bartholomew, Tambourine, Pomegranate.

**Result: zero hits across the controls. One hit for Baphomet.**

I looked at that for a while and then looked at the column I had not been looking
at, which was the number of spellings each name generated.

Baphomet: **216**. The controls: between 4 and 72.

My pattern for Baphomet had three optional vowel-letter positions and the control
patterns had fewer. **I had given the claim four to fifty times as many chances as
its controls and then reported that it alone succeeded.**

That is not a control. It is a demonstration that a larger search space finds more,
which was known. And it is exactly the error this book's ninth part exists to
identify — committed by the author, in the course of trying to identify it.

---

### Second attempt

Matched skeletons. I built control names with the identical consonant-vowel
structure — same number of optional positions, comparable letter classes — and ran
them again.

Nine names, one of them Baphomet. Search spaces now comparable: 27 to 216 for the
controls, 216 for Baphomet.

**Controls: zero hits in 777 spellings. Baphomet: one hit in 216.**

I also computed the base rate directly. Taking random strings over the Hebrew
alphabet, how often does Atbash produce a word on the target list? For a five-letter
string, the analytic figure is **0.000116 per cent** — about one in a million — and
two hundred thousand Monte Carlo trials agreed.[^3]

So: a properly matched control returning nothing, and a base rate of one in a
million.

On that showing the Baphomet result stands up, and I want to be straightforward
about what that felt like. I had expected a collapse, run the test that should have
produced it, and the claim survived. An honest analyst at that point has to say so,
and I drafted a paragraph saying so.

---

### Third attempt, and the actual finding

Then I asked where the target list came from.

I wrote it. I wrote it knowing the answer.

*Sophia* was on that list because Schonfield's claim put it there. And *Sophia* is
a **Greek** word, rendered into Hebrew consonants in a defective spelling, which is
not a form that appears in any Hebrew lexicon. No independently compiled list of
Hebrew words — no dictionary, no concordance, nothing built by someone who had not
heard of this claim — would contain שופיא.

So I removed it and ran the same procedure again.

**Baphomet: zero hits. Identical to every control.**[^4]

That is the finding, and it took three attempts to reach it.

---

### What this establishes, precisely

I want to be careful, because both over- and under-statement are available here.

**The transformation is real.** בפומת does map to שופיא under Atbash. The arithmetic
is exact, it is reproducible, and the script is published with this book. Anyone
claiming that the calculation is wrong is mistaken.

**The interpretation carries no weight**, because the criterion for what counted as
a meaningful output was written by someone who already knew the answer. A test whose
success condition is set after the result is not a test. It is a description of the
result with a procedure attached.

**And the claim fails on independent grounds anyway**, which the control exercise
almost distracted me from. The Templar depositions are inquisitorial testimony
taken under torture, in which the name appears in wildly variant forms. There is no
evidence that the Templars used Atbash, or Hebrew. And even if every step held, it
would be a claim about the Templars — and the Templar-to-Masonry chain fails for
reasons set out at length in Book I.

The full ten-test assessment is in Chapter 42. Atbash/Baphomet passes tests 3 and 7
— the technique is genuinely attested and the computation genuinely reproduces —
and fails the rest.

---

### The Templar problem, independently

Set the computation aside entirely and the claim has a historical difficulty that
would sink it even if the arithmetic had held.

**"Baphomet" is not a Templar word in any secure sense.**

It appears in the depositions taken during the proceedings against the order from
1307 — testimony extracted under judicial torture, in a prosecution with a
predetermined outcome, by interrogators working from a prepared list of charges.

**The forms vary wildly.** Baphomet, Baffometi, Baphometh, and others. Witnesses
described the alleged object of veneration in mutually incompatible terms: a head,
a cat, an idol with three faces, a bearded figure. **The testimony does not agree
with itself**, which is what testimony under torture characteristically looks like.

And there is a strong philological alternative that has been available for a long
time: that *Baphomet* is a corruption of *Mahomet* — that the charge was of
venerating an idol of Muhammad, which was a standard accusation against groups
suspected of eastern contamination, and which Christians of the period wrongly
believed Muslims practised.

**That explanation requires nothing but the ordinary behaviour of medieval
polemic**, and it accounts for the variant forms as transmission noise.

**So the claim asks us to apply a Hebrew cipher to a word of uncertain form, from
coerced testimony, attributed to an order with no demonstrated knowledge of Hebrew
or of Atbash, in order to recover a Greek term.**

Test 1 and Test 8, and each is fatal on its own.

**And the Masonic relevance is nil even if every step held**, because it would be a
finding about the Templars, and the documentary chain from the Templars to
Freemasonry fails at the fifteenth-to-eighteenth-century gap — which Book I of this
series established at length and which no subsequent work has repaired.

---

### Why the sequence is printed

I could have reported only the third attempt. It would have been shorter and I
would have looked better.

I have printed all three because the second one is the instructive one. **A
matched control returned zero and the claim survived it**, and if I had stopped
there I would have published a result that was wrong in a way no reader could have
caught, because the flaw was not in the arithmetic or the controls. It was in a
list I had written myself, which looked like part of the apparatus and was actually
part of the answer.

That failure mode is not rare. It is, I think, the commonest way that careful people
go wrong on this material: not by miscalculating, and not by skipping the controls,
but by **building the acceptance criterion out of the thing they are testing**.

The question to ask of any decipherment — including one's own, and especially one
that has survived a control — is: *who decided what would count as a meaningful
output, and when?*

If the answer is *the person who already had the output*, then nothing has been
tested, however rigorous everything downstream looks.

---

### NOTES

[^1]: Computed by the author; script, output and the full Atbash table in Appendix
C and published with this book. Jeremiah 25:26, 51:41, 51:1. On Atbash in the
prophetic text see the standard commentaries; whether the device is protective or
rhetorical is disputed.

[^2]: Hugh Schonfield, *The Essene Odyssey* (Shaftesbury: Element, 1984). Cited as
the origin of the claim and as evidence of a belief, not as historical
authority.

[^3]: Analytic figure and Monte Carlo confirmation in Appendix I; 200,000 trials,
seeded, reproducible.

[^4]: The three scripts — the confounded first control, the matched second, and the
target-list test — are all published with this book, including the one containing
the error. Appendix I gives the figures.
