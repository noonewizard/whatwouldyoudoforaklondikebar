# CHAPTER TWENTY-FIVE
## Caesar, Alberti, Vigenère

This chapter exists to establish one fact, and the fact is decisive for the
argument of Part V.

**By the time Freemasonry acquired its lost word, European cryptography had four
hundred years of development behind it, and the Craft used none of it.**

That is a negative finding and it is a strong one. To see why, you have to know
what was available.

---

### The classical inheritance

**Caesar.** Suetonius reports that Julius Caesar corresponded in a shifted
alphabet, replacing each letter with the one three places further on. The key space
is twenty-five. It was adequate in a world where most adversaries were illiterate
and nobody had a theory of cryptanalysis, and it is trivial now.[^1]

**The skytale**, a Spartan device in which a strip of leather wound round a rod of
particular diameter reveals a message that is gibberish unwound. This is
transposition rather than substitution — the letters are the right letters in the
wrong order. Its historicity as a cipher device is disputed.

**And Atbash**, which Chapter 16 dealt with: older than both, Hebrew, scriptural,
and a literary figure rather than a security measure.

---

### The break: al-Kindi

Everything changes in ninth-century Baghdad.

Abu Yusuf al-Kindi, working in a milieu where the statistical study of Arabic texts
was being pursued for entirely non-military reasons — questions about the
authenticity of prophetic traditions, among others — wrote a treatise on
deciphering cryptographic messages. In it he described **frequency analysis**.

The insight is simple enough to state in a sentence and it ends an era. Letters
occur with characteristic frequencies in any language; a substitution cipher
preserves those frequencies while changing the labels; therefore count the symbols
in the ciphertext, match the counts against the known distribution of the language,
and the substitution reveals itself.[^2]

**From this point every monoalphabetic cipher in every language is breakable by
anyone who knows the technique**, regardless of key space. The knowledge reached
Europe through the usual channels and was standard in diplomatic practice by the
later Middle Ages.

Hold the date. **Ninth century.** Everything Freemasonry did with ciphers happened
eight hundred years after the method that defeats it was published.

---

### The European answer: Alberti

Leon Battista Alberti — architect, mathematician, author of the treatise on
painting that gave the Renaissance its perspective theory — wrote around 1467 a
short work on ciphers that solved the problem al-Kindi had created.

His device was a disc: two concentric rings bearing alphabets, rotatable against
each other. Set the rings, encipher some letters, **then move the rings and carry
on**.

The consequence is that the same plaintext letter is enciphered differently at
different points in the message. E is no longer always Q. **The frequencies are
smeared**, and frequency analysis no longer bites.

This is the birth of **polyalphabetic** cryptography, and it is a genuine
intellectual achievement. Alberti has been called the father of Western
cryptography with some justice.[^3]

**1467.** Two and a half centuries before the third degree.

---

### Trithemius and the concealment of concealment

Johannes Trithemius, Benedictine abbot of Sponheim, composed the *Steganographia*
in 1499, though it was not printed until 1606 and was placed on the Index.

Its status is one of the strangest in this book. On its face it is a work of angel
magic — invocations, spirit names, instructions for conveying messages by spiritual
means. In fact, substantial parts of it are a cryptographic manual: the
"invocations" conceal ciphertext, and the spirit names are keys.

**A magical text that is an encoded text**, and one whose encoding was subsequently
demonstrated by people who published the method and showed their working.

His *Polygraphia* of 1518 is a more straightforward cryptographic treatise and
includes the tabula recta — a square array of shifted alphabets — that Vigenère
would later build on.

---

### Vigenère

Blaise de Vigenère's *Traicté des chiffres* of 1586 gives the system that bears his
name, though it synthesises work by Alberti, Trithemius and Bellaso rather than
originating from nothing.

The method: take a keyword. Repeat it along the message. Use each letter of the
keyword to select which shifted alphabet enciphers the corresponding letter of the
plaintext.

The result is polyalphabetic and the key space is enormous — as large as the number
of possible keywords, which for a keyword of reasonable length is astronomically
large. And unlike Alberti's disc it requires no apparatus, only a table and a word
you can remember.

**It was believed unbreakable for nearly three centuries**, and acquired the name
*le chiffre indéchiffrable*. Kasiski published a general method of attack in 1863,
and Babbage had worked out something similar earlier without publishing.[^4]

