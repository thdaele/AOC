import importlib

from aocd.models import Puzzle


def _solve(year, day, data):
    mod_name = "aoc_thdaele.aoc{}.day{:02d}".format(year, day)
    mod = importlib.import_module(mod_name)

    a = mod.part_a(data)
    b = mod.part_b(data)
    return a, b


def solve(year, day):
    puzzle = Puzzle(year, day)
    a, b = _solve(year, day, puzzle.example_data)
    print(a, b)


def solveAndSubmit(year, day, level):
    puzzle = Puzzle(year, day)
    a, b = _solve(year, day, puzzle.input_data)
    if level == "a" or level == "ab":
        puzzle.answer_a = a
    if level == "b" or level == "ab":
        puzzle.answer_b = b
