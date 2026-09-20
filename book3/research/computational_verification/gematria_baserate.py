# -*- coding: utf-8 -*-
"""
GEMATRIA COLLISION BASE RATE — Book III
Question: how impressive is 'mashiach = nachash = 358'?
Method (fixed in advance): build a corpus of Hebrew words, compute standard
gematria for each, count how many words share a value with at least one other.
"""
from collections import defaultdict
HEB="אבגדהוזחטיכלמנסעפצקרשת"
VAL=dict(zip(HEB,[1,2,3,4,5,6,7,8,9,10,20,30,40,50,60,70,80,90,100,200,300,400]))
SOF={"ך":"כ","ם":"מ","ן":"נ","ף":"פ","ץ":"צ"}
def g(w): return sum(VAL[c] for c in ("".join(SOF.get(x,x) for x in w)) if c in VAL)

# A corpus of common Biblical Hebrew words, chosen for frequency, not for value.
CORPUS = """אב אם בן בת איש אשה מלך שר עבד כהן נביא עם גוי ארץ שמים ים נהר הר מדבר
עיר בית שער חומה דלת מפתח אבן עץ זהב כסף נחשת ברזל לחם מים יין שמן דם בשר עצם
יד רגל ראש עין אזן פה לשון לב נפש רוח בשר אור חשך יום לילה בקר ערב שנה חדש
שבת מועד שמש ירח כוכב אש רוח סוד דבר קול שם ברית תורה חק משפט צדק חסד אמת
אמונה שלום מלחמה חרב מגן חץ קשת סוס רכב אהל משכן היכל מזבח קרבן זבח עלה
מנחה קדש טהור טמא חטא עון פשע כפר גאל ישע חיים מות קבר שאול עולם נצח
כבוד הוד עז חיל גבור חכמה בינה דעת מוסר תבונה אולת כסיל צדיק רשע תם ישר
משיח נחש אריה כלב צאן בקר שור כבש עז יונה נשר דג תנין""".split()
CORPUS = sorted(set(CORPUS))

buckets=defaultdict(list)
for w in CORPUS: buckets[g(w)].append(w)
colliding = sum(len(v) for v in buckets.values() if len(v)>1)
pairs = sum(len(v)*(len(v)-1)//2 for v in buckets.values())

print(f"corpus size: {len(CORPUS)} distinct words")
print(f"distinct gematria values: {len(buckets)}")
print(f"words sharing a value with at least one other: {colliding} ({colliding/len(CORPUS):.1%})")
print(f"total colliding pairs available: {pairs}")
print()
print("=== A SAMPLE OF THE COLLISIONS THIS CORPUS PRODUCES ===")
shown=0
for v,ws in sorted(buckets.items()):
    if len(ws)>1 and shown<14:
        print(f"  {v:>4}: {' = '.join(ws)}")
        shown+=1
print()
print(f"""=== FINDING ===
Roughly {colliding/len(CORPUS):.0%} of ordinary Hebrew words in a frequency-chosen corpus
share a gematria value with some other word in the same corpus, and this list of
{len(CORPUS)} words alone yields {pairs} equal-valued pairs. Any of them can be given a
homiletic reading, and the tradition has given many of them one.

mashiach = nachash = 358 is therefore arithmetically correct and evidentially
weightless taken alone. It is one of {pairs} pairs available in a small corpus; a
full lexicon of ~8,000 Biblical Hebrew words yields them by the thousand.

This is NOT an argument that rabbinic gematria is foolish. Within its own
tradition gematria is a homiletic device - a way of proposing a connection for
reflection - not a method for recovering concealed information, and it was not
offered as one. The error is modern: treating a homiletic technique as a cipher,
and a collision as a discovery.""")
