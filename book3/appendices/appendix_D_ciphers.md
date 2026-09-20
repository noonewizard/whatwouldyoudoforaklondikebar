# APPENDIX D
## Historical Cipher Systems

Reference for Part V. Key-space figures computed by the author.

---

## THE SYSTEMS

| System | Date | Type | Key space | Breaks to |
|---|---|---|---|---|
| **Atbash** | c. 600 BCE | Fixed substitution | **1** | Knowing the alphabet |
| **Skytale** | c. 400 BCE | Transposition | Rod diameters | Trial |
| **Caesar** | c. 50 BCE | Shift substitution | **25** | Exhaustive search, minutes |
| **Arbitrary monoalphabetic** | ancient | Substitution | 26! ≈ 4×10²⁶ | **Frequency analysis** |
| **Alberti disc** | 1467 | Polyalphabetic | Large | Kasiski-type analysis |
| **Trithemius tabula recta** | 1518 | Polyalphabetic | Large | As above |
| **Vigenère** | 1586 | Polyalphabetic, keyword | Very large | Kasiski, 1863 |
| **Pigpen** | 18th–19th c. use | Fixed substitution | **1** | Knowing the layout |
| **Magical alphabets** | 1533 printing | Fixed substitution | **1** | Knowing the table |
| **One-time pad** | 20th c. | Stream | = message length | **Nothing** (Shannon, 1949) |

---

## THE FOUR COMPONENTS

Naming these separately is half the analysis.

**Plaintext** — what you want to say.
**Algorithm** — the general method of transformation.
**Key** — the particular setting of the algorithm.
**Ciphertext** — what comes out.

**Kerckhoffs's principle** (1883): a system should remain secure even if everything
about it except the key is public. Algorithms leak — personnel change sides,
documents are captured — and a system whose security depends on method-secrecy fails
permanently the first time anything goes wrong.

**A key space of 1 is not a weak cipher. It is a notation.** Atbash, pigpen and the
magical alphabets are all in this category: alternative ways of writing, offering
no protection against anyone who has seen the correspondence.

---

## WHY A LARGE KEY SPACE IS NOT ENOUGH

Arbitrary monoalphabetic substitution has 26! ≈ 4×10²⁶ keys and is broken by a
schoolchild, because **the cipher preserves the statistics of the language.**

E is the commonest letter in English, then T, A, O, I, N. Substitute E for Q and Q
becomes the commonest symbol in the ciphertext. Count the frequencies, match them
against the known distribution, and the key falls out.

**Al-Kindi described this in ninth-century Baghdad.** From that point every
monoalphabetic cipher in every language is breakable regardless of key space.

**Every Masonic cipher use postdates this by eight hundred years.**

---

## THE PIGPEN ANALYSIS

Computed for Chapter 26.

**Layout.** Two 3×3 grids and two X-shapes; each letter represented by the shape of
its enclosing compartment, the second set distinguished by a dot. Twenty-six
letters mapped.

**Key space: 1.** Not keyed. The layout is the whole system and the layout was
printed in the nineteenth century.

**Frequency preservation: total.** Computed against a sample sentence; the
ciphertext symbol distribution is identical to the plaintext by construction. Top
five plaintext frequencies in the test sentence: E(8), T(6), O(4), H(3), S(3) —
which pass through unchanged under any relabelling.

**Word division: preserved** in every surviving Masonic example. The single largest
concession a cipher clerk can make.

**Documented uses:** gravestone inscriptions, minute-book headings, certificates,
jewels, mark-books. Eighteenth and nineteenth centuries, chiefly Britain and
America. **[SOURCE VERIFICATION REQUIRED — specific artefacts with dates and
holding institutions. This is a declared Tier-1 gap.]**

**Also called** the Rosicrucian cipher and the Napoleon cipher; used by Union forces
in the American Civil War. Claimed medieval or Templar antecedents are weak.
**"The Masonic cipher" is a modern convention and this book does not use it without
qualification.**

---

## STEGANOGRAPHY

**Cryptography conceals the content of a message.** Anyone can see a message exists;
they cannot read it.

**Steganography conceals the existence of the message.** The carrier looks like
something else, and an observer does not know there is anything to look for.

Different failure modes: broken cryptography yields the message; broken
steganography yields the fact that you were concealing something, which in many
historical situations was the more dangerous disclosure.

**Trithemius's *Steganographia* (1499) is the great monument**, and its peculiarity
is instructive: it presents itself as angel magic, and substantial parts of it are a
cryptographic manual with ciphertext hidden inside the invocations and spirit names
functioning as keys.

**It is the best precedent for the claim that a ritual conceals something**, and it
does not help the Masonic case — because the concealment was *demonstrated*, by
people who published the key and showed their working, and no comparable
demonstration exists for Masonry.

---

## THE DIAGNOSTIC

Reduce all of it to one question.

**Who was the adversary?**

Every genuine cryptographic system has one: a specific party, with capabilities,
from whom something specific is being withheld, whose resources shape the design.

For Masonic practice the answers are small and consistent: the curious servant who
might open a minute book; the stranger reading a gravestone; the neighbour
wondering what the certificate says.

**Against those, an unkeyed monoalphabetic grid is entirely adequate.**

Against a hostile state, a determined scholar, or posterity, it is nothing — and a
body concealing doctrine from those would have chosen differently, because better
systems had been in print for two centuries.

**The cipher's weakness is evidence about its purpose.**
