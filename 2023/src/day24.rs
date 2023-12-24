use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet;

#[derive(Debug, PartialEq, Eq)]
struct ParseVec3DError;

#[derive(Debug, Copy, Clone)]
struct Vec3D {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3D {
    fn sum(&self) -> f64 {
        self.x + self.y + self.z
    }
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

impl Mul<f64> for Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3D {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl Mul for Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3D {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

impl PartialEq<Self> for Vec3D {
    fn eq(&self, other: &Self) -> bool {
        self.x.round() == other.x.round() && self.y.round() == other.y.round() && self.z.round() == other.z.round()
    }
}

#[derive(PartialEq, Copy, Clone)]
struct Hailstone {
    position: Vec3D,
    velocity: Vec3D
}

fn cross(vec1: &Vec3D, vec2: &Vec3D) -> f64 {
    vec1.x * vec2.y - vec1.y * vec2.x
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRayError;

impl FromStr for Hailstone {
    type Err = ParseRayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once('@').ok_or(ParseRayError)?;
        Ok(Hailstone {position: position.parse().unwrap(), velocity: velocity.parse().unwrap()})
    }
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Vec<Hailstone> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve(hailstones: &[Hailstone], min: f64, max: f64) -> u32 {
    let mut result = 0;

    // https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            let hailstone1 = &hailstones[i];
            let hailstone2 = &hailstones[j];
            let rxs = cross(&hailstone1.velocity, &hailstone2.velocity);
            if rxs == 0f64 { // No intersection
                continue
            }
            let p_diff = hailstone2.position - hailstone1.position;
            let t = cross(&p_diff, &hailstone2.velocity) / rxs;
            let u = cross(&p_diff, &hailstone1.velocity) / rxs;
            if t < 0f64 || u < 0f64 { // Past
                continue
            }
            let intersection = hailstone1.position + hailstone1.velocity * t;
            if intersection.x >= min && intersection.x <= max
                && intersection.y >= min && intersection.y <= max {
                result += 1;
            }
        }
    }
    result
}

fn intersection_3d(p1: Vec3D, p2: Vec3D, v1: Vec3D, v2: Vec3D) -> Option<(Vec3D, Vec3D)> {
    let p_diff = p1 - p2;
    let norm_p_diff_ray2 = (p_diff * v2).sum();
    let norm_p_diff_ray1 = (p_diff * v1).sum();

    let norm_ray1_ray2 = (v1 * v2).sum();

    let norm_ray2 = (v2 * v2).sum();
    let norm_ray1 = (v1 * v1).sum();

    let denom = norm_ray1 * norm_ray2 - norm_ray1_ray2 * norm_ray1_ray2;
    if denom == 0f64 {
        return None
    }
    let numer = norm_p_diff_ray2 * norm_ray1_ray2 - norm_p_diff_ray1 * norm_ray2;
    let t = numer / denom;
    let v = (norm_p_diff_ray2 + norm_ray1_ray2 * t) / norm_ray2;

    // Create the 2 points
    let p1 = p1 + v1 * t;
    let p2 = p2 + v2 * v;
    Some((p1, p2))
}

#[aoc(day24, part1)]
fn part1(hailstones: &[Hailstone]) -> u32 {
    solve(hailstones, 200000000000000f64, 400000000000000f64)
}

#[aoc(day24, part2)]
fn part2(hailstones: &[Hailstone]) -> f64 {
    let mut dx_candidate = FxHashSet::from_iter(-5000..=5000);
    let mut dy_candidate = FxHashSet::from_iter(-5000..=5000);
    let mut dz_candidate = FxHashSet::from_iter(-5000..=5000);

    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            let hailstone1 = &hailstones[i];
            let hailstone2 = &hailstones[j];

            if hailstone1.velocity.x == hailstone2.velocity.x {
                let mut result = FxHashSet::default();
                dx_candidate.iter().for_each(|&v| {
                    if v as f64 != hailstone1.velocity.x.round() && (hailstone1.position.x - hailstone2.position.x).rem_euclid(v as f64 - hailstone1.velocity.x).round() == 0f64 {
                        result.insert(v);
                    }
                });
                dx_candidate = dx_candidate.intersection(&result).copied().collect();
            }
            if hailstone1.velocity.y == hailstone2.velocity.y {
                let mut result = FxHashSet::default();
                dy_candidate.iter().for_each(|&v| {
                    if v as f64 != hailstone1.velocity.y.round() && (hailstone1.position.y - hailstone2.position.y).rem_euclid(v as f64 - hailstone1.velocity.y).round() == 0f64 {
                        result.insert(v);
                    }
                });
                dy_candidate = dy_candidate.intersection(&result).copied().collect();
            }
            if hailstone1.velocity.z == hailstone2.velocity.z {
                let mut result = FxHashSet::default();
                dz_candidate.iter().for_each(|&v| {
                    if v as f64 != hailstone1.velocity.z.round() && (hailstone1.position.z - hailstone2.position.z).rem_euclid(v as f64 - hailstone1.velocity.z).round() == 0f64 {
                        result.insert(v);
                    }
                });
                dz_candidate = dz_candidate.intersection(&result).copied().collect();
            }
        }
    }
    println!("{:?} {:?} {:?}", dx_candidate, dy_candidate, dz_candidate);

    let hailstone1 = hailstones[0];
    let hailstone2 = hailstones[1];
    let hailstone3 = hailstones[2];
    for x in &dx_candidate {
        for y in &dy_candidate {
            for z in &dz_candidate {
                let v = Vec3D{x: *x as f64, y: *y as f64, z: *z as f64};
                let p1 = intersection_3d(hailstone1.position, hailstone2.position, hailstone1.velocity - v, hailstone2.velocity - v);
                let p2 = intersection_3d(hailstone2.position, hailstone3.position, hailstone2.velocity - v, hailstone3.velocity - v);

                if let Some((p11, p12)) = p1 {
                    if let Some((p21, p22)) = p2 {
                        if p11 == p12 && p21 == p22 && p11 == p22 {
                            return p11.x + p11.y + p11.z;
                        }
                    }
                }
            }
        }
    }
    unreachable!()
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
        assert_eq!(solve(&parse(EXAMPLE), 7f64, 27f64), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 47f64);
    }

    #[test]
    fn intersection_3d_test() {
        let p1 = Vec3D{x: 24f64, y: 13f64, z: 10f64};
        let v1 = Vec3D{x: -3f64, y: 1f64, z: 2f64};

        let p2 = Vec3D{x: 19f64, y: 13f64, z: 30f64};
        let v2 = Vec3D{x: -2f64, y: 1f64, z: -2f64};

        let (p1, p2) = intersection_3d(p1, p2, v1, v2).unwrap();
        assert_eq!(p1, p2);
    }
}