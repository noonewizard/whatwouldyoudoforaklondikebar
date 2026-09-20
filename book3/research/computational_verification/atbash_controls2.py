# -*- coding: utf-8 -*-
"""
ATBASH CONTROL TEST, v2 — MATCHED DESIGN
v1 was confounded: BAPHOMET's pattern carried three optional mater positions,
giving it a search space of 216 against 12-72 for the controls. A larger search
space finds more by chance, so the comparison was not a control at all.
v2 fixes this two ways:
  (A) matched-shape controls: identical skeleton C-V-C-V-C-V-C
  (B) a direct base rate: random strings over the same alphabet
"""
import itertools, random
HEB = list("אבגדהוזחטיכלמנסעפצקרשת")
ATBASH = {HEB[i]: HEB[21-i] for i in range(22)}
def atbash(s): return "".join(ATBASH[c] for c in s)
T = {"א":"'","ב":"b","ג":"g","ד":"d","ה":"h","ו":"w","ז":"z","ח":"ch","ט":"t",
     "י":"y","כ":"k","ל":"l","מ":"m","נ":"n","ס":"s","ע":"`","פ":"p","צ":"ts",
     "ק":"q","ר":"r","ש":"sh","ת":"t"}
def tr(s): return "".join(T[c] for c in s)

CLASS = {"b":"בו","p":"פב","m":"מ","t":"תט","d":"ד","k":"כק","g":"ג","s":"סשצ",
 "sh":"ש","z":"זס","ch":"חכ","h":"הח","l":"ל","n":"נ","r":"ר","q":"קכ","ts":"צ",
 "f":"פ","V":"וי"}

TARGETS = set("""שופיא שפיא חכמה אור אמת מלך בית יהוה אדני אלהימ תורה שמ רוח נפש לב עינ
דעת בינה כתר יסוד חסד גבורה תפארת נצח הוד מלכות סוד אמונה שלומ מים אש ארצ שמש ירח
כוכב דבר קול פה יד עמוד אבנ היכל קדש ברית""".split())

def spellings(pattern):
    opts=[]
    for snd,req in pattern:
        ch=list(CLASS[snd])
        if not req: ch=ch+[""]
        opts.append(ch)
    return {"".join(c) for c in itertools.product(*opts) if 3<=len("".join(c))<=7}

# MATCHED SHAPE: C V C V C V C, first/third/fifth/seventh consonantal, V optional
matched = {
 "BAPHOMET ":[("b",True),("V",False),("p",True),("V",False),("m",True),("V",False),("t",True)],
 "MAKOTET  ":[("m",True),("V",False),("k",True),("V",False),("t",True),("V",False),("t",True)],
 "DALOZER  ":[("d",True),("V",False),("l",True),("V",False),("z",True),("V",False),("r",True)],
 "GASHINEL ":[("g",True),("V",False),("sh",True),("V",False),("n",True),("V",False),("l",True)],
 "TERAPHOD ":[("t",True),("V",False),("r",True),("V",False),("p",True),("V",False),("d",True)],
 "KALOMESH ":[("k",True),("V",False),("l",True),("V",False),("m",True),("V",False),("sh",True)],
 "PEDOTARN ":[("p",True),("V",False),("d",True),("V",False),("t",True),("V",False),("n",True)],
 "SHOBANEK ":[("sh",True),("V",False),("b",True),("V",False),("n",True),("V",False),("k",True)],
 "ZAMOCHIL ":[("z",True),("V",False),("m",True),("V",False),("ch",True),("V",False),("l",True)],
}
print("=== (A) MATCHED-SHAPE CONTROLS ===")
print(f"{'name':<11}{'spellings':>10}{'hits':>6}   result")
tot_sp=tot_hit=0
for name,pat in matched.items():
    sp=spellings(pat); hits=[(s,atbash(s)) for s in sp if atbash(s) in TARGETS]
    if name.strip()!="BAPHOMET": tot_sp+=len(sp); tot_hit+=len(hits)
    ex="; ".join(f"{tr(s)}->{tr(a)}" for s,a in hits[:2]) or "-"
    print(f"{name:<11}{len(sp):>10}{len(hits):>6}   {ex}")
print(f"\n  Controls pooled: {tot_hit} hits / {tot_sp} spellings = {tot_hit/tot_sp:.2%}")
print(f"  Baphomet:        1 hit  / {len(spellings(matched['BAPHOMET '])):} spellings = {1/len(spellings(matched['BAPHOMET '])):.2%}")

print()
print("=== (B) BASE RATE: RANDOM STRINGS ===")
random.seed(1776)
for L in (4,5,6):
    N=200000; hit=0
    for _ in range(N):
        s="".join(random.choice(HEB) for _ in range(L))
        if atbash(s) in TARGETS: hit+=1
    # analytic: |targets of length L| / 22^L
    tl=sum(1 for t in TARGETS if len(t)==L)
    print(f"  length {L}: {hit}/{N} = {hit/N:.6%}   (targets of this length: {tl}; analytic {tl/22**L:.6%})")

print()
print("=== (C) WHAT THE NUMBERS MEAN ===")
print("""  A single random string almost never lands on a target: the base rate is
  ~1 in 10^4 to 10^6. That is the sense in which Baphomet->Sophia is striking.

  But the claim does not test a single string. It tests a SET of admissible
  spellings against a SET of acceptable outputs, and the interpreter chooses
  both sets after seeing the result. With 216 spellings and a target list of
  44, the expected number of chance hits rises by four orders of magnitude over
  the single-string case - and the matched controls, run over the same space
  with the same generosity, return the rate printed above.""")
