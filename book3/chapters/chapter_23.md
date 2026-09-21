# CHAPTER TWENTY-THREE
## The Cipher

What makes a message secret?

Not obscurity. Not unfamiliarity. Not the fact that most people cannot read it.
Latin is unreadable to most people and is not a secret. The question has a precise
answer and the precision is what this chapter is for, because almost every claim in
Part IX turns on it.

---

### The anatomy

A cipher system has four components, and naming them separately is half the work.

**The plaintext** — what you want to say.

**The algorithm** — the general method of transformation. Substitute each letter
for another; shift each letter three places; rearrange according to a grid.

**The key** — the particular setting of the algorithm. *Which* substitution;
shift by *how many*; *what* grid.

**The ciphertext** — what comes out.

The distinction between algorithm and key is the one that matters, and it was not
clearly stated until the nineteenth century, when Auguste Kerckhoffs formulated the
principle that bears his name: **a system should remain secure even if everything
about it except the key is public knowledge.**[^1]

The reasoning is practical. Algorithms leak. Personnel change sides, documents are
captured, devices are lost. A system whose security depends on the enemy not
knowing the method is a system that fails permanently the first time anything goes
wrong. A system whose security depends only on the key can recover: change the key
and carry on.

**Hold Kerckhoffs's principle.** It is the instrument that disposes of the pigpen
cipher in Chapter 26, and it does so in one sentence.

---

### Key space

The measure of a cipher's strength, to a first approximation, is how many keys it
has — because an attacker who can try them all will.

**Caesar shift**: twenty-five possible keys, since shifting by zero does nothing. An
attacker tries all twenty-five and reads the message. This takes a competent person
a few minutes with a pencil.

**Arbitrary monoalphabetic substitution**: any of the twenty-six letters can map to
any other, giving 26 factorial arrangements. That is roughly 4 × 10²⁶ — four
hundred septillion — which sounds decisive and is not, for reasons the next section
gives.

**Atbash**: **one**. There is no key. The transformation is fixed. Anyone who knows
the alphabet knows the cipher.

**Pigpen**: **one**, in the form Masonry used it. The layout is the whole of it, and
the layout was eventually printed.

A key space of one is not a weak cipher. It is **not a cipher at all** in the
security sense. It is a notation — an alternative way of writing, like shorthand or
a foreign script, offering no protection against anyone who has seen it before.

---

### Why a huge key space is not enough

The 26-factorial figure is the standard trap and it is worth dismantling, because
the intuition it produces is wrong in a way that matters for the historical
argument.

A monoalphabetic substitution has an enormous key space and is nevertheless broken
by a schoolchild, because the attacker does not have to try the keys. **The cipher
preserves the statistics of the language.**

In English, E is the commonest letter by a wide margin, then T, then A, O, I, N.
Substitute E for Q throughout and Q becomes the commonest letter in the ciphertext.
The frequencies travel. Count them, match them against the known distribution of
the language, and the key falls out — not all at once, but enough of it to guess the
rest from context.

This is **frequency analysis**, and it was described by al-Kindi in Baghdad in the
ninth century, in a treatise on deciphering cryptographic messages. It is not a
modern technique. It was known in Europe by the later Middle Ages and was routine
in diplomatic practice by the sixteenth century.[^2]

**From the ninth century onward, every monoalphabetic cipher is breakable by anyone
who knows this**, regardless of how many keys it has.

That fact will do a great deal of work in Chapter 26.

---

### What a message needs to survive

Beyond the algorithm, three practical features separate a real cryptographic
practice from a decorative one, and their absence in Masonic use is diagnostic.

**Suppression of word division.** Spaces between words are an enormous gift to an
attacker: they reveal word lengths, and a three-letter word ending in the commonest
letter is probably "the". Serious practice runs the ciphertext together, usually in
groups of five. Masonic pigpen inscriptions preserve word division.

**Suppression of frequencies.** Achieved by polyalphabetic substitution, by
homophones — several symbols for E, used in rotation — or by nulls. None appears in
Masonic use.

**Key management.** If the key changes, both parties must know when and how. A
practice with no key management is a practice with no key.

---

### The other kind of secrecy

One further distinction, because it is a genuine alternative and Chapter 25 returns
to it.

