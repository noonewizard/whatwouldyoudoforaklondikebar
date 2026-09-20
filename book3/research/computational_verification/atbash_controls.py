# -*- coding: utf-8 -*-
"""
ATBASH CONTROL TEST — Book III
Question: does 'Baphomet -> Sophia' under Atbash indicate design?
Method fixed in advance:
  (1) enumerate the search space of admissible Hebrew spellings of a foreign name
  (2) apply Atbash to every spelling
  (3) count how many outputs are 'acceptable' under the SAME latitude the claim uses
  (4) repeat for control names that nobody alleges are encoded
"""
import itertools, random
HEB = list("אבגדהוזחטיכלמנסעפצקרשת")
ATBASH = {HEB[i]: HEB[21-i] for i in range(22)}
def atbash(s): return "".join(ATBASH[c] for c in s)

T = {"א":"'","ב":"b","ג":"g","ד":"d","ה":"h","ו":"w","ז":"z","ח":"ch","ט":"t",
     "י":"y","כ":"k","ל":"l","מ":"m","נ":"n","ס":"s","ע":"`","פ":"p","צ":"ts",
     "ק":"q","ר":"r","ש":"sh","ת":"t"}
def tr(s): return "".join(T[c] for c in s)

# Consonant classes: which Hebrew letters can plausibly render each sound when a
# foreign name is transcribed. This is the latitude the claim itself relies on.
CLASS = {
 "b": "בו", "p":"פב", "f":"פ", "m":"מ", "t":"תט", "d":"ד", "k":"כק", "g":"ג",
 "s":"סשצ", "sh":"ש", "z":"זס", "ch":"חכ", "h":"הח", "'":"אע", "w":"ו", "y":"י",
 "l":"ל", "n":"נ", "r":"ר", "q":"קכ", "ts":"צ",
 # vowel-letters (matres lectionis) are optional in Hebrew consonantal spelling
 "V": "וי",
}

def spellings(pattern):
    """pattern = list of (sound, required?) ; V entries are optional."""
    opts = []
    for snd, req in pattern:
        choices = list(CLASS[snd])
        if not req: choices = choices + [""]     # mater may be omitted
        opts.append(choices)
    out = set()
    for combo in itertools.product(*opts):
        s = "".join(combo)
        if 3 <= len(s) <= 7: out.add(s)
    return out

# BAPHOMET: B - A - PH - O - M - E - T  -> consonantal skeleton b,(V),p,(V),m,(V),t
baph = [("b",True),("V",False),("p",True),("V",False),("m",True),("V",False),("t",True)]
# Controls: names of comparable length that no one alleges is an Atbash cipher
controls = {
 "MELCHIOR":[("m",True),("V",False),("l",True),("k",True),("V",False),("r",True)],
 "BALTHAZAR":[("b",True),("V",False),("l",True),("t",True),("V",False),("z",True),("r",True)],
 "PANTAGRUEL":[("p",True),("n",True),("t",True),("g",True),("r",True),("V",False),("l",True)],
 "BARTHOLOMEW":[("b",True),("r",True),("t",True),("V",False),("l",True),("m",True)],
 "TAMBOURINE":[("t",True),("m",True),("b",True),("V",False),("r",True),("n",True)],
 "POMEGRANATE":[("p",True),("m",True),("g",True),("r",True),("n",True),("t",True)],
}

# A target list: Hebrew/Greek/Latin words a motivated interpreter would accept as
# "a meaningful result". Deliberately generous - that generosity is the point.
TARGETS = {
 "שופיא":"Sophia (wisdom)","שפיא":"Sophia (defective spelling)",
 "חכמה":"Chokhmah (wisdom)","אור":"or (light)","אמת":"emet (truth)",
 "מלך":"melekh (king)","בית":"bayit (house)","יהוה":"the Tetragrammaton",
 "אדני":"Adonai","אלהימ":"Elohim","תורה":"Torah","שמ":"shem (name)",
 "רוח":"ruach (spirit)","נפש":"nefesh (soul)","לב":"lev (heart)",
 "עינ":"ayin (eye)","דעת":"da'at (knowledge)","בינה":"Binah",
 "כתר":"Keter","יסוד":"Yesod","חסד":"Chesed","גבורה":"Gevurah",
 "תפארת":"Tiferet","נצח":"Netzach","הוד":"Hod","מלכות":"Malkhut",
 "סוד":"sod (secret)","אמונה":"emunah (faith)","שלומ":"shalom",
 "מים":"mayim (water)","אש":"esh (fire)","ארצ":"eretz (earth)",
 "שמש":"shemesh (sun)","ירח":"yareach (moon)","כוכב":"kokhav (star)",
 "דבר":"davar (word)","קול":"qol (voice)","פה":"peh (mouth)",
 "יד":"yad (hand)","עמוד":"amud (pillar)","אבנ":"even (stone)",
 "היכל":"heikhal (temple)","קדש":"qodesh (holy)","ברית":"berit (covenant)",
}

def test(name, pattern):
    sp = spellings(pattern)
    hits = []
    for s in sp:
        a = atbash(s)
        if a in TARGETS: hits.append((s,a,TARGETS[a]))
    return sp, hits

print("=== SEARCH SPACE AND HIT RATE ===")
print(f"{'name':<14}{'spellings':>10}{'hits':>7}{'rate':>9}   examples")
rows=[]
allsets = [("BAPHOMET", baph)] + list(controls.items())
for name, pat in allsets:
    sp, hits = test(name, pat)
    rate = len(hits)/len(sp) if sp else 0
    ex = "; ".join(f"{tr(s)}->{tr(a)} = {g}" for s,a,g in hits[:2])
    print(f"{name:<14}{len(sp):>10}{len(hits):>7}{rate:>8.1%}   {ex}")
    rows.append((name,len(sp),len(hits)))

print()
print("=== THE BAPHOMET SPELLINGS THAT 'WORK' ===")
sp, hits = test("BAPHOMET", baph)
for s,a,g in sorted(hits):
    print(f"  {tr(s):<16} -> {tr(a):<16} {g}")
print(f"  ({len(hits)} of {len(sp)} admissible spellings)")
