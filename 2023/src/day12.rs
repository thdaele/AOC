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
    memoization: FxHashMap<(u8, u8), u64>,
    count_no_working: u8,
    sum_conditions: u8
}

impl Record {
    fn new(springs: Box<[u8]>, conditions: Box<[u8]>) -> Record {
        let count_no_working = springs.iter().filter(|&&char| char != b'.').count() as u8;
        let sum_conditions = conditions.iter().sum();
        Record {springs, conditions, memoization: FxHashMap::default(), count_no_working, sum_conditions}
    }

    fn solve(&mut self, springs_index: u8, conditions_index: u8, placed_working: u8) -> u64 {
        // Base cases
        if springs_index == self.springs.len() as u8 {
            return (conditions_index == self.conditions.len() as u8) as u64
        } else if conditions_index == self.conditions.len() as u8 {
            // Linear unroll to avoid recursive calls
            return self.springs[springs_index as usize..].iter().all(|&char| char != b'#') as u64
        }
        if self.count_no_working - placed_working < self.sum_conditions {
            // Replace to many ? with . -> not enough ? left to place all the needed #
            return 0
        }
        // Memoization
        let key = (springs_index, conditions_index);
        if let Some(&result) = self.memoization.get(&key) {
            return result
        }
        let mut result = 0;
        let char = self.springs[springs_index as usize];
        if char == b'.' || char == b'?' {
            let mut skip_char = 1;
            if char == b'.' {
                // Linear unroll to avoid recursive calls
                skip_char = self.springs[springs_index as usize..].iter().take_while(|&&char| char == b'.').count() as u8;
            }
            result += self.solve(springs_index + skip_char, conditions_index, placed_working + (char == b'?') as u8)
        }
        if char == b'#' || char == b'?' {
            // We try to form an entire group of # at once
            let from = springs_index as usize;
            let to = (springs_index + self.conditions[conditions_index as usize]) as usize;

            // Check if all the char in the range are # or ?
            // false it is not possible to place the entire group of here #
            let possible = match to.cmp(&self.springs.len()) {
                std::cmp::Ordering::Greater => false,
                std::cmp::Ordering::Equal => {
                    self.springs[from..to].iter().all(|&spring| spring != b'.')
                },
                std::cmp::Ordering::Less => {
                    self.springs[from..to].iter().all(|&spring| spring != b'.')
                        && self.springs[to] != b'#'
                }
            };
            if !possible {
                return result
            }
            // End here avoid a recursive call
            if to == self.springs.len() {
                return result + (conditions_index == self.conditions.len() as u8 - 1) as u64
            }
            result += self.solve(to as u8 + 1, conditions_index + 1, placed_working);
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