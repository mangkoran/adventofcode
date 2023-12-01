#!/usr/bin/env python3

# ref: https://github.com/mer-sublime/advent-of-code

# import string
# from unittest import TestCase

INPUT_FILE = 'input.txt'
TEST_INPUT_FILE = 'test_input.txt'

class CampCleanup:
    def __init__(self, test=False):
        self.input = TEST_INPUT_FILE if test else INPUT_FILE
        with open(self.input, 'r') as f:
            pairs = [[[int(section) for section in elf.split('-')] for elf in pair.rstrip().split(',')] for pair in f.readlines()]

        self.pairs = pairs

    def part_one(self) -> int:
        with open(self.input, 'r') as f:
            count = 0
            for pair in self.pairs: count += 1 if (pair[0][0] >= pair[1][0] and pair[0][1] <= pair[1][1] or pair[0][0] <= pair[1][0] and pair[0][1] >= pair[1][1]) else 0
            #                                 -------lower bound------     -------upper bound------
            return count

    def part_two(self) -> int:
        with open(self.input, 'r') as f:
            count = 0
            for pair in self.pairs: count += 1 if (pair[0][0] >= pair[1][0] and pair[0][1] <= pair[1][1] or pair[0][0] <= pair[1][0] and pair[0][1] >= pair[1][1] or pair[0][0] >= pair[1][0] and pair[0][0] <= pair[1][1] or pair[0][1] <= pair[1][1] and pair[0][1] >= pair[1][0]) else 0
            return count


if __name__ == "__main__":
    print('Part one:', CampCleanup().part_one())
    print('Part two:', CampCleanup().part_two())
