---
name: threat-modelling
description: The house procedure for adding to or revising the threat model. Use when adding a component, changing a trust boundary, or recording a red-team finding.
---

# Threat modelling

## The procedure

1. **Draw the boundary.** Name the component, what crosses into it, what
   crosses out, and who controls each side. A threat model without a
   boundary is a list of worries.

2. **Enumerate with STRIDE per interaction**, not per component:
   Spoofing, Tampering, Repudiation, Information disclosure, Denial of
   service, Elevation of privilege. Per-component enumeration misses the
   threats that live in the interaction.

3. **For each threat, record:**
   - identifier (`T-NN`), title, and the interaction it applies to;
   - the adversary: capability, position, motive;
   - the impact in protocol terms -- which accounting outcome changes;
   - the control, or the explicit acceptance;
   - **the test** that exercises the control.

4. **Classify the control honestly:**
   - *Prevented* -- the attack cannot succeed.
   - *Detected* -- it succeeds and is discovered; say by whom and how fast.
   - *Deterred* -- it succeeds, is discovered later, and has consequences.
   - *Accepted* -- it succeeds; name who accepted it and why.

   Most transparency-log properties are *detected*, not *prevented*. Writing
   "prevented" there is the commonest error in this field.

5. **Economic and governance threats are in scope.** A protocol whose
   cryptography is sound and whose incentives reward misreporting has not
   been threat modelled.

## Writing it up

`THREAT_MODEL.md` holds the enumeration, `SECURITY.md` the arguments, and
`security/findings.md` the open findings. A threat with no control, no
acceptance and no finding is an omission, and the red team treats it as one.
