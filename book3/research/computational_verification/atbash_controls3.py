# -*- coding: utf-8 -*-
"""
ATBASH CONTROL TEST, v3 — TARGET-LIST CONTAMINATION
v2 returned 0 hits across 777 control spellings and 1 hit for BAPHOMET, which
reads as support for the claim. It is not, and the reason is the target list.

I wrote that list. I wrote it knowing the answer. 'Sophia' is on it only
because Schonfield's claim put it there - and it is a GREEK word, defectively
transliterated, which no independently-built Hebrew lexicon would contain.
v3 asks what happens when the target list is built without knowledge of the
result, which is the only condition under which the test means anything.
"""
import itertools
HEB=list("אבגדהוזחטיכלמנסעפצקרשת")
ATBASH={HEB[i]:HEB[21-i] for i in range(22)}
def atbash(s): return "".join(ATBASH[c] for c in s)
T={"א":"'","ב":"b","ג":"g","ד":"d","ה":"h","ו":"w","ז":"z","ח":"ch","ט":"t","י":"y",
   "כ":"k","ל":"l","מ":"m","נ":"n","ס":"s","ע":"`","פ":"p","צ":"ts","ק":"q","ר":"r",
   "ש":"sh","ת":"t"}
def tr(s): return "".join(T[c] for c in s)
CLASS={"b":"בו","p":"פב","m":"מ","t":"תט","V":"וי"}
def spellings(pat):
    opts=[]
    for s,req in pat:
        ch=list(CLASS[s])
        if not req: ch=ch+[""]
        opts.append(ch)
    return {"".join(c) for c in itertools.product(*opts) if 3<=len("".join(c))<=7}

baph=[("b",True),("V",False),("p",True),("V",False),("m",True),("V",False),("t",True)]

# LIST 1 - as used in v2: a Hebrew lexicon WITH the Greek loanword added
WITH_SOPHIA = set("""שופיא שפיא חכמה אור אמת מלך בית יהוה אדני אלהימ תורה שמ רוח נפש לב
עינ דעת בינה כתר יסוד חסד גבורה תפארת נצח הוד מלכות סוד אמונה שלומ מים אש ארצ שמש
ירח כוכב דבר קול פה יד עמוד אבנ היכל קדש ברית""".split())
# LIST 2 - the same list with the Greek loanword removed: a plain Hebrew lexicon
WITHOUT = WITH_SOPHIA - {"שופיא","שפיא"}

sp = spellings(baph)
for label, tgt in (("WITH Sophia on the list", WITH_SOPHIA), ("WITHOUT Sophia (plain Hebrew lexicon)", WITHOUT)):
    hits=[(s,atbash(s)) for s in sp if atbash(s) in tgt]
    print(f"{label:<40} hits: {len(hits)}   {'; '.join(tr(s)+'->'+tr(a) for s,a in hits) or '-'}")

print(f"\n  BAPHOMET admissible spellings: {len(sp)}")
print("""
=== THE ACTUAL FINDING ===
The entire result rests on one word being on the target list, and that word is
there because the claim put it there. Remove it and the hit count is zero -
identical to every control.

This does NOT show the transformation is wrong. b-p-w-m-t does map to sh-w-p-y-'
under Atbash; the arithmetic is exact and reproducible above. What it shows is
that the transformation carries no evidential weight, because the criterion for
'a meaningful output' was fixed after the output was known.

The test that would carry weight: specify the target lexicon in advance, from a
source independent of the claim (a standard Biblical Hebrew dictionary), then
run the transformation. Sophia - Greek, and a defective spelling at that - is
not in such a lexicon. The claim cannot survive its own control.
""")
