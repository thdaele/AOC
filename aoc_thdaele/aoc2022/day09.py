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


def part_b(data):
    pass
