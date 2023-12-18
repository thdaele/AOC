use std::ops::Add;
use aoc_runner_derive::{aoc, aoc_generator};
use num::abs;

#[derive(Clone, Copy)]
struct Point {
    y: i64,
    x: i64
}

impl Point {
    fn new(y: i64, x: i64) -> Point {
        Point{y, x}
    }
}

impl Add<(i64, i64)> for Point {
    type Output = Point;

    fn add(self, rhs: (i64, i64)) -> Self::Output {
        Point::new(self.y + rhs.0, self.x + rhs.1)
    }
}

#[aoc_generator(day18, part1)]
fn parse_1(input: &str) -> (u64, Vec<Point>) {
    let mut border = 0;
    let mut cur = Point::new(0, 0);
    let points = input.lines().map(|line| {
        let mut parts = line.split(' ');
        let dir = parts.next().unwrap();
        let length: u64 = parts.next().unwrap().parse().unwrap();
        border += length;
        let update = match dir {
            "R" => (0, length as i64),
            "D" => (length as i64, 0),
            "L" => (0, -(length as i64)),
            "U" => (-(length as i64), 0),
            _ => unreachable!()
        };
        let new_point = cur + update;
        cur = new_point;
        new_point
    }).collect();
    (border, points)
}

#[aoc_generator(day18, part2)]
fn parse_2(input: &str) -> (u64, Vec<Point>) {
    let mut border = 0;
    let mut cur = Point::new(0, 0);
    let points = input.lines().map(|line| {
        let (_, hex) = line.split_once('#').unwrap();
        let length = &hex[0..5];
        let dir = &hex[5..6];
        let length = u64::from_str_radix(length, 16).unwrap();
        border += length;
        let update = match dir {
            "0" => (0, length as i64),
            "1" => (length as i64, 0),
            "2" => (0, -(length as i64)),
            "3" => (-(length as i64), 0),
            _ => unreachable!()
        };
        let new_point = cur + update;
        cur = new_point;
        new_point
    }).collect();
    (border, points)
}

#[aoc(day18, part1)]
fn part1(input: &(u64, Vec<Point>)) -> u64 {
    let (border, vertices) = input;
    // Shoelace formula
    let mut area = 0;
    let mut j = vertices.last().unwrap();
    for pos in vertices {
        area += (pos.x * j.y) - (pos.y * j.x);
        j = pos;
    }
    let area = abs(area / 2);

    // Pick's theorem
    let interior_points = (area - (*border as i64 / 2) + 1) as u64;
    interior_points + border
}

#[aoc(day18, part2)]
fn part2(input: &(u64, Vec<Point>)) -> u64 {
    let (border, vertices) = input;
    // Shoelace formula
    let mut area = 0;
    let mut j = vertices.last().unwrap();
    for pos in vertices {
        area += (pos.x * j.y) - (pos.y * j.x);
        j = pos;
    }
    let area = abs(area / 2);

    // Pick's theorem
    let interior_points = (area - (*border as i64 / 2) + 1) as u64;
    interior_points + border
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_1(EXAMPLE)), 62);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_2(EXAMPLE)), 952408144115);
    }
}