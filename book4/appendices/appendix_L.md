# APPENDIX L
## Computational Verification

Every geometric, numerical, and cryptographic claim in this book was produced by a
script, and the scripts are published with the book's research materials at
`research/computational_verification/`. Nothing is asserted that was not computed.

| Script | Establishes | Result |
|---|---|---|
| `temple_dimensions.py` | Raw Masoretic ratios; the Holy-of-Holies cube; the molten sea π crux and its rim/bore resolution | Verified |
| `golden_ratio_test.py` | φ tested against all pairwise ratios of the stated dimensions, with one matched control | **The control reproduces the hit rate** |
| `golden_ratio_controls.py` | The same method against 10 named comparison sets and a 20,000-run synthetic null | **The Temple scores below all ten; 95.66% of random sets match or beat it** |
| `temple_gematria.py` | Standard gematria and Atbash on 18 terms against a fixed 62-word independent lexicon | **0/18, as the base rate predicts.** One input-sanitisation bug caught and fixed on first run |
| `dc_geometry_test.py` | Hexagram-fitting control on a jittered street lattice | **8.79% of circumradius on a lattice containing nothing** |
| `dc_geometry_controls.py` | The same across point-set sizes, densities, and a structureless random cloud | **Absolute deviation flat; the percentage figure has a free denominator** |

### Why the code is published

This book's argument is that confident presentation is not evidence. A book making
that argument has to supply something checkable, and the computations are the only
part of it that can be independently confirmed without a library.

Each script runs in seconds and has no dependencies beyond the Python standard
library. A reader who disagrees with a result can change a parameter and see what
happens — which is the point.

### The bug, reported rather than corrected silently

`temple_gematria.py` crashed on its first run. The Hebrew for Holy of Holies is
written as two words with a space; the gematria function filtered its input, and
the Atbash and transliteration functions did not. Both raised a key error.

The functions were patched to filter identically and the run repeated in full. The
fault is recorded in the script's own comments and in Chapter 74, because a routine
that does not filter its input will fail loudly on some inputs and silently on
others — and this one failed loudly, which was luck.

### The declared gap that was closed

The research README originally declared an open gap: the golden-ratio test had one
illustrative control rather than a distribution. That gap was closed before
drafting Chapter 73, by `golden_ratio_controls.py`, and the README records the
closure rather than the original declaration.

### The prediction that only partly held

`dc_geometry_controls.py` was run against a prediction, made before the run, that
fit quality would improve with point density. It was only partly borne out —
absolute deviation was flat across every configuration, and the percentage figure
moved non-monotonically. Chapter 79 reports this as a partial failure and explains
what replaced the prediction, because a book that hid it would forfeit the standing
to criticise anyone else's reporting.
