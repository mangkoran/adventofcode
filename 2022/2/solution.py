#!/usr/bin/env python3

f                         = open("input.txt", "r")
lines                     = f.readlines()
f.close()

part1                     = 0
part2                     = 0
win                       = ["A Y", "B Z", "C X"]

# A X = rock
# B Y = paper
# C Z = scissor

for line in lines:
    score                 = 0
    line                  = line.replace('\n', '')
    current               = line.split(' ')
    if current[1]         == 'X':
        score             += 1
        current[1]        = 'A'
    elif current[1]       == 'Y':
        score             += 2
        current[1]        = 'B'
    elif current[1]       == 'Z':
        score             += 3
        current[1]        = 'C'
    if line in win: score += 6
    elif current[0]       == current[1]: score += 3
    part1                 += score


# X = lose
# Y = draw
# Z = win

for line in lines:
    score                 = 0
    line                  = line.replace('\n', '')
    current               = line.split(' ')
    if current[1]         == 'X':            # lose
        if current[0]     == 'A': score += 3 # rock    vs scissor
        elif current[0]   == 'B': score += 1 # paper   vs rock
        elif current[0]   == 'C': score += 2 # scissor vs paper
    elif current[1]       == 'Y':            # draw
        score             += 3
        if current[0]     == 'A': score += 1 # rock    vs rock
        elif current[0]   == 'B': score += 2 # paper   vs paper
        elif current[0]   == 'C': score += 3 # scissor vs scissor
    elif current[1]       == 'Z':            # win
        score             += 6
        if current[0]     == 'A': score += 2 # rock    vs paper
        elif current[0]   == 'B': score += 3 # paper   vs scissor
        elif current[0]   == 'C': score += 1 # scissor vs rock
    part2                 += score

print(part1)
print(part2)
