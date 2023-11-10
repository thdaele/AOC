"""
--- Day 22: Monkey Map ---
https://adventofcode.com/2022/day/22
"""
import enum


class Tile(enum.Enum):
    Open = 0
    Wall = 1


class Direction(enum.Enum):
    Right = 0
    Down = 1
    Left = 2
    Up = 3


DIR_TO_COORDS = {
    Direction.Right: (0, 1),
    Direction.Down: (1, 0),
    Direction.Left: (0, -1),
    Direction.Up: (-1, 0),
}


def parse_data(data):
    tiles = dict()
    start = (-1, -1)
    map, instructions = data.split('\n\n')
    for y, row in enumerate(map.splitlines()):
        for x, character in enumerate(row):
            if character == '#':
                tiles[y, x] = Tile.Wall
            elif character == '.':
                tiles[y, x] = Tile.Open
                if start == (-1, -1):
                    start = (y, x)
            elif character == ' ':
                continue
            else:
                raise ValueError(f'Unknown character {character}')

    instructionList = list()
    instructions = instructions.replace('R', ' R ').replace('L', ' L ').split()
    for instruction in instructions:
        if instruction == 'R' or instruction == 'L':
            instructionList.append(instruction)
        else:
            instructionList.append(int(instruction))
    return tiles, start, instructionList


def wraparound(tiles, current, direction):
    if direction == Direction.Right:
        newX = min(x for y, x in tiles.keys() if y == current[0])
        return current[0], newX
    elif direction == Direction.Down:
        newY = min(y for y, x in tiles.keys() if x == current[1])
        return newY, current[1]
    elif direction == Direction.Left:
        newX = max(x for y, x in tiles.keys() if y == current[0])
        return current[0], newX
    elif direction == Direction.Up:
        newY = max(y for y, x in tiles.keys() if x == current[1])
        return newY, current[1]
    else:
        raise ValueError(f'Unknown direction {direction}')


def wrapcube(current, direction):
    # Did hardcode the cube like everyone did, but I'm not proud of it
    # It is possible to programmatically find the cube, but I'm not going to do that
    # bc I don't have the time for it with university

    # Example input
    # face_size = 4
    # pos_to_cube_face = {
    #     (0, 2): 1,
    #     (1, 0): 2,
    #     (1, 1): 3,
    #     (1, 2): 4,
    #     (2, 2): 5,
    #     (2, 3): 6
    # }
    #
    # connections = {
    #     (1, Direction.Right): (lambda pos: (2 * face_size + pos[0] - 3, 4 * face_size - 1), Direction.Left),
    #     (1, Direction.Left): (lambda pos: (face_size, face_size + pos[0]), Direction.Down),
    #     (1, Direction.Up): (lambda pos: (face_size, 3 * face_size - 1 - pos[1]), Direction.Down),
    #
    #     (2, Direction.Left): (lambda pos: (3 * face_size - 1, 5 * face_size - 1 - pos[0]), Direction.Up),
    #     (2, Direction.Up): (lambda pos: (0, face_size + 1 + pos[1]), Direction.Down),
    #     (2, Direction.Down): (lambda pos: (3 * face_size - 1, face_size + 1 + pos[1]), Direction.Up),
    #
    #     (3, Direction.Up): (lambda pos: (pos[1] - face_size, 2 * face_size), Direction.Right),
    #     (3, Direction.Down): (lambda pos: (pos[1] + face_size, 2 * face_size), Direction.Right),
    #
    #     (4, Direction.Right): (lambda pos: (2 * face_size, 5 * face_size - 1 - pos[0]), Direction.Down),
    #
    #     (5, Direction.Left): (lambda pos: (2 * face_size - 1, 4 * face_size - 1 - pos[0]), Direction.Up),
    #     (5, Direction.Down): (lambda pos: (2 * face_size - 1, 3 * face_size - 1 - pos[1]), Direction.Up),
    #
    #     (6, Direction.Up): (lambda pos: (5 * face_size - 1 - pos[1], 3 * face_size), Direction.Left),
    #     (6, Direction.Right): (lambda pos: (3 * face_size - 1 - pos[0], 3 * face_size), Direction.Left),
    #     (6, Direction.Down): (lambda pos: (5 * face_size - 1 - pos[1], 0), Direction.Right),
    # }

    # Final input
    face_size = 50
    pos_to_cube_face = {
        (0, 1): 1,
        (0, 2): 2,
        (1, 1): 3,
        (2, 0): 4,
        (2, 1): 5,
        (3, 0): 6
    }
    connections = {
        (1, Direction.Left): (lambda pos: (3 * face_size - 1 - pos[0], 0), Direction.Right),
        (1, Direction.Up): (lambda pos: (2 * face_size + pos[1], 0), Direction.Right),

        (2, Direction.Up): (lambda pos: (4 * face_size - 1, pos[1] - 2 * face_size), Direction.Up),
        (2, Direction.Right): (lambda pos: (3 * face_size - 1 - pos[0], 2 * face_size - 1), Direction.Left),
        (2, Direction.Down): (lambda pos: (-1 * face_size + pos[1], 2 * face_size - 1), Direction.Left),

        (3, Direction.Left): (lambda pos: (2 * face_size, pos[0] - face_size), Direction.Down),
        (3, Direction.Right): (lambda pos: (face_size - 1, pos[0] + face_size), Direction.Up),

        (4, Direction.Left): (lambda pos: (3 * face_size - 1 - pos[0], face_size), Direction.Right),
        (4, Direction.Up): (lambda pos: (face_size + pos[1], face_size), Direction.Right),

        (5, Direction.Right): (lambda pos: (3 * face_size - 1 - pos[0], 3 * face_size - 1), Direction.Left),
        (5, Direction.Down): (lambda pos: (2 * face_size + pos[1], face_size - 1), Direction.Left),

        (6, Direction.Right): (lambda pos: (3 * face_size - 1, -2 * face_size + pos[0]), Direction.Up),
        (6, Direction.Down): (lambda pos: (0, 2 * face_size + pos[1]), Direction.Down),
        (6, Direction.Left): (lambda pos: (0, -2 * face_size + pos[0]), Direction.Down),
    }

    y, x = current
    face = pos_to_cube_face[y // face_size, x // face_size]
    coord_transform, new_dir = connections[face, direction]
    new_pos = coord_transform((y, x))
    return new_pos, new_dir


def _solve(tiles, start, instructions, part2=False):
    direction = Direction.Right

    current = start
    for command in instructions:
        if command == 'L' or command == 'R':
            if command == 'L':
                direction = Direction((direction.value - 1) % 4)
            else:
                direction = Direction((direction.value + 1) % 4)
        else:
            for _ in range(command):
                y, x = current
                dy, dx = DIR_TO_COORDS[direction]
                current = (y + dy, x + dx)
                old_direction = direction
                if current not in tiles:
                    if not part2:
                        current = wraparound(tiles, current, direction)
                    else:
                        current, direction = wrapcube((y, x), direction)
                if tiles[current] == Tile.Wall:
                    current = (y, x)
                    direction = old_direction
    return (current[0] + 1, current[1] + 1), direction


def part_a(data):
    tiles, start, instructionList = parse_data(data)
    location, direction = _solve(tiles, start, instructionList)
    return location[0] * 1000 + location[1] * 4 + direction.value


def part_b(data):
    tiles, start, instructionList = parse_data(data)
    location, direction = _solve(tiles, start, instructionList, part2=True)
    return location[0] * 1000 + location[1] * 4 + direction.value
