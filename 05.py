from aocd import data


def parse(data):
    t = data.split(":")[2:]
    m = [[] for _ in range(7)]
    for i, p in enumerate(t):
        lines = p.split("\n")
        for line in lines:
            if any([x.isdigit() for x in line]):
                m[i].append(tuple(map(int, line.split())))
    return m



def sol(s, i, m):
    if i == 7:
        return s
    for (sr, d, r) in m[i]:
        if sr <= s < sr + r:
            return sol(d + s - sr, i + 1, m)
    return sol(s, i + 1, m)


def part1(seeds, m):
    m = m[::-1]
    y = 330000000
    while True:
        z = sol(y, 0, m)
        if z in seeds:
            return y
        y += 1
    # return min([sol(s, 0, m) for s in seeds])

def part2(seeds, m):
    m = m[::-1]
    y = 0
    while True:
        # print(y)
        z = sol(y, 0, m)
        for i in range(0, len(seeds), 2):
            if seeds[i] <= z < seeds[i] + seeds[i+1]:
                return y
        y += 1 

# with open("in", "r") as f:
#     data = f.read()
seeds = list(map(int, data.split("\n")[0].split(":")[-1].split()))
# print(seeds)
m = parse(data)
# print(m)
print(part1(seeds, m))
print(part2(seeds, m))


