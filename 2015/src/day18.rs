// #[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|line|
        line.chars().map(|c|
            match c {
                '.' => false,
                '#' => true,
                _ => unreachable!()
            }).collect()
    ).collect()
}

pub fn solve(input: &str, part_2: bool) -> usize {
    let input = &mut input_generator(input);
    let default = vec![false];
    let max_y = input.len() - 1;
    let row = input.get_mut(0).unwrap();
    let max_x = row.len() - 1;
    if part_2 {
        // Turn on the corners
        *input.get_mut(0).unwrap().get_mut(0).unwrap() = true;
        *input.get_mut(max_y).unwrap().get_mut(0).unwrap() = true;
        *input.get_mut(0).unwrap().get_mut(max_x).unwrap() = true;
        *input.get_mut(max_y).unwrap().get_mut(max_x).unwrap() = true;
    }
    for _ in 0..100 {
        let copy = input.clone();
        for row_index in 0..input.len() {
            let row = input.get_mut(row_index).unwrap();
            for column_index in 0..row.len() {
                if part_2 &&
                    ((row_index == 0 || row_index == max_y) && (column_index == 0 || column_index == max_x)) {
                    // Don't turn corners off as early as possible in the calculations
                    continue;
                }
                let value = row.get_mut(column_index).unwrap();
                let mut count = 0;
                for y in -1..=1 {
                    for x in -1..=1 {
                        let first: isize = row_index as isize + y;
                        let second: isize = column_index as isize + x;
                        if first < 0 || second < 0 || (x == 0 && y == 0) {
                            continue
                        }
                        let boolean = copy.
                            get(first as usize).unwrap_or(&default).
                            get(second as usize).unwrap_or(&false);
                        if *boolean {
                            count += 1;
                        }
                    }
                }
                *value = matches!((&value, count), (true, 2..=3) | (false, 3));
            }
        }
    }
    input.iter().map(|row| row.iter().filter(|e| **e).count()).sum()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &str) -> usize {
    solve(input, false)
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &str) -> usize {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";
        assert_eq!(solve_part1(input), 4)
    }

    #[test]
    fn example2() {
        let input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";
        assert_eq!(solve_part2(input), 7)
    }
}
