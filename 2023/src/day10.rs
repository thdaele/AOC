use std::ops::Add;

use aoc_runner_derive::{aoc, aoc_generator};
use num::abs;

#[derive(PartialEq, Debug, Copy, Clone)]
struct Pos {
    y: u8,
    x: u8
}

impl Pos {
    fn new(y: u8, x: u8) -> Pos {
        Pos { y, x }
    }
}

impl Add<(i8, i8)> for Pos {
    type Output = Pos;

    fn add(self, rhs: (i8, i8)) -> Self::Output {
        Pos::new((self.y as i8 + rhs.0) as u8, (self.x as i8 + rhs.1) as u8)
    }
}

#[derive(PartialEq, Copy, Clone)]
struct Tile {
    pos: Pos,
    neighbours: Option<(Pos, Pos)>,
    corner: bool,
    char: char
}

impl Tile {
    fn next(self, last: Pos) -> Pos {
        let (neighbour1, neighbour2) = self.neighbours.unwrap();
        if !(neighbour1 == last || neighbour2 == last) {
            // n1 links to n2 but n2 doesn't link back to n1
            panic!("{:?} links to {:?}, but {:?} doesn't link back to {:?}", last, self.pos, self.pos, last)
        }
        if neighbour1 == last {
            neighbour2
        } else {
            neighbour1
        }
    }

    fn connect_start(&mut self, map: &[Vec<Tile>]) {
        // Start not connected yet
        assert_eq!(self.neighbours, None, "Start is already connected");
        let mut start_connections = Vec::with_capacity(4); // 4 Directions so max 4 items inserted
        for dir in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let neighbour_pos = self.pos + dir;
            if !bound_check(map, neighbour_pos) {
                continue;
            }
            let neighbour = get_tile(map, neighbour_pos);
            if neighbour.neighbours.is_none() {
                // Neighbour is no pipe
                continue;
            }
            let connections = neighbour.neighbours.unwrap();
            // If links back to start
            if connections.0 == self.pos || connections.1 == self.pos {
                start_connections.push(neighbour_pos);
            }
        }
        assert_eq!(start_connections.len(), 2, "Too much connections on start");
        let neighbour1 = start_connections.first().unwrap();
        let neighbour2 = start_connections.last().unwrap();
        self.neighbours = Some((*neighbour1, *neighbour2));
        self.corner = !(neighbour1.x == neighbour2.x || neighbour1.y == neighbour2.y);
    }
}

#[inline]
fn get_tile(map: &[Vec<Tile>], pos: Pos) -> &Tile {
    map.get(pos.y as usize - 1).unwrap().get(pos.x as usize - 1).unwrap()
}

#[inline]
fn bound_check(map: &[Vec<Tile>], pos: Pos) -> bool {
    let y_range = 1..=map.len() as u8;
    let x_range = 1..=map.first().unwrap().len() as u8;

    x_range.contains(&pos.x) && y_range.contains(&pos.y)
}


