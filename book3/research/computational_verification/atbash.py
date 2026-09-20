# -*- coding: utf-8 -*-
"""
ATBASH VERIFICATION — Book III
Input:  Hebrew consonantal strings (transliterated, right-to-left order preserved)
Method: Atbash substitution, alef<->taw, bet<->shin, ...
Output: transformed string + lexical assessment
Controls: the same transformation applied to a matched control corpus.
Procedure fixed BEFORE any transformation was computed.
"""
# Hebrew alphabet in order
HEB = list("אבגדהוזחטיכלמנסעפצקרשת")
assert len(HEB) == 22
ATBASH = {HEB[i]: HEB[21-i] for i in range(22)}

# Final forms normalised to their medial equivalents
FINALS = {"ך":"כ","ם":"מ","ן":"נ","ף":"פ","ץ":"צ"}

def norm(s):
    return "".join(FINALS.get(c, c) for c in s if c in HEB or c in FINALS)

def atbash(s):
    return "".join(ATBASH[c] for c in norm(s))

def translit(s):
    T = {"א":"'","ב":"b","ג":"g","ד":"d","ה":"h","ו":"w","ז":"z","ח":"ch","ט":"t",
         "י":"y","כ":"k","ל":"l","מ":"m","נ":"n","ס":"s","ע":"`","פ":"p","צ":"ts",
         "ק":"q","ר":"r","ש":"sh","ת":"t"}
    return "-".join(T[c] for c in s)

print("=== ATBASH TABLE ===")
for i in range(11):
    a, b = HEB[i], HEB[21-i]
    print(f"  {a} ({translit(a)}) <-> {b} ({translit(b)})")

print()
print("=== TEST SET 1: THE BIBLICAL ATTESTATIONS (the only undisputed cases) ===")
biblical = [("ששך","Sheshach, Jer 25:26 / 51:41 — expected: Babel"),
            ("לב קמי","Leb Qamai, Jer 51:1 — expected: Kasdim (Chaldea)")]
for w, note in biblical:
    print(f"  {w}  ->  {atbash(w)}   [{translit(norm(w))} -> {translit(atbash(w))}]")
    print(f"      {note}")

print()
print("=== TEST SET 2: THE BAPHOMET CLAIM (Schonfield 1984) ===")
w = "בפומת"
print(f"  BAPHOMET  {w} ({translit(norm(w))})")
print(f"     Atbash -> {atbash(w)} ({translit(atbash(w))})")
print(f"     Claim: resolves to Sophia (Greek sophia, wisdom).")
