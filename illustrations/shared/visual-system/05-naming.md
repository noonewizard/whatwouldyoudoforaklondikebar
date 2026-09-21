# DELIVERABLE 5 — NAMING CONVENTION

## Figure ID

```
FIG-<book>-<serial>
FIG-04-037
```

- `book` — zero-padded 01–06
- `serial` — zero-padded, **assigned in order of creation, never renumbered**

**Serials are permanent and non-sequential in the book.** A figure cut from the
manuscript keeps its ID and is marked `status: withdrawn`; the ID is never
reissued. This is what makes the register a durable record rather than a
regenerated list.

Printed figure numbers (*Figure 4.12*) are produced by Quarto/Typst at render and
have **no relationship** to the serial. The author never types a figure number.

## Quarto label

```
@fig-<book-slug>-<kebab-topic>
@fig-temple-orientation
@fig-hiram-chronology
@fig-atbash-controls
```

Stable, human-readable, and independent of both the serial and the printed
number. Renaming a label is a breaking change and is caught by CI.

## Filenames

```
FIG-04-037/
  source/   villalpando-1604-plate-ii_bnf_[shelfmark].tif
  working/  FIG-04-037_temple-orientation_v03.ai
  data/     temple-dimensions_1kings.csv
  scripts/  build.py
  exports/  FIG-04-037_temple-orientation_print.pdf
            FIG-04-037_temple-orientation_web.svg
            FIG-04-037_temple-orientation_web.png
```

Rules: lowercase kebab-case after the ID; `_v01` version suffix on working files
only; export target (`print` / `web` / `epub`) always last before the extension.

## Figma layer & component naming

```
FIG/Architecture/Plan/Temple-Orientation
FIG/Cipher/Transformation/Atbash-Worked
FIG/Evidence/Badge/Speculative
```

## Git branch

```
fig/FIG-04-037-temple-orientation
```
