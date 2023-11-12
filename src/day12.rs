use itertools::Itertools;
use regex::Regex;

#[aoc(day12, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let re = Regex::new(r"-?\d+").unwrap();
    let matches: Vec<_> = re.find_iter(input).map(|m| m.as_str().parse().unwrap()).collect();
    matches.iter().sum()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let json: serde_json::Value = serde_json::from_str(input).unwrap();
    recursive_json(&json)
}

fn recursive_json(json: &serde_json::Value) -> i64 {
    if let Some(obj) = json.as_object() {
        if obj.values().contains(&serde_json::Value::String(String::from("red"))) {
            return 0
        }
        return obj.values().map(recursive_json).sum()
    }
    if let Some(list) = json.as_array() {
        return list.iter().map(recursive_json).sum()
    }
    if let Some(value) = json.as_i64() {
        return value;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(solve_part1("[1,2,3]"), 6);
        assert_eq!(solve_part1("[[[3]]]"), 3);
    }
}