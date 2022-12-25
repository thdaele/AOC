"""
--- Day 25: Full of Hot Air ---
https://adventofcode.com/2022/day/25
"""


def _solve(data):
    total = 0
    for line in data.splitlines():
        number = 0
        n = len(line)
        for i, c in enumerate(line):
            if c == '=':
                digit = -2
            elif c == '-':
                digit = -1
            else:
                digit = int(c)
            number += digit * (5 ** (n - i - 1))
        total += number
    return total


def convert_to_SNAFU(number):
    r_to_s = {0: "0", 1: "1", 2: "2", 3: "=", 4: "-"}
    a = ""
    while number > 0:
        r = number % 5
        if r > 2:
            # With -1 and -2 instead of 4 and 3 the original number is 5 larger
            number += 5
        number //= 5
        a += r_to_s[r]
    return a[::-1]


def part_a(data):
    decimal = _solve(data)
    return convert_to_SNAFU(decimal)


def part_b(data):
    # No part b, sad :(
    pass
