"""
--- Day 15: Beacon Exclusion Zone ---
https://adventofcode.com/2022/day/15
"""
import re
from tqdm import tqdm

int_pattern = re.compile(r"-?\d+")
ROW = 2000000
MAX_COORD = 4000000


def manhattan_distance(x1, y1, x2, y2):
    return abs(x1 - x2) + abs(y1 - y2)


def part_a(data):
    result = set()
    for line in data.splitlines():
        sx, sy, bx, by = map(int, int_pattern.findall(line))
        d = manhattan_distance(sx, sy, bx, by)
        d_to_row = manhattan_distance(sx, sy, sx, ROW)
        if d_to_row > d:
            # This sensor doesn't affect the row that we need to check
            continue

        offset = d - d_to_row

        for i in range(sx - offset, sx + offset + 1):
            if by == ROW and i == bx:
                continue
            result.add(i)
    return len(result)


def part_b(data):
    lines = [tuple(map(int, int_pattern.findall(line))) for line in data.splitlines()]
    for y in tqdm(range(MAX_COORD + 1)):
        intervals = list()
        for sx, sy, bx, by in lines:
            d = manhattan_distance(sx, sy, bx, by)
            d_to_row = manhattan_distance(sx, sy, sx, y)
            if d_to_row > d:
                # This sensor doesn't affect the row that we need to check
                continue

            offset = d - d_to_row
            intervals.append((sx - offset, sx + offset))

        intervals.sort()
        full_x_high = intervals[0][1]
        for x_low, x_high in intervals[1:]:
            if x_low - 1 <= full_x_high:
                full_x_high = max(full_x_high, x_high)
            else:
                return (x_low - 1) * MAX_COORD + y
    return None
