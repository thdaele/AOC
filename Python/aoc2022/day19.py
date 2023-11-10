"""
--- Day 19: Not Enough Minerals ---
https://adventofcode.com/2022/day/19
"""
import copy
import re


class Blueprint:
    def __init__(self, data):
        numbers = list(map(int, re.findall(r"\d+", data)))
        self.index = numbers[0]
        self.resources = [numbers[1], numbers[2], numbers[3:5], numbers[5:7]]

    def can_make(self, robot, resources):
        if robot == 0 or robot == 1:
            return resources[0] >= self.resources[robot]
        elif robot == 2 or robot == 3:
            return resources[0] >= self.resources[robot][0] and resources[robot - 1] >= self.resources[robot][1]


class State:
    def __init__(self, time, target_robot):
        self.time = time
        self.target_robot = target_robot
        self.resources = [0, 0, 0, 0]
        self.robots = [1, 0, 0, 0]

    def update(self):
        self.time -= 1
        for i in range(4):
            self.resources[i] += self.robots[i]

    def make_robot(self, robot_type, blueprint):
        if blueprint.can_make(robot_type, self.resources):
            self.update()
            self.robots[robot_type] += 1
            if robot_type == 0 or robot_type == 1:
                self.resources[0] -= blueprint.resources[robot_type]
            elif robot_type == 2 or robot_type == 3:
                self.resources[0] -= blueprint.resources[robot_type][0]
                self.resources[robot_type - 1] -= blueprint.resources[robot_type][1]
            return True
        return False

    def prune(self, result, blueprint):
        # Credits to https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j0tls7a/?utm_source=share&utm_medium=web2x&context=3
        # For the pruning idea's
        if self.time <= 0:
            return True
        if self.target_robot == 0:
            if max(blueprint.resources[0], blueprint.resources[1], blueprint.resources[2][0], blueprint.resources[3][0]) <= self.robots[0]:
                return True
        elif self.target_robot == 1:
            if blueprint.resources[2][1] <= self.robots[1]:
                return True
        elif self.target_robot == 2:
            if self.robots[1] == 0:
                return True
            if blueprint.resources[3][1] <= self.robots[2]:
                return True
        elif self.target_robot == 3:
            if self.robots[2] == 0:
                return True
        if self.resources[3] + self.robots[3] * self.time + (self.time - 1) * self.time // 2 <= result:
            return True
        return False


def dfs(blueprint, b=False):
    # dfs iterative
    result = 0
    if not b:
        stack = [State(24, robot_type) for robot_type in range(4)]
    else:
        stack = [State(32, robot_type) for robot_type in range(4)]
    while stack:
        state = stack.pop()
        if state.prune(result, blueprint):
            continue
        added_state = False
        if state.make_robot(state.target_robot, blueprint):
            added_state = True
            for robot_type in range(4):
                new_state = copy.deepcopy(state)
                new_state.target_robot = robot_type
                stack.append(new_state)
        if state.time > 0 and not added_state:
            state.update()
            stack.append(state)
        result = max(result, state.resources[3])
    return result


def part_a(data):
    result = 0
    for blueprint_data in data.split("\n\n"):
        blueprint = Blueprint(blueprint_data)
        result += dfs(blueprint) * blueprint.index
    return result


def part_b(data):
    result = 1
    for blueprint_data in data.split("\n")[:3]:
        blueprint = Blueprint(blueprint_data)
        result *= dfs(blueprint, True)
    return result