#[aoc_generator(day10)]
fn parse(input: &str) -> (Vec<Vec<Tile>>, Pos) {
    let mut start_pos = None;
    let mut result = vec![];
    for (y, line) in input.lines().enumerate() {
        // Make y bigger so we can check neighbours and don't have to worry for underflow on usize
        let y_increased = y as u8 + 1;
        let mut grid_line = vec![];
        for (x, char) in line.chars().enumerate() {
            // Make y bigger so we can check neighbours and don't have to worry for underflow on usize
            let x_increased = x as u8 + 1;
            let pos = Pos::new(y_increased, x_increased);
            let north = Pos::new(y_increased - 1, x_increased);
            let east = Pos::new(y_increased, x_increased + 1);
            let south = Pos::new(y_increased + 1, x_increased);
            let west = Pos::new(y_increased, x_increased - 1);
            let tile = match char {
                '|' => Tile{pos, neighbours: Some((north, south)), corner: false, char},
                '-' => Tile{pos, neighbours: Some((east, west)), corner: false, char},
                'L' => Tile{pos, neighbours: Some((north, east)), corner: true, char},
                'J' => Tile{pos, neighbours: Some((north, west)), corner: true, char},
                '7' => Tile{pos, neighbours: Some((south, west)), corner: true, char},
                'F' => Tile{pos, neighbours: Some((south, east)), corner: true, char},
                '.' => Tile{pos, neighbours: None, corner: false, char},
                'S' => Tile{pos, neighbours: None, corner: false, char},
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
    let start_tile = map.get_mut(start.y as usize - 1).unwrap().get_mut(start.x as usize - 1).unwrap();
    start_tile.connect_start(map_input);

    let start_tile = get_tile(map, *start);

    let mut lasts = (*start, *start);
    let mut nexts = start_tile.neighbours.unwrap();
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

#[aoc(day10, part2, formula)]
fn part2_f(input: &(Vec<Vec<Tile>>, Pos)) -> u32 {
    let (map_input, start) = input;
    let map = &mut map_input.clone();
    let start_tile = map.get_mut(start.y as usize - 1).unwrap().get_mut(start.x as usize - 1).unwrap();
    start_tile.connect_start(map_input);

    let start_tile = get_tile(map, *start);

    let mut lasts = (*start, *start);
    let mut nexts = start_tile.neighbours.unwrap();
    let mut vertices = vec![*start];
    let mut boundary_length = 1;
    // Check if we close the loop
    while nexts.0 != nexts.1 {
        // find next
        let pos = nexts;
        let tile0 = get_tile(map, pos.0);
        let tile1 = get_tile(map, pos.1);

        // Only the corners are needed to calculate the area with Shoelace formula
        if tile0.corner {
            vertices.insert(0, nexts.0);
        }
        if tile1.corner {
            vertices.push(nexts.1);
        }
        boundary_length += 2;

        let new_nexts = (tile0.next(lasts.0), tile1.next(lasts.1));
        lasts = nexts;
        nexts = new_nexts;
    }
    let tile = get_tile(map, nexts.1);
    if tile.corner {
        vertices.push(nexts.1);
    }
    boundary_length += 1;

    // Shoelace formula
    let mut area = 0;
    let mut j = vertices.last().unwrap();
    for pos in &vertices {
        area += (j.x as i32 + pos.x as i32) * (j.y as i32 - pos.y as i32);
        j = pos;
    }
    let area = abs(area / 2);

    // Pick's theorem
    (area - (boundary_length / 2) + 1) as u32
}

#[aoc(day10, part2, boundary_cross)]
fn part2_bc(input: &(Vec<Vec<Tile>>, Pos)) -> u32 {
    let (map_input, start) = input;
    let map = &mut map_input.clone();
    let start_tile = map.get_mut(start.y as usize - 1).unwrap().get_mut(start.x as usize - 1).unwrap();
    start_tile.connect_start(map_input);

    // Replace the char on the start tile by the correct letter for the connection
    let (neighbour1, neighbour2) = start_tile.neighbours.unwrap();
    start_tile.char = match (neighbour1.y as i8 - neighbour2.y as i8, neighbour1.x as i8 - neighbour2.x as i8) {
        (-2, 0) => '|',
        (0, -2) => '-',
        (-1, -1) => '7',
        (-1, 1) => 'F',
        (1, 1) => 'L',
        (1, -1) => 'J',
        _ => unreachable!()
    };
    let start_tile = get_tile(map, *start);

    let mut lasts = (*start, *start);
    let mut nexts = start_tile.neighbours.unwrap();
    let mut vertices = vec![*start];
    // Check if we close the loop
    while nexts.0 != nexts.1 {
        // find next
        let pos = nexts;
        let tile0 = get_tile(map, pos.0);
        let tile1 = get_tile(map, pos.1);

        vertices.insert(0, nexts.0);
        vertices.push(nexts.1);

        let new_nexts = (tile0.next(lasts.0), tile1.next(lasts.1));
        lasts = nexts;
        nexts = new_nexts;
    }
    vertices.push(nexts.1);

    // Calculate inside points based on when the boundary is crossed
    let mut inside_points = 0;
    for row in map {
        let mut inside = false;
        let mut c1 = None;
        for tile in row {
            if vertices.contains(&tile.pos) {
                if tile.char == '|' {
                    inside = !inside;
                } else if tile.char == 'F' || tile.char == 'L' {
                    c1 = Some(tile.char);
                } else if c1.is_some() {
                    let c = c1.unwrap();
                    if (c == 'F' && tile.char == 'J') || (c == 'L' && tile.char == '7') {
                        inside = !inside;
                        c1 = None;
                    }
                }
            } else if inside {
                inside_points += 1;
            }
        }
    }
    inside_points
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
        assert_eq!(part2_f(&parse(EXAMPLE_1_PART_2)), 4);
        assert_eq!(part2_bc(&parse(EXAMPLE_1_PART_2)), 4);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2_f(&parse(EXAMPLE_2_PART_2)), 8);
        assert_eq!(part2_bc(&parse(EXAMPLE_2_PART_2)), 8);
    }

    #[test]
    fn part2_example_3() {
        assert_eq!(part2_f(&parse(EXAMPLE_3_PART_2)), 10);
        assert_eq!(part2_bc(&parse(EXAMPLE_3_PART_2)), 10);
    }
}