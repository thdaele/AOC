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

fn solve(rocks: &[Vec<u32>], start: &Point, size: &Point, step_count: u32, part2: bool) -> Vec<i32> {
    let mut gardens = FxHashSet::default();
    gardens.insert(*start);

    let mut previous = FxHashSet::default();

    let mut counts = vec![1];

    for _ in 1..=step_count {
        let curr = gardens.clone();

        gardens = gardens.iter().flat_map(|&garden| {
            let mut new_gardens = vec![];
            for offset in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let new_garden = garden + offset;
                if !part2 && !new_garden.bound_check(*size) {
                    continue;
                }
                if previous.contains(&new_garden) {
                    // Prevents us from going back to a position from 2 steps ago
                    // After a position is visited at time x we can always get back there at time x + multiple of 2
                    // Like this node -> neighbour -> node and we can keep repeating this
                    continue;
                }
                // Check for rocks
                let row = rocks.get(new_garden.y.rem_euclid(size.y) as usize).unwrap();
                if row.contains(&(new_garden.x.rem_euclid(size.x) as u32)) {
                    continue;
                }
                new_gardens.push(new_garden);
            }
            new_gardens
        }).collect();

        counts.push(gardens.len() as i32);

        previous = curr;
    }
    counts
}

#[aoc(day21, part1)]
fn part1(input: &(Vec<Vec<u32>>, Point, Point)) -> i32 {
    let (rocks, start, size) = input;
    let counts = solve(rocks, start, size, 64, false);
    counts.iter().step_by(2).sum()
}

fn part2_solver(iterations: u32, sequence_length: u8, rocks: &[Vec<u32>], start: &Point, size: &Point) -> i64 {
    debug_assert_eq!(size.y, size.x);
    debug_assert!(sequence_length > 0);
    let remainder = iterations.rem_euclid(size.y as u32);

    // If the divisor in our modulo operation is uneven we are switching between nodes reached in
    // even and uneven steps, this won't let us find an easy polynomial
    // so we make the divisor even
    let mut divisor = size.y as u32;
    if divisor % 2 == 1 {
        divisor *= 2;
    }
    let step_count = remainder + divisor * (sequence_length - 1) as u32;
    let counts = solve(rocks, start, size, step_count, true);
    debug_assert!(!counts.is_empty());
    let mut sequence: Vec<i32> = vec![];
    sequence.push(counts[..=remainder as usize].iter().skip((remainder % 2) as usize).step_by(2).sum());

    for i in 1..sequence_length {
        let previous = sequence[i as usize - 1];
        let start_index = (remainder + (i as u32 - 1) * divisor) as usize;
        let end_index = (remainder + i as u32 * divisor) as usize;
        // skip(2) to not count start_index double
        sequence.push(previous + counts[start_index..=end_index].iter().skip(2).step_by(2).sum::<i32>());
    }
    debug_assert_eq!(sequence.len(), sequence_length as usize);

    let quotient = iterations.div_euclid(divisor) as i64;

    // This code is to make the examples work bc they don't have the same nice properties as the
    // actual input
    // For the examples first part of the sequence is irregular so we don't use it to construct
    // the polynomial
    let i = 50u32.saturating_sub(remainder).div_ceil(divisor);
    let quotient = quotient - i as i64;
    sequence = sequence[i as usize..].to_vec();
    // Till here

    let sequence_length = sequence.len();
    let mut sequence_depth = 0;
    // let mut string = "".to_string();
    // let mut prefix = "".to_string();
    let mut result = 0;
    let mut multiply = 1;
    let mut factorial = 1;
    while !sequence.iter().all(|&value| value == 0) {
        if sequence_depth == 0 {
            // string.push_str(sequence.first().unwrap().to_string().as_str());
            result += *sequence.first().unwrap() as i64;
        } else {
            multiply *= quotient - (sequence_depth - 1);
            factorial *= sequence_depth;
            result += multiply * (*sequence.first().unwrap() as i64 / factorial);

            // string.push_str(format!(" + (i - {}) {prefix} * ({} / {}!)", sequence_depth - 1,
            //                         sequence.first().unwrap(), sequence_depth).as_str());
            // prefix.push_str(format!("* (i - {})", sequence_depth - 1).as_str());
        }
        sequence_depth += 1;

        // println!("{:?}", sequence);

        let mut next_sequence = vec![];
        for window in sequence.windows(2) {
            next_sequence.push(window.last().unwrap() - window.first().unwrap());
        }
        sequence = next_sequence;
    }
    if sequence_length == sequence_depth as usize {
        println!("Might have not enough datapoints to construct the correct polynomial")
    }
    // println!("{}", string);
    result
}

#[aoc(day21, part2)]
fn part2(input: &(Vec<Vec<u32>>, Point, Point)) -> i64 {
    let (rocks, start, size) = input;

    part2_solver(26501365, 5, rocks, start, size)
}


#[cfg(test)]
mod tests {
    use std::iter::zip;
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
        let counts = solve(rocks, start, size, 6, false);
        assert_eq!(counts.iter().step_by(2).sum::<i32>(), 16);
    }

    #[test]
    fn part2_example() {
        let (rocks, start, size) = &parse(EXAMPLE);
        let iterations = vec![6, 10, 50, 100, 500, 1000, 5000];
        let results = vec![16, 50, 1594, 6536, 167004, 668697, 16733044];

        for (iteration, result) in zip(iterations, results) {
            if iteration <= 500 {
                // Solve using part 1
                let counts = solve(rocks, start, size, iteration, true);
                assert_eq!(counts.iter().step_by(2).sum::<i32>(), result);
            } else {
                assert_eq!(part2_solver(iteration, 10, rocks, start, size), result as i64);
            }
        }
    }
}