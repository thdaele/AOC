"""
--- Day 3: Rucksack Reorganization ---
https://adventofcode.com/2022/day/3
"""
from aocd.models import Puzzle


def part_a(puzzle: Puzzle):
    score = 0
    for line in puzzle.input_data.splitlines():
        middle_index = len(line) // 2
        compartment1, compartment2 = line[middle_index:], line[:middle_index]
        [c] = set(compartment1).intersection(compartment2)
        if c.isupper():
            score += ord(c) - 38
        else:
            score += ord(c) - 96
    return score


def part_b(puzzle: Puzzle):
    score = 0
    lines = puzzle.input_data.splitlines()
    for i in range(2, len(lines), 3):
        elf1 = lines[i - 2]
        elf2 = lines[i - 1]
        elf3 = lines[i]
        [c] = set(elf1).intersection(elf2, elf3)
        if c.isupper():
            score += ord(c) - 38
        else:
            score += ord(c) - 96
    return score
