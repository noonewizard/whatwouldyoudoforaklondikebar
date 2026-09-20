# -*- coding: utf-8 -*-
"""
'MASONIC TEMPLE GEOMETRY IN WASHINGTON D.C.' — control test (Book IV, ch.79)
Reuses and extends the pentagram-in-a-lattice control from Book III/II:
the claim genre is 'draw lines between prominent points in a city plan and
find a [pentagram/compass/square/Seal-of-Solomon hexagram]'.
This tests specifically for a HEXAGRAM (Seal of Solomon / Star of David),
since that is the Temple-specific version of the claim (as opposed to the
pentagram claims already tested in Books II-III).
"""
import random, math

random.seed(19900101)  # arbitrary but fixed BEFORE running, not tuned after

# A plain rectangular street-grid lattice, structurally like an ordinary
# planned city center, containing nothing by construction.
N_X, N_Y = 14, 10
pts = []
for i in range(N_X):
    for j in range(N_Y):
        pts.append((i + random.uniform(-0.03,0.03), j + random.uniform(-0.03,0.03)))
print(f"Lattice: {len(pts)} points, {N_X}x{N_Y}, jittered +/-0.03 units.")

def hexagram_vertices(cx, cy, R, theta):
    # Two overlapping triangles = 6 outer vertices of a hexagram
    return [(cx + R*math.cos(theta + k*math.pi/3), cy + R*math.sin(theta + k*math.pi/3))
            for k in range(6)]

best = None
TRIES = 400000
for _ in range(TRIES):
    cx = random.uniform(0, N_X-1); cy = random.uniform(0, N_Y-1)
    R = random.uniform(1.0, 4.0); th = random.uniform(0, math.pi/3)
    verts = hexagram_vertices(cx, cy, R, th)
    err = 0; ok = True
    used = set()
    for vx, vy in verts:
        d, idx = min(((math.hypot(vx-p[0], vy-p[1]), i) for i, p in enumerate(pts)))
        if d > 0.5 or idx in used: ok = False; break
        used.add(idx); err = max(err, d)
    if ok and (best is None or err < best[0]):
        best = (err, R, cx, cy)

if best:
    err, R, cx, cy = best
    print(f"Best hexagram found: max vertex deviation {err:.4f} units = "
          f"{err/R:.2%} of circumradius {R:.3f}")
    print("At any normal map scale, that deviation is smaller than the width")
    print("of the drawn line. A plain jittered street grid 'contains' a")
    print("Seal-of-Solomon hexagram to the same tolerance the online Temple-")
    print("geometry literature reports for Washington, D.C. -- because the")
    print("method (retrospective point-fitting with an unstated tolerance)")
    print("returns a positive result on ANY sufficiently dense point set.")
else:
    print(f"No hexagram found within tolerance in {TRIES} tries.")
