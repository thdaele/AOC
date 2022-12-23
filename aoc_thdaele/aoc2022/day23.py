"""
--- Day 23: Unstable Diffusion ---
https://adventofcode.com/2022/day/23
"""


directions = ["N", "S", "W", "E"]
direction_offset = {
    "N": (-1, 0),
    "S": (1, 0),
    "W": (0, -1),
    "E": (0, 1),
    "NE": (-1, 1),
    "NW": (-1, -1),
    "SE": (1, 1),
    "SW": (1, -1),
}
dir_to_scan = {
    "N": ["N", "NE", "NW"],
    "S": ["S", "SE", "SW"],
    "W": ["W", "NW", "SW"],
    "E": ["E", "NE", "SE"],
}


def parse_data(data):
    elves = {}
    for y, line in enumerate(data.splitlines()):
        for x, char in enumerate(line):
            if char == '#':
                elves[y, x] = None
    return elves


def check_double_destinations(elves, elf, destination):
    for other_elf, other_destination in elves.items():
        if other_elf != elf and other_destination == destination:
            return True
    return False


def _solve(elves, part_b=False):
    direction_index = 0
    i = 1
    while True:
        elf_moved = False
        for elf in elves.keys():
            # Check position around the elf
            found_elf = False
            for direction, offset in direction_offset.items():
                if (elf[0] + offset[0], elf[1] + offset[1]) in elves:
                    found_elf = True
                    break
            if not found_elf:
                continue
            # Propose move
            for j in range(4):
                direction = directions[(direction_index + j) % len(directions)]
                scan = dir_to_scan[direction]
                found_elf = False
                for dir in scan:
                    offset = direction_offset[dir]
                    if (elf[0] + offset[0], elf[1] + offset[1]) in elves:
                        found_elf = True
                        break
                if not found_elf:
                    offset = direction_offset[direction]
                    elves[elf] = elf[0] + offset[0], elf[1] + offset[1]
                    elf_moved = True
                    break
        if not elf_moved and part_b:
            return i
        direction_index = (direction_index + 1) % len(directions)
        # Move elves
        new_elves = {}
        for elf, new_position in elves.items():
            if check_double_destinations(elves, elf, new_position):
                new_elves[elf] = None
                continue
            new_elves[new_position] = None
        elves = new_elves
        i += 1
        if not part_b and i > 10:
            break
    return elves


def result(elves):
    xmax = max(elves, key=lambda x: x[1])[1]
    xmin = min(elves, key=lambda x: x[1])[1]
    ymax = max(elves, key=lambda x: x[0])[0]
    ymin = min(elves, key=lambda x: x[0])[0]
    xsize = xmax - xmin + 1
    ysize = ymax - ymin + 1
    return xsize * ysize - len(elves)


def part_a(data):
    elves = parse_data(data)
    elves = _solve(elves)
    return result(elves)


def part_b(data):
    elves = parse_data(data)
    return _solve(elves, True)
