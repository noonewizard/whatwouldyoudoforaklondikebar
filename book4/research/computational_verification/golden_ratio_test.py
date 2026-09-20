# -*- coding: utf-8 -*-
"""
GOLDEN RATIO CLAIM TEST (Book IV, ch.15/73)
Claim commonly made online: "Solomon's Temple encodes the golden ratio (phi
approx 1.618)."
Method fixed in advance: take every pairwise ratio of the EXPLICITLY STATED
textual dimensions (1 Kings 6-7), plus a control set of ratios from an
unrelated ancient building whose dimensions are also textually/archaeologically
fixed, and see how close ANY pairing comes to phi under a stated tolerance.
No cherry-picking of which two numbers to divide - ALL pairs are tested.
"""
import itertools

PHI = (1 + 5**0.5) / 2
TOL = 0.05  # 5% tolerance, STATED IN ADVANCE

def test_set(name, dims):
    print(f"=== {name} ===")
    print(f"  dimensions (cubits): {dims}")
    hits = []
    for a, b in itertools.permutations(dims, 2):
        if b == 0: continue
        r = a / b
        if abs(r - PHI) / PHI <= TOL:
            hits.append((a, b, r))
    print(f"  pairwise ratios tested: {len(list(itertools.permutations(dims,2)))}")
    if hits:
        for a, b, r in hits:
            print(f"  HIT: {a}/{b} = {r:.4f}  (phi = {PHI:.4f}, "
                  f"error {abs(r-PHI)/PHI:.2%})")
    else:
        print("  No pair within tolerance.")
    print()
    return hits

# Set 1: every dimension explicitly stated for the Temple house proper,
# porch, and Holy of Holies in 1 Kings 6 (cubits). Fixed BEFORE running.
temple_dims = [60, 20, 30,   # house L, W, H
               20, 10,       # porch L, W (depth uncertain, included anyway)
               20, 20, 20,   # Holy of Holies L, W, H (cube)
               5, 6, 7]      # side chamber height tiers (1 Kings 6:6, 6:10)

# Control set: dimensions of an unrelated structure with no golden-ratio
# claim attached to it in the literature -- a plain Iron Age four-room
# house footprint, values chosen to be structurally ordinary (not tuned).
control_dims = [11, 9, 4, 3, 7, 2, 13, 6]

h1 = test_set("Temple (1 Kings 6-7 stated dimensions)", temple_dims)
h2 = test_set("Control (ordinary Iron Age four-room house, illustrative figures)", control_dims)

print("=== INTERPRETATION ===")
print(f"Temple set: {len(h1)} hit(s) out of "
      f"{len(list(itertools.permutations(temple_dims,2)))} pairs tested.")
print(f"Control set: {len(h2)} hit(s) out of "
      f"{len(list(itertools.permutations(control_dims,2)))} pairs tested.")
print("""
With an 11-number set, ~110 ordered pairs are tested; a 5% tolerance band
around phi (1.538-1.699) is wide enough that SOME small-integer ratio will
often land inside it by chance alone, especially once duplicate values
(20,20,20) inflate the pair count without adding information. A single hit
in a large, un-pre-registered search space of ~100+ pairs is not a finding;
it is the expected behavior of the search. This is the chapter's worked
demonstration of Chapter 42/72's 'problem of retrofitting': the same method
applied to a control building with no golden-ratio mythology attached to it
will tend to produce comparable hits, which is exactly what a real control
is for.
""")
