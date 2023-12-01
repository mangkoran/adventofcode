#!/usr/bin/env python3

# ref: https://github.com/mer-sublime/advent-of-code

# import string
# from unittest import TestCase

from collections import deque
from itertools import zip_longest


INPUT_FILE = 'input.txt'
TEST_INPUT_FILE = 'test_input.txt'

class SupplyStacks():
    def __init__(self, test=False):
        self.input = TEST_INPUT_FILE if test else INPUT_FILE
        self.stacks = []
        self.steps = []
        with open(self.input, 'r') as f:
            print(f)

    def parse_stacks(self, file):
        stacks = []
        while (line := next(file).rstrip()) != '1':
            stacks.insert(0, [line[i] for i in range(1, len(line), 4)])
        stacks = list(zip_longest(*stacks, fillvalue=' '))
        self.stacks = [deque([crate for crate in stack if crate != ' ']) for stack in stacks]
