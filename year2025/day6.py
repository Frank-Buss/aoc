#!/usr/bin/env python3

import re

with open('day6.txt', 'r') as file:
    lines = file.readlines()

numbers = []

for l in lines:
    inp = re.split(' +', l.strip())
    if l[0] != '*' and l[0] != '+':
        x = [int(x) for x in inp]
        numbers.append(x)
    else:
        ops = inp

s = 0
for x in range(len(numbers[0])):
    c = 0
    if ops[x] == '*':
        c = 1
    for y in range(len(numbers)):
        if ops[x] == '*':
            c = c * numbers[y][x]
        else:
            c = c + numbers[y][x]
    s += c
print(s)

s2 = 0
x = 0
for op in ops:
    cs = []
    while True:
        n = ''
        for l in lines[:-1]:
            l += ' '
            n += l[x]
        print(n)
        x += 1
        n = n.strip()
        if len(n) > 0:
            cs.append(int(n.strip()))
        else:
            s = 0
            if op == '*':
                s = 1
            for c in cs:
                if op == '*':
                    s = s * c
                else:
                    s = s + c
            s2 += s
            break
print(s2)
