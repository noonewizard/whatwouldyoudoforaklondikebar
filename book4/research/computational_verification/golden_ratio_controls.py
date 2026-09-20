# -*- coding: utf-8 -*-
"""
GOLDEN RATIO CLAIM — SYSTEMATIC CONTROL RUN (Book IV, ch.73)

Closes the gap declared in README.md ("STILL TO RUN BEFORE DRAFTING: a
systematic (not illustrative) run of the golden-ratio test against 5-10
additional control buildings, to report a distribution rather than one
control").

The single control in golden_ratio_test.py showed that ONE arbitrary building
reproduces the Temple's hit rate. That is suggestive but anecdotal. This script
replaces the anecdote with a null distribution:

  (A) 10 named comparison buildings with independently-fixed dimensions,
      declared in advance, none of which carries a golden-ratio claim in the
      literature that this book relies on.
  (B) 20,000 synthetic buildings matched to the Temple set on cardinality and
      on the range of its values, giving an empirical p-value for the Temple's
      observed hit count.

METHOD FIXED IN ADVANCE (identical to golden_ratio_test.py, unchanged):
  - ALL ordered pairwise ratios of a dimension list are tested. No pair is
    chosen by the analyst.
  - Tolerance: 5% relative error around phi. Stated before the run.
  - The statistic is the NUMBER OF HITS per building.

NOTE ON THE COMPARISON BUILDINGS: several are given as approximate figures
rounded to whole cubits/metres from standard reference descriptions. They are
used here as STRUCTURALLY ORDINARY NUMBER SETS for control purposes, not as
precise metrological claims about those buildings. The test is about what the
METHOD does to a list of plausible building numbers; it is not an argument
about any of these structures. [SVR: precise dimensions of the comparison
buildings are not load-bearing for this result and are not asserted as exact.]
"""
import itertools, random, statistics

PHI = (1 + 5**0.5) / 2
TOL = 0.05
SEED = 20260920
random.seed(SEED)

def hits(dims):
    out = []
    for a, b in itertools.permutations(dims, 2):
        if b == 0:
            continue
        r = a / b
        if abs(r - PHI) / PHI <= TOL:
            out.append((a, b, r))
    return out

def npairs(dims):
    return len(list(itertools.permutations(dims, 2)))

# ---------------------------------------------------------------- TARGET
temple = [60, 20, 30, 20, 10, 20, 20, 20, 5, 6, 7]

# ------------------------------------------------- (A) NAMED COMPARISONS
# Declared before running. Each is an 11-value list to match the Temple's
# cardinality exactly, so hit counts are directly comparable.
comparisons = {
    "Iron Age four-room house (illustrative)":
        [11, 9, 4, 3, 7, 2, 13, 6, 8, 5, 10],
    "'Ain Dara temple (approx., whole units)":
        [30, 20, 10, 6, 4, 32, 16, 8, 3, 5, 12],
    "Tabernacle (Exodus 26-27, cubits)":
        [30, 10, 10, 20, 10, 100, 50, 5, 3, 1, 2],
    "Ezekiel's temple, selected (Ezek 40-41, cubits)":
        [100, 50, 60, 20, 40, 25, 6, 5, 4, 2, 11],
    "Noah's ark (Genesis 6, cubits)":
        [300, 50, 30, 1, 3, 2, 15, 10, 6, 20, 5],
    "Parthenon (approx. metres, whole)":
        [70, 31, 14, 8, 2, 17, 10, 4, 6, 23, 13],
    "Pantheon Rome (approx. metres, whole)":
        [43, 43, 22, 9, 8, 5, 30, 12, 6, 4, 2],
    "Stonehenge sarsen circle (approx. metres)":
        [33, 30, 4, 7, 2, 1, 16, 5, 3, 11, 6],
    "Typical Gothic bay (approx. metres)":
        [12, 6, 30, 4, 3, 9, 2, 5, 15, 8, 20],
    "Arbitrary small integers (no building)":
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
}

# ---------------------------------------------- (B) SYNTHETIC NULL
# Matched on cardinality (11) and on the Temple's value range (5..60).
LO, HI, N = min(temple), max(temple), 20000
null_counts = []
for _ in range(N):
    dims = [random.randint(LO, HI) for _ in range(len(temple))]
    null_counts.append(len(hits(dims)))

# ---------------------------------------------------------------- REPORT
t_hits = hits(temple)
print("=" * 70)
print("GOLDEN RATIO TEST — SYSTEMATIC CONTROLS")
print(f"phi = {PHI:.6f}   tolerance = {TOL:.0%}   seed = {SEED}")
print("=" * 70)

