#!/usr/bin/env python3

with open('day4.txt', 'r') as file:
    lines = file.readlines()
for i, line in enumerate(lines):
    lines[i] = list(lines[i])

w = len(lines[0])
h = len(lines)

def is_roll(x, y):
    if x < 0:
        return False
    if y < 0:
        return False
    if x >= w:
        return False
    if y >= h:
        return False
    return lines[y][x] == '@'

def test(remove):
    rolls = 0
    to_remove = []
    for x in range(w):
        for y in range(h):
            if lines[y][x] == '@':
                s = 0
                if is_roll(x + 1, y): s += 1
                if is_roll(x + 1, y + 1): s += 1
                if is_roll(x, y + 1): s += 1
                if is_roll(x - 1, y + 1): s += 1
                if is_roll(x - 1, y): s += 1
                if is_roll(x - 1, y - 1): s += 1
                if is_roll(x, y - 1): s += 1
                if is_roll(x + 1, y - 1): s += 1
                if s < 4:
                    if remove:
                        to_remove.append((x, y))
                    rolls += 1
    for (x, y) in to_remove:
        lines[y][x] = '.'
    return rolls

print(test(False))
s = 0
while True:
    r = test(True)
    if r == 0:
        break
    s += r
print(s)
