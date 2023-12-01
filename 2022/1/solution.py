#!/usr/bin/env python3

f     = open("input.txt", "r")
lines = f.readlines()
f.close()

elves = [0]
i     = 0
sum   = 0

for line in lines:
    line = line.replace("\n", "")
    if line == "":
        line    = "Blank line. Next Elf"
        elves.append(0)
        i += 1
    else:
        elves[i] += int(line)

    # print(line)

elves.sort(reverse=True)
for i in range(3): sum += elves[i]

print()
print("part 1: ", elves[0])
print("part 2: ", sum)