**1586.** In print, in French, widely known among the educated, a hundred and thirty
years before the first Grand Lodge.

---

### What this means

Line the dates up.

- Frequency analysis: **9th century.**
- Polyalphabetic substitution: **1467.**
- Steganographic concealment: **1499.**
- Vigenère: **1586.**
- The first Grand Lodge: **1717 or 1721.**
- The third degree and the Lost Word: **the 1720s.**
- Masonic use of an unkeyed monoalphabetic grid: **eighteenth and nineteenth
  centuries.**

**The Craft adopted, in the 1700s, a system that had been obsolete for concealment
since the 1400s.**

This is not a criticism of Freemasonry. It is evidence about what Freemasonry was
doing, and the inference runs in one direction only.

A body concealing doctrine from posterity — from hostile churches, from suspicious
governments, from future scholars — had better options, in print, in the languages
its members read. Some of its members were Fellows of the Royal Society. The
circle that produced the 1723 *Constitutions* was thick with experimental
philosophers, and the idea that such men were unaware of polyalphabetic ciphers is
not sustainable.

**They did not use them because they did not need them.** The next chapter sets out
what they did use, and what it was for.

---

### The nomenclator, and what serious practice looked like

One more item, because it shows by contrast exactly what Masonic cipher use was
not.

From the fifteenth century onward, European diplomacy ran on the **nomenclator** —
a hybrid system combining a substitution alphabet with a codebook. Common words,
names of rulers, cities and ministers each received a symbol or a number, while the
rest of the message was enciphered letter by letter.

These documents are substantial. A working nomenclator might run to hundreds or
thousands of entries, and maintaining one was an administrative undertaking: copies
had to be produced, distributed securely, kept current, and replaced when
compromised.

**Consider what that implies.** A state using a nomenclator has a cipher secretary,
a distribution problem, a key-management procedure, and a replacement schedule. The
Papal and Venetian services employed professional cryptanalysts. Codebreaking was a
recognised office with a salary.

**This is what a body with something to hide actually does**, and the documentary
traces are everywhere: the codebooks survive, the correspondence about them
survives, the salaries are in the accounts.

**Now compare the Masonic record.** No codebook. No cipher secretary. No key
management, because there is no key. No correspondence about compromise. No
replacement when the system was published. Nothing in the minutes about any of it.

The pigpen grid required none of this because it protected nothing that required
it, and the complete absence of administrative traces is as strong a negative
finding as this subject offers. **Concealment at scale leaves paperwork**, and there
is none.

---

### The modern coda, briefly

Two further dates, because Chapter 49 will need them.

**1949.** Claude Shannon's "Communication Theory of Secrecy Systems" puts
cryptography on a mathematical footing and defines perfect secrecy — proving, among
other things, that the one-time pad is unbreakable and explaining precisely why.[^5]

**1976–77.** Diffie and Hellman, then Rivest, Shamir and Adleman: public-key
cryptography, which solves a problem every system in this chapter had to work
around — how two parties who have never met agree on a key.

The relevance is this. **Authentication becomes a mathematical discipline in the
twentieth century**, and when it does, it rediscovers the structure that the
Masonic recognition bundle had arrived at empirically: multiple independent
factors, challenge-response rather than static secrets, and a web of trust in which
known parties vouch for unknown ones.

Chapter 49 works that comparison properly and states, again, that it is an analogy
and not a lineage.

---

### NOTES

[^1]: Suetonius, *Divus Julius* 56. The report is brief and does not amount to a
technical description.

[^2]: Al-Kindi, *Risalah fi Istikhraj al-Mu'amma*. See Simon Singh, *The Code Book*
(London: Fourth Estate, 1999), and David Kahn, *The Codebreakers*, rev. edn (New
York: Scribner, 1996). **[PAGE VERIFICATION REQUIRED.]**

[^3]: Leon Battista Alberti, *De componendis cifris*, c. 1467.

[^4]: Blaise de Vigenère, *Traicté des chiffres, ou secretes manieres d'escrire*
(Paris, 1586). On Kasiski and Babbage see Kahn, *The Codebreakers*.

[^5]: Claude E. Shannon, "Communication Theory of Secrecy Systems," *Bell System
Technical Journal* 28 (1949): 656–715.
