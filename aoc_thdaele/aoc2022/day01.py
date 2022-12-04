"""
--- Day 1: Calorie Counting ---
https://adventofcode.com/2022/day/1
"""
from aocd.models import Puzzle


def calorie_list(input_data):
    calories = list()
    for elf in input_data.split("\n\n"):
        calories.append(sum(map(int, elf.splitlines())))
    return calories


def part_a(puzzle: Puzzle):
    return max(calorie_list(puzzle.input_data))


def part_b(puzzle: Puzzle):
    calories = calorie_list(puzzle.input_data)
    calories = sorted(calories, reverse=True)
    return sum(calories[:3])
