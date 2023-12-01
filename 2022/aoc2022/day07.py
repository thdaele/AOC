"""
--- Day 7: No Space Left On Device ---
https://adventofcode.com/2022/day/7
"""


def parse(data):
    root = {}
    cwd = root
    stack = []
    for line in data.split('\n'):
        if line.startswith('$'):
            if line[2] == 'c':
                dir = line.split()[-1]
                if dir == "/":
                    cwd = root
                elif dir == "..":
                    cwd = stack.pop()
                else:
                    if dir not in cwd:
                        cwd[dir] = {}
                    stack.append(cwd)
                    cwd = cwd[dir]
        else:
            part1, part2 = line.split()
            if part1 == "dir":
                if part2 not in cwd:
                    cwd[part2] = {}
            else:
                cwd[part2] = int(part1)
    return root


def _solve1(dir, dirSize=None):
    size = 0
    answer = 0
    for k, v in dir.items():
        if type(v) == int:
            size += v
        else:
            s, a = _solve1(v, dirSize)
            size += s
            answer += a
    if size <= 100000:
        answer += size
    if dirSize is not None:
        dirSize.append(size)
    return size, answer


def part_a(data):
    root = parse(data)
    size, ans = _solve1(root)
    return ans


def part_b(data):
    dirSize = []
    root = parse(data)
    size, ans = _solve1(root, dirSize=dirSize)
    to_free = size - (70000000 - 30000000)
    large_enough = [s for s in dirSize if s > to_free]
    return min(large_enough)
