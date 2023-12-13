use std::cmp::min;
use std::iter::zip;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut result = vec![];
    for grid in input.split("\n\n") {
        result.push(grid.lines().map(|line| line.chars().collect()).collect());
    }
    result
}

fn transpose(original: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!original.is_empty());
    let mut transposed: Vec<Vec<char>> = (0..original[0].len()).map(|_| vec![]).collect();

    original.iter().for_each(|original_row| {
        original_row.iter().zip(&mut transposed)
            .for_each(|(item, transposed_row)| transposed_row.push(*item))
    });

    transposed
}

fn solve(grid: &Vec<Vec<char>>, factor: u32, mistakes_allowed: u32) -> u32 {
    // Find the rows where the mirror is in between
    let x_len = grid.first().unwrap().len();
    for (row1, row2) in zip(0..grid.len()-1, 1..grid.len()) {
        let max_offset = min(row1 + 1, grid.len() - row2);

        let smudge: usize = (0..x_len).map(|x| {
            (0..max_offset).filter(|y| grid[row1 - y][x] != grid[row2 + y][x]).count()
        }).sum();

        if smudge as u32 == mistakes_allowed {
            // + 1 bc the puzzle is not zero indexed
            return factor * (row1 + 1) as u32
        }
    }
    0
}

#[aoc(day13, part1)]
fn part1(input: &[Vec<Vec<char>>]) -> u32 {
    let transposed: Vec<Vec<Vec<char>>> = input.iter().map(transpose).collect();

    input.iter().map(|grid| solve(grid, 100, 0)).sum::<u32>()
        + transposed.iter().map(|grid| solve(grid, 1, 0)).sum::<u32>()
}

#[aoc(day13, part2)]
fn part2(input: &[Vec<Vec<char>>]) -> u32 {
    let transposed: Vec<Vec<Vec<char>>> = input.iter().map(transpose).collect();

    input.iter().map(|grid| solve(grid, 100, 1)).sum::<u32>()
        + transposed.iter().map(|grid| solve(grid, 1, 1)).sum::<u32>()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 405);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 400);
    }
}