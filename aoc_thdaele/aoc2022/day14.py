"""
--- Day 14: Regolith Reservoir ---
https://adventofcode.com/2022/day/14
"""


def parse_lines(data):
    X = [500]
    Y = [0]
    lines = list()
    for line in data.splitlines():
        points = line.split('->')
        previous_point = None
        for point in points:
            point = point.strip()
            x, y = point.split(',')
            x = int(x)
            y = int(y)
            if previous_point is None:
                previous_point = (x, y)
                X.append(x)
                Y.append(y)
                continue
            lines.append((previous_point, (x, y)))
            previous_point = (x, y)
            X.append(x)
            Y.append(y)
    return X, Y, lines


def create_cave(X, Y, lines):
    x_max, x_min = max(X), min(X)
    y_max, y_min = max(Y), min(Y)
    cave = [['.' for _ in range(x_max - x_min + 1)] for _ in range(y_max - y_min + 1)]
    for line in lines:
        x1, y1 = line[0]
        x2, y2 = line[1]
        if x1 == x2:
            if y1 > y2:
                y1, y2 = y2, y1
            for y in range(y1, y2 + 1):
                cave[y - y_min][x1 - x_min] = '#'
        elif y1 == y2:
            if x1 > x2:
                x1, x2 = x2, x1
            for x in range(x1, x2 + 1):
                cave[y1 - y_min][x - x_min] = '#'
    cave[0 - y_min][500 - x_min] = '+'
    return cave


def solve_cave(cave, X, Y):
    sands = 0
    x_max, x_min = max(X), min(X)
    y_max, y_min = max(Y), min(Y)

    outbounds = False
    while not outbounds:
        sandY, sandX = 0 - y_min, 500 - x_min
        while True:
            if sandY + 1 >= len(cave):
                outbounds = True
                break
            if cave[sandY + 1][sandX] == '.':
                sandY += 1
            elif sandX - 1 < 0:
                outbounds = True
                break
            elif cave[sandY + 1][sandX - 1] == '.':
                sandX -= 1
                sandY += 1
            elif sandX + 1 >= len(cave[sandY + 1]):
                outbounds = True
                break
            elif cave[sandY + 1][sandX + 1] == '.':
                sandX += 1
                sandY += 1
            elif cave[sandY][sandX] == '+':
                cave[sandY][sandX] = 'o'
                sands += 1
                outbounds = True
                break
            else:
                cave[sandY][sandX] = 'o'
                sands += 1
                break
    return sands


def part_a(data):
    X, Y, lines = parse_lines(data)
    cave = create_cave(X, Y, lines)
    return solve_cave(cave, X, Y)


def part_b(data):
    X, Y, lines = parse_lines(data)
    cave = create_cave(X, Y, lines)
    x_max, x_min = max(X), min(X)
    y_max, y_min = max(Y), min(Y)

    floor = y_max - y_min + 2
    x_min_rows = floor - (500 - x_min)
    x_max_rows = floor - (x_max - 500)
    for row in cave:
        for i in range(x_min_rows):
            row.insert(0, '.')
        for i in range(x_max_rows):
            row.append('.')

    cave.append(['.' for _ in range(x_max + x_max_rows - (x_min - x_min_rows) + 1)])
    cave.append(['#' for _ in range(x_max + x_max_rows - (x_min - x_min_rows) + 1)])

    X.append(x_max + x_max_rows)
    X.append(x_min - x_min_rows)
    Y.append(floor)
    return solve_cave(cave, X, Y)