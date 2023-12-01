"""
--- Day 18: Boiling Boulders ---
https://adventofcode.com/2022/day/18
"""


class Cube:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y and self.z == other.z

    def __hash__(self):
        return hash((self.x, self.y, self.z))

    def neighbours(self):
        return [Cube(self.x - 1, self.y, self.z), Cube(self.x + 1, self.y, self.z),
                Cube(self.x, self.y - 1, self.z), Cube(self.x, self.y + 1, self.z),
                Cube(self.x, self.y, self.z - 1), Cube(self.x, self.y, self.z + 1)]


def part_a(data):
    cubes = list()
    for lines in data.splitlines():
        x, y, z = lines.split(',')
        cubes.append(Cube(int(x), int(y), int(z)))
    result = 0
    for cube in cubes:
        for neighbour in cube.neighbours():
            if neighbour not in cubes:
                result += 1
    return result


def dfs(cube, cubes, minx, maxx, miny, maxy, minz, maxz):
    # dfs iteratively
    stack = [cube]
    visited = set()
    while stack:
        current = stack.pop()
        if current not in visited:
            visited.add(current)
            for neighbour in current.neighbours():
                if neighbour in visited or neighbour in cubes:
                    continue
                if neighbour.x < minx or neighbour.x > maxx or neighbour.y < miny or neighbour.y > maxy or neighbour.z < minz or neighbour.z > maxz:
                    continue
                stack.append(neighbour)
    return visited


def part_b(data):
    cubes = list()
    for lines in data.splitlines():
        x, y, z = lines.split(',')
        cubes.append(Cube(int(x), int(y), int(z)))
    minx = min([cube.x for cube in cubes]) - 1
    maxx = max([cube.x for cube in cubes]) + 1
    miny = min([cube.y for cube in cubes]) - 1
    maxy = max([cube.y for cube in cubes]) + 1
    minz = min([cube.z for cube in cubes]) - 1
    maxz = max([cube.z for cube in cubes]) + 1

    # Flood fill the area outside the cubes
    visited = dfs(Cube(minx, miny, minz), cubes, minx, maxx, miny, maxy, minz, maxz)
    # Do part A but check instead if a surface of the cube is in the flooded area
    result = 0
    for cube in cubes:
        for neighbour in cube.neighbours():
            if neighbour in visited:
                result += 1
    return result
