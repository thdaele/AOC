use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{concat, Itertools};

const EXPANSION_FACTOR: u32 = 1_000_000;

#[derive(Debug)]
struct Pos {
    y: u32,
    x: u32
}

impl Pos {
    fn new(y: u32, x: u32) -> Pos {
        Pos {y, x}
    }

    fn distance(&self, other: &Pos) -> u64 {
        let y_diff = if self.y > other.y {self.y - other.y} else {other.y - self.y};
        let x_diff = if self.x > other.x {self.x - other.x} else {other.x - self.x};
        y_diff as u64 + x_diff as u64
    }
}

fn parse(input: &str, part2: bool) -> Vec<Pos> {
    let galaxy_pos: Vec<Vec<Pos>> = input.lines().enumerate()
        .map(|(y, line)|
            line.chars().enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| Pos::new(y as u32, x as u32))
                .collect()
        ).collect();
    let mut galaxy_pos = concat(galaxy_pos);

    let mut y_size = galaxy_pos.iter().max_by_key(|pos| pos.y).unwrap().y;
    let mut x_size = galaxy_pos.iter().max_by_key(|pos| pos.x).unwrap().x;

    // Expand galaxy
    // Y Expand
    let mut y = 0;
    while y < y_size {
        y += 1;
        // Check for galaxy on y
        let expand_needed = !galaxy_pos.iter().any(|pos| pos.y == y);
        if expand_needed {
            // Expand galaxy
            galaxy_pos.iter_mut()
                .filter(|pos| pos.y > y)
                .for_each(|pos| pos.y += if !part2 { 1 } else {EXPANSION_FACTOR - 1});

            y_size += if !part2 { 1 } else {EXPANSION_FACTOR - 1};
            y += if !part2 { 1 } else {EXPANSION_FACTOR - 1};
        }
    }

    // X Expand
    let mut x = 0;
    while x < x_size {
        x += 1;
        // Check for galaxy on x
        let expand_needed = !galaxy_pos.iter().any(|pos| pos.x == x);
        if expand_needed {
            // Expand galaxy
            galaxy_pos.iter_mut()
                .filter(|pos| pos.x > x)
                .for_each(|pos| pos.x += if !part2 { 1 } else {EXPANSION_FACTOR - 1});

            x_size += if !part2 { 1 } else {EXPANSION_FACTOR - 1};
            x += if !part2 { 1 } else {EXPANSION_FACTOR - 1};
        }
    }

    galaxy_pos
}

#[aoc_generator(day11, part1)]
fn parse1(input: &str) -> Vec<Pos> {
    parse(input, false)
}

#[aoc_generator(day11, part2)]
fn parse2(input: &str) -> Vec<Pos> {
    parse(input, true)
}

#[aoc(day11, part1)]
fn part1(input: &[Pos]) -> u64 {
    input.iter().combinations(2)
        .map(|comb| comb.first().unwrap().distance(comb.last().unwrap()))
        .sum()
}

#[aoc(day11, part2)]
fn part2(input: &[Pos]) -> u64 {
    input.iter().combinations(2)
        .map(|comb| comb.first().unwrap().distance(comb.last().unwrap()))
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(EXAMPLE)), 374);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(EXAMPLE)), 82000210);
    }
}