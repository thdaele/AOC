use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day24)]
fn parse(input: &str) -> Vec<usize> {
    let mut packages: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    packages.sort_unstable();
    packages
}

#[aoc(day24, part1)]
fn part1(input: &[usize]) -> usize {
    let target = input.iter().sum::<usize>() / 3;

    solve(input, target)
}

#[aoc(day24, part2)]
fn part2(input: &[usize]) -> usize {
    let target = input.iter().sum::<usize>() / 4;

    solve(input, target)
}

fn solve(input: &[usize], target: usize) -> usize {
    for n in 0..input.len() {
        // Get combinations with min amount of packages that is equal to target
        // This is for the passenger compartment
        // It is trivial to divide the remaining items equally in weight over the 2 or 3 other compartments
        let combs: Vec<Vec<&usize>> = input.iter().combinations(n)
            .filter(|comb| comb.iter().cloned().sum::<usize>() == target).collect();

        if !combs.is_empty() {
            // Search comb with smallest quantum entanglement
            return combs.iter()
                .map(|comb| comb.iter().copied().product::<usize>())
                .min().unwrap()
        }
    }
    unreachable!()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
3
5
7
13
17
19
23
29
31
37
41
43
53
59
61
67
71
73
79
83
89
97
101
103
107
109
113";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 10723906903);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 74850409);
    }
}