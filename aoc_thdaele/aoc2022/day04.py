"""
--- Day 4: Camp Cleanup ---
https://adventofcode.com/2022/day/4
"""
from aocd.models import Puzzle


def part_a(puzzle: Puzzle):
    score = 0
    for line in puzzle.input_data.splitlines():
        elf1, elf2 = line.split(",")
        l1, r1 = elf1.split("-")
        l1, r1 = int(l1), int(r1)
        l2, r2 = elf2.split("-")
        l2, r2 = int(l2), int(r2)
        if l2 <= l1 and r1 <= r2:
            score += 1
        elif l1 <= l2 and r2 <= r1:
            score += 1
    return score


def part_b(puzzle: Puzzle):
    score = 0
    for line in puzzle.input_data.splitlines():
        elf1, elf2 = line.split(",")
        l1, r1 = elf1.split("-")
        l1, r1 = int(l1), int(r1)
        l2, r2 = elf2.split("-")
        l2, r2 = int(l2), int(r2)
        if r2 >= l1 >= l2:
            score += 1
        elif r1 >= l2 >= l1:
            score += 1
    return score
