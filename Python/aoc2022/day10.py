"""
--- Day 10: Cathode-Ray Tube ---
https://adventofcode.com/2022/day/10
"""


def part_a(data):
    signalStrength = []
    register = 1
    clock = 1
    for line in data.splitlines():
        if line.startswith('noop'):
            clock += 1
        elif line.startswith('addx'):
            _, number = line.split()
            register += int(number)
            clock += 2
            if clock % 40 == 21:
                signalStrength.append((register - int(number)) * (clock - 1))
        if clock % 40 == 20:
            signalStrength.append(register * clock)
    return sum(signalStrength)


def draw_screen(screen, register, clock):
    if clock % 40 == 1:
        screen.append([])
    row = screen[-1]
    pixel = len(row)
    if abs(register - pixel) <= 1:
        row.append('#')
    else:
        row.append('.')


def part_b(data):
    screen = list()
    temp = None
    register = 1
    clock = 1
    line = 0
    lines = data.splitlines()
    remaining_clock = 0
    while True:
        if remaining_clock > 0:
            draw_screen(screen, register, clock)
            clock += 1
            remaining_clock -= 1
            continue
        elif temp is not None:
            register += temp
            temp = None
        if line >= len(lines):
            break
        lineStr = lines[line]
        line += 1
        if lineStr.startswith('noop'):
            remaining_clock += 1
            temp = None
        elif lineStr.startswith('addx'):
            _, number = lineStr.split()
            temp = int(number)
            remaining_clock += 2

        draw_screen(screen, register, clock)
        remaining_clock -= 1
        clock += 1

    for row in screen:
        print(''.join(row))
    return None
