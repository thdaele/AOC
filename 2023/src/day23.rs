use std::cmp::{max, Reverse};
use std::collections::{BinaryHeap, VecDeque};
use aoc_runner_derive::aoc;
use num::abs;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn next(&self) -> [Direction; 2] {
        match self {
            Direction::North => [Direction::East, Direction::West],
            Direction::East => [Direction::South, Direction::North],
            Direction::South => [Direction::West, Direction::East],
            Direction::West => [Direction::North, Direction::South]
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    y: u8,
    x: u8
}

impl Point {
    fn new(y: u8, x: u8) -> Point {
        Point{y, x}
    }

    fn update(&self, y_len: u8, x_len: u8, direction: Direction) -> Option<Point> {
        if (direction == Direction::North && self.y == 0)
            || (direction == Direction::South && self.y == y_len - 1)
            || (direction == Direction::East && self.x == x_len - 1)
            || (direction == Direction::West && self.x == 0) {
            return None
        }
        let (y, x) = match direction {
            Direction::North => (self.y - 1, self.x),
            Direction::South => (self.y + 1, self.x),
            Direction::East => (self.y, self.x + 1),
            Direction::West => (self.y, self.x - 1)
        };
        Some(Point::new(y, x))
    }
}


// #[aoc_generator(day23)]
fn parse(input: &str) -> (Vec<&[u8]>, (usize, usize)) {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    debug_assert!(!grid.is_empty());
    let y_len = grid.len();
    let x_len = grid.first().unwrap().len();
    (grid, (y_len, x_len))
}

fn find_start_end(grid: &Vec<&[u8]>) -> (Point, Point) {
    let mut start = None;
    let mut end = None;

    assert!(!grid.is_empty());
    // Start is single path tile on the top row
    let top_row = grid.first().unwrap();
    for (x, item) in top_row.iter().enumerate() {
        if *item == b'.' {
            start = Some(Point::new(0, x as u8));
            break
        }
    }

    let bottom_row = grid.last().unwrap();
    for (x, item) in bottom_row.iter().enumerate() {
        if *item == b'.' {
            end = Some(Point::new((grid.len() - 1) as u8, x as u8));
            break
        }
    }

    assert!(start.is_some() && end.is_some());
    (start.unwrap(), end.unwrap())
}

// instead of copying seen, we can make it a boolean grid and set seen to true before recursive call
// and back to false after the recursive call ends
fn solve(grid: &[&[u8]], start: Point, end: Point, size: (usize, usize), part2: bool) -> u32 {
    debug_assert!(!grid.is_empty());
    let (y_len, x_len) = size;

    let mut result = 0;

    let mut todo = VecDeque::new();
    let mut seen = FxHashSet::default();

    seen.insert(start);
    todo.push_back((0u32, start, seen));

    while let Some((distance, position, seen)) = todo.pop_front() {
        if position == end {
            result = max(result, distance);
            continue
        }

        let tile = grid[position.y as usize][position.x as usize];
        let mut next_directions = Vec::with_capacity(4);
        match tile {
            b'^' if !part2 => next_directions.push(Direction::North),
            b'>' if !part2 => next_directions.push(Direction::East),
            b'v' if !part2 => next_directions.push(Direction::South),
            b'<' if !part2 => next_directions.push(Direction::West),
            _ => next_directions.extend([Direction::North, Direction::South, Direction::East, Direction::West])
        }

        let neighbours: Vec<Point> = next_directions.iter().filter_map(|&dir| {
            let pos = position.update(y_len as u8, x_len as u8, dir);
            if let Some(pos) = pos {
                if grid[pos.y as usize][pos.x as usize] == b'#' {
                    return None
                }
                return Some(pos)
            }
            None
        }).collect();

        for neighbour in neighbours {
            let mut seen_copy = seen.clone();
            if seen_copy.insert(neighbour) {
                todo.push_back((distance + 1, neighbour, seen_copy));
            }
        }
    }
    result
}

#[aoc(day23, part1)]
fn part1(input: &str) -> u32 {
    let (grid, size) = parse(input);
    let (start, end) = find_start_end(&grid);

    solve(&grid, start, end, size, false)
}

#[aoc(day23, part2)]
fn part2(input: &str) -> u32 {
    let (grid, size) = parse(input);
    let (start, end) = find_start_end(&grid);

    solve(&grid, start, end, size, true)
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 94);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 154);
    }
}