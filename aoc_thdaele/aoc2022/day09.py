"""
--- Day 9: Rope Bridge ---
https://adventofcode.com/2022/day/9
"""


def sign_function(x):
    if x > 0:
        return 1
    elif x == 0:
        return 0
    else:
        return -1


def part_a(data):
    return 0
    visited = set()
    hx, hy, tx, ty = 0, 0, 0, 0
    for line in data.splitlines():
        direction, distance = line.split()
        distance = int(distance)
        if direction == "R":
            hx += distance
        elif direction == "L":
            hx -= distance
        elif direction == "U":
            hy += distance
        elif direction == "D":
            hy -= distance

        for i in range(distance):
            if direction == "R" and abs(hx - tx) > 1:
                tx += 1
                ty += sign_function(hy - ty)
            elif direction == "L" and abs(hx - tx) > 1:
                tx -= 1
                ty += sign_function(hy - ty)
            elif direction == "U" and abs(hy - ty) > 1:
                ty += 1
                tx += sign_function(hx - tx)
            elif direction == "D" and abs(hy - ty) > 1:
                ty -= 1
                tx += sign_function(hx - tx)
            visited.add((ty, tx))
    return len(visited)


def move_knot(previous, current):
    px, py = previous
    cx, cy = current

    dx, dy = cx - px, cy - py
    if abs(dx) > 1 and abs(dy) > 1:
        return cx - sign_function(dx), cy - sign_function(dy)
    elif abs(dx) > 1:
        return cx - sign_function(dx), cy
    elif abs(dy) > 1:
        return cx, cy - sign_function(dy)
    return cx, cy


def part_b(data):
    data = """R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"""
    visited = set()
    hx, hy = 0, 0
    knots = [(0, 0) for _ in range(9)]

    for line in data.split("\n"):
        direction, distance = line.split()
        distance = int(distance)

        for i in range(distance):
            if direction == "R":
                hx += 1
            elif direction == "L":
                hx -= 1
            elif direction == "U":
                hy += 1
            elif direction == "D":
                hy -= 1

            knots[0] = move_knot((hx, hy), knots[0])
            for j in range(1, len(knots)):
                knots[j] = move_knot(knots[j - 1], knots[j])
            print(knots[8])
            visited.add(knots[8])
    return len(visited)
