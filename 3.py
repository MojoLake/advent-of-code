from aocd import data
import re



def get_coords(lines, cmp):
    r = []
    for i, line in enumerate(lines):
        j = 0
        while j < len(line):
            if cmp(line[j]):
                k = j
                while k + 1 < len(line) and cmp(line[k+1]):
                    k += 1
                r.append(((i, j), (i, k)))
                j = k
            j += 1
    return r


def solve(lines):
    nums = get_coords(lines, lambda x: x.isdigit())
    syms = [(sy, j) for ((sy, sx), (_, ex)) in get_coords(lines, lambda x: not x.isdigit() and x != ".") for j in range(sx, ex + 1)]
    adj = {}
    r, m = [], 0
    for ((y, sx), (_, ex)) in nums:
        for ssy, ssx in syms:
            if any(map(lambda x: max(abs(x[0][0] - x[1][0]) , abs(x[0][1] - x[1][1])) <= 1, zip([(y, j) for j in range(sx, ex + 1)], [(ssy, ssx) for _ in range(sx, ex + 1)]))):
                r.append(int(lines[y][sx:ex+1]))
                if lines[ssy][ssx] == "*":
                    if (ssy, ssx) in adj.keys():
                        adj[(ssy, ssx)].append(int(lines[y][sx:ex+1]))
                    else:
                        adj[((ssy, ssx))] = [int(lines[y][sx:ex+1])]

    for _, v in zip(adj.keys(), adj.values()):
        if len(v) == 2:
            m += v[0] * v[1]
    return (r, m)



lines = data.split('\n')
# with open("in", "r") as f:
#     lines = f.read().split("\n")
sol = solve(lines)
print(sum(sol[0]))
print(sol[1])
