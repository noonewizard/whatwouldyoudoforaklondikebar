# PROJECT STYLE SHEET
### Six-Book Research Series — Robert E. L. Ringler

**Scope note.** Four books exist and have been standardized (I–IV). Books V and VI
are not yet written; this sheet governs them when they are.

---

## 1. CORE SETTINGS

| Setting | Value |
|---|---|
| Language | English |
| Variant | **American English** |
| Spelling | American (`-ize`, `-or`, `-er`, `-se`, single `-l` before suffix) |
| Serial (Oxford) comma | **Yes**, consistently |
| Quotation style | American (double for quotation, single for quote-within-quote) |
| Punctuation inside quotation marks | American convention (periods and commas inside) |
| Date style | Day-month-year in running prose ("19 April 1897"); centuries spelled out ("the eighteenth century"); ranges with en dash ("1596–1604") |
| Era markers | BCE / CE, no periods |
| Number style | Spelled out to one hundred in prose; numerals for measurements, dimensions, statistics, percentages, and all computational results |
| Percentages | "per cent" in running prose; `%` in tables and computational output |
| Em dash | Unspaced `—` |
| En dash | Numeric and date ranges `–` |
| Ellipsis | `…` (single glyph) in authorial prose |
| Citation style | Author, *Title* (Place: Publisher, Year). **No page numbers** unless verified |
| Source format | Markdown; typographic quote conversion deferred to Pandoc/Quarto render |

### Why straight quotes remain in source

The manuscripts are Markdown destined for Quarto → Typst. Pandoc's `smart`
extension converts straight quotes and apostrophes to typographic forms at render
time. Converting them in source would be redundant and would risk corrupting
Hebrew strings, code spans, and cipher tables. **Source keeps straight quotes;
output is typographically correct.** This is a deliberate decision, not an
oversight.

---

## 2. SPELLING DECISIONS APPLIED

| Category | Rule | Example |
|---|---|---|
| `-ise` → `-ize` | All verbs and derivatives | organize, recognize, realize, symbolize, memorize, generalize, standardize |
| `-isation` → `-ization` | All | organization, realization, centralization |
| `-our` → `-or` | All | behavior, color, honor, favor, labor, splendor, rigor |
| `-re` → `-er` | All | center, theater, meter, fiber |
| `-ce` → `-se` | Noun/verb collapse to American | defense, offense, license, pretense |
| `practise` → `practice` | Noun **and** verb | practice, practiced, practicing |
| Doubled `-ll-` | Single before suffix | traveled, labeled, modeled, signaling, counselor, jewelry |
| Single `-l` | Doubled where American | fulfill, fulfillment, enrollment, skillful |
| Misc | | catalog, program, judgment, acknowledgment, skeptic, gray, story (floor), draftsman, medieval, encyclopedic |
| Directionals | Drop terminal `-s` | afterward, backward, forward, toward |

---

## 3. FORMS DELIBERATELY **NOT** CHANGED

These are correct American English and were verified, not overlooked.

| Form | Count | Reason |
|---|---|---|
| **archaeology, archaeological** | 38 | Dominant American scholarly usage; `archeology` is a minority variant |
| **symbolism, symbolist** | 139 | Identical in both variants |
| **dialogue, monologue, analogue** | 11 | Standard American; only `catalogue → catalog` applies |
| **specialist, specialists** | 8 | Correct American |
| **analyses** (noun plural of *analysis*) | 3 | Correct American. The one **verb** instance was changed to *analyzes* |
| **realistic, realistically** | 2 | Correct American |
| **Organist** | 1 | Lodge office; correct American |
| **Antient** | all | Archaic spelling in Masonic titles; historical evidence |

---

## 4. DO NOT CHANGE LIST

Protected absolutely. British and archaic spelling here is **evidence**.