print(f"\nTARGET — Solomon's Temple (1 Kings 6-7 stated dimensions)")
print(f"  values: {temple}")
print(f"  ordered pairs tested: {npairs(temple)}")
print(f"  HITS: {len(t_hits)}")
for a, b, r in t_hits:
    print(f"    {a}/{b} = {r:.4f}  (error {abs(r-PHI)/PHI:.2%})")

print("\n(A) NAMED COMPARISON BUILDINGS (11 values each, same method)")
print(f"  {'set':<48} {'pairs':>6} {'hits':>5}")
comp_counts = []
for name, dims in comparisons.items():
    h = hits(dims)
    comp_counts.append(len(h))
    print(f"  {name:<48} {npairs(dims):>6} {len(h):>5}")
print(f"\n  comparison mean hits: {statistics.mean(comp_counts):.2f}"
      f"   median: {statistics.median(comp_counts):.1f}"
      f"   range: {min(comp_counts)}-{max(comp_counts)}")
print(f"  Temple hits: {len(t_hits)}")
n_ge = sum(1 for c in comp_counts if c >= len(t_hits))
print(f"  comparison sets scoring >= the Temple: {n_ge}/{len(comp_counts)}")

print(f"\n(B) SYNTHETIC NULL — {N:,} random 11-value sets drawn from "
      f"[{LO},{HI}]")
print(f"  mean hits: {statistics.mean(null_counts):.3f}")
print(f"  median hits: {statistics.median(null_counts):.1f}")
print(f"  proportion with >= 1 hit: "
      f"{sum(1 for c in null_counts if c >= 1)/N:.4f}")
p = sum(1 for c in null_counts if c >= len(t_hits)) / N
print(f"  proportion with >= {len(t_hits)} hit(s) (the Temple's count): {p:.4f}")
dist = {}
for c in null_counts:
    dist[c] = dist.get(c, 0) + 1
print("  hit-count distribution:")
for k in sorted(dist):
    print(f"    {k:>2} hit(s): {dist[k]:>6} ({dist[k]/N:6.2%})")

print("\n" + "=" * 70)
print("VERDICT")
print("=" * 70)
print(f"""
The Temple's stated dimensions produce {len(t_hits)} ratio within 5% of phi out
of {npairs(temple)} ordered pairs tested.

A random 11-value integer set drawn from the same range produces at least that
many {p:.1%} of the time. The method's own null distribution therefore
reproduces the Temple's result in roughly {round(p*100)} runs out of 100.

This is not evidence that the Temple encodes phi. It is a measurement of what
the METHOD does: search enough pairs of plausible building numbers under a 5%
band and a hit is the ordinary outcome, not the exceptional one.

Stated as a rule for the chapter: a positive result that a matched control
reproduces at a comparable rate is not a positive result. It is a description
of the search.
""")

# ---------------------------------------------------------------- DIAGNOSTIC
# Why does the Temple UNDERPERFORM the controls? Added after seeing the result,
# and flagged as post-hoc: this is an explanation of an observed number, not a
# prediction that was made in advance.
print("=" * 70)
print("POST-HOC DIAGNOSTIC (flagged as post-hoc, not pre-registered)")
print("=" * 70)

def distinct_ratios(dims):
    return len({round(a / b, 6) for a, b in itertools.permutations(dims, 2) if b})

print(f"  Temple: {len(temple)} values, "
      f"{len(set(temple))} distinct, "
      f"{distinct_ratios(temple)} distinct ordered ratios out of "
      f"{npairs(temple)} pairs")
for name, dims in comparisons.items():
    print(f"  {name[:44]:<46} {len(set(dims)):>2} distinct values, "
          f"{distinct_ratios(dims):>3} distinct ratios")

print(f"""
The Temple's dimension list contains {len(temple)} values but only
{len(set(temple))} distinct ones -- 20 occurs four times (house width, Holy of
Holies length, width and height) and the remainder are {sorted(set(temple))}.
Its {npairs(temple)} ordered pairs therefore collapse to only
{distinct_ratios(temple)} distinct ratios, and those ratios are dominated by
clean small-integer values (3, 2, 6, 1.5, 0.5, ...) which sit far from phi.

A random integer set of the same size has many more distinct values and
therefore many more distinct ratios, giving phi more chances to be approached.

So the Temple scores BELOW the controls for a reason that is itself the
chapter's point: the building's stated dimensions are simple and
small-integer-related, which is the OPPOSITE of what an irrational-ratio design
would produce. The numbers are not merely non-phi. They are the kind of numbers
that make phi hard to hit.
""")
