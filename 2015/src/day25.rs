use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day25)]
fn parse(input: &str) -> (usize, usize) {
    let re = Regex::new(r"-?\d+").unwrap();
    let matches: Vec<_> = re.find_iter(input).map(|m| m.as_str().parse().unwrap()).collect();
    assert_eq!(matches.len(), 2);
    (*matches.first().unwrap(), *matches.get(1).unwrap())
}

#[aoc(day25, part1)]
fn part1(input: &(usize, usize)) -> usize {
    let (row, column) = input;

    let n = column + row - 1;
    let triangle = (n * (n + 1)) / 2;
    let index = triangle - row;

    // https://en.wikipedia.org/wiki/Modular_exponentiation
    (20151125 * modular_pow(252533, index, 33554393)) % 33554393
}

#[aoc(day25, part2)]
fn part2(_input: &(usize, usize)) -> String {
    String::from("n/a")
}

fn modular_pow(base: usize, exponent: usize, modulus: usize) -> usize {
    if modulus == 1 {
        return 0;
    }
    let mut c = 1;
    for _ in 0..exponent {
        c = (c * base) % modulus
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("Enter the code at row 3010, column 3019.")), 8997277);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("Enter the code at row 3010, column 3019.")), "n/a");
    }
}