use aoc_runner_derive::aoc;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

fn parse(input: &str, part2: bool) -> Vec<Record> {
    input.lines()
        .map(|line | {
            let (springs, conditions) = line.split_once(' ').unwrap();
            let conditions = conditions.split(',').map(|num| num.parse().unwrap()).collect();
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
    memoization: FxHashMap<(u8, u8, u8), u64>,
    count_no_working: u8,
    sum_conditions: u8
}

impl Record {
    fn new(springs: Box<[u8]>, conditions: Box<[u8]>) -> Record {
        let count_no_working = springs.iter().filter(|&&char| char != b'.').count() as u8;
        let sum_conditions = conditions.iter().sum();
        Record {springs, conditions, memoization: FxHashMap::default(), count_no_working, sum_conditions}
    }

    fn solve(&mut self, springs_index: u8, conditions_index: u8, mut placed_working: u8, count: u8) -> u64 {
        let key = (springs_index, conditions_index, count);
        if let Some(&result) = self.memoization.get(&key) {
            return result;
        }

        if springs_index >= self.springs.len() as u8 {
            return if (conditions_index == self.conditions.len() as u8 && count == 0)
                || (conditions_index == self.conditions.len() as u8 - 1 && self.conditions[conditions_index as usize] == count) {
                1
            } else {
                0
            }
        }
        else if conditions_index == self.conditions.len() as u8 {
            return self.springs[springs_index as usize..].iter().all(|&char| char == b'.' || char == b'?') as u64
        }
        if self.count_no_working - placed_working < self.sum_conditions {
            return 0
        }


        let mut result = 0;
        let char = self.springs[springs_index as usize];
        if conditions_index < self.conditions.len() as u8 && self.conditions[conditions_index as usize] > count && (char == b'#' || char == b'?') {
            result += self.solve(springs_index + 1, conditions_index, placed_working, count + 1);
        }
        if char == b'.' || char == b'?' {
            let mut skip_char = 1;
            if char == b'?' {
                placed_working += 1;
            } else {
                let z = (springs_index + 1 != self.springs.len() as u8) as u8;
                skip_char = self.springs[(springs_index + z) as usize..].iter().take_while(|&&char| char == b'.').count() as u8 + 1;
            }
            if count == 0 {
                result += self.solve(springs_index + skip_char, conditions_index, placed_working, 0);
            } else if conditions_index < self.conditions.len() as u8 && self.conditions[conditions_index as usize] == count {
                result += self.solve(springs_index + skip_char, conditions_index + 1, placed_working, 0);
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
        .map(|record| record.solve(0, 0, 0,0))
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> u64 {
    let mut input = parse(input, true);
    input.par_iter_mut()
        .map(|record| record.solve(0, 0, 0, 0))
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