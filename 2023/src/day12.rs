use std::cell::RefCell;
use std::collections::HashMap;
use aoc_runner_derive::aoc;
use itertools::{intersperse, repeat_n};

// #[aoc_generator(day12)]
fn parse(input: &str) -> Vec<(&[u8], Vec<u32>)> {
    input.lines()
        .map(|line | {
            let (springs, conditions) = line.split_once(' ').unwrap();
            let conditions = conditions.split(',').map(|num| num.parse().unwrap()).collect();
            (springs.as_bytes(), conditions)
        })
        .collect()
}

fn solve<'a>(springs: &'a [u8], conditions: &'a [u32], count: u32, memo: &RefCell<HashMap<(&'a [u8], &'a [u32], u32), u64>>) -> u64 {
    let key = (springs, conditions, count);
    if let Some(&result) = memo.borrow().get(&key) {
        return result;
    }
    if springs.is_empty() {
        return if (conditions.is_empty() && count == 0)
            || (conditions.len() == 1 && *conditions.first().unwrap() == count) {
            1
        } else {
            0
        }
    }

    let mut result = 0;

    let char = *springs.first().unwrap();
    if char == b'#' {
        result += solve(&springs[1..], conditions, count + 1, memo);
    } else if char == b'.' {
        if count == 0 {
            result += solve(&springs[1..], conditions, 0, memo)
        }
        if !conditions.is_empty() && *conditions.first().unwrap() == count {
            result += solve(&springs[1..], &conditions[1..], 0, memo)
        }
    } else if char == b'?' {
        // assume Damaged
        result += solve(&springs[1..], conditions, count + 1, memo);
        // assume Operational
        if count == 0 {
            result += solve(&springs[1..], conditions, 0, memo)
        }
        if !conditions.is_empty() && *conditions.first().unwrap() == count {
            result += solve(&springs[1..], &conditions[1..], 0, memo)
        }
    }
    memo.borrow_mut().insert(key, result);
    result
}

#[aoc(day12, part1)]
fn part1(input: &str) -> u64 {
    let input = parse(input);
    input.iter()
        .map(|(springs, conditions)| solve(springs, conditions, 0, &RefCell::new(HashMap::new())))
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> u64 {
    let input = parse(input);

    let mut result = 0;
    for (springs, conditions) in input {
        let springs: Vec<u8> = intersperse(repeat_n(springs.to_vec(), 5), vec![b'?']).flatten().collect();
        let conditions: Vec<u32> = repeat_n(conditions, 5).flatten().collect();
        result += solve(&springs, &conditions, 0, &RefCell::new(HashMap::new()));
    }

    result
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