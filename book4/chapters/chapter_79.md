# CHAPTER SEVENTY-NINE
## The New World Temple

The claim that Washington, D.C. was laid out as a Masonic or Solomonic figure —
that its avenues form a pentagram, a square and compasses, or a Seal of Solomon —
is the most widely circulated piece of Temple conspiracy material in the world.

This chapter tests it computationally, reports a prediction that only partly held,
and then explains why the whole genre of claim is unanswerable in the form it is
usually made.

---

### The documented facts

**The plan.** Pierre Charles L'Enfant, a French-born engineer who had served in the
Continental Army, prepared a plan for the federal city in 1791. He was dismissed in
1792. Andrew Ellicott revised and published the plan, assisted in the survey by
Benjamin Banneker.

**The Masonic element that is real.** George Washington was a Freemason, initiated
in Fredericksburg in 1752. He laid the cornerstone of the Capitol on 18 September
1793 in a Masonic ceremony, wearing a Masonic apron.

That ceremony was **public**. It was announced, attended, processed through the
streets, and reported in the newspapers. This is the single most important fact in
the chapter, and it is routinely presented as though it were a discovery. A
conspiracy whose central ritual was performed in the open with press coverage is a
conspiracy that has misunderstood its own genre.

**The Masonic element that is not evidenced.** There is no documentary evidence
that L'Enfant was a Freemason. The claim is asserted constantly and, so far as this
book has been able to establish, rests on nothing. [SOURCE VERIFICATION REQUIRED
for any assertion of L'Enfant's membership; the burden lies with whoever makes it.]

---

### Two claims that fail on dates and geography

Before any computation, two of the popular figures collapse on documentary
grounds.

**The pentagram** said to be formed by Connecticut, Massachusetts, Rhode Island and
Vermont Avenues around the White House is incomplete. One of its lines is not
there. Proponents supply it, and the supplying is the claim.

**Figures involving the Jefferson Memorial** cannot be in L'Enfant's plan, because
the Jefferson Memorial was built between 1939 and 1943 — a century and a half
later. The Washington Monument, similarly, does not stand where L'Enfant intended
it, having been moved because the intended site was unsuitable ground.

A figure that requires a twentieth-century building and a relocated monument is not
an eighteenth-century design. This is not a subtle objection.

---

### The computation

The remaining question is whether the fits that *are* achievable are meaningful.
This book tested it directly, on point sets constructed to contain nothing.

**Method, fixed before running.** Generate a jittered rectangular lattice — a plain
street grid with nothing in it by construction. Search random hexagram placements:
centre, radius, and rotation chosen at random, with each of the six vertices
required to match a distinct lattice point within a fixed capture radius. Report
the best fit found.

**The single-run result.** A 14 × 10 jittered lattice, 140 points, 400,000
placements tried: best hexagram found with a maximum vertex deviation of **0.1848
units — 8.79 per cent of its own circumradius.** At any normal map scale, that
deviation is narrower than the width of the drawn line.

A street grid containing nothing yields a Seal of Solomon to roughly the tolerance
this literature reports for Washington.

**The systematic run.** One demonstration is an anecdote, so the test was extended
across point-set sizes and three independent seeds each: lattices of 30, 70, 140
and 280 points, plus a uniform random cloud of 140 points with no grid structure
at all.

The prediction made in advance was that fit quality would improve with point
density.

---

### Where the prediction failed, reported

It only partly held, and the failure is more useful than a confirmation would have
been.

**Absolute deviation was essentially flat.** Across every configuration — 30 points
or 280, lattice or random cloud — the best fit landed between roughly 0.18 and 0.25
units. Density barely mattered. What set the figure was the capture radius and the
jitter: the resolution of the search, not the properties of the target.

**The percentage figure moved, but not monotonically.** The 70-point lattice scored
*worse* in percentage terms than the 30-point one, which the prediction did not
allow for.

The reason is the finding that replaces the prediction, and it is sharper than the
prediction was.

**The percentage has a free denominator.** "The figure fits to within three per
cent" is not a measurement of fit. It is a measurement of fit divided by a radius
that the analyst's own search selected. A search that happens to settle on a larger
figure reports a smaller percentage for exactly the same absolute sloppiness.
Fitting a bigger star to the same city makes the same error look more precise.

And a uniform random cloud with no structure whatever performed as well as the
lattice of equal size.

---

### What an honest version of the claim would report

Four numbers, none of which this book has ever seen reported in the genre:

**The number of candidate points** the analyst allowed themselves. Washington has
hundreds of intersections, circles, and monuments. A figure fitted to six of them,
chosen from four hundred, is a selection from an enormous space.

**The tolerance, fixed in advance.** How far may a line miss a point and still
count?

**The number of figures searched for.** Pentagram, hexagram, square, compasses,
owl, and any other shape looked at and rejected — all of them count toward the
multiple-comparisons burden, whether or not they are mentioned.

**The control.** What does the identical procedure produce on a city of equal
density with no claim attached?

Report those four and the claims evaporate. Omit them — which is universal — and
any result at all can be presented as remarkable.

---

### The mundane explanation, which is also the true one

L'Enfant's plan is a **Baroque city plan**: long diagonal avenues superimposed on a
rectilinear grid, in the manner of the European planning tradition he came from.
The diagonals exist for sightlines, for vistas terminating on important buildings,
and for practical movement across a grid that would otherwise require
right-angled detours.

Superimposing diagonals on a grid necessarily produces triangles, stars, and
polygons at the intersections. It is a geometric consequence of the method, not a
concealed intention within it. Any Baroque plan will do the same, and the pattern
hunters have found the same figures in Paris, in Rome, and in a great many other
cities.

The plan contains stars because that is what happens when you draw diagonals across
squares. Everything else has been supplied by people looking.
