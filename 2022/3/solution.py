#!/usr/bin/env python3
import string

f     = open("input.txt", "r")
lines = f.readlines()
f.close()

itemPrio = {letter: index + 1 for index, letter in enumerate(string.ascii_lowercase + string.ascii_uppercase)}
commonItems = [next(iter(set(line[len(line) // 2:]).intersection(line[:len(line) // 2]))) for line in lines]

prioSum = 0
for item in commonItems:
    prioSum += itemPrio[item]

rucksackGroups = list(zip(*[iter(lines)]*3))
badges = [next(iter(set(group[0].rstrip()).intersection(group[1].rstrip()).intersection(group[2].rstrip()))) for group in rucksackGroups]

badgesSum = 0
for letter in badges:
    badgesSum += itemPrio[letter]

print(prioSum)
print(badgesSum)
