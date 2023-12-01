"""
--- Day 1: Calorie Counting ---
https://adventofcode.com/2022/day/1
"""


def calorie_list(input_data):
    calories = list()
    for elf in input_data.split("\n\n"):
        calories.append(sum(map(int, elf.splitlines())))
    return calories


def part_a(data):
    return max(calorie_list(data))


def part_b(data):
    calories = calorie_list(data)
    calories = sorted(calories, reverse=True)
    return sum(calories[:3])
