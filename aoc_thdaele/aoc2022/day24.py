"""
--- Day 24: Blizzard Basin ---
https://adventofcode.com/2022/day/24
"""
from collections import deque


def parse_data(data):
    start = None
    end = None
    blizzards = list()
    for y, line in enumerate(data.splitlines()):
        for x, char in enumerate(line):
            if start is None and char == ".":
                start = (y, x)
            elif end is None and char == "." and y == len(data.splitlines()) - 1:
                end = (y, x)
            elif char == ">" or char == "<" or char == "^" or char == "v":
                blizzards.append((y, x, char))
    return blizzards, start, end


def _move_blizzards(blizzards, end):
    result = list()
    for y, x, direction in blizzards:
        if direction == ">":
            if x + 1 == end[1] + 1:
                x = 0
            result.append((y, (x + 1) % (end[1] + 2), ">"))
        elif direction == "<":
            if x - 1 == 0:
                x = end[1] + 1
            result.append((y, (x - 1) % (end[1] + 2), "<"))
        elif direction == "^":
            if y - 1 == 0:
                y = end[0]
            result.append(((y - 1) % (end[0] + 1), x, "^"))
        elif direction == "v":
            if y + 1 == end[0]:
                y = 0
            result.append(((y + 1) % (end[0] + 1), x, "v"))
    return result


def print_blizzards(blizzards, start, end, time):
    # Function to debug blizzards and their movement

    grid = list()
    for y in range(end[0] + 1):
        row = list()
        for x in range(end[1] + 2):
            if (y, x) == start or (y, x) == end:
                row.append(".")
            elif y == 0 or y == end[0] or x == 0 or x == end[1] + 1:
                row.append("#")
            else:
                row.append(".")
        grid.append(row)
    for index1, (y1, x1, direction1) in enumerate(blizzards):
        count = 0
        for index2, (y2, x2, direction2) in enumerate(blizzards):
            if index1 != index2 and y1 == y2 and x1 == x2:
                count += 1
        if count == 0:
            grid[y1][x1] = direction1
        else:
            grid[y1][x1] = str(count + 1)
    print(f"Time: {time}")
    for row in grid:
        print("".join(row))

    print("")


def _inbound(node, start, end, ROWS, COLS):
    y, x = node
    return (0 < y < ROWS - 1 and 0 < x < COLS - 1) or node == end or node == start


def get_moves(node, blizzards, start, end, ROWS, COLS):
    y, x = node
    moves = [(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1), (y, x)]
    return [m for m in moves if m not in blizzards and _inbound(m, start, end, ROWS, COLS)]


def _solve(blizzards, start, end, time, ROWS, COLS):
    q = deque()
    q.append((start, time))
    visited = set()
    visited.add((start, time))

    while q:
        node, time = q.popleft()
        if node == end:
            return time
        moves = get_moves(node, blizzards[time + 1], start, end, ROWS, COLS)
        for move in moves:
            if (move, time + 1) not in visited:
                q.append((move, time + 1))
                visited.add((move, time + 1))


def part_a(data):
    blizzards, start, end = parse_data(data)
    blizzards_timed = dict()
    blizzards_timed[0] = set((y, x) for y, x, _ in blizzards)
    for i in range(1, 500):
        blizzards = _move_blizzards(blizzards, end)
        blizzards_timed[i] = set((y, x) for y, x, _ in blizzards)
    ROWS = end[0] + 1
    COLS = end[1] + 2
    return _solve(blizzards_timed, start, end, 0, ROWS, COLS)


def part_b(data):
    blizzards, start, end = parse_data(data)
    blizzards_timed = dict()
    blizzards_timed[0] = set((y, x) for y, x, _ in blizzards)
    for i in range(1, 1000):
        blizzards = _move_blizzards(blizzards, end)
        blizzards_timed[i] = set((y, x) for y, x, _ in blizzards)
    ROWS = end[0] + 1
    COLS = end[1] + 2
    trip1 = _solve(blizzards_timed, start, end, 0, ROWS, COLS)
    trip2 = _solve(blizzards_timed, end, start, trip1, ROWS, COLS)
    trip3 = _solve(blizzards_timed, start, end, trip2, ROWS, COLS)
    return trip3
