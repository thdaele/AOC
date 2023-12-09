use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line|
        line.split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect()
    ).collect()
}

fn solve(history: &[i32], part2: bool) -> i32 {
    let mut history = history.to_vec();
    if part2 {
        history.reverse();
    }

    let mut result = 0;
    while !history.iter().all(|&value| value == 0) {
        result += history.last().unwrap();

        let mut next_history = vec![];
        for window in history.windows(2) {
            next_history.push(window.last().unwrap() - window.first().unwrap());
        }
        history = next_history;
    }
    result
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<i32>]) -> i32 {
    input.iter()
        .map(|history| solve(history, false))
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<i32>]) -> i32 {
    input.iter()
        .map(|history| solve(history, true))
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 2);
    }
}