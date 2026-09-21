# 0005 — Does pricing data usage increase collection?

**Status:** open, unstudied · 2026-09-21

## Problem

DUAP's premise is that making data usage measurable and priced improves
the position of data subjects. There is a plausible mechanism by which it
does the opposite.

Today, collecting and using personal data carries an *unquantified*
reputational and regulatory risk. Unquantified risks are hard to budget
for and are often handled by avoidance. Converting that into a *priced,
budgetable input* may make it easier to justify: a line item with a known
cost is a normal business decision, where an unbounded reputational
exposure is an argument-stopper.

This is the moral-licensing or crowding-out effect familiar from other
domains.

## Known work

Gneezy and Rustichini's day-care study (*A Fine Is a Price*, Journal of
Legal Studies, 2000) is the canonical example: introducing a fine for late
pickup *increased* lateness, because it converted a social obligation into
a purchasable service. The carbon-offset literature contains an extended
argument about whether pricing emissions licenses them. Titmuss's work on
blood donation is the classic case of payment reducing supply of a
pro-social behaviour.

We are not aware of a study of this effect in data collection specifically,
which is itself notable given how often data pricing is proposed.

## Hypothesis

Two effects run in opposite directions, and which dominates is an
empirical question:

1. **Restraint.** A priced input is one that a budget owner will try to
   reduce, and the protocol makes the reduction measurable.
2. **Licensing.** A priced input is one that a budget owner can justify
   buying more of, and the price removes the argument that the use is
   illegitimate.

## Experiment

**Not run, and not straightforwardly runnable.** A useful design would be a
controlled study of procurement behaviour under priced and unpriced
regimes, which requires participants who do not exist yet. A weaker but
feasible substitute is an agent-based simulation in `simulations/`,
calibrated to observed procurement elasticities in an adjacent priced
input, with the elasticity as an explicit and acknowledged assumption.

## Result

None.

## Conclusion

None. This is recorded because it is the most uncomfortable open question
about the protocol's premise, and a repository that omits it is making a
claim by silence.

## Remaining uncertainty

Almost all of it. What would change the answer: evidence from any adjacent
domain where an unpriced externality became priced and the quantity moved
in a measurable direction; or a natural experiment in a jurisdiction that
mandates data-use accounting.
