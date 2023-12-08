use aoc_runner_derive::aoc;
use num::Integer;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

// #[aoc_generator(day8)]
fn parse(input: &str) -> (Vec<char>, FxHashMap<&str, (&str, &str)>) {
    let (movement, connections) = input.split_once("\n\n").unwrap();
    let movement = movement.chars().collect();

    let mut map = FxHashMap::default();
    for line in connections.lines() {
        let (key, values) = line.split_once('=').unwrap();
        let (left, right) = values.trim().split_once(',').unwrap();
        // Remove the ()
        let left = left[1..].trim();
        let right = right[..right.len() - 1].trim();

        let connection = (left, right);
        map.insert(key.trim(), connection);
    }
    (movement, map)
}

fn solve<'a>(mut current: &'a str, movement: &Vec<char>, connections: &'a FxHashMap<&str, (&str, &str)>, part2: bool) -> usize {
    let mut count = 0;

    while (part2 || current != "ZZZ") && (!part2 || !current.ends_with('Z')) {
        let (left, right) = connections.get(current).unwrap();
        if movement[count % movement.len()] == 'L' {
            current = left;
        } else {
            current = right;
        }
        count += 1;
    }
    count
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let (movement, connections) = &parse(input);

    let current = "AAA";
    solve(current, movement, connections, false)
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    let (movement, connections) = &parse(input);

    let counts: Vec<usize> = connections.par_iter()
        .filter(|(key, _)| key.ends_with('A'))
        .map(|(current, _)| solve(current, movement, connections, true))
        .collect();

    counts.iter().skip(1).fold(*counts.first().unwrap(), |acc, num| acc.lcm(num))
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_PART2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(EXAMPLE_1), 2);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(EXAMPLE_2), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_PART2), 6);
    }
}