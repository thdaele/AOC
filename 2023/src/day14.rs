use aoc_runner_derive::aoc;
use rustc_hash::FxHashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Rock {
    Round(u8),
    Cube(u8)
}

// #[aoc_generator(day14)]
fn parse(input: &str) -> (Vec<Vec<Rock>>, (u8, u8)) {
    let x_len = input.chars().position(|char| char == '\n').unwrap();
    let mut result: Vec<Vec<Rock>> = (0..x_len).map(|_| vec![]).collect();

    let mut y_len = 0;
    input.lines().enumerate()
        .for_each(|(y, line)| {
            y_len = y + 1;
            line.chars().enumerate()
                .filter(|(_, char)| *char != '.')
                .for_each(|(x, char)| {
                    let rock = match char {
                        'O' => Rock::Round(y as u8),
                        '#' => Rock::Cube(y as u8),
                        _ => unreachable!()
                    };
                    result[x].push(rock)
                })
        });

    (result, (y_len as u8, x_len as u8))
}

fn till(grid: &mut [Vec<Rock>], reverse: bool, size: u8) {
    if !reverse {
        for column in grid.iter_mut() {
            let mut update_index = 0;
            for rock in column.iter_mut() {
                match rock {
                    Rock::Round(row) => {
                        *row = update_index;
                        update_index += 1;
                    },
                    Rock::Cube(row) => {
                        update_index = *row + 1;
                    }
                }
            }
        }
    } else {
        for column in grid.iter_mut() {
            let mut update_index = size - 1;
            for rock in column.iter_mut().rev() {
                match rock {
                    Rock::Round(row) => {
                        *row = update_index;
                        update_index = update_index.saturating_sub(1);
                    },
                    Rock::Cube(row) => {
                        update_index = row.saturating_sub(1);
                    }
                }
            }
        }
    }
}

fn score(grid: &[Vec<Rock>], len: u8) -> u32 {
    grid.iter().map(|column| {
        column.iter()
            .filter_map(|rock| {
                match rock {
                    Rock::Cube(_) => None,
                    Rock::Round(y) => Some((len - y) as u32)
                }
            })
            .sum::<u32>()
    }).sum()
}

fn transpose(original: &mut Vec<Vec<Rock>>, size: u8) {
    assert!(!original.is_empty());
    let mut transposed: Vec<Vec<Rock>> = (0..size).map(|_| Vec::with_capacity(size as usize)).collect();

    original.iter().enumerate().for_each(|(x, original_column)| {
        original_column.iter()
            .for_each(|item| {
                let (y, new_item) = match item {
                    Rock::Round(y) => (y, Rock::Round(x as u8)),
                    Rock::Cube(y) => (y, Rock::Cube(x as u8))
                };
                transposed[*y as usize].push(new_item);
            })
    });

    *original = transposed;
}

#[allow(dead_code)]
fn print(grid: &mut Vec<Vec<Rock>>, swap_b: bool, x_len: u8, y_len: u8) {
    if swap_b {
        transpose(grid, y_len);
    }

    for column in grid.iter() {
        let mut string = "".to_string();
        let mut next = 0;
        for item in column {
            let y = *match item {
                Rock::Cube(y) => y,
                Rock::Round(y) => y
            };

            while next != y {
                string.push('.');
                next += 1;
            }
            next += 1;

            string.push(match item {
                Rock::Cube(_) => '#',
                Rock::Round(_) => 'O'
            })
        }
        while next != y_len {
            string.push('.');
            next += 1;
        }
        println!("{string}");
    }
    println!();
    if swap_b {
        transpose(grid, x_len);
    }
}

#[aoc(day14, part1)]
fn part1(input: &str) -> u32 {
    let (mut grid, (y_len, _)) = parse(input);
    till(&mut grid, false, y_len);

    score(&grid, y_len)
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u32 {
    let (mut grid, (y_len, x_len)) = parse(input);

    let mut cache = FxHashMap::default();

    let grid_clone: Vec<Vec<Rock>> = grid.clone();
    let mut cycle = 0;
    cache.insert(grid_clone, cycle);
    let (cycle_start, cycle_end) = loop {
        // North, West, South, East
        for (b, len) in [(false, y_len), (false, x_len), (true, y_len), (true, x_len)] {
            till(&mut grid, b, len);

            // Change column and row
            transpose(&mut grid, len);
        }
        cycle += 1;
        if let Some(cycle_start) = cache.get(&grid) {
            break (cycle_start, cycle);
        }

        let grid_clone: Vec<Vec<Rock>> = grid.clone();
        cache.insert(grid_clone, cycle);
    };

    let cycle_length = cycle_end - cycle_start;

    let index = cycle_start + (1000000000 - cycle_start) % cycle_length;

    let (grid, _) = cache.iter().find(|(_, value)| **value == index).unwrap();

    score(grid, y_len)
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&EXAMPLE), 136);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&EXAMPLE), 64);
    }
}