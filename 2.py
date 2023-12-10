from aocd import data
from numpy import prod


cubes = {
    "red": 12,
    "green": 13,
    "blue": 14,
}

ans = 0

for i, line in enumerate(data.split('\n')):
    rounds = line.split(":")[-1].strip().split(";")
    s = {"red": 0, "green": 0, "blue": 0}
    for round in rounds:
        for box in round.split(","):
            box = box.split()
            s[box[1]] = max(s[box[1]], int(box[0]))
    ans += prod(list(s.values()))

print(ans)