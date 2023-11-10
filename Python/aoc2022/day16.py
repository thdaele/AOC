"""
--- Day 16: Proboscidea Volcanium ---
https://adventofcode.com/2022/day/16
"""

import numpy as np


def parse_data(data):
    pressure = dict()
    connections = dict()
    for line in data.splitlines():
        parts = line.split()
        pressure[parts[1]] = int(parts[4].split('=')[1].strip(';'))
        part_connections = parts[9:]
        if parts[1] not in connections:
            connections[parts[1]] = list()
        for connection in part_connections:
            connection = connection.strip(',')
            connections[parts[1]].append(connection)
    return pressure, connections


def Floyd_Warshall(pressure, connections):
    dist = [[float('inf') for _j in pressure.keys()] for _i in pressure.keys()]
    vertexL = list()
    for index, vertex in enumerate(pressure.keys()):
        dist[index][index] = 0
        vertexL.append(vertex)
    for start, cList in connections.items():
        si = vertexL.index(start)
        for end in cList:
            ei = vertexL.index(end)
            dist[si][ei] = 1
    for index1, vertex1 in enumerate(pressure.keys()):
        for index2, vertex2 in enumerate(pressure.keys()):
            for index3, vertex3 in enumerate(pressure.keys()):
                if dist[index2][index3] > dist[index2][index1] + dist[index1][index3]:
                    dist[index2][index3] = dist[index2][index1] + dist[index1][index3]
    indexes_to_delete = list()
    for index, (item, value) in enumerate(pressure.items()):
        if value == 0 and item != 'AA':
            indexes_to_delete.append(index)
    indexes_to_delete.sort(reverse=True)
    for index in indexes_to_delete:
        dist = np.delete(dist, index, axis=0)
        dist = np.delete(dist, index, axis=1)
        vertexL = np.delete(vertexL, index)
    return vertexL, dist


def _solve1(pressure, vertexs, dist):
    state = (1, "AA", [], 0, 0)

    result = 0
    # DFS limited on time
    stack = [state]
    while stack:
        time, current, opened, rate, total = stack.pop()
        if time == 30:
            # Time is up
            continue
        # Calculate the result given we visit no more extra states
        result = max(result, total + rate * (30 - time + 1))
        if current not in opened and current != "AA":
            # Open the valve, skip "AA" since it is pressure is 0
            # Update the time, rate and total
            # Don't add the current pressure to total because the pressure release start one minute later
            stack.append((time + 1, current, opened + [current], rate + pressure[current], total + rate))
            continue
        # Try for every possible next state
        for index, vertex in enumerate(vertexs):
            distance = dist[vertexs.index(current)][index]
            # Only if valve is closed and the distance to it could be made in time
            if vertex not in opened and time + distance <= 29 and distance > 0:
                # Update time and total based on the distance (aka the travel time)
                stack.append((time + distance, vertex, opened, rate, total + (distance * rate)))
    return result


def _solve2(pressure, vertexs, dist):
    state = (1, "AA", [], 0, 0)

    states = dict()

    # DFS limited on time
    stack = [state]
    while stack:
        time, current, opened, rate, total = stack.pop()
        if time == 26:
            # Time is up
            continue

        # For every valve opened combination keep the best result
        key = tuple(sorted(opened))
        if states.get(key, 0) < total + rate * (26 - time + 1):
            states[key] = total + rate * (26 - time + 1)

        if current not in opened and current != "AA":
            # Open the valve, skip "AA" since it is pressure is 0
            # Update the time, rate and total
            # Don't add the current pressure to total because the pressure release start one minute later
            stack.append((time + 1, current, opened + [current], rate + pressure[current], total + rate))
            continue
        # Try for every possible next state
        for index, vertex in enumerate(vertexs):
            distance = dist[vertexs.index(current)][index]
            # Only if valve is closed and the distance to it could be made in time
            if vertex not in opened and time + distance <= 29 and distance > 0:
                # Update time and total based on the distance (aka the travel time)
                stack.append((time + distance, vertex, opened, rate, total + (distance * rate)))
    result = 0
    # Find 2 sets of opened valves that give the best result
    # The 2 sets have an empty intersection
    for hkey, hvalue in states.items():
        for ekey, evalue in states.items():
            if len(set(hkey).intersection(set(ekey))) == 0:
                result = max(result, hvalue + evalue)
    return result


def part_a(data):
    pressure, connections = parse_data(data)
    vertexs, dist = Floyd_Warshall(pressure, connections)
    return _solve1(pressure, list(vertexs), dist)


def part_b(data):
    pressure, connections = parse_data(data)
    vertexs, dist = Floyd_Warshall(pressure, connections)
    return _solve2(pressure, list(vertexs), dist)
