"""
--- Day 21: Monkey Math ---
https://adventofcode.com/2022/day/21
"""


def parse_data(data):
    result = dict()
    for line in data.splitlines():
        key, value = line.split(": ")
        if value[0].isdigit():
            result[key] = int(value)
        else:
            result[key] = (value[:4], value[5], value[7:])
    return result


def _solve(key, operations, pair=False):
    operation = operations[key]
    if isinstance(operation, int):
        return operation
    if isinstance(operation, tuple):
        a = _solve(operation[0], operations)
        b = _solve(operation[2], operations)
        if pair:
            return a, b
        if operation[1] == "+":
            return a + b
        if operation[1] == "-":
            return a - b
        if operation[1] == "*":
            return a * b
        if operation[1] == "/":
            return a // b
    raise ValueError("Invalid operation")


def part_a(data):
    operations = parse_data(data)
    return _solve("root", operations)


def part_b(data):
    operations = parse_data(data)
    human = operations["humn"]
    operations["humn"] = 0

    node = "root"
    while node != "humn":
        before = _solve(node, operations, True)
        operations["humn"] = human
        after = _solve(node, operations, True)
        if before[0] != after[0]:
            constantTerm = before[1]
        elif before[1] != after[1]:
            constantTerm = before[0]
        else:
            raise ValueError("No constant term found")
        if node == "root":
            human = constantTerm
        else:
            operator = operations[node][1]
            if operator == "+":
                human -= constantTerm
            elif operator == "-":
                if before[0] != after[0]:
                    human += constantTerm
                else:
                    human = constantTerm - human
            elif operator == "*":
                human //= constantTerm
            elif operator == "/":
                if before[0] != after[0]:
                    human *= constantTerm
                else:
                    human = constantTerm // human
        if before[0] != after[0]:
            node = operations[node][0]
        elif before[1] != after[1]:
            node = operations[node][2]
    return human

