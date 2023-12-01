use itertools::Itertools;

#[aoc(day17, part1)]
pub fn solve_part1(input: &str) -> usize {
    let containers: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    // Bruteforce ftw
    let mut count = 0;
    for i in 0..containers.len() {
        count += containers.iter().combinations(i)
            .filter(|comb| comb.iter().copied().sum::<usize>() == 150).count();
    }
    count
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &str) -> usize {
    let containers: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    // Bruteforce ftw
    let mut count = 0;
    for i in 0..containers.len() {
        count += containers.iter().combinations(i)
            .filter(|comb| comb.iter().copied().sum::<usize>() == 150).count();
        if count > 0 {
            return count
        }
    }
    unreachable!()
}