- Direct quotations and historical quotations
- Block quotes (`>`)
- Primary-source transcriptions, inscriptions, archival documents
- Book, article, and journal titles
- Organization and institutional names
- Proper nouns, personal names, place names
- Original-language text: Hebrew, Greek, Latin, French, German
- Bibliographic metadata (author, title, publisher, journal, place, year)
- Citation keys, DOIs, ISBNs, URLs, archive identifiers
- Quarto cross-reference labels (`@fig-…`), BibTeX keys
- File paths, code, code comments, variable and function names
- Computational output and preserved script results

### Named protected strings in this series

| String | Why |
|---|---|
| *The New Book of Constitutions of the Antient and Honourable Fraternity of Free and Accepted Masons* | Anderson (1738) — book title |
| the Honourable Fraternity of Ancient Freemasons | Real organization name |
| *The Sceptical Chymist* | Boyle (1661) — book title |
| *From Ritual to Theatre: The Human Seriousness of Play* | Turner — book title |
| *A Serious and Impartial Enquiry…* | Dassigny (1744) — book title |
| *A True and Faithful Relation… Dr. John Dee … and Some Spirits* | Casaubon (1659) — title, with its internal elisions |
| **Tyre** (41 instances) | Phoenician city — proper noun, not British `tyre` |

---

## 5. CAPITALIZATION

| Term | Convention |
|---|---|
| Freemasonry, Freemason, Masonic, Masonry | Capitalized |
| Craft (the institution) | Capitalized; *craft* (the trade) lowercase |
| Grand Lodge (specific body) | Capitalized; *grand lodge* (generic) lowercase |
| Master Mason, Third Degree (the specific degree) | Capitalized |
| lodge (the room or local body) | Lowercase unless part of a name |
| Temple (Solomon's, the Jerusalem Temple) | Capitalized; *temple* generic lowercase |
| Renaissance, Enlightenment, Romanticism | Capitalized |
| Western esotericism, Hermeticism, Rosicrucianism | As shown |
| Kabbalah / Cabala / Qabalah | See §6 — **never normalized** |
| Great Architect of the Universe | Capitalized |
| degree (generic) | Lowercase |

Historical capitalization inside sources is **not** imposed on authorial prose.

---

## 6. TERMINOLOGY — MEANINGFUL DISTINCTIONS PRESERVED

Editorial consistency never collapses these. Each spelling denotes a different
body of material with a different date.

| Form | Denotes |
|---|---|
| **Kabbalah** | The Jewish tradition |
| **Cabala** | The Christian appropriation (Pico, Reuchlin, Knorr) |
| **Qabalah** | Modern Hermetic/occult system (post-1854) |

Likewise preserved as distinct:

- historical Hiram · Hiram King of Tyre · Hiram the craftsman · **Hiram Abiff**
- historical Freemasonry · later Masonic interpretation · occult Freemasonry · modern occult reinterpretation
- Atbash · notarikon · temurah · gematria — never interchanged
- Tetragrammaton · Logos · the Word · the Mason Word · the Lost Word

---

## 7. EPISTEMIC LANGUAGE — UNTOUCHABLE

Editorial standardization must never strengthen or weaken a claim. The following
hedges are argument, not style, and were preserved without exception:

*may · might · appears · suggests · according to · was interpreted as · has been
argued · evidence indicates · evidence does not establish · plausible · disputed ·
speculative · unsupported*

The series' E1–E6 ratings and A–F source levels are terms of art. They are never
paraphrased, rounded, or softened.

---

## 8. HYPHENATION

| Pattern | Form |
|---|---|
| Compound modifier before noun | hyphenated: *eighteenth-century lodge*, *evidence-based claim*, *source-critical method* |
| Same phrase after noun | open: *the lodge is eighteenth century* |
| `-ly` adverb + adjective | never hyphenated: *historically grounded* |
| Century adjectives | *eighteenth-century*, not *18th-century*, in prose |
| Established closed forms | *nonetheless, notwithstanding, insofar, counterexample, pre-registered* |
