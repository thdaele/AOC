use std::cmp::max;
use std::collections::VecDeque;

use aoc_runner_derive::aoc;
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

fn solve(grid: &[&[u8]], start: Point, end: Point, size: (usize, usize)) -> u32 {
    debug_assert!(!grid.is_empty());
    let (y_len, x_len) = size;

    let mut seen = vec![vec![false; x_len]; y_len];

    solve_recursive(grid, &mut seen, start, end, 0, size)
}

fn solve_recursive(grid: &[&[u8]], seen: &mut Vec<Vec<bool>>, position: Point, end: Point, distance: u32, size: (usize, usize)) -> u32 {
    let (y_len, x_len) = size;

    if position == end {
        return distance
    }

    let mut result = 0;

    let tile = grid[position.y as usize][position.x as usize];
    let mut next_directions = Vec::with_capacity(4);
    match tile {
        b'^' => next_directions.push(Direction::North),
        b'>' => next_directions.push(Direction::East),
        b'v' => next_directions.push(Direction::South),
        b'<' => next_directions.push(Direction::West),
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
        if !seen[neighbour.y as usize][neighbour.x as usize] {
            seen[neighbour.y as usize][neighbour.x as usize] = true;
            let dist = solve_recursive(grid, seen, neighbour, end, distance + 1, size);
            result = max(result, dist);
            seen[neighbour.y as usize][neighbour.x as usize] = false;
        }
    }
    result
}

fn bfs(grid: &[&[u8]], poi: &FxHashSet<Point>, start: Point, size: (usize, usize)) -> Vec<(Point, u32)> {
    debug_assert!(!grid.is_empty());
    let (y_len, x_len) = size;

    let mut todo = VecDeque::new();
    let mut seen = FxHashSet::default();
    let mut result = Vec::new();

    todo.push_back((start, 0));
    seen.insert(start);

    while let Some((position, distance)) = todo.pop_front() {
        if position != start && poi.contains(&position) {
            result.push((position, distance));
            continue;
        }

        let directions = [Direction::North, Direction::South, Direction::East, Direction::West];
        let neighbours: Vec<Point> = directions.iter().filter_map(|&dir| {
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
            if seen.insert(neighbour) {
                todo.push_back((neighbour, distance + 1));
            }
        }
    }
    result
}

fn solve_recursive_2(edges: &FxHashMap<&Point, Vec<(Point, u32)>>, seen: &mut FxHashMap<&Point, bool>, position: Point, end: Point, distance: u32) -> u32 {
    if position == end {
        return distance
    }
    let mut result = 0;

    for &(next, extra) in &edges[&position] {
        if !seen[&next] {
            *seen.get_mut(&next).unwrap() = true;
            let dist = solve_recursive_2(edges, seen, next, end, distance + extra);
            result = max(result, dist);
            *seen.get_mut(&next).unwrap() = false;
        }
    }
    result
}

fn solve_2(grid: &[&[u8]], start: Point, end: Point, size: (usize, usize)) -> u32 {
    debug_assert!(!grid.is_empty());
    let (y_len, x_len) = size;

    let mut poi = FxHashSet::default();
    poi.insert(start);
    poi.insert(end);

    let directions = [Direction::North, Direction::South, Direction::East, Direction::West];
    for y in 0..y_len {
        for x in 0..x_len {
            if grid[y][x] != b'#' {
                let position = Point::new(y as u8, x as u8);
                let neighbours = directions.iter().filter_map(|&dir| {
                    let pos = position.update(y_len as u8, x_len as u8, dir);
                    if let Some(pos) = pos {
                        if grid[pos.y as usize][pos.x as usize] == b'#' {
                            return None
                        }
                        return Some(pos)
                    }
                    None
                }).count();
                if neighbours > 2 {
                    // Not in a corridor
                    poi.insert(position);
                }
            }
        }
    }
    let mut edges = FxHashMap::default();
    let mut seen = FxHashMap::default();

    for start in &poi {
        edges.insert(start, bfs(grid, &poi, *start, size));
        seen.insert(start, false);
    }

    solve_recursive_2(&edges, &mut seen, start, end, 0)
}

#[aoc(day23, part1)]
fn part1(input: &str) -> u32 {
    let (grid, size) = parse(input);
    let (start, end) = find_start_end(&grid);

    solve(&grid, start, end, size)
}

#[aoc(day23, part2)]
fn part2(input: &str) -> u32 {
    let (grid, size) = parse(input);
    let (start, end) = find_start_end(&grid);

    solve_2(&grid, start, end, size)
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