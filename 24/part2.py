import sys
from sympy import Symbol, solve, solve_linear_system

stones = []
for x in sys.stdin.readlines():
    points = [int(x) for x in x.strip().replace("@", ",").split(",")]
    stones.append([points[:3], points[3:]])


functions = []
syms = []
for i in range(3):
    idx = 1
    p = Symbol("p" + str(i))
    v = Symbol("v" + str(i))
    syms.append(p)
    syms.append(v)
    for x in stones[:4]:
        px = x[0][i]
        vx = x[1][i]
        s = "t" + str(idx)
        idx += 1
        t = Symbol(s)
        f = px + t * vx - p - t * v
        if i == 0:
            syms.append(t)
        functions.append(f)

ans = solve(functions, syms)[0]

print(ans[0], ans[-2], ans[-4])
print(sum([ans[0], ans[-2], ans[-4]]))
