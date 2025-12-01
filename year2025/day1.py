#!/usr/bin/env python3

pos = 50
c = 0
c2 = 0
with open('day1.txt', 'r') as file:
    lines = file.readlines()
for line in lines:
    dir = line[0]
    ofs = int(line[1:])
    if dir == 'L':
        while ofs > 0:
            pos -= 1
            if pos == 0:
                c2 += 1
            if pos < 0:
                pos += 100
            ofs -= 1
    else:
        while ofs > 0:
            pos += 1
            if pos > 99:
                pos -= 100
            if pos == 0:
                c2 += 1
            ofs -= 1
    if pos == 0:
        c += 1
print(c)
print(c2)
