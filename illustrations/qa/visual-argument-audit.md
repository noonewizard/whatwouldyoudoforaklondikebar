# DELIVERABLE 28 — VISUAL ARGUMENT AUDIT

**Mandatory. Blocking. Every figure, no exceptions.**

A figure cannot reach `qa_status: passed` without a completed audit record.

## The governing question

> **Does this image make the author's argument appear stronger than the evidence
> actually permits?**

## The seven questions

Answered in writing, per figure, stored at `qa/audits/FIG-0N-NNN.md`.

| # | Question | Fail condition |
|---|---|---|
| 1 | Is the image descriptive, interpretive, reconstructive, or speculative? | Answer disagrees with `interpretation_status` |
| 2 | Does every element's visual weight match its evidential weight? | A conjectural element drawn as solidly as a documented one |
| 3 | Could the figure's visual authority mislead a reader who does not read the caption? | Yes, and the figure is not relabeled |
| 4 | Does the caption adequately constrain interpretation? | `does_not_show` is absent, vague, or not reflected in the caption |
| 5 | If a similarity is shown, is it distinguished from a demonstrated relationship? | Similarity presented as descent |
| 6 | If a pattern is shown, is the search space shown with it? | A hit without its denominator |
| 7 | Would removing this figure weaken the argument, or only the page? | Only the page → reclassify to priority C or D |

## The layer test

For any figure with mixed evidence levels, the auditor **toggles off the inferred
and conjectural layers** (Illustrator layers 04 and 05, or the Figma evidence
variant) and looks at what remains.

> **If what remains does not support the caption's claim, the figure is
> misrepresenting its evidence.**

This is the most useful single check in the system and it takes ten seconds.

## The three specific traps in this series

Each has a dedicated check because each is a documented failure mode in the
subject matter the books analyze.

**1. The reconstruction trap.** Villalpando's engravings are the case study: a
drawing supplies what the text omits, and the supplied parts become invisible.
*Check:* is every element not stated in the source drawn in a distinct stroke?

**2. The resemblance trap.** Two symbols side by side imply descent whether or not
the caption says so. Books I, II and V are full of comparanda.
*Check:* does the figure include the **distance** — chronological gap, absence of
a transmission channel — as a visible element, not merely a caption note?

**3. The pattern trap.** A figure showing a found pattern without its control
reproduces the error the books diagnose.
*Check:* is the control in the same frame, at the same scale? If the control is
on another page, the reader will not make the comparison.

## Record format

```markdown
# VISUAL ARGUMENT AUDIT — FIG-04-022
Auditor:  Date:  Revision audited:

1. Register  : reconstructive  |  Audit finds: reconstructive  |  AGREE
2. Weight    : PASS — conjectural roof dashed, documented walls solid
3. Authority : RISK — reads as an architectural record at a glance
                → mitigated: "reconstruction" moved to the caption's first line
4. Caption   : PASS — names the source text and the reconstructor
5. Similarity: NOT APPLICABLE
6. Search    : NOT APPLICABLE
7. Necessity : PASS — priority A; ch. 67's argument depends on it

VERDICT: PASS WITH MITIGATION (item 3)
```
