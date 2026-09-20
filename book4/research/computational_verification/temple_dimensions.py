# -*- coding: utf-8 -*-
"""
TEMPLE DIMENSIONS — verification (Book IV)
Source: 1 Kings 6 (MT). Unit: the cubit (ammah), itself disputed — see notes.
This script does ONLY arithmetic on the stated textual numbers. It does not
assert a physical length for the cubit; that is a separate, disputed question
addressed in the chapter and Appendix D.
"""
from fractions import Fraction as F

# 1 Kings 6:2 — the house Solomon built for the LORD: length 60 cubits,
# breadth 20 cubits, height 30 cubits.
L, W, H = 60, 20, 30

print("=== RAW MT FIGURES (1 Kings 6:2), IN CUBITS ===")
print(f"  Length {L}  Width {W}  Height {H}")
print(f"  L:W  = {F(L,W)}  = {L/W:.3f}")
print(f"  L:H  = {F(L,H)}  = {L/H:.3f}")
print(f"  W:H  = {F(W,H)}  = {W/H:.3f}")

print()
print("=== THE PORCH / VESTIBULE (1 Kings 6:3) ===")
Lp, Wp = 20, 10  # porch: 20 cubits long (across the width of the house), 10 deep
print(f"  Porch: {Lp} x {Wp} cubits.  L:W = {F(Lp,Wp)} = {Lp/Wp}")
print("  NOTE: porch width (front-to-back depth) not given in MT; LXX/some")
print("  traditions differ. Treated as textually uncertain — see ch.11 notes.")

print()
print("=== THE HOLY OF HOLIES (1 Kings 6:20) ===")
d = 20
print(f"  A cube: {d} x {d} x {d} cubits.")
print(f"  Volume = {d**3} cubic cubits.")
print("  This IS a stated cube in the Hebrew text — not an inference.")
print("  Note it is exactly 1/3 of the house's length (20/60) and 2/3 of the")
print(f"  height is unused if the house is 30 high: {d}/{H} = {F(d,H)} -- i.e.")
print("  the Holy of Holies as built does NOT reach the full roof height on")
print("  the plain reading; rabbinic and modern commentators disagree on how")
print("  the vertical gap above it was used. See ch.3, ch.7.")

print()
print("=== THE MOLTEN SEA (1 Kings 7:23) ===")
diam, height, circ = 10, 5, 30
print(f"  Diameter {diam}, height {height}, 'a line of {circ} cubits did")
print("  compass it round about' (circumference).")
import math
true_circ = math.pi * diam
print(f"  True circumference for diameter {diam} (using math.pi): {true_circ:.4f}")
print(f"  Text's circumference: {circ}")
print(f"  Implied value of pi in the text: {circ/diam} = {F(circ,diam)}")
print(f"  Error vs actual pi: {(circ/diam - math.pi)/math.pi:.2%}")
print("""
  This is a genuinely famous textual crux. It does NOT require that ancient
  Israelites 'believed pi = 3' — a rim/brim is separately described
  (1 Kings 7:26, 'a handbreadth thick, brim like a cup, with flowers of
  lilies'), and if the 30-cubit line measured around the OUTSIDE of a rim
  while the 10-cubit diameter measured the inside (bore), both figures can
  be simultaneously true of the same object without any approximation of pi
  at all. This is a real proposed resolution in the scholarly and rabbinic
  literature (cf. the Talmudic discussion, Eruvin 14a, and later
  harmonizations), not a modern apologetic invention -- but it is NOT the
  only proposal, and this script does not adjudicate between them. It is
  reported here as a worked example of a 'numerical anomaly' that dissolves
  under a non-numerological explanation (rim vs. bore), which is exactly the
  kind of alternative-explanation test this book's evidence protocol
  requires before treating a numerical discrepancy as meaningful.
""")
