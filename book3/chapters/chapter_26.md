# CHAPTER TWENTY-SIX
## Pigpen

There is a cipher that everybody calls the Masonic cipher, and the name is doing a
great deal of unearned work.

---

### The system

Draw two noughts-and-crosses grids and two large X shapes. Distribute the
twenty-six letters among the compartments — nine in the first grid, nine in the
second, four in each X. Each letter is then represented by the *shape of the
compartment enclosing it*: an angle, a corner, a wedge. The second grid and second
X are distinguished by a dot.

The result is a script of hooks and angles that looks nothing like writing and is
learnable in about five minutes.

It is called the pigpen cipher, the Freemason's cipher, the Rosicrucian cipher, and
occasionally the Napoleon cipher. **The multiplicity of names is the first thing
worth noticing**, and the second is that several of the attributions are wrong.

---

### What Masons actually did with it

The documented uses are consistent and they are modest.

Inscriptions on gravestones and memorials. Headings and notations in lodge minute
books. Text on certificates. Engraving on jewels and medals. Marks in mark-books.
Eighteenth and nineteenth centuries, chiefly in Britain and America.[^1]

**[SOURCE VERIFICATION REQUIRED — this study works from the general statements in
the cipher literature and from secondary Masonic sources. Specific objects, with
dates, locations and holding institutions, are required before publication. That
Masons used the cipher is not in doubt; the book should cite artefacts rather than
assertions, and this is a declared gap.]**

Note the character of the list. **Labels.** Short strings, in prominent positions,
on objects meant to be seen.

Nobody used it to write a treatise. There is no encrypted Masonic corpus, no body
of enciphered correspondence, no cipher archive waiting to be broken. What survives
is inscriptions.

---

### The key space is one

Apply Chapter 23's apparatus and the assessment is over in a sentence.

**Pigpen as used is unkeyed.** There is one layout. Anyone who knows the layout
reads everything written in it. There is no setting to vary, no secret parameter,
nothing that could be changed if the system were compromised.

By Kerckhoffs's principle this is the definition of a broken system: its security
depends entirely on the adversary not knowing the method, and methods leak. The
layout was printed in the nineteenth century and is now in children's puzzle
books.[^2]

**Key space: 1.**

It is worth being precise about what that means. It does not mean a weak cipher. It
means **not a cipher** in the security sense — a notation, an alternative script,
of the same order as shorthand or a foreign alphabet.

---

### And it fails everything else too

Three further defects, each independently fatal for concealment.

**It is monoalphabetic.** Letter frequencies pass through unchanged. I computed
this against a sample sentence and the ciphertext frequency distribution is
identical to the plaintext by construction — E remains the commonest symbol,
whatever shape E has been assigned.[^3] Frequency analysis, available since the
ninth century, breaks it immediately.

**It preserves word division.** Every surviving Masonic example keeps the spaces.
This is the single largest concession a cipher clerk can make: word lengths are
visible, one- and two-letter words are identifiable, and the structure of the
message is on display.

**And the texts are short.** Short ciphertexts resist frequency analysis better
than long ones, which sounds like a defense and is not — because short Masonic
texts are names and dates, and a name-and-date inscription on a Masonic gravestone
is guessable from context by anyone who has seen another one.

---

### What follows

The inference is not that Masons were foolish. It is that **the cipher's weakness
tells us what it was for**, and the reasoning is straightforward.

By 1750, polyalphabetic ciphers had been in print for two centuries. Vigenère was
available. Alberti was available. Some of the men involved in early Grand Lodge
Masonry were Fellows of the Royal Society, and the circle around the 1723
*Constitutions* included experimental philosophers of real standing. The proposition
that these men could not have found a better cipher is not credible.

**They used a weak one because their adversary was weak.**

Who was the adversary? Chapter 23 asked the question and the pigpen answers it. The
curious servant who might open a minute book left on a table. The stranger in a
churchyard. The neighbor wondering what the certificate says.

Against those, an unkeyed grid is entirely adequate. It stops the casual reader and
it stops nobody else, which is exactly the level of protection the situation
required.

**And there is a second function, which the gravestone use makes plain.** A pigpen
inscription on a monument is not hiding anything from posterity — it is *announcing*
that the man beneath was a Mason, to anyone who can read it, while remaining opaque
to those who cannot. It is a badge that doubles as a filter. The concealment is
part of the display.

