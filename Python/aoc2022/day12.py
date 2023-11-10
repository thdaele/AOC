"""
--- Day 12: Hill Climbing Algorithm ---
https://adventofcode.com/2022/day/12
"""
from collections import deque


def part_a(data):
    start, goal = None, None
    grid = [list(x) for x in data.splitlines()]
    for r, row in enumerate(grid):
        for c, elevation in enumerate(row):
            if elevation == "S":
                start = (r, c)
                grid[r][c] = "a"
            elif elevation == "E":
                goal = (r, c)
                grid[r][c] = "z"
    if start is None:
        return "No start"
    q = deque()
    q.append((start, 0))
    visited = set()
    visited.add(start)

    while q:
        pos, dist = q.popleft()
        if pos == goal:
            return dist
        r, c = pos
        for dr, dc in ((0, 1), (1, 0), (0, -1), (-1, 0)):
            nr, nc = r + dr, c + dc
            if 0 <= nr < len(grid) and 0 <= nc < len(grid[0]) and (nr, nc) not in visited and ord(grid[nr][nc]) - ord(grid[r][c]) <= 1:
                visited.add((nr, nc))
                q.append(((nr, nc), dist + 1))
    return "No path"


def part_b(data):
    starts, goal = [], None
    grid = [list(x) for x in data.splitlines()]
    for r, row in enumerate(grid):
        for c, elevation in enumerate(row):
            if elevation == "S":
                starts.append((r, c))
                grid[r][c] = "a"
            elif elevation == "E":
                goal = (r, c)
                grid[r][c] = "z"
            elif elevation == "a":
                starts.append((r, c))
    if len(starts) == 0:
        return "No start"

    shortest = []
    for start in starts:
        q = deque()
        q.append((start, 0))
        visited = set()
        visited.add(start)

        while q:
            pos, dist = q.popleft()
            if pos == goal:
                shortest.append(dist)
                break
            r, c = pos
            for dr, dc in ((0, 1), (1, 0), (0, -1), (-1, 0)):
                nr, nc = r + dr, c + dc
                if 0 <= nr < len(grid) and 0 <= nc < len(grid[0]) and (nr, nc) not in visited and ord(grid[nr][nc]) - ord(
                        grid[r][c]) <= 1:
                    visited.add((nr, nc))
                    q.append(((nr, nc), dist + 1))
    return min(shortest) if shortest else "No path"
