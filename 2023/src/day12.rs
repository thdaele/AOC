use aoc_runner_derive::aoc;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

fn parse(input: &str, part2: bool) -> Vec<Record> {
    input.lines()
        .map(|line | {
            let (springs, conditions) = line.split_once(' ').unwrap();
            let conditions: Box<[u8]> = conditions.split(',').map(|num| num.parse().unwrap()).collect();
            let springs = springs.as_bytes();
            if !part2 {
                Record::new(springs.into(), conditions)
            } else {
                let springs= [springs; 5].join(&b'?').into();
                let conditions= conditions.repeat(5).into_boxed_slice();
                Record::new(springs, conditions)
            }

        })
        .collect()
}

// #[aoc_generator(day12, part1)]
// fn parse1(input: &str) -> Vec<Record> {
//     parse(input, false)
// }
//
// #[aoc_generator(day12, part2)]
// fn parse2(input: &str) -> Vec<Record> {
//     parse(input, true)
// }

struct Record {
    springs: Box<[u8]>,
    conditions: Box<[u8]>,
    memoization: FxHashMap<(u8, u8, u8), u64>
}

impl Record {
    fn new(springs: Box<[u8]>, conditions: Box<[u8]>) -> Record {
        Record {springs, conditions, memoization: FxHashMap::default()}
    }

    fn solve(&mut self, springs_index: u8, conditions_index: u8, count: u8) -> u64 {
        let key = (springs_index, conditions_index, count);
        if let Some(&result) = self.memoization.get(&key) {
            return result;
        }
        if springs_index as usize == self.springs.len() {
            return if (conditions_index as usize == self.conditions.len() && count == 0)
                || (conditions_index as usize == self.conditions.len() - 1 && self.conditions[conditions_index as usize] == count) {
                1
            } else {
                0
            }
        }

        let mut result = 0;
        let char = self.springs[springs_index as usize];
        if char == b'#' || char == b'?' {
            result += self.solve(springs_index + 1, conditions_index, count + 1);
        }
        if char == b'.' || char == b'?' {
            if count == 0 {
                result += self.solve(springs_index + 1, conditions_index, 0);
            }
            if (conditions_index as usize) < self.conditions.len() && self.conditions[conditions_index as usize] == count {
                result += self.solve(springs_index + 1, conditions_index + 1, 0);
            }
        }
        self.memoization.insert(key, result);
        result
    }
}

#[aoc(day12, part1)]
fn part1(input: &str) -> u64 {
    let mut input = parse(input, false);
    input.par_iter_mut()
        .map(|record| record.solve(0, 0, 0))
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> u64 {
    let mut input = parse(input, true);
    input.par_iter_mut()
        .map(|record| record.solve(0, 0, 0))
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&EXAMPLE), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&EXAMPLE), 525152);
    }
}