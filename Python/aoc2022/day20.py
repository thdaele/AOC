"""
--- Day 20: Grove Positioning System ---
https://adventofcode.com/2022/day/20
"""
from collections import deque


def part_a(data):
    values = list()
    for line in map(int, data.splitlines()):
        values.append(line)
    newList = deque((index, value) for index, value in enumerate(values))
    for index, value in enumerate(values):
        if value == 0:
            continue
        index_from = newList.index((index, value))
        del newList[index_from]
        index_to = (index_from + value) % len(newList)
        if index_to == 0:
            index_to = len(newList)
        newList.insert(index_to, (index, value))

    result = list(map(lambda x: x[1], newList))
    value = 0
    for i in [1000, 2000, 3000]:
        value += result[(result.index(0) + i) % len(result)]
    return value


def part_b(data):
    values = list()
    for line in map(int, data.splitlines()):
        values.append(line * 811589153)
    newList = deque((index, value) for index, value in enumerate(values))
    for _ in range(10):
        for index, value in enumerate(values):
            if value == 0:
                continue
            index_from = newList.index((index, value))
            del newList[index_from]
            index_to = (index_from + value) % len(newList)
            if index_to == 0:
                index_to = len(newList)
            newList.insert(index_to, (index, value))

    result = list(map(lambda x: x[1], newList))
    value = 0
    for i in [1000, 2000, 3000]:
        value += result[(result.index(0) + i) % len(result)]
    return value