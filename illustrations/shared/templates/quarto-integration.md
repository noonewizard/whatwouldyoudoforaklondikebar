# DELIVERABLE 12 & 35 — QUARTO INTEGRATION

## The rule

**The author never types a figure number.** Every reference is a label. Quarto
resolves numbering at render; changing a figure's position renumbers everything
automatically and breaks nothing.

```markdown
Wrong:  As shown in Figure 37, the axis is reversed.
Right:  As shown in @fig-temple-orientation, the axis is reversed.
```

CI rejects any manually written "Figure N" that is followed by a digit in prose.

## Standard figure block

```markdown
![Lodge orientation against Temple orientation. Analytical diagram, not a
historical claim about any specific building. Drawn from the textual sources
named; dashed elements are not specified in those
sources.](../illustrations/book-04/FIG-04-012/exports/FIG-04-012_temple-orientation_print.pdf){
  #fig-temple-orientation
  fig-alt="Two plans at the same scale. Left, the Temple: entry east, sanctuary west. Right, a lodge: Master east, entry west. Arrows show the sacred pole pointing in opposite directions."
  width=100%
}
```

## Evidence badge via a shortcode

Defined once in `_extensions/ringler/evidence/`:

```markdown
{{< evidence speculative >}}
```

renders the badge, the stroke key, and the status word in the evidence gutter,
so the encoding cannot drift between figures.

## Per-format source selection

```yaml
# _quarto.yml
format:
  typst:
    keep-typ: true
    include-in-header: illustrations/shared/styles/figures.typ
  html:
    fig-format: svg
  epub:
    fig-format: png
    fig-dpi: 300

crossref:
  fig-title: Figure
  fig-prefix: Figure
  labels: arabic
  subref-labels: alpha a

filters:
  - ringler-evidence      # injects the evidence gutter
  - ringler-figcheck      # fails the render on an unregistered label
```

## The `ringler-figcheck` filter

A Lua filter that, for every `Image` with an identifier:

1. looks the label up in `register/illustrations.yml`;
2. **fails the render** if it is absent — no figure appears in the book that is
   not in the register;
3. fails if `register.status` is not `approved`;
4. fails if `caption` in the manuscript diverges from `caption` in the register;
5. warns if the figure is referenced in prose but placed in a different chapter
   than `register.chapter`.

This closes the gap that otherwise makes registers rot: a figure can no longer be
dropped into the book without a provenance record.

## Long descriptions

```markdown
::: {.figure-longdesc #longdesc-temple-orientation}
The plan is a long rectangle divided into three sequential zones…
:::
```

HTML/EPUB render it as a `<details>` block linked from the figure via
`aria-describedby`; Typst routes it to the back matter.
