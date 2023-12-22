use std::cmp::{max, min};
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: u32,
    y: u32,
    z: u32
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl Point {
    fn new(x: u32, y: u32, z: u32) -> Point {
        Point {x, y, z}
    }
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, ',');
        let x = parts.next().ok_or(ParsePointError)?.parse().unwrap();
        let y = parts.next().ok_or(ParsePointError)?.parse().unwrap();
        let z = parts.next().ok_or(ParsePointError)?.parse().unwrap();
        Ok(Point {x, y, z})
    }
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<(Point, Point)> {
    input.lines().map(|line| {
        let (cube1, cube2) = line.split_once('~').unwrap();
        (cube1.parse().unwrap(), cube2.parse().unwrap())
    }).collect()
}

fn expand(brick: &(Point, Point)) -> Vec<Point> {
    let mut result = vec![];

    if brick.0.z != brick.1.z {
        for z in min(brick.0.z, brick.1.z)..=max(brick.0.z, brick.1.z) {
            result.push(Point::new(brick.0.x, brick.0.y, z));
        }
    } else if brick.0.y != brick.1.y {
        for y in min(brick.0.y, brick.1.y)..=max(brick.0.y, brick.1.y) {
            result.push(Point::new(brick.0.x, y, brick.0.z));
        }
    } else if brick.0.x != brick.1.x {
        for x in min(brick.0.x, brick.1.x)..=max(brick.0.x, brick.1.x) {
            result.push(Point::new(x, brick.0.y, brick.0.z));
        }
    } else {
        // Single brick situation
        result.push(brick.0);
    }
    result
}

fn fall(bricks: &[Vec<Point>], ignore: Option<&Vec<Point>>) -> (Vec<Vec<Point>>, u32) {
    let mut bricks = bricks.to_vec();
    bricks.sort_unstable_by_key(|points| points.iter().min_by_key(|p| p.z).unwrap().z);

    let mut fallen: FxHashSet<Point> = FxHashSet::default();
    let mut new = vec![];

    let mut fallen_count = 0;

    for brick in &bricks {
        if Some(brick) == ignore {
            continue;
        }
        let mut brick_clone = brick.clone();

        loop {
            let mut brick_update = brick_clone.clone();
            brick_update.iter_mut().for_each(|p| p.z -= 1);

            if brick_update.iter().any(|p| fallen.contains(p) || p.z == 0) {
                new.push(brick_clone.clone());
                fallen.extend(brick_clone.clone());

                if &brick_clone != brick {
                    fallen_count += 1;
                }
                break
            }
            brick_clone = brick_update;
        }
    }
    (new, fallen_count)
}

#[aoc(day22, part1)]
fn part1(input: &[(Point, Point)]) -> usize {
    let mut bricks: Vec<Vec<Point>> = input.iter().map(expand).collect();

    (bricks, _) = fall(&bricks, None);
    bricks.iter().map(|brick| fall(&bricks, Some(brick)).1).filter(|&v| v == 0).count()
}

#[aoc(day22, part2)]
fn part2(input: &[(Point, Point)]) -> u32 {
    let mut bricks: Vec<Vec<Point>> = input.iter().map(expand).collect();

    (bricks, _) = fall(&bricks, None);
    bricks.iter().map(|brick| fall(&bricks, Some(brick)).1).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 7);
    }
}