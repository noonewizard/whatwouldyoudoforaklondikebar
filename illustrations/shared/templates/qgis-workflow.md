# DELIVERABLE 10 & 33 — QGIS WORKFLOW

## Project template — `ringler-historical.qgz`

**CRS policy.** Store all data in EPSG:4326. Project to a locally appropriate
equal-area or conformal projection **per map**, and **name the projection in the
caption**. A map that does not name its projection fails QA — projection choice
is an argument about what the map is for.

## Layer stack

```
70  Annotation & labels
60  Evidence overlay        ← attested / inferred / conjectural extents
50  Sites — DOCUMENTED      ← excavated, published
40  Sites — ATTESTED-TEXT   ← named in a source, location uncertain
30  Sites — CONJECTURAL     ← proposed identification only
20  Routes                  ← by evidence class
10  Historical base         ← georeferenced historical map, if any
00  Modern reference        ← minimal; often suppressed entirely
```

## The uncertainty rules

These are the point of using GIS for historical geography at all.

**1. A point asserts precision. Most historical locations do not have it.**
Sites are rendered as:

| Certainty | Symbol |
|---|---|
| Excavated, published | Filled circle, 2.5 mm |
| Identified, uncontested | Open circle, 2.5 mm |
| Identified, disputed | Open circle with a second concentric ring |
| **Location uncertain** | **Circle of uncertainty, radius = the actual uncertainty**, no center dot |
| Named in a source only, unlocated | Listed in the margin, **not plotted** |

**A site whose location is unknown is never given a dot.** This is the single most
common failure in historical cartography and the easiest to avoid.

**2. Borders and extents.** Political and cultural extents get a graded
buffer, not a line, unless a treaty boundary is actually documented.

**3. Transmission maps get no arrows unless the route is attested.** An arrow
from Alexandria to Florence asserts a path. Where only the endpoints are known,
the figure shows two marked points and a dotted link labelled *route not
attested*.

**4. Anachronistic coastlines.** Where the shoreline has moved materially since
the period mapped, either use a reconstructed coastline and cite it, or state in
the caption that the modern coastline is shown.

## Styling

QML style files in `shared/templates/qgis/` bind symbology to an `evidence`
attribute, exactly as the Figma library does. Print composer templates: single
column, full measure, full page, spread.

## Export

`Project → Layout → Export as PDF`, vector, 300 dpi rasterization fallback, text
as text (never outlines — Typst embeds the fonts).
