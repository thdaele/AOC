"""
--- Day 13: Distress Signal ---
https://adventofcode.com/2022/day/13
"""
from functools import cmp_to_key


def closing_bracket(line):
    i = 0
    depth = 0
    while i < len(line):
        if line[i] == "[":
            depth += 1
        elif line[i] == "]":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return -1


def parse_line(line):
    result = list()
    line = line[1: -1]
    while len(line) > 0:
        c = line[0]
        if c == "[":
            index = closing_bracket(line)
            inner_list = parse_line(line[:index + 1])
            result.append(inner_list)
            line = line[index + 1:]
        elif c == ",":
            line = line[1:]
        else:
            # Parse number
            num = ""
            while len(line) > 0 and line[0] in "0123456789":
                num += line[0]
                line = line[1:]
            result.append(int(num))
    return result


def parse_data(data):
    result = list()
    pairs = data.split("\n\n")
    for pair in pairs:
        pair1, pair2 = pair.split("\n")
        pair1 = parse_line(pair1)
        pair2 = parse_line(pair2)
        result.append((pair1, pair2))
    return result


def test_order(pair1, pair2):
    for e1, e2 in zip(pair1, pair2):
        if type(e1) == int and type(e2) == int:
            if e1 > e2:
                return False
            elif e1 < e2:
                return True
        elif type(e1) == list and type(e2) == list:
            result = test_order(e1, e2)
            if result is not None:
                return result
        elif type(e1) == int and type(e2) == list:
            result = test_order([e1], e2)
            if result is not None:
                return result
        elif type(e1) == list and type(e2) == int:
            result = test_order(e1, [e2])
            if result is not None:
                return result
    if len(pair1) == len(pair2):
        return None
    return len(pair1) < len(pair2)


def part_a(data):
    result = 0
    data = parse_data(data)
    for index, (pair1, pair2) in enumerate(data):
        boolean = test_order(pair1, pair2)
        if boolean or boolean is None:
            result += index + 1
    return result


def compare(pair1, pair2):
    boolean = test_order(pair1, pair2)
    if boolean or boolean is None:
        return -1
    return 1


def part_b(data):
    divider_indexes = list()
    packets = list()
    divider_one = [[2]]
    divider_two = [[6]]
    packets.append(divider_one)
    packets.append(divider_two)
    data = parse_data(data)
    for pair1, pair2 in data:
        packets.append(pair1)
        packets.append(pair2)
    sorted_packets = sorted(packets, key=cmp_to_key(compare))
    for index, packet in enumerate(sorted_packets):
        if packet == divider_one or packet == divider_two:
            divider_indexes.append(index + 1)
    return divider_indexes[1] * divider_indexes[0]
