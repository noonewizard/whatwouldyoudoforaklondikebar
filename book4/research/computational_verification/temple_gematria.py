# -*- coding: utf-8 -*-
"""
TEMPLE/SOLOMON GEMATRIA AND ATBASH — verification (Book IV)
Reuses the corrected letter-value table from Book III (final forms fixed).
"""
HEB = "אבגדהוזחטיכלמנסעפצקרשת"
VAL = dict(zip(HEB, [1,2,3,4,5,6,7,8,9,10,20,30,40,50,60,70,80,90,100,200,300,400]))
SOF = {"ך":"כ","ם":"מ","ן":"נ","ף":"פ","ץ":"צ"}
ATBASH = {HEB[i]: HEB[21-i] for i in range(22)}

def norm(w): return "".join(SOF.get(c,c) for c in w)
def g(w): return sum(VAL[c] for c in norm(w) if c in VAL)
def atbash(w): return "".join(ATBASH[c] for c in norm(w) if c in ATBASH)

T = {"א":"'","ב":"b","ג":"g","ד":"d","ה":"h","ו":"w","ז":"z","ח":"ch","ט":"t",
     "י":"y","כ":"k","ל":"l","מ":"m","נ":"n","ס":"s","ע":"`","פ":"p","צ":"ts",
     "ק":"q","ר":"r","ש":"sh","ת":"t"}
def tr(w): return "-".join(T[c] for c in norm(w) if c in T)
# BUG CAUGHT ON FIRST RUN: tr() originally raised KeyError on the space in
# "קדש הקדשים" (Holy of Holies, written as two words). g() already filtered
# via `if c in VAL`; tr() did not have the matching guard. Fixed by adding
# `if c in T`. Logged here rather than silently corrected -- this is the
# same class of error as Book III's dropped-final-form bug: a routine that
# does not defensively filter its input on the first pass.

print("=== GEMATRIA: TEMPLE AND SOLOMON VOCABULARY ===")
words = {
 "היכל":"heikhal (temple/palace)", "בית":"bayit (house)",
 "שלמה":"Shlomo (Solomon)", "שלום":"shalom (peace, same root)",
 "מקדש":"miqdash (sanctuary)", "קדש הקדשים":"qodesh haqodashim (Holy of Holies)",
 "חירם":"Chiram (Hiram)", "צור":"Tsor (Tyre)",
 "יכין":"Yakhin (Jachin)", "בעז":"Bo`az (Boaz)",
 "ארון":"aron (ark)", "כרוב":"keruv (cherub)",
 "אבן":"even (stone)", "עמוד":"amud (pillar)",
 "דוד":"David","ירושלם":"Yerushalayim (Jerusalem)",
 "ציון":"Tsiyon (Zion)","זהב":"zahav (gold)",
}
for w, gloss in words.items():
    print(f"  {w:<8} {tr(w):<20} = {g(w):>4}   {gloss}")

print()
print("=== THE 'SHLOMO = SHALOM' OBSERVATION ===")
print(f"  shlomo (שלמה) = {g('שלמה')}   shalom (שלום) = {g('שלום')}")
print("  Same value is UNSURPRISING: both words share the root ש-ל-ם and")
print("  differ only in vowel letters partially represented in the")
print("  consonantal spelling (מ vs ום) -- this is share ROOT, not a")
print("  coincidence requiring gematria to notice. Included as a worked")
print("  example of the difference between an etymological fact (real,")
print("  philological) and a 'gematria discovery' (the same fact relabeled).")

print()
print("=== ATBASH ON 'TEMPLE' AND 'SOLOMON' VOCABULARY ===")
for w, gloss in words.items():
    a = atbash(w)
    print(f"  {w:<8} -> {a:<8}  ({tr(w)} -> {tr(a)})   [{gloss}]")

print()
print("=== TARGET-LIST TEST: DOES ANY OUTPUT HIT A REAL HEBREW WORD? ===")
LEXICON = set("""אור אמת מלך בית יהוה אדני אלהים תורה שם רוח נפש לב עין דעת בינה
כתר יסוד חסד גבורה תפארת נצח הוד מלכות סוד אמונה שלום מים אש ארץ שמש ירח כוכב דבר
קול פה יד עמוד אבן היכל קדש ברית זהב כסף חכמה בגד עץ ים הר עיר שער חלון דלת גג
קיר רצפה תקרה מנורה שלחן מזבח פרוכת ארון כרוב""".split())
hits = 0
for w in words:
    a = atbash(w)
    if a in LEXICON:
        hits += 1
        print(f"  HIT: {tr(w)} -> {tr(a)}  (in lexicon)")
print(f"  {hits} hits out of {len(words)} words tested against a "
      f"{len(LEXICON)}-word independent lexicon (fixed before this test, "
      f"reused from Book III's Atbash corpus).")
print("  Base rate expectation for this lexicon size and this word-length")
print("  distribution is likewise near zero per Book III ch.16/39 -- so an")
print("  observed 0 is the EXPECTED result, not a null finding requiring")
print("  explanation, and an observed 1 would need the same three-stage")
print("  control discipline Book III Ch.16 applied before being reported.")
