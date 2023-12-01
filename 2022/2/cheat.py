#!/usr/bin/env python3

# aoc202202.py
 
import pathlib
import sys
 
# As score combinations are unique, make a lookup table for them
scores: dict[str, int] = {
    "AX": 4,
    "AY": 8,
    "AZ": 3,
    "BX": 1,
    "BY": 5,
    "BZ": 9,
    "CX": 7,
    "CY": 2,
    "CZ": 6,
}
 
 
def parse(puzzle_input: str) -> list[str]:
    """Parse input"""
    return [line.replace(' ', '') for line in puzzle_input.splitlines()]
 
 
def part1(data: list[str]) -> int:
    """Solve part 1"""
    score: int = 0
    for line in data:
        score += scores.get(line)
    return score
 
 
def choose_strategy(play: str) -> str:
    """Amend play based on XYZ
   X means lose
   Y means draw
   Z means win"""
    strategy: str = play[1]
    opp_choice: str = play[0]
    my_choice: str = play[1]
    match strategy:
        case "X":
            # Play to lose
            if opp_choice == "A":
                my_choice = "Z"
            elif opp_choice == "C":
                my_choice = "Y"
        case "Y":
            # Play to draw
            if opp_choice == "A":
                my_choice = "X"
            elif opp_choice == "C":
                my_choice = "Z"
        case "Z":
            # Play to win
            if opp_choice == "A":
                my_choice = "Y"
            elif opp_choice == "C":
                my_choice = "X"
    return opp_choice + my_choice
 
 
def part2(data: list[str]) -> int:
    """Solve part 2"""
    score: int = 0
    for line in data:
        score += scores.get(choose_strategy(line))
    return score
 
 
def solve(puzzle_input: str) -> tuple[int, int]:
    """Solve the puzzle for the given input"""
    data = parse(puzzle_input)
    solution1: int = part1(data)  # Correct answer was 13526 (with my data)
    solution2: int = part2(data)  # Correct answer was 14204 (with my data)
 
    return solution1, solution2
 
 
if __name__ == "__main__":
    for path in sys.argv[1:]:
        print(f"{path}:")
        puzzle_input = pathlib.Path(path).read_text().strip()
        solutions = solve(puzzle_input)
        print("\n".join(str(solution) for solution in solutions))