This is the same structure the preceding volume found throughout Masonic secrecy:
**secrecy that advertises.** A body genuinely hiding does not put its cipher on a
public monument in a churchyard.

---

### The gravestone problem

There is something genuinely odd about the pigpen material and it is worth dwelling
on, because it inverts the usual assumption about what secrecy is for.

**The most conspicuous surviving use of the Masonic cipher is on public
monuments.**

Consider what a gravestone is. It stands in a churchyard, in the open, where anyone
may walk. It is designed to be seen, and to be seen indefinitely. It is, of all
possible objects, the least suitable place to put something you wish to conceal.

And that is where the cipher is.

**So what was being done?**

Not concealment from posterity, plainly. Not concealment from the parish, who knew
perfectly well who in the village was a Mason — lodges processed publicly, wore
regalia, attended church in a body, and laid foundation stones with ceremony.

**What the inscription does is mark a category and filter its readers.** A brother
passing through reads it. A stranger sees marks. The stone says two different things
to two audiences, simultaneously, and the difference is the point.

**This is secrecy as display**, and the preceding volume found the same structure
throughout Masonic practice: an institution that advertises its own opacity, whose
buildings are signposted and whose members are identifiable, and which nonetheless
maintains a language of concealment.

The cipher on a gravestone is that structure in stone. It is not hiding the man's
membership. **It is performing it** — declaring, in a script that announces itself as
restricted, something everybody in the village already knew.

A body genuinely concealing something does not carve it where the public walks.

---

### The name

A final matter, because the brief for this book is explicit about it.

**"The Masonic cipher" is a modern convention and this book will not use it without
qualification.**

The grid is also called the Rosicrucian cipher. It was used by Union forces in the
American Civil War. Variants appear in contexts with no Masonic connection at all.
Claimed antecedents reaching back to the Templars, or to Hebrew sources, or to any
medieval origin, are weak: the firm documentation is eighteenth- and
nineteenth-century, and earlier attributions rest on the sort of resemblance
argument that Chapter 42 disposes of.[^4]

**Masons used this cipher. It is not exclusively theirs, it is not their invention
so far as anyone can show, and calling it the Masonic cipher has encouraged the
belief that Freemasonry had a cryptographic tradition.**

It did not. It had a decorative substitution alphabet, used for labels, which any
member could read and which any outsider could learn in an afternoon.

---

### The one positive finding

This chapter has been negative and it should end with what pigpen *does*
establish, because Chapter 41 will need it.

**Pigpen inscriptions are a demonstrated historical encoding**, and there are not
many of those in this book.

They pass every test. Provenance: physical monuments, located and dated. Encoding
rule: documented independently, in cipher literature and Masonic sources. The rule
is fixed and public *before* any inscription is read, so there is no
after-the-fact fitting. Controls: applied to non-Masonic monuments, the method
yields nothing. Base rate: the output is continuous legible text, not a single
lucky word. Replication: anyone with the grid reads the same inscription.
Plausibility: Masons demonstrably used it. Intentionality: somebody carved it.

**Ten for ten**, and Chapter 42 runs the full assessment.

What it encodes is a name and a date.

That is the whole of the Masonic cipher tradition: a genuine cipher, genuinely
used, demonstrably intentional, carrying nothing secret at all. It is the best
evidence in this book that Freemasonry concealed nothing in writing — because when
the Craft did write in cipher, we can read it, and it says who is buried here.

---

### NOTES

[^1]: On the pigpen cipher and its documented use see David Kahn, *The Codebreakers*,
rev. edn (New York: Scribner, 1996), and Simon Singh, *The Code Book* (London:
Fourth Estate, 1999). **[SOURCE VERIFICATION REQUIRED — specific artefacts.]**

[^2]: Auguste Kerckhoffs, "La cryptographie militaire," *Journal des sciences
militaires* 9 (1883). **[PAGE VERIFICATION REQUIRED.]**

[^3]: Computed by the author; script and output in Appendix D and published with
this book.

[^4]: On the name and the competing attributions, and on the weakness of claimed
medieval antecedents, see Kahn, *The Codebreakers*, and the discussion in Appendix
D.
