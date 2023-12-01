use std::collections::HashSet;

type Position = (i32, i32);

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut houses: HashSet<Position> = HashSet::new();
    let mut current_pos = (0, 0);

    houses.insert(current_pos);
    for c in input.chars() {
        match c {
            '>' => current_pos.0 += 1,
            '<' => current_pos.0 -= 1,
            '^' => current_pos.1 += 1,
            'v' => current_pos.1 -= 1,
            _ => unreachable!()
        }

        houses.insert(current_pos);
    }

    houses.len()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut houses: HashSet<Position> = HashSet::new();
    let mut santa = (0, 0);
    let mut robot = (0, 0);

    houses.insert(santa);

    for (i, c) in input.chars().enumerate() {
        let current_pos = if i % 2 == 0 {&mut santa} else {&mut robot};

        match c {
            '>' => current_pos.0 += 1,
            '<' => current_pos.0 -= 1,
            '^' => current_pos.1 += 1,
            'v' => current_pos.1 -= 1,
            _ => unreachable!()
        }

        houses.insert(*current_pos);
    }

    houses.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(solve_part1(">"), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part1("^>v<"), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(solve_part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn example4() {
        assert_eq!(solve_part2("^v"), 3);
    }

    #[test]
    fn example5() {
        assert_eq!(solve_part2("^>v<"), 3);
    }

    #[test]
    fn example6() {
        assert_eq!(solve_part2("^v^v^v^v^v"), 11);
    }
}