from aocd import data 
import numpy as np

lines = data.split('\n')

def cards(lines):
    n,vals,amount = len(lines), matches(lines), np.ones(n)
    for i in range(n):
        amount[i+1:i+1+vals[i]] += amount[i]
    return amount


def matches(lines):
    x = [line.split(":")[-1].split("|") for line in lines]
    return [sum([b.split().count(y) for y in a.split()]) for a, b in x]


print(sum(map(lambda x: 2**(x-1) if x != 0 else 0, matches(lines))))
print(sum(cards(lines)))