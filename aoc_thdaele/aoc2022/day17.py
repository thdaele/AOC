"""
--- Day 17: Pyroclastic Flow ---
https://adventofcode.com/2022/day/17
"""
import copy

l = [(2, 0), (3, 0), (4, 0), (5, 0)]
p = [(2, 1), (3, 1), (4, 1), (3, 2), (3, 0)]
b = [(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)]
d = [(2, 0), (2, 1), (2, 2), (2, 3)]
s = [(2, 0), (2, 1), (3, 1), (3, 0)]
shapes = [l, p, b, d, s]


class Rock:
    def __init__(self, y, shape):
        self.y = y
        self.shape = copy.deepcopy(shapes[shape])
        self.old_shape = None

        for i in range(len(self.shape)):
            self.shape[i] = (self.shape[i][0], self.shape[i][1] + y + 4)

    def isValid(self, settled):
        for i in range(len(self.shape)):
            if 0 > self.shape[i][0] or self.shape[i][0] > 6 or self.shape[i][1] < 0:
                self.shape = self.old_shape
                return False
            if self.shape[i] in settled:
                self.shape = self.old_shape
                return False
        return True

    def fall(self):
        self.old_shape = copy.deepcopy(self.shape)
        for i in range(len(self.shape)):
            self.shape[i] = (self.shape[i][0], self.shape[i][1] - 1)

    def move(self, character):
        if character == '<':
            direction = -1
        elif character == '>':
            direction = 1
        else:
            return
        self.old_shape = copy.deepcopy(self.shape)
        for i in range(len(self.shape)):
            self.shape[i] = (self.shape[i][0] + direction, self.shape[i][1])


def part_a(data):
    rock = None
    rock_index = 0
    line_index = 0
    rocks = 0
    settled = set()
    maxY = -1
    while True:
        if rock is None:
            rock = Rock(maxY, rock_index)
            rock_index = (rock_index + 1) % len(shapes)
        character = data[line_index]
        line_index = (line_index + 1) % len(data)
        rock.move(character)
        rock.isValid(settled)
        rock.fall()
        if not rock.isValid(settled):
            settled.update(rock.shape)
            rock = None
            maxY = max(settled, key=lambda x: x[1])[1]
            rocks += 1
            if rocks == 2022:
                # maxY starts at 0, so add 1
                return maxY + 1


def part_b(data):
    rock = None
    rock_index = 0
    line_index = 0
    rocks = 0
    settled = set()
    maxY = -1
    hashMap = dict()
    while True:
        if rock is None:
            rock = Rock(maxY, rock_index)
            rock_index = (rock_index + 1) % len(shapes)
        character = data[line_index]
        line_index = (line_index + 1) % len(data)
        rock.move(character)
        rock.isValid(settled)
        rock.fall()
        if not rock.isValid(settled):
            settled.update(rock.shape)
            rock = None
            maxY = max(settled, key=lambda x: x[1])[1]
            rocks += 1
            hash = rock_index + line_index * 6
            if hash in hashMap.keys():
                old_rocks, old_height = hashMap[hash]
                if (1000000000000 - old_rocks) % (rocks - old_rocks) == 0:
                    print("Found a cycle", old_rocks, old_height, rocks, maxY + 1)
                    return ((1000000000000 - old_rocks) // (rocks - old_rocks)) * (maxY + 1 - old_height) + old_height
            else:
                hashMap[hash] = (rocks, maxY + 1)
