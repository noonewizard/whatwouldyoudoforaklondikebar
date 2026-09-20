from collections import defaultdict
def g(w): return sum(ord(c)-64 for c in w.upper() if c.isalpha())
M="Boaz Jachin Hiram Solomon Tubalcain acacia mason lodge temple square compass apron gavel plumb level ashlar widow Tyre pillar light".split()
C="bus potato Tesco carpet dustbin spanner trousers omelette hamster kettle parsnip linoleum badger sausage turnip pylon gutter biscuit puddle drainpipe".split()
SIG={3,5,7,33,47,72,93}
for name,S in (("MASONIC",M),("CONTROL",C)):
    print("==",name)
    for w in S: print(f"| {w.upper()} | {g(w)} |")
    d=defaultdict(list)
    for w in S: d[g(w)].append(w.upper())
    print("-- pairs:", [(k,v) for k,v in sorted(d.items()) if len(v)>1])
    print("-- sig hits:", [(w.upper(),g(w)) for w in S if g(w) in SIG])
