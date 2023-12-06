use std::iter::zip;
use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day6, part1)]
fn parse1(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (time, distance) = input.split_once('\n').unwrap();
    let time = time.split_whitespace().skip(1).map(|n| n.parse().unwrap()).collect();
    let distance = distance.split_whitespace().skip(1).map(|n| n.parse().unwrap()).collect();
    (time, distance)
}

#[aoc_generator(day6, part2)]
fn parse2(input: &str) -> (u64, u64) {
    let (time, distance) = input.split_once('\n').unwrap();
    let time = time.split(':').nth(1).unwrap().replace(' ', "");
    let distance = distance.split(':').nth(1).unwrap().replace(' ', "");
    (time.parse().unwrap(), distance.parse().unwrap())
}

#[aoc(day6, part1)]
fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (times, distances) = input;
    let mut result = 1;
    for (time, distance) in zip(times, distances) {
        let mut count = 0;
        for i in 0..=*time {
            if i * (time - i) > *distance {
                count += 1
            }
        }
        result *= count;
    }
    result
}

#[aoc(day6, part1, quadractic)]
fn part1_q(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (times, distances) = input;
    let mut result = 1;
    for (&time, &distance) in zip(times, distances) {
        let min = ((time as f32) - ((time.pow(2) - 4 * distance) as f32).sqrt()) / 2.0;
        let max = ((time as f32) + ((time.pow(2) - 4 * distance) as f32).sqrt()) / 2.0;

        result *= (max.ceil() - min.floor() - 1.0) as u32
    }
    result
}

#[aoc(day6, part2)]
fn part2(input: &(u64, u64)) -> u64 {
    let (time, distance) = input;
    let mut count = 0;
    for i in 0..=*time {
        if i * (time - i) > *distance {
            count += 1
        }
    }
    count
}

#[aoc(day6, part2, quadractic)]
fn part2_q(input: &(u64, u64)) -> u64 {
    let (time, distance) = *input;
    let min = ((time as f64) - ((time.pow(2) - 4 * distance) as f64).sqrt()) / 2.0;
    let max = ((time as f64) + ((time.pow(2) - 4 * distance) as f64).sqrt()) / 2.0;

    (max.ceil() - min.floor() - 1.0) as u64
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(EXAMPLE)), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(EXAMPLE)), 71503);
    }
}