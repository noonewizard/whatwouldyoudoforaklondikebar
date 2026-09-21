# DELIVERABLE 14 — CI/CD ARCHITECTURE

## Pipeline

```
push / PR
   │
   ├─ register        schema · integrity rules · negative controls · files · provenance
   ├─ references      no hand-written "Figure N" · every @fig- label registered
   ├─ accessibility   alt text · long descriptions
   ├─ palette         both palettes re-validated at --pairs all
   └─ reproducibility computational figures regenerate · series scripts re-run
        │
        └─ render     Quarto → Typst / HTML / EPUB          [needs all above]
             │
             └─ print-gate   [tag: print-*]
                  rights cleared · audits complete · no expired licenses · grayscale
```

## Why these checks

| Job | Prevents |
|---|---|
| `register` | A figure in the book with no provenance record; an analytical model wearing the authority of evidence |
| `references` | Figure numbers that rot when a figure moves |
| `accessibility` | Alt text written as an afterthought at proof stage |
| `palette` | Silent color drift away from the validated set |
| `reproducibility` | A figure that no longer matches the data it claims to show |
| `print-gate` | Going to press with an uncleared permission or an unaudited figure |

## The negative-control job

`test_validator.py` runs in CI on every push. It mutates the register in eleven
specific ways — an analytical figure claiming documented status, an AI image
registered as evidence, a computational figure with no seed, alt text copied
from the caption — and **asserts that the validator rejects each one**, plus a
positive control asserting the real register still passes.

A validator that has never rejected anything is not known to work. This job is
what makes the rest of the pipeline trustworthy, and it is the direct analogue of
the control discipline the manuscripts themselves apply.

## Branch protection

`main` requires: `register`, `references`, `accessibility`, `palette`, `render`.
Tags matching `print-*` additionally require `print-gate` and
`reproducibility`.

## Release artifacts

A `print-*` tag publishes the print package (Deliverable 18), the figure
manifest, the rights-clearance bundle, and the grayscale proof.
