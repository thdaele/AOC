use aoc_runner_derive::aoc;
use rustc_hash::FxHashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Rock {
    Round(u32),
    Cube(u32)
}

// #[aoc_generator(day14)]
fn parse(input: &str) -> (Vec<Vec<Rock>>, (usize, usize)) {
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
                        'O' => Rock::Round(y as u32),
                        '#' => Rock::Cube(y as u32),
                        _ => unreachable!()
                    };
                    result[x].push(rock)
                })
        });

    (result, (y_len, x_len))
}

fn till(grid: &mut Vec<Vec<Rock>>, reverse: bool, size: usize) {
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
            let mut update_index = size as u32 - 1;
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

fn score(grid: &Vec<Vec<Rock>>) -> u32 {
    let len = grid.len() as u32;
    grid.iter().map(|column| {
        column.iter()
            .filter_map(|rock| {
                match rock {
                    Rock::Cube(_) => None,
                    Rock::Round(y) => Some(len - y)
                }
            })
            .sum::<u32>()
    }).sum()
}

fn swap(original: &mut Vec<Vec<Rock>>, size: usize) {
    assert!(!original.is_empty());
    let mut transposed: Vec<Vec<Rock>> = (0..size).map(|_| vec![]).collect();

    original.iter().enumerate().for_each(|(x, original_column)| {
        original_column.iter()
            .for_each(|item| {
                let (y, new_item) = match item {
                    Rock::Round(y) => (y, Rock::Round(x as u32)),
                    Rock::Cube(y) => (y, Rock::Cube(x as u32))
                };
                transposed[*y as usize].push(new_item);
            })
    });

    *original = transposed;
}

#[allow(dead_code)]
fn print(grid: &mut Vec<Vec<Rock>>, swap_b: bool, x_len: usize, y_len: usize) {
    if swap_b {
        swap(grid, y_len);
    }

    for column in grid.iter() {
        let mut string = "".to_string();
        let mut next = 0;
        for item in column.iter() {
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
        while next != y_len as u32 {
            string.push('.');
            next += 1;
        }
        println!("{string}");
    }
    println!();
    if swap_b {
        swap(grid, x_len);
    }
}

#[aoc(day14, part1)]
fn part1(input: &str) -> u32 {
    let (mut grid, (y_len, _)) = parse(input);
    till(&mut grid, false, y_len);

    score(&grid)
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u32 {
    let (mut grid, (y_len, x_len)) = parse(input);

    let mut cache = FxHashMap::default();

    let grid_clone: Vec<Vec<Rock>> = grid.clone();
    let mut cycle: i64 = 0;
    cache.insert(grid_clone, cycle);
    let (cycle_start, cycle_end) = loop {
        for (b, len) in [(false, y_len), (false, x_len), (true, y_len), (true, x_len)] {
            till(&mut grid, b, len);

            // Change column and row
            swap(&mut grid, len);
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

    score(grid)
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