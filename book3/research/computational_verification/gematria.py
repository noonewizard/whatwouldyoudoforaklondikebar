# -*- coding: utf-8 -*-
"""GEMATRIA SYSTEMS — Book III. Values verified against the standard tables."""
HEB="אבגדהוזחטיכלמנסעפצקרשת"
NAMES=["alef","bet","gimel","dalet","he","waw","zayin","chet","tet","yod","kaf",
       "lamed","mem","nun","samekh","ayin","pe","tsadi","qof","resh","shin","taw"]
MISPAR_HECHRACHI=[1,2,3,4,5,6,7,8,9,10,20,30,40,50,60,70,80,90,100,200,300,400]
FINALS={"ך":500,"ם":600,"ן":700,"ף":800,"ץ":900}
STD=dict(zip(HEB,MISPAR_HECHRACHI))
# Final (sofit) forms. In the STANDARD reckoning they carry the same value as the
# medial letter; the 500-900 series is an alternative convention (mispar gadol).
# v1 of this script omitted them entirely, which silently dropped the letter and
# gave Elohim = 46 instead of 86. Fixed here; see audits/CIPHER_AUDIT.md.
SOFIT_TO_MEDIAL={"ך":"כ","ם":"מ","ן":"נ","ף":"פ","ץ":"צ"}
def normalize(w): return "".join(SOFIT_TO_MEDIAL.get(c,c) for c in w)
# Mispar Katan: digits reduced (1-9)
def katan(v):
    while v>9: v=sum(int(d) for d in str(v))
    return v
KATAN={c:katan(v) for c,v in STD.items()}
# Mispar Siduri: ordinal position 1-22
SIDURI={c:i+1 for i,c in enumerate(HEB)}
# Mispar Boneh / squared (AtBash value) omitted - not needed

print("=== HEBREW ALPHABET: THE THREE COMMON GEMATRIA SYSTEMS (Appendix B/E) ===")
print(f"{'#':>3} {'ltr':^4} {'name':<9}{'hechrachi':>10}{'siduri':>8}{'katan':>7}")
for i,c in enumerate(HEB):
    print(f"{i+1:>3} {c:^4} {NAMES[i]:<9}{STD[c]:>10}{SIDURI[c]:>8}{KATAN[c]:>7}")
print("  final forms (hechrachi, when counted): kaf 500, mem 600, nun 700, pe 800, tsadi 900")

def g(w,tbl=STD): return sum(tbl[c] for c in normalize(w) if c in tbl)

print()
print("=== VALUES CHECKED AGAINST THE FIGURES MOST OFTEN QUOTED ===")
checks=[("יהוה",26,"Tetragrammaton = 26"),
        ("אחד",13,"echad (one) = 13"),
        ("אהבה",13,"ahavah (love) = 13"),
        ("חי",18,"chai (life) = 18"),
        ("אלהים",86,"Elohim = 86 (with final mem as 40)"),
        ("אדני",65,"Adonai = 65"),
        ("שדי",314,"Shaddai = 314"),
        ("בראשית",913,"bereshit (Gen 1:1 first word) = 913"),
        ("משיח",358,"mashiach = 358"),
        ("נחש",358,"nachash (serpent) = 358")]
ok=0
for w,expect,note in checks:
    got=g(w)
    flag="OK " if got==expect else "!! "
    if got==expect: ok+=1
    print(f"  {flag}{w:<10} computed {got:>4}  expected {expect:>4}   {note}")
print(f"  {ok}/{len(checks)} reproduce the standard figures.")

print()
print("=== THE 13 = 13 = 26 RELATION (the most-cited Hebrew gematria claim) ===")
print(f"  echad {g('אחד')} + ahavah {g('אהבה')} = {g('אחד')+g('אהבה')} = YHWH {g('יהוה')}")
print("  Real arithmetic; a genuine rabbinic observation. Note what it is NOT:")
print("  evidence of a cipher, because nothing was concealed and nothing recovered.")

print()
print("=== MASHIACH = NACHASH = 358 ===")
print(f"  mashiach {g('משיח')}  nachash {g('נחש')}  -- equal, and much used homiletically.")
print("  Base rate: how many Hebrew words share a value with any other?")
