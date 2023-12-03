use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

struct Number {
    value: usize,
    index_range: RangeInclusive<usize>
}

impl Number {
    fn has_adjacent_symbol(&self, prev_symbol: &[usize], cur_symbol: &[usize], next_symbol: &[usize]) -> bool {
        prev_symbol.iter().any(|idx| self.index_range.contains(idx))
        || cur_symbol.iter().any(|idx| self.index_range.contains(idx))
        || next_symbol.iter().any(|idx| self.index_range.contains(idx))
    }

    fn contains(&self, column: &usize) -> bool {
        self.index_range.contains(column)
    }
}

#[aoc_generator(day3, part1)]
fn parse_part1(input: &str) -> (Vec<Vec<Number>>, Vec<Vec<usize>>) {
    parse(input, false)
}

#[aoc_generator(day3, part2)]
fn parse_part2(input: &str) -> (Vec<Vec<Number>>, Vec<Vec<usize>>) {
    parse(input, true)
}

fn parse(input: &str, part2: bool) -> (Vec<Vec<Number>>, Vec<Vec<usize>>) {
    let mut numbers = vec![];
    let mut symbols = vec![];

    for line in input.lines() {
        let re = Regex::new(r"\d+").unwrap();
        let line_numbers = re.find_iter(line)
            .map(|digit| Number{
                value: digit.as_str().parse().unwrap(),
                index_range: digit.start().saturating_sub(1)..=digit.end()
            }).collect();
        numbers.push(line_numbers);

        let re = match part2 {
            false => Regex::new(r"[^0-9.]").unwrap(),
            true => Regex::new(r"[*]").unwrap()
        };
        let line_symbols = re.find_iter(line)
            .map(|symbol| symbol.start())
            .collect();
        symbols.push(line_symbols);
    }
    (numbers, symbols)
}

#[aoc(day3, part1)]
fn part1(input: &(Vec<Vec<Number>>, Vec<Vec<usize>>)) -> usize {
    let (numbers, symbols) = input;
    let mut result = 0;
    for (idx, current) in symbols.iter().enumerate() {
        let default = vec![];
        let prev = symbols.get(idx.wrapping_sub(1)).unwrap_or(&default);
        let next = symbols.get(idx + 1).unwrap_or(&default);
        let line_numbers = numbers.get(idx).unwrap();
        for number in line_numbers {
            if number.has_adjacent_symbol(prev, current, next) {
                result += number.value;
            }
        }
    }
    result
}

#[aoc(day3, part2)]
fn part2(input: &(Vec<Vec<Number>>, Vec<Vec<usize>>)) -> usize {
    let (numbers, symbols) = input;
    let mut result = 0;
    for (idx, current) in numbers.iter().enumerate() {
        let default = vec![];
        let prev = numbers.get(idx.wrapping_sub(1)).unwrap_or(&default);
        let next = numbers.get(idx + 1).unwrap_or(&default);
        let line_symbols = symbols.get(idx).unwrap();
        for symbol in line_symbols {
            let prev_neighbors: Vec<&Number> = prev.iter().filter(|number| number.contains(symbol)).collect();
            let cur_neighbors: Vec<&Number> = current.iter().filter(|number| number.contains(symbol)).collect();
            let next_neighbors: Vec<&Number> = next.iter().filter(|number| number.contains(symbol)).collect();
            let neighbors = [prev_neighbors, cur_neighbors, next_neighbors].concat();

            if neighbors.len() == 2 {
                result += neighbors.first().unwrap().value * neighbors.last().unwrap().value;
            }
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_part1(INPUT)), 4361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(INPUT)), 467835);
    }
}