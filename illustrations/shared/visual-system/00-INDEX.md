# VISUAL SYSTEM — INDEX

| D | Deliverable | Location |
|---|---|---|
| 1 | Visual philosophy | `01-philosophy.md` |
| 2 | Series identity | `02-identity.md` |
| 3 | Color & typography | `03-color-typography.md` · `../styles/ringler-print.mplstyle` |
| 4 | Figure taxonomy | `04-taxonomy.md` |
| 5 | Naming convention | `05-naming.md` |
| 6 | Register schema | `../../register/schema.yml` |
| 7 | Directory architecture | `../../README.md` |
| 8 | Figma system | `../templates/figma-system.md` |
| 9 | Adobe workflow | `../templates/adobe-workflow.md` |
| 10 | QGIS workflow | `../templates/qgis-workflow.md` |
| 11 | Python workflow | `../templates/python-workflow.md` |
| 12 | Quarto integration | `../templates/quarto-integration.md` |
| 13 | Typst integration | `../styles/figures.typ` |
| 14 | CI/CD | `../../qa/cicd.md` · `/.github/workflows/figures.yml` |
| 15 | Rights system | `../../rights/rights-system.md` |
| 16 | AI-image policy | `../../rights/ai-image-policy.md` |
| 17 | Accessibility | `17-accessibility.md` |
| 18 | Print production | `18-print.md` |
| 19–24 | Book plans I–VI | `../../book-0N/PLAN.md` |
| 25 | Master register | `../../register/illustrations.yml` |
| 26 | Caption templates | `../templates/captions.md` |
| 27 | Visual QA checklist | `../../qa/visual-qa-checklist.md` |
| 28 | Visual Argument Audit | `../../qa/visual-argument-audit.md` |
| 29 | Provenance schema · pattern controls | `../../provenance/schema.yml` · `../../qa/pattern-claim-controls.md` |
| 30 | Reproducibility | `../../qa/reproducibility.md` |
| 31–34 | Tool templates | `../templates/` · `../templates/python/` |
| 35 | Quarto example | `../templates/quarto-integration.md` |
| 36 | Typst example | `../styles/figures.typ` |
| 37 | Actions example | `/.github/workflows/figures.yml` |
| 38 | Production checklist | `../../qa/production-checklist.md` |

## Enforcement scripts

| Script | Checks |
|---|---|
| `scripts/validate_register.py` | Schema + 11 research-integrity rules |
| `scripts/test_validator.py` | **Negative controls** — 11 cases, all caught |
| `scripts/check_files.py` | Declared exports exist |
| `scripts/check_provenance.py` | Provenance record complete |
| `scripts/check_labels.py` | Labels ↔ register, both directions |
| `scripts/check_alt_text.py` | Alt text present, bounded, not boilerplate |
| `scripts/check_longdesc.py` | Long descriptions where required |
| `scripts/check_rights.py` | Permissions and expiry |
| `scripts/check_audits.py` | Visual Argument Audit complete |
| `scripts/check_grayscale.py` | No color-only encoding |
| `scripts/check_reproducible.py` | Computational figures regenerate |
