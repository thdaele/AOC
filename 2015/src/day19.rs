use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

// #[aoc_generator(day19)]
fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut result = HashMap::new();

    for line in input.lines() {
        if let Some((key, value)) = line.split_once("=>") {
            let key = key.trim();
            let value = value.trim();

            let vec = match result.get_mut(key) {
                None => {
                    result.insert(key, vec![]);
                    result.get_mut(key).unwrap()
                },
                Some(vec) => vec
            };
            vec.push(value);
        }
    }
    result
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    if let Some((rules, molecule)) = input.split_once("\n\n") {
        let molecule = molecule.to_string();
        let mut set = HashSet::new();
        let rules = parse(rules);

        for (key, values) in rules {
            for m in molecule.match_indices(key) {
                for value in &values {
                    let mut new_molecule = molecule.clone();
                    let range = m.0..m.0 + key.len();

                    new_molecule.replace_range(range, value);
                    set.insert(new_molecule);
                }
            }
        }
        return set.len()
    }
    unreachable!()
}

#[aoc(day19, part2, backward)]
fn part2_b(input: &str) -> usize {
    if let Some((rules, molecule)) = input.split_once("\n\n") {
        let mut molecule = molecule.to_string();
        let rules = parse(rules);
        let mut count = 0;

        loop {
            let mut done = true;
            for (key, values) in &rules {
                for value in values {
                    if let Some(pos) = molecule.find(value) {
                        let range = pos..pos + value.len();

                        molecule.replace_range(range, key);
                        count += 1;
                        done = false;
                    }
                }
            }
            if done {
                break;
            }
        }
        return count;
    }
    unreachable!()
}

#[aoc(day19, part2, formula)]
fn part2_f(input: &str) -> usize {
    if let Some((_rules, molecule)) = input.split_once("\n\n") {
        let elements = molecule.chars().filter(char::is_ascii_uppercase).count();
        let rn = molecule.matches("Rn").count();
        let ar = molecule.matches("Ar").count();
        let y = molecule.matches('Y').count();

        return elements - ar - rn - 2 * y - 1
    }
    unreachable!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = "H => HO
H => OH
O => HH

HOH";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn part1_example2() {
        let input = "H => HO
H => OH
O => HH

HOHOHO";
        assert_eq!(part1(input), 7);
    }

    #[test]
    fn part2_example1() {
        let input = "e => H
e => O
H => HO
H => OH
O => HH

HOH";
        assert_eq!(part2_b(input), 3);
    }

    #[test]
    fn part2_example2() {
        let input = "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO";
        assert_eq!(part2_b(input), 6);
    }
}