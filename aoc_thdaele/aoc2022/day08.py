"""
--- Day 8: Treetop Tree House ---
https://adventofcode.com/2022/day/8
"""


def read_grid(data):
    grid = list()
    for line in data.splitlines():
        grid.append([int(c) for c in line])
    return grid


def part_a(data):
    grid = read_grid(data)
    result = set()
    for x in range(1, len(grid) - 1):
        for y in range(1, len(grid[x]) - 1):
            tree = grid[y][x]
            for x_directions in [1, -1]:
                neighbor_x = x
                while True:
                    neighbor_x += x_directions
                    if neighbor_x == -1 or neighbor_x == len(grid[y]):
                        result.add((y, x))
                        break
                    if tree <= grid[y][neighbor_x]:
                        break
            for y_directions in [1, -1]:
                neighbor_y = y
                while True:
                    neighbor_y += y_directions
                    if neighbor_y == -1 or neighbor_y == len(grid):
                        result.add((y, x))
                        break
                    if tree <= grid[neighbor_y][x]:
                        break
    return len(result) + 2 * len(grid) + 2 * len(grid[0]) - 4


def part_b(data):
    grid = read_grid(data)
    scores = []
    for x in range(1, len(grid) - 1):
        for y in range(1, len(grid[x]) - 1):
            tree = grid[y][x]
            total_score = 1
            for x_directions in [1, -1]:
                score = 0
                neighbor_x = x
                while True:
                    neighbor_x += x_directions
                    if neighbor_x == -1 or neighbor_x == len(grid[y]):
                        break
                    score += 1
                    if tree <= grid[y][neighbor_x]:
                        break
                total_score *= score
            for y_directions in [1, -1]:
                neighbor_y = y
                score = 0
                while True:
                    neighbor_y += y_directions
                    if neighbor_y == -1 or neighbor_y == len(grid):
                        break
                    score += 1
                    if tree <= grid[neighbor_y][x]:
                        break
                total_score *= score
            scores.append(total_score)
    return max(scores)