**Cryptography conceals the content of a message.** Anyone can see that a message
exists; they cannot read it.

**Steganography conceals the existence of the message.** The carrier looks like
something else entirely — an innocent letter, a picture, a devotional text — and an
observer does not know there is anything to look for.

The two have different failure modes. Broken cryptography yields the message.
Broken steganography yields the *fact that you were concealing something*, which in
many historical situations was the more dangerous disclosure.

Trithemius's *Steganographia* of 1499 is the great early monument here, and its
peculiarity is instructive: it presents itself as a work of angel magic, with
invocations of spirits, and substantial parts of it are actually a cryptographic
manual with the ciphertext hidden inside the invocations. A magical text that *is*
an encoded text.[^3]

**This is the best available precedent for the claim that a ritual conceals
something**, and it is worth stating why it does not help the Masonic case. We know
about Trithemius because the key was published and the method reproduced. The
concealment was *demonstrated*, by people who showed their working. No comparable
demonstration exists for Masonic ritual, and Chapter 42 sets out what one would have
to look like.

---

### Codes, ciphers, and the distinction that matters

A terminological point that does real work, because the words are used
interchangeably in popular writing and mean different things.

**A cipher operates on letters.** Each unit of the plaintext is transformed
individually, by rule. Substitution and transposition are both ciphers.

**A code operates on meanings.** A codebook assigns a symbol to a whole word or
concept — this number means *army*, that group means *the Ambassador* — and there is
no rule connecting the symbol to what it stands for. The relation is arbitrary and
must be looked up.

The difference has two consequences that matter for this book.

**A code is much harder to break by analysis**, because there is no internal
structure to exploit. Frequency analysis works on a cipher because the underlying
letters have statistical properties that survive substitution. A codebook has no
such properties: the number for *army* tells you nothing about the number for
*navy*.

**And a code requires a physical object.** Both parties must hold the same
codebook, which must be produced, distributed and protected. Codes are
administratively expensive in exactly the way Chapter 25's nomenclators were.

**Masonic practice has neither**, which is worth stating plainly. The grid is a
cipher of the weakest kind, and there is no Masonic codebook — no list assigning
concealed meanings to words, no glossary requiring possession.

**What Masonry does have is *jargon*, and jargon is a third thing.** Technical
vocabulary — terms with specialized meanings understood within a trade or body — is
not concealment. Every profession has it. It makes outsiders feel excluded and it
exists to be precise, not to hide, and the exposures translated it without
difficulty.

A great deal of what is taken for Masonic secrecy is jargon read as code.

---

### The question to ask

Reduce all of the above to a single diagnostic and it comes out like this.

**Who was the adversary?**

Every genuine cryptographic system has one. A specific party, with capabilities,
from whom something specific is being withheld. The system's design is shaped by
that party: how much effort they can spend, what they already know, what happens if
they succeed.

Ask this of Masonic practice and the answers are small and consistent. The curious
servant who might open a minute book. The stranger in the churchyard reading a
gravestone. The neighbor who wonders what the certificate on the wall says.

**Against those adversaries, an unkeyed monoalphabetic grid is entirely adequate.**

Against a hostile state, a determined scholar, or posterity, it is nothing at all —
and a body that had been concealing doctrine from those would have chosen
differently, because by 1720 better systems had been in print for two centuries.

Chapter 26 runs that argument to its conclusion.

---

### NOTES

[^1]: Auguste Kerckhoffs, "La cryptographie militaire," *Journal des sciences
militaires* 9 (1883): 5–38, 161–91. **[PAGE VERIFICATION REQUIRED.]**

[^2]: Al-Kindi, *Risalah fi Istikhraj al-Mu'amma* (9th c.). On the history of
frequency analysis and its transmission to Europe see Simon Singh, *The Code Book*
(London: Fourth Estate, 1999), and David Kahn, *The Codebreakers*, rev. edn (New
York: Scribner, 1996).

[^3]: Johannes Trithemius, *Steganographia*, composed 1499, published 1606. The
cryptographic content of the first two books was demonstrated in the seventeenth
century; the third book's status was settled considerably later. See Kahn,
*The Codebreakers*, and Chapter 25 below.

[^4]: Key-space figures computed by the author; see Appendix D.
