// DELIVERABLE 13 & 36 — TYPST FIGURE SYSTEM
// Included by Quarto via include-in-header.
// Numbering, placement, and the evidence gutter are defined once, here.

#let ink-primary   = rgb("#1A1917")
#let ink-secondary = rgb("#55524C")
#let ink-muted     = rgb("#8A857C")
#let paper         = rgb("#FAF8F4")

// Validated categorical palette (print). Fixed order; never cycled.
#let slot = (rgb("#3A5FA8"), rgb("#C08420"), rgb("#0E8C72"), rgb("#A83A33"))

// ---------------------------------------------------------------- evidence
#let evidence-key = (
  documented:  (badge: "■", word: "DOCUMENTED"),
  supported:   (badge: "▣", word: "SUPPORTED"),
  disputed:    (badge: "⧗", word: "DISPUTED"),
  speculative: (badge: "◌", word: "SPECULATIVE"),
  unsupported: (badge: "✕", word: "UNSUPPORTED"),
  conceptual:  (badge: "◇", word: "ANALYTICAL MODEL"),
)

#let evidence-gutter(status, source, fid) = {
  let e = evidence-key.at(status)
  block(width: 100%, inset: (top: 3pt), stroke: (top: 0.4pt + ink-muted))[
    #set text(size: 6pt, fill: ink-secondary)
    #grid(columns: (auto, 1fr, auto), column-gutter: 8pt,
      [#e.badge #smallcaps(e.word)],
      [#source],
      [#text(font: "Source Code Pro", fid)])
  ]
}

// ---------------------------------------------------------------- figures
#let ringler-figure(
  body, caption: none, label: none, status: "documented",
  source: "", fid: "", placement: auto, full-page: false,
) = {
  let f = figure(
    block(breakable: false)[
      #body
      #evidence-gutter(status, source, fid)
    ],
    caption: caption,
    placement: if full-page { none } else { placement },
    gap: 6pt,
  )
  if label != none { [#f #label] } else { f }
}

#show figure.caption: it => {
  set text(size: 8.5pt, fill: ink-primary)
  set par(leading: 0.45em, justify: false)
  align(left, block(width: 92%, it))
}

#set figure(numbering: n => numbering("1.1", counter(heading).get().first(), n))

// Full-page and landscape plates
#let plate(body, caption: none, label: none, status: "documented",
           source: "", fid: "") = page(margin: (x: 12mm, y: 14mm))[
  #ringler-figure(body, caption: caption, label: label, status: status,
                  source: source, fid: fid, full-page: true)
]

#let landscape-plate(body, ..args) = page(flipped: true)[
  #ringler-figure(body, ..args, full-page: true)
]

// Oversize diagrams: tile across a spread with a registration mark rather
// than reducing labels below the 6pt minimum.
#let spread-figure(left-half, right-half, caption: none, label: none,
                   status: "documented", source: "", fid: "") = {
  page(margin: (x: 10mm))[#left-half  #place(bottom + right)[#text(5pt)[▸]]]
  page(margin: (x: 10mm))[#right-half
    #place(bottom + left)[#text(5pt)[◂]]
    #ringler-figure([], caption: caption, label: label, status: status,
                    source: source, fid: fid)]
}
