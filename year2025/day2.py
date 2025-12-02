#!/usr/bin/env python3

s1 = 0
s2 = 0
with open('day2.txt', 'r') as file:
    line = file.read()

def invalid1(id):
    r = len(id)
    if r % 2 == 1:
        return False
    r = r // 2
    return id[0:r] == id[r:]

def invalid2(id):
    for i in range(1, len(id)):
        test = id[:i]
        if (len(id) % len(test)) == 0:
            r = len(id) // len(test)
            if test * r == id:
                return True
    return False

for r in line.split(','):
    ft = r.split('-')
    f = int(ft[0])
    t = int(ft[1])
    for id in range(f, t+1):
        if invalid1(str(id)):
            s1 += id
        if invalid2(str(id)):
            s2 += id

print(s1)
print(s2)
