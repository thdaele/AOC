use std::ops::{Add, Mul, Sub};
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
struct ParseVec3DError;

#[derive(PartialEq, Copy, Clone)]
struct Vec3D {
    x: i64,
    y: i64,
    z: i64
}

impl FromStr for Vec3D {
    type Err = ParseVec3DError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.splitn(3, ',');
        let x = coords.next().ok_or(ParseVec3DError)?.trim().parse().unwrap();
        let y = coords.next().ok_or(ParseVec3DError)?.trim().parse().unwrap();
        let z = coords.next().ok_or(ParseVec3DError)?.trim().parse().unwrap();
        Ok(Vec3D{x, y, z})
    }
}

impl Add for Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3D {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl Sub for Vec3D {
    type Output = Vec3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3D {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl Mul<i64> for Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: i64) -> Self::Output {
        Vec3D {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

#[derive(PartialEq)]
struct Ray {
    position: Vec3D,
    velocity: Vec3D
}

fn cross(vec1: &Vec3D, vec2: &Vec3D) -> i64 {
    vec1.x * vec2.y - vec1.y * vec2.x
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRayError;

impl FromStr for Ray {
    type Err = ParseRayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once('@').ok_or(ParseRayError)?;
        Ok(Ray{position: position.parse().unwrap(), velocity: velocity.parse().unwrap()})
    }
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Vec<Ray> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve(rays: &[Ray], min: i64, max: i64) -> u32 {
    let mut result = 0;

    // https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
    for i in 0..rays.len() {
        for j in i + 1..rays.len() {
            let ray1 = &rays[i];
            let ray2 = &rays[j];
            if ray1 == ray2 {
                continue;
            }
            let rxs = cross(&ray1.velocity, &ray2.velocity);
            if rxs == 0 { // No intersection
                continue
            }
            let p_diff = ray2.position - ray1.position;
            let t = cross(&p_diff, &ray2.velocity) / rxs;
            let u = cross(&p_diff, &ray1.velocity) / rxs;
            if t < 0 || u < 0 { // Past
                continue
            }
            let intersection = ray1.position + ray1.velocity * t;
            if intersection.x >= min && intersection.x <= max
                && intersection.y >= min && intersection.y <= max {
                result += 1;
            }
        }
    }
    result
}

#[aoc(day24, part1)]
fn part1(input: &[Ray]) -> u32 {
    solve(input, 200000000000000, 400000000000000)
}

#[aoc(day24, part2)]
fn part2(input: &[Ray]) -> u32 {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn part1_example() {
        assert_eq!(solve(&parse(EXAMPLE), 7, 27), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 47);
    }
}