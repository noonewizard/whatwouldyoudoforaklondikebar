# MASONIC CIPHER HISTORY

Per §X and §XII. The governing rule: **do not attribute a cipher to Freemasonry
without evidence.**

---

## WHAT MASONRY DEMONSTRABLY USED

**The grid cipher ("pigpen", "Freemason's cipher", "Rosicrucian cipher").**
A monoalphabetic substitution in which letters are placed in two tic-tac-toe grids
and two X-shapes, each letter represented by the shape of the cell enclosing it,
with dots distinguishing the second set.

*Documented Masonic use:* eighteenth- and nineteenth-century lodge records,
certificates, jewels, and gravestone inscriptions — chiefly in Britain and
America. **[SVR — specific documented examples with dates, locations and holding
institutions. This is a Tier-1 research gap and must be discharged before
publication; the claim that Masons used it is not in doubt, but the book must cite
objects, not assertions.]**

*Properties* (computed in `research/computational_verification/pigpen.py`):
- **Key space: 1.** The cipher is not keyed. The layout *is* the secret, and the
  layout was published.
- Monoalphabetic: letter frequencies pass through unchanged.
- Word division preserved in surviving Masonic use.
- Breakable by frequency analysis — a technique described by al-Kindi in the ninth
  century and standard in European practice by 1700.

*What follows.* A body concealing doctrine from posterity does not select a system
that a schoolchild with a frequency table breaks in an afternoon, **when Alberti
(1467) and Vigenère (1586) were in print and polyalphabetic ciphers were known to
any educated person.** A body that wants its minute book unreadable by a curious
servant, and its monuments to declare fraternal membership legibly to brethren and
opaquely to strangers, chooses exactly this. The cipher's weakness is evidence
about its purpose.

*On the name.* "The Masonic cipher" is a modern convention. The grid is also called
the Rosicrucian cipher, it was used by the Union Army in the American Civil War,
and antecedent claims reaching back to Templar or earlier use are weak. **Per §XII,
this book does not call it "the Masonic cipher" without qualifying that the
designation is conventional and the cipher is not exclusively Masonic.**

---

## WHAT MASONRY DID NOT USE

Each of the following is proposed somewhere in the popular literature. None has
Masonic ritual or documentary support.

| System | Status in Masonry | Level |
|---|---|---|
| Atbash | **No Masonic use attested.** Biblical and Jewish-exegetical | E6 |
| Hebrew gematria on Masonic words | Not a Masonic practice | E6 |
| English gematria | Modern; fails base-rate test | E6 |
| Notarikon / temurah | Jewish exegesis; no Masonic route | E6 |
| Vigenère | Known in Europe; no Masonic use attested | E6 |
| Steganographic concealment in ritual text | No evidence | E6 |
| Acrostics in the Constitutions | Searched; nothing surviving Test 4 | E6 |
| "Letter and halve it" as an encoding rule | A delivery convention; see `words/letter_and_halve_it.md` | E6 as cipher |

---

## THE REAL HISTORY OF EUROPEAN CRYPTOGRAPHY (ch 25)

Needed so the reader can see what was actually available.

- **c. 600 BCE — Atbash**, Jer 25:26, 51:41, 51:1. Hebrew, scriptural, literary.
- **c. 400 BCE — skytale**, Spartan transposition (disputed).
- **c. 50 BCE — Caesar shift**, reported by Suetonius. Key space 25.
- **9th c. — al-Kindi**, frequency analysis. *Every monoalphabetic cipher becomes
  breakable from this point.*
- **1467 — Alberti**, polyalphabetic cipher and cipher disk.
- **1499/1606 — Trithemius**, *Steganographia* — concealment of the existence of a
  message, not merely its content.
- **1586 — Vigenère**, *Traicté des chiffres*. Believed unbreakable for centuries.
- **1863 — Kasiski** breaks Vigenère.
- **1949 — Shannon** formalises secrecy; perfect secrecy defined.
- **1976 — Diffie–Hellman**; **1977 — RSA**. Authentication becomes mathematics.

**The point for the book:** by the 1720s, when the Lost Word enters the record,
European cryptography had four hundred years of development behind it and
Freemasonry used none of it. **This is a negative finding and a strong one.** The
Craft's secrecy was oral and performative, not cryptographic — which is precisely
what Book II concluded from the other direction.

---

## THE STEGANOGRAPHY CAUTION

Trithemius is the one place where a genuine case can be argued for something
subtler, and it should be argued carefully. *Steganographia* presents itself as a
work of angel magic and is in substantial part a cryptographic manual in
disguise — the "angelic" invocations conceal ciphertext. This is a documented case
of a magical text that *is* an encoded text.

It is therefore the best available precedent for the claim that Masonic ritual
conceals something. It is also **a century and a half earlier, in Latin, in a
monastic context, with no route to the Craft degrees** — and, decisively, Trithemius
was *discovered* to be a cryptographer because the key was published and the method
reproduced. That is what a demonstrated concealment looks like, and no comparable
demonstration exists for Masonry.
