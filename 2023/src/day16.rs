use std::collections::VecDeque;

use aoc_runner_derive::aoc;
use itertools::max;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

// #[aoc_generator(day16)]
fn parse(input: &str) -> (Box<[&[u8]]>, (usize, usize)) {
    let grid: Box<[&[u8]]> = input.lines().map(|line| line.as_bytes()).collect::<Vec<&[u8]>>().into_boxed_slice();
    let y_len = grid.len();
    let x_len = grid.first().unwrap().len();
    (grid, (y_len, x_len))
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
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

fn solve(grid: &[&[u8]], start: Point, direction: Direction, size: (usize, usize)) -> usize {
    debug_assert!(!grid.is_empty());
    let (y_len, x_len) = size;
    let mut queue = VecDeque::from([(start, direction)]);
    let mut visited: FxHashSet<(Point, Direction)> = FxHashSet::default();

    while let Some((point, direction)) = queue.pop_front() {
        if visited.insert((point, direction)) {
            let char = grid[point.y as usize][point.x as usize];
            match char {
                b'/' => {
                    let updated_direction = match direction {
                        Direction::North => Direction::East,
                        Direction::South => Direction::West,
                        Direction::East => Direction::North,
                        Direction::West => Direction::South,
                    };
                    if let Some(updated_point) = point.update(y_len as u8, x_len as u8, updated_direction) {
                        queue.push_back((updated_point, updated_direction));
                    }
                },
                b'\\' => {
                    let updated_direction = match direction {
                        Direction::North => Direction::West,
                        Direction::South => Direction::East,
                        Direction::East => Direction::South,
                        Direction::West => Direction::North,
                    };
                    if let Some(updated_point) = point.update(y_len as u8, x_len as u8, updated_direction) {
                        queue.push_back((updated_point, updated_direction));
                    }
                },
                b'|' => {
                    if direction == Direction::East || direction == Direction::West {
                        if let Some(updated_point) = point.update(y_len as u8, x_len as u8, Direction::North) {
                            queue.push_back((updated_point, Direction::North));
                        }
                        if let Some(updated_point) = point.update(y_len as u8, x_len as u8, Direction::South) {
                            queue.push_back((updated_point, Direction::South));
                        }
                    } else if let Some(updated_point) = point.update(y_len as u8, x_len as u8, direction) {
                        queue.push_back((updated_point, direction));
                    }
                },
                b'-' => {
                    if direction == Direction::North || direction == Direction::South {
                        if let Some(updated_point) = point.update(y_len as u8, x_len as u8, Direction::East) {
                            queue.push_back((updated_point, Direction::East));
                        }
                        if let Some(updated_point) = point.update(y_len as u8, x_len as u8, Direction::West) {
                            queue.push_back((updated_point, Direction::West));
                        }
                    } else if let Some(updated_point) = point.update(y_len as u8, x_len as u8, direction) {
                        queue.push_back((updated_point, direction));
                    }
                },
                b'.' => {
                    if let Some(updated_point) = point.update(y_len as u8, x_len as u8, direction) {
                        queue.push_back((updated_point, direction));
                    }
                },
                _ => unreachable!()
            }
        }
    }
    let set: FxHashSet<&Point> = visited.iter().map(|(point, _)| point).collect();
    set.len()
}

#[aoc(day16, part1)]
fn part1(input: &str) -> usize {
    let (grid, size) = parse(input);
    solve(&grid, Point::new(0, 0), Direction::East, size)
}

#[aoc(day16, part2)]
fn part2(input: &str) -> usize {
    let (grid, size) = parse(input);
    let (y_len, x_len) = size;

    let max1 = (0..y_len).into_par_iter().map(|y| solve(&grid, Point::new(y as u8, 0), Direction::East, size)).max().unwrap();
    let max2 = (0..y_len).into_par_iter().map(|y| solve(&grid, Point::new(y as u8, x_len as u8 - 1), Direction::West, size)).max().unwrap();
    let max3 = (0..x_len).into_par_iter().map(|x| solve(&grid, Point::new(0, x as u8), Direction::South, size)).max().unwrap();
    let max4 = (0..x_len).into_par_iter().map(|x| solve(&grid, Point::new(y_len as u8 - 1, x as u8), Direction::North, size)).max().unwrap();

    max([max1, max2, max3, max4]).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 46);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 51);
    }
}