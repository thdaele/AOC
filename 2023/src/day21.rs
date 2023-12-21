use std::collections::VecDeque;
use std::ops::Add;
use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    y: i32,
    x: i32
}

impl Point {
    fn new(y: i32, x: i32) -> Point {
        Point{y, x}
    }

    fn bound_check(&self, size: Point) -> bool {
        if self.y < 0 || self.x < 0 {
            return false
        }
        if self.y >= size.y || self.x >= size.x {
            return false
        }
        true
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Point::new(self.y + rhs.0, self.x + rhs.1)
    }
}

#[aoc_generator(day21)]
fn parse(input: &str) -> (Vec<Vec<u32>>, Point, Point) {
    let mut start = None;
    let mut rocks = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => row.push(x as u32),
                'S' => start = Some(Point::new(y as i32, x as i32)),
                _ => continue
            }
        }
        rocks.push(row);
    }
    let y_size = rocks.len() as i32;
    let x_size = input.find(|char| char == '\n').unwrap() as i32;
    debug_assert!(start.is_some());
    (rocks, start.unwrap(), Point::new(y_size, x_size))
}

fn solve(rocks: &[Vec<u32>], start: &Point, size: &Point, step_size: u32, part2: bool) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((*start, 0));

    let mut gardens = FxHashSet::default();
    let mut visited = FxHashSet::default();

    while let Some((position, steps)) = queue.pop_front() {
        if !visited.insert((position, steps)) {
            continue;
        }
        if steps == step_size {
            // Add point to visited gardens
            gardens.insert(position);
            continue
        }
        for offset in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let new_pos = position + offset;
            if !part2 && !new_pos.bound_check(*size) {
                continue;
            }
            // Check for rocks
            let row = rocks.get((new_pos.y % size.y) as usize).unwrap();
            if row.contains(&((new_pos.x % size.y) as u32)) {
                continue;
            }
            queue.push_back((new_pos, steps + 1));
        }
    }
    gardens.len()
}

#[aoc(day21, part1)]
fn part1(input: &(Vec<Vec<u32>>, Point, Point)) -> usize {
    let (rocks, start, size) = input;
    solve(rocks, start, size, 64, false)
}

#[aoc(day21, part2)]
fn part2(input: &(Vec<Vec<u32>>, Point, Point)) -> usize {
    let (rocks, start, size) = input;

    0
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn part1_example() {
        let (rocks, start, size) = &parse(EXAMPLE);
        assert_eq!(solve(rocks, start, size, 6, false), 16);
    }

    #[test]
    fn part2_example() {
        let (rocks, start, size) = &parse(EXAMPLE);
        assert_eq!(solve(rocks, start, size, 6, false), 16);
        assert_eq!(solve(rocks, start, size, 10, false), 50);
        assert_eq!(solve(rocks, start, size, 50, false), 1594);
        assert_eq!(solve(rocks, start, size, 100, false), 6536);
        assert_eq!(solve(rocks, start, size, 500, false), 167004);
        assert_eq!(solve(rocks, start, size, 1000, false), 668697);
        assert_eq!(solve(rocks, start, size, 5000, false), 16733044);
    }
}