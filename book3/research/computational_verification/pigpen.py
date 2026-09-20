# -*- coding: utf-8 -*-
"""
PIGPEN / 'MASONIC CIPHER' — structural analysis (Book III)
The historical questions are answered in research/cryptography.md; this file
establishes the cipher's mechanical properties, which bear on the claim that it
ever concealed doctrine.
"""
import collections, math
# Standard 4-grid arrangement (one of several attested layouts)
GRID1=[["A","B","C"],["D","E","F"],["G","H","I"]]      # plain cells
GRID2=[["J","K","L"],["M","N","O"],["P","Q","R"]]      # dotted cells
X1=["S","T","U","V"]                                    # plain X
X2=["W","X","Y","Z"]                                    # dotted X
order=[c for row in GRID1 for c in row]+[c for row in GRID2 for c in row]+X1+X2
print("Layout (this variant):", "".join(order))
print(f"Letters mapped: {len(order)}  (a simple monoalphabetic substitution)")
print()
print("=== KEY SPACE ===")
print("  Pigpen as used: ONE fixed public alphabet. Key space = 1.")
print("  It is not keyed. Everyone who knows the layout reads everything.")
print(f"  For comparison, an arbitrary monoalphabetic substitution: 26! = {math.factorial(26):.3e}")
print("  A cipher with a key space of 1 is a NOTATION, not an encryption system.")
print()
print("=== WHY IT CANNOT HAVE CONCEALED DOCTRINE ===")
print("""  1. Unkeyed. Security rests entirely on the layout being unknown, and the
     layout was printed in the nineteenth century and is now in puzzle books.
  2. Monoalphabetic. Preserves letter frequencies exactly, so it falls to the
     same frequency analysis that breaks any Caesar shift - a technique
     described by al-Kindi in the ninth century and routine in Europe by 1700.
  3. Preserves word division in every surviving Masonic use, which is the single
     largest concession a cipher clerk can make.
  4. Its documented applications are labels: gravestone inscriptions, minute-book
     headings, certificates, tokens. Short strings, ornamental placement.

  A body concealing doctrine from posterity does not choose a system breakable in
  an afternoon by a schoolchild with a frequency table. A body that wants its
  records to LOOK like a fraternity's records, and to be unreadable by a curious
  servant or a passing stranger, chooses exactly this.""")
print()
print("=== FREQUENCY PRESERVATION, DEMONSTRATED ===")
msg="THE LODGE MEETS ON THE FIRST TUESDAY OF EVERY MONTH"
sub={c:f"<{i}>" for i,c in enumerate(order)}
enc="".join(sub.get(c,c) for c in msg)
fp=collections.Counter(c for c in msg if c.isalpha())
print(f"  plaintext letter freq (top 5): {fp.most_common(5)}")
print("  ciphertext symbol freq is identical by construction - that is the flaw.")
