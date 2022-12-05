"""
--- Day 2: Rock Paper Scissors ---
https://adventofcode.com/2022/day/2
"""
from enum import Enum

from aocd.models import Puzzle


class Options(Enum):
    A = X = ROCK = 1
    B = Y = PAPER = 2
    C = Z = SCISSORS = 3


def wins(a: Options, b: Options) -> bool:
    return a.value == b.value % 3 + 1


def loses(a: Options, b: Options) -> bool:
    return wins(b, a)


def part_a(puzzle: Puzzle):
    score = 0
    for line in puzzle.input_data.splitlines():
        elf, player = line.split()
        elf = Options[elf]
        player = Options[player]
        if elf == player:
            score += player.value + 3
        elif wins(player, elf):
            score += player.value + 6
        elif loses(player, elf):
            score += player.value
    return score


def part_b(puzzle: Puzzle):
    score = 0
    for line in puzzle.input_data.splitlines():
        elf, outcome = line.split()
        elf = Options[elf]

        if outcome == "X":
            # Lose
            player = Options((elf.value + 1) % 3 + 1)
            score += player.value
        elif outcome == "Y":
            # Draw
            player = Options(elf.value)
            score += player.value + 3
        elif outcome == "Z":
            # Win
            player = Options(elf.value % 3 + 1)
            score += player.value + 6
    return score
