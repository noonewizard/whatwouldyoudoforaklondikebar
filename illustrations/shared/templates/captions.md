# DELIVERABLE 26 — CAPTION SYSTEM

## The four questions

Every caption answers all four. A caption that answers only the first is a label.

1. **What am I looking at?**
2. **Why is it here?**
3. **What source supports it?**
4. **What should the reader NOT infer from it?**

Question 4 is the one almost every published caption omits, and it is the reason
figures mislead. It is a **required field** in the register (`does_not_show`) and
CI fails without it.

## Templates

### Archival image
> **Figure N.N — [Object title].** [Creator], [date]. [Institution], [collection],
> [object ID]. [One sentence: why it is here.] [Modifications, if any.]

> Figure 1.14 — Title page of *The Constitutions of the Free-Masons*. James
> Anderson, London, 1723. [Institution, shelfmark.] Reproduced to show the
> work's self-presentation as a constitutional document rather than a history.
> Image cropped to the plate area; no other modification.

### Reconstruction
> **Figure N.N — [Subject], reconstruction.** Based on [sources]. [Which elements
> are documented and which inferred.] [What this reconstruction is not.]

> Figure 4.22 — The Temple of Solomon as reconstructed by Villalpando,
> 1596–1604. Engraving after the *In Ezechielem Explanationes*. **This is a
> reconstruction of Ezekiel's visionary temple, not of the building described in
> 1 Kings**, rendered by its author in the classical orders. It is reproduced as
> evidence of how Europe pictured the Temple, not as evidence of how the Temple
> looked.

### Map
> **Figure N.N — [Subject].** [Projection]. Sites shown by certainty: filled =
> excavated; open = identified; ring = disputed; circle of uncertainty = location
> approximate. [What is not plotted, and why.]

### Diagram / conceptual model
> **Figure N.N — [Title].** Analytical model, not a historical claim. [What it
> organizes.] [The evidence status of its inputs.]

### Data / computational figure
> **Figure N.N — [Title].** Computational result, reproducible from the script
> and seed cited. [Method, tolerance, and whether it was fixed in advance.]
> [Search space.] [What the result does not establish.]

### Comparative matrix
> **Figure N.N — [Comparison].** Similarity shown is **structural resemblance,
> not demonstrated transmission.** [What would be required to establish
> transmission.]

### AI-generated
> **Figure N.N — [Title].** **Artistic visualization — generated image, not a
> historical source.** [What it evokes.] [Tool, date.]

## Prohibited caption moves

| Prohibited | Why |
|---|---|
| "As can be seen…" | The figure shows; it does not prove |
| "This proves…" | No figure proves anything |
| "Reconstruction of the Temple" (unqualified) | Which Temple, from which text, by whom |
| Naming a date with no source | Every date in a caption is a claim |
| Letting a resemblance stand without comment | The series' core error, visualized |
| Omitting the projection on a map | The projection is an argument |
| Omitting *n* or the search space on a result | Deliverable 29 |

## Length

Captions run long in this series **deliberately**. A caption that constrains
interpretation is doing load-bearing work and is not padding. Typical: 40–90
words. Full-page plates may run to 150.
