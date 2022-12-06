"""
--- Day 6: Tuning Trouble ---
https://adventofcode.com/2022/day/6
"""


def part_a(data):
    for i in range(3, len(data)):
        lastFourCharsSet = {data[i - 3], data[i - 2], data[i - 1], data[i]}
        if len(lastFourCharsSet) == 4:
            # Answer is not null indexed
            return i + 1
    # Lol shouldn't happen
    return -1


def part_b(data):
    for i in range(13, len(data)):
        lastChars = set()
        for j in range(0, 14):
            lastChars.add(data[i - j])
        if len(lastChars) == 14:
            # Answer is not null indexed
            return i + 1
    # Lol shouldn't happen
    return -1
