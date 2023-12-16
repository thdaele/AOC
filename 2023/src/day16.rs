use std::collections::VecDeque;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::max;
use rustc_hash::FxHashSet;
use rayon::prelude::*;

#[aoc_generator(day16)]
fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
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
    y: u32,
    x: u32
}

impl Point {
    fn new(y: u32, x: u32) -> Point {
        Point{y, x}
    }

    fn update(&self, y_len: u32, x_len: u32, direction: Direction) -> Option<Point> {
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

fn solve(grid: &[Vec<char>], start: Point, direction: Direction, size: (usize, usize)) -> usize {
    assert!(!grid.is_empty());
    let (y_len, x_len) = size;
    let mut queue = VecDeque::from([(start, direction)]);
    let mut visited: FxHashSet<(Point, Direction)> = FxHashSet::default();

    while let Some((point, direction)) = queue.pop_front() {
        if !visited.contains(&(point, direction)) {
            visited.insert((point, direction));

            let char = grid[point.y as usize][point.x as usize];
            match char {
                '/' => {
                    let updated_direction = match direction {
                        Direction::North => Direction::East,
                        Direction::South => Direction::West,
                        Direction::East => Direction::North,
                        Direction::West => Direction::South,
                    };
                    let updated_point = point.update(y_len as u32, x_len as u32, updated_direction);
                    if let Some(updated_point) = updated_point {
                        queue.push_back((updated_point, updated_direction));
                    }
                },
                '\\' => {
                    let updated_direction = match direction {
                        Direction::North => Direction::West,
                        Direction::South => Direction::East,
                        Direction::East => Direction::South,
                        Direction::West => Direction::North,
                    };
                    let updated_point = point.update(y_len as u32, x_len as u32, updated_direction);
                    if let Some(updated_point) = updated_point {
                        queue.push_back((updated_point, updated_direction));
                    }
                },
                '|' => {
                    if direction == Direction::East || direction == Direction::West {
                        let updated_point = point.update(y_len as u32, x_len as u32, Direction::North);
                        if let Some(updated_point) = updated_point {
                            queue.push_back((updated_point, Direction::North));
                        }
                        let updated_point = point.update(y_len as u32, x_len as u32, Direction::South);
                        if let Some(updated_point) = updated_point {
                            queue.push_back((updated_point, Direction::South));
                        }
                    } else {
                        let updated_point = point.update(y_len as u32, x_len as u32, direction);
                        if let Some(updated_point) = updated_point {
                            queue.push_back((updated_point, direction));
                        }
                    }
                },
                '-' => {
                    if direction == Direction::North || direction == Direction::South {
                        let updated_point = point.update(y_len as u32, x_len as u32, Direction::East);
                        if let Some(updated_point) = updated_point {
                            queue.push_back((updated_point, Direction::East));
                        }
                        let updated_point = point.update(y_len as u32, x_len as u32, Direction::West);
                        if let Some(updated_point) = updated_point {
                            queue.push_back((updated_point, Direction::West));
                        }
                    } else {
                        let updated_point = point.update(y_len as u32, x_len as u32, direction);
                        if let Some(updated_point) = updated_point {
                            queue.push_back((updated_point, direction));
                        }
                    }
                },
                '.' => {
                    let updated_point = point.update(y_len as u32, x_len as u32, direction);
                    if let Some(updated_point) = updated_point {
                        queue.push_back((updated_point, direction));
                    }
                },
                _ => unreachable!()
            }
        }
    }
    // Print the grid
    let set: FxHashSet<&Point> = visited.iter().map(|(point, _)| point).collect();
    // let mut string = "".to_string();
    // for (y, row) in grid.iter().enumerate() {
    //     for (x, char) in row.iter().enumerate() {
    //         let point = Point::new(y as u32, x as u32);
    //         if set.contains(&point) {
    //             string.push('#');
    //         } else {
    //             string.push('.');
    //         }
    //     }
    //     string.push('\n');
    // }
    // println!("{string}");

    set.len()
}

#[aoc(day16, part1)]
fn part1(input: &(Vec<Vec<char>>, (usize, usize))) -> usize {
    let (grid, size) = input;
    solve(grid, Point::new(0, 0), Direction::East, *size)
}

#[aoc(day16, part2)]
fn part2(input: &(Vec<Vec<char>>, (usize, usize))) -> usize {
    let (grid, size) = input;
    let (y_len, x_len) = size;

    let max1 = (0..*y_len).into_par_iter().map(|y| solve(grid, Point::new(y as u32, 0), Direction::East, *size)).max().unwrap();
    let max2 = (0..*y_len).map(|y| solve(grid, Point::new(y as u32, *x_len as u32 - 1), Direction::West, *size)).max().unwrap();
    let max3 = (0..*x_len).map(|x| solve(grid, Point::new(0, x as u32), Direction::South, *size)).max().unwrap();
    let max4 = (0..*x_len).map(|x| solve(grid, Point::new(*y_len as u32 - 1, x as u32), Direction::North, *size)).max().unwrap();

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
        assert_eq!(part1(&parse(EXAMPLE)), 46);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 51);
    }
}