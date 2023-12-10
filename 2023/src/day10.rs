use std::ops::Add;

use aoc_runner_derive::{aoc, aoc_generator};
use num::abs;

use crate::day10::Tile::{Ground, Pipe, Start};

#[derive(PartialEq, Debug, Copy, Clone)]
struct Pos {
    y: i32,
    x: i32
}

impl Pos {
    fn new(y: i32, x: i32) -> Pos {
        Pos { y, x }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos::new(self.y + rhs.y, self.x + rhs.x)
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Tile {
    Pipe(Pos, Pos, Pos, bool),
    Start(Pos, Option<Pos>, Option<Pos>),
    Ground(Pos)
}

impl Tile {
    fn connections(&self) -> Option<(Pos, Pos)> {
        match self {
            Ground(_) => None,
            Pipe(_, neighbour1, neighbour2, _) => Some((*neighbour1, *neighbour2)),
            Start(_, neighbour1, neighbour2) => Some((neighbour1.unwrap(), neighbour2.unwrap()))
        }
    }

    fn next(self, last: Pos) -> Pos {
        let (neighbour1, neighbour2) = match self {
            Pipe(_, neighbour1, neighbour2, _) => (neighbour1, neighbour2),
            Start(_, neighbour1, neighbour2) => (neighbour1.unwrap(), neighbour2.unwrap()),
            _ => unreachable!()
        };
        if !(neighbour1 == last || neighbour2 == last) {
            unreachable!()
        }
        if neighbour1 == last {
            neighbour2
        } else {
            neighbour1
        }
    }

    fn connect_start(&mut self, map: &[Vec<Tile>]) {
        if let Start(pos, neighbour1, neighbour2) = self {
            // Start not connected yet
            assert_eq!(*neighbour1, None, "Start is already connected");
            assert_eq!(*neighbour2, None, "Start is already connected");
            for dir in [Pos::new(-1, 0), Pos::new(0, 1), Pos::new(1, 0), Pos::new(0, -1)] {
                let neighbour_pos = *pos + dir;
                if !bound_check(map, neighbour_pos) {
                    continue;
                }
                let neighbour = get_tile(map, neighbour_pos);
                let connections = neighbour.connections();
                if connections.is_none() {
                    // Neighbour is no pipe
                    continue;
                }
                let connections = connections.unwrap();
                // If links back to start
                if connections.0 == *pos || connections.1 == *pos {
                    if neighbour1.is_none() {
                        *neighbour1 = Some(neighbour_pos);
                    } else if neighbour2.is_none() {
                        *neighbour2 = Some(neighbour_pos);
                    } else {
                        unreachable!()
                    }
                }
            }
            if neighbour1.is_none() || neighbour2.is_none() {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }
}

#[inline]
fn get_tile(map: &[Vec<Tile>], pos: Pos) -> &Tile {
    map.get(pos.y as usize).unwrap().get(pos.x as usize).unwrap()
}

#[inline]
fn bound_check(map: &[Vec<Tile>], pos: Pos) -> bool {
    let y_range = 0..map.len() as i32;
    let x_range = 0..map.first().unwrap().len() as i32;

    x_range.contains(&pos.x) && y_range.contains(&pos.y)
}


#[aoc_generator(day10)]
fn parse(input: &str) -> (Vec<Vec<Tile>>, Pos) {
    let mut start_pos = None;
    let mut result = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut grid_line = vec![];
        for (x, char) in line.chars().enumerate() {
            let pos = Pos::new(y as i32, x as i32);
            let north = Pos::new(y as i32 - 1, x as i32);
            let east = Pos::new(y as i32, x as i32 + 1);
            let south = Pos::new(y as i32 + 1, x as i32);
            let west = Pos::new(y as i32, x as i32 - 1);
            let tile = match char {
                '|' => Pipe(pos, north, south, false),
                '-' => Pipe(pos, east, west, false),
                'L' => Pipe(pos, north, east, true),
                'J' => Pipe(pos, north, west, true),
                '7' => Pipe(pos, south, west, true),
                'F' => Pipe(pos, south, east, true),
                '.' => Ground(pos),
                'S' => Start(pos, None, None),
                _ => unreachable!()
            };
            if char == 'S' {
                start_pos = Some(pos);
            }
            grid_line.push(tile);
        }
        result.push(grid_line);
    }
    if start_pos.is_none() {
        unreachable!()
    }
    (result, start_pos.unwrap())
}

#[aoc(day10, part1)]
fn part1(input: &(Vec<Vec<Tile>>, Pos)) -> u32 {
    let (map_input, start) = input;
    let map = &mut map_input.clone();
    let start_tile = map.get_mut(start.y as usize).unwrap().get_mut(start.x as usize).unwrap();
    start_tile.connect_start(map_input);

    let start_tile = get_tile(map, *start);

    let mut lasts = (*start, *start);
    let mut nexts = start_tile.connections().unwrap();
    let mut result = 1;
    // Check if we close the loop
    while nexts.0 != nexts.1 {
        result += 1;
        // find next
        let pos = nexts;
        let tile0 = get_tile(map, pos.0);
        let tile1 = get_tile(map, pos.1);
        let new_nexts = (tile0.next(lasts.0), tile1.next(lasts.1));
        lasts = nexts;
        nexts = new_nexts;
    }
    result
}

#[aoc(day10, part2)]
fn part2(input: &(Vec<Vec<Tile>>, Pos)) -> u32 {
    let (map_input, start) = input;
    let map = &mut map_input.clone();
    let start_tile = map.get_mut(start.y as usize).unwrap().get_mut(start.x as usize).unwrap();
    start_tile.connect_start(map_input);

    let start_tile = get_tile(map, *start);

    let mut lasts = (*start, *start);
    let mut nexts = start_tile.connections().unwrap();
    let mut vertices = vec![*start];
    // Check if we close the loop
    while nexts.0 != nexts.1 {
        vertices.insert(0, nexts.0);
        vertices.push(nexts.1);

        // find next
        let pos = nexts;
        let tile0 = get_tile(map, pos.0);
        let tile1 = get_tile(map, pos.1);
        let new_nexts = (tile0.next(lasts.0), tile1.next(lasts.1));
        lasts = nexts;
        nexts = new_nexts;
    }
    vertices.push(nexts.1);

    let mut area = 0;
    let mut j = vertices.last().unwrap();
    for pos in &vertices {
        area += (j.x + pos.x) * (j.y - pos.y);
        j = pos;
    }
    let area = abs(area / 2);

    (area - (vertices.len() as i32 / 2) + 1) as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1_PART_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const EXAMPLE_2_PART_1: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const EXAMPLE_1_PART_2: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const EXAMPLE_2_PART_2: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const EXAMPLE_3_PART_2: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(&parse(EXAMPLE_1_PART_1)), 4);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part1(&parse(EXAMPLE_2_PART_1)), 8);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&parse(EXAMPLE_1_PART_2)), 4);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&parse(EXAMPLE_2_PART_2)), 8);
    }

    #[test]
    fn part2_example_3() {
        assert_eq!(part2(&parse(EXAMPLE_3_PART_2)), 10);
    }
}