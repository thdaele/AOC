"""
--- Day 5: Supply Stacks ---
https://adventofcode.com/2022/day/5
"""


def parse_data(data):
    stackLines, instructionLines = data.split("\n\n")
    stackLines = [line for line in stackLines.splitlines()]
    numberLine = stackLines[-1]
    stacks = list()
    for index, char in enumerate(numberLine):
        if char.isdigit():
            stack = list()
            for line in stackLines[-2::-1]:
                if line[index].isalpha():
                    stack.append(line[index])
            stacks.append(stack)
    instructions = list()
    for line in instructionLines.splitlines():
        a, b, c = None, None, None
        index = 0
        while index < len(line):
            char = line[index]
            i = 1
            if char.isdigit():
                # Parse numbers with multiple digits
                while index + i < len(line) and line[index + i].isdigit():
                    i += 1
                if a is None:
                    a = line[index:index + i]
                elif b is None:
                    b = line[index:index + i]
                else:
                    c = line[index:index + i]
            index += i
        instructions.append((a, b, c))
    return stacks, instructions


def part_a(data):
    data, instructions = parse_data(data)
    for instruction in instructions:
        a, b, c = instruction
        a, b, c = int(a), int(b), int(c)
        if b == c:
            continue
        for i in range(a):
            if len(data[b - 1]) != 0:
                data[c - 1].append(data[b - 1].pop())
    result = ""
    for stack in data:
        if len(stack) != 0:
            result += stack.pop()
        else:
            result += " "
    return result


def part_b(data):
    data, instructions = parse_data(data)
    for instruction in instructions:
        a, b, c = instruction
        a, b, c = int(a), int(b), int(c)
        if b == c:
            continue
        move_list = list()
        for i in range(a):
            if len(data[b - 1]) != 0:
                move_list = [data[b - 1].pop()] + move_list
        data[c - 1] += move_list
    result = ""
    for stack in data:
        if len(stack) != 0:
            result += stack.pop()
        else:
            result += " "
    return result
