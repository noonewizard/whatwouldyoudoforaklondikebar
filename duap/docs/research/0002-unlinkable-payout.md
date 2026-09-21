# 0002 — Can payout avoid reintroducing linkage?

**Status:** open · 2026-09-21

## Problem

A subject presents a different pseudonym to each controller, so no
controller and no pair of controllers can link them. To *pay* that subject
for usage recorded under many pseudonyms, some party must learn that those
pseudonyms belong to one person. In the reference design that party is the
settlement agent, and the exposure is stated in `PRIVACY.md`.

Can the linkage be avoided entirely, so that no party — including the
clearing node and the settlement agent — learns which pseudonyms are the
same person, while the person still gets paid the right total?

## Known work

Chaum's blind signatures (CACM 1985) for unlinkable tokens. Camenisch and
Lysyanskaya's anonymous credentials (EUROCRYPT 2001) for showing a
credential without linkability. Privacy Pass (Davidson et al., PoPETs
2018) as a deployed blind-token system. Cryptographic accumulators for
set membership without revealing the member.

## Hypothesis

A blind-signature scheme over per-pseudonym payout tokens would let a
subject accumulate value under each pseudonym, unblind the tokens, and
redeem the total at a settlement agent that cannot link the redemption to
the accruals.

## Experiment

**Not yet run.** What it would require: implement a blind-signature issuance
of value-bearing tokens by the clearing node, per accrual period; a
redemption protocol that proves the tokens' validity and sums them without
revealing which pseudonym each came from; and a double-spend register. Then
measure (a) whether the anonymity set is large enough in a realistic
deployment to be meaningful, and (b) the cost per accrual, which at the
frequency DUAP accrues may dominate the amounts involved.

## Result

None yet.

## Conclusion

None yet. The reference design accepts the exposure and states it.

## Remaining uncertainty

The hard part is probably not the cryptography but the anonymity set. If a
subject's accruals are distinctive in size and timing, unlinkability of the
*token* does not give unlinkability of the *payment*: a redemption of an
unusual amount at an unusual time is linkable by inspection. A scheme would
need denomination bucketing and delay, both of which cost the subject
something real. That trade has not been analysed.
