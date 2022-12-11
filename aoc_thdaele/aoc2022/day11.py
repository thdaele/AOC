"""
--- Day 11: Monkey in the Middle ---
https://adventofcode.com/2022/day/11
"""


class Monkey:
    def __init__(self, string, part2=False):
        lines = string.splitlines()
        self.id = int(lines[0].split(" ")[1][:-1])
        items = lines[1].split(":")[1].strip()
        items = items.split(", ") if items else []
        self.items = []
        for item in items:
            self.items.append(int(item))
        self.operation = lines[2].split(" ")[-2]
        self.operationL = lines[2].split(" ")[-3]
        self.operationR = lines[2].split(" ")[-1]
        self.test = int(lines[3].split(" ")[-1])
        self.true = int(lines[4].split(" ")[-1])
        self.false = int(lines[5].split(" ")[-1])
        self.inspect = 0
        self.part2 = part2

    def addItem(self, item):
        if type(item) != int:
            raise ValueError(f"Invalid item: {item}")
        self.items.append(item)

    def operationItem(self, old):
        if self.operationL == "old":
            left = old
        else:
            left = int(self.operationL)
        if self.operationR == "old":
            right = old
        else:
            right = int(self.operationR)
        if self.operation == "+":
            return left + right
        elif self.operation == "*":
            return left * right
        else:
            raise ValueError(f"Invalid operation: {self.operation}")

    def testItem(self, item):
        if item % self.test == 0:
            return self.true
        else:
            return self.false

    def throwStuff(self, monkeys: dict, mod):
        for item in self.items:
            self.inspect += 1
            newItemLevel = self.operationItem(item)
            if not self.part2:
                newItemLevel = newItemLevel // 3
            monkey = self.testItem(newItemLevel)
            if self.part2:
                newItemLevel = newItemLevel % mod
            monkeys[monkey].addItem(newItemLevel)
        self.items = []


def part_a(data):
    monkeys = dict()
    for monkeyInput in data.split("\n\n"):
        monkey = Monkey(monkeyInput)
        monkeys[monkey.id] = monkey

    for round in range(20):
        for monkey in monkeys.values():
            monkey.throwStuff(monkeys, None)
    active = list()
    for monkey in monkeys.values():
        active.append(monkey.inspect)
    most_active = sorted(active, reverse=True)[:2]
    return most_active[0] * most_active[1]


def part_b(data):
    monkeys = dict()
    mod = 1
    for monkeyInput in data.split("\n\n"):
        monkey = Monkey(monkeyInput, part2=True)
        monkeys[monkey.id] = monkey
        mod *= monkey.test

    for round in range(10000):
        for monkey in monkeys.values():
            monkey.throwStuff(monkeys, mod)
    active = list()
    for monkey in monkeys.values():
        active.append(monkey.inspect)
    most_active = sorted(active, reverse=True)[:2]
    return most_active[0] * most_active[1]
