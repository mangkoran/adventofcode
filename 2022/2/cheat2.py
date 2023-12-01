#!/usr/bin/env python3

# 2022 Advent of Code Day 2
# from os import read
# from File_Location import Day2_2022

# Read strategy guide file.
with open("input.txt") as f:
    data = f.read()
f.close()

data = data.splitlines()
score_list = {'rock':1,'paper':2,'scissors':3,'win':6,'draw':3,'lose':0} # Score-ctionary
my_score1 = 0
my_score2 = 0

# Part 1 - What would your total score be if everything goes exactly according to your strategy guide?
def outcome_p1(elf_choice, player_choice):
    if elf_choice == "A":
        if player_choice == "X": return score_list['draw']
        elif player_choice == "Y": return score_list['win']
        else: return score_list['lose']
    elif elf_choice == "B":
        if player_choice == "X": return score_list['lose']
        elif player_choice == "Y": return score_list['draw']
        else: return score_list['win']
    else:
        if player_choice == "X": return score_list['win']
        elif player_choice == "Y": return score_list['lose']
        else: return score_list['draw']

for key in range(0,len(data)):
    if data[key][2] == "X":
        my_score1 += score_list['rock']
        my_score1 += outcome_p1(data[key][0],data[key][2])
    elif data[key][2] == "Y":
        my_score1 += score_list['paper']
        my_score1 += outcome_p1(data[key][0],data[key][2])
    else:
        my_score1 += score_list['scissors']
        my_score1 += outcome_p1(data[key][0],data[key][2])

    print(data[key], my_score1)

print("My total score in the first game: %d" %my_score1)

# Part 2 - The Elf finishes helping with the tent and sneaks back over to you.
# "Anyway, the second column says how the round needs to end: X means you need
# to lose, Y means you need to end the round in a draw, and Z means you need
# to win. Good luck!" Following the Elf's instructions for the second column,
# what would your total score be if everything goes exactly according to your strategy guide?

'Choice values, Rock = 1, Paper = 2, Scissors = 3'
lose_list = {'A':3,'B':1,'C':2}
win_list = {'A':2,'B':3,'C':1}
draw_list = {'A':1,'B':2,'C':3}

def outcome_p2(elf_choice, player_choice):
	if player_choice == "X": return score_list['lose'] + lose_list[elf_choice]
	elif player_choice == "Y": return score_list['draw'] + draw_list[elf_choice]
	else: return score_list['win'] + win_list[elf_choice]

for key in range(0,len(data)):
	if data[key][2] == "X":
		my_score2 += outcome_p2(data[key][0],data[key][2])
	elif data[key][2] == "Y":
		my_score2 += outcome_p2(data[key][0],data[key][2])
	else:
		my_score2 += outcome_p2(data[key][0],data[key][2])

print("My total score in the second game: %d" %my_score2)
