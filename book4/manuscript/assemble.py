#!/usr/bin/env python3
"""Assemble the full Book IV manuscript from its component files.

Run from book4/:  python3 manuscript/assemble.py
Part headings live at the top of the chapter file that opens each Part, so
this script only concatenates; it does not inject structure.
"""
import os, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
os.chdir(ROOT)

TITLE = """# THE TEMPLE OF SOLOMON
## Sacred Architecture, Memory, and the Masonic Imagination

**ROBERT E. LEE RINGLER**

*Book IV of a series comprising*

*THE CARRIER FLAME — Freemasonry, Magic, and the Western Esoteric Tradition*

*THE ENGINE IN THE DARK — Freemasonry and the Architecture of Human Transformation*

*THE LOST WORD — Freemasonry, Sacred Language, and the Problem of Knowing*

*THE TEMPLE OF SOLOMON — Sacred Architecture, Memory, and the Masonic Imagination*

---
"""

def read(path):
    with open(path) as f:
        return f.read().rstrip() + "\n\n---\n"

parts = [TITLE]
parts.append(read("manuscript/front_matter.md"))
parts.append(read("chapters/chapter_00_prologue.md"))
for n in range(1, 91):
    parts.append(read(f"chapters/chapter_{n:02d}.md"))
parts.append(read("chapters/chapter_91_epilogue.md"))

parts.append("\n# APPENDICES\n\n---\n")
for L in "ABCDEFGHIJKLMNOPQRST":
    parts.append(read(f"appendices/appendix_{L}.md"))

parts.append("\n# APPARATUS\n\n---\n")
parts.append(read("manuscript/illustration_list.md"))
parts.append(read("citations/bibliography.md"))
parts.append(read("manuscript/index_framework.md"))

text = "\n".join(parts)
with open("manuscript/full_manuscript.md", "w") as f:
    f.write(text)

words = len(text.split())
print(f"assembled manuscript/full_manuscript.md — {words:,} words")
