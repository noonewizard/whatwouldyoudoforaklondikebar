import math, random, itertools
random.seed(20260920)
# An arbitrary rectangular grid of "significant points": 12 x 8 lattice, unit spacing,
# with small jitter to mimic a real surveyed plan.
pts=[]
for i in range(12):
    for j in range(8):
        pts.append((i+random.uniform(-0.03,0.03), j+random.uniform(-0.03,0.03)))
N=len(pts)

def fit_error(center, R, theta, chosen):
    # chosen: 5 points; measure max deviation from ideal pentagram vertices
    err=0
    for k,p in enumerate(chosen):
        a=theta+2*math.pi*k/5
        vx=center[0]+R*math.cos(a); vy=center[1]+R*math.sin(a)
        err=max(err, math.hypot(p[0]-vx,p[1]-vy))
    return err

best=None; tries=0
for _ in range(400000):
    tries+=1
    cx=random.uniform(0,11); cy=random.uniform(0,7)
    R=random.uniform(1.0,4.0); th=random.uniform(0,2*math.pi)
    chosen=[]
    ok=True
    for k in range(5):
        a=th+2*math.pi*k/5
        vx=cx+R*math.cos(a); vy=cy+R*math.sin(a)
        # nearest lattice point
        d,bp=min(((math.hypot(p[0]-vx,p[1]-vy),p) for p in pts))
        if d>0.5: ok=False; break
        chosen.append(bp)
    if not ok or len(set(chosen))<5: continue
    e=fit_error((cx,cy),R,th,chosen)
    if best is None or e<best[0]:
        best=(e,(cx,cy),R,th,chosen)

e,c,R,th,chosen=best
print(f"points in grid: {N}")
print(f"random placements tried: {tries}")
print(f"best max vertex deviation: {e:.4f} grid units  ({e/R*100:.2f}% of circumradius R={R:.3f})")
print("vertices landed on lattice points:")
for p in chosen: print(f"   ({p[0]:.3f}, {p[1]:.3f})")

# Local refinement: for the chosen five lattice points, find the best-fitting regular
# pentagon by optimising centre, radius and rotation.
def best_for(chosen):
    cx=sum(p[0] for p in chosen)/5; cy=sum(p[1] for p in chosen)/5
    R0=sum(math.hypot(p[0]-cx,p[1]-cy) for p in chosen)/5
    bestl=None
    for _ in range(200000):
        c2=(cx+random.gauss(0,0.05), cy+random.gauss(0,0.05))
        R2=R0+random.gauss(0,0.05); th2=random.uniform(0,2*math.pi/5)
        # assign each chosen point to nearest ideal vertex, require bijection
        verts=[(c2[0]+R2*math.cos(th2+2*math.pi*k/5), c2[1]+R2*math.sin(th2+2*math.pi*k/5)) for k in range(5)]
        used=set(); err=0; ok=True
        for p in chosen:
            d,idx=min(((math.hypot(p[0]-v[0],p[1]-v[1]),i) for i,v in enumerate(verts)))
            if idx in used: ok=False;break
            used.add(idx); err=max(err,d)
        if ok and (bestl is None or err<bestl[0]): bestl=(err,R2)
    return bestl
r=best_for(chosen)
print(f"refined max deviation: {r[0]:.4f} units = {r[0]/r[1]*100:.2f}% of circumradius")
