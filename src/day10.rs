#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut string = String::from(input);
    for _ in 0..40 {
        string = look_and_say(string);
    }
    string.len()
}

fn look_and_say(input: String) -> String {
    let mut result = String::new();
    let mut iter = input.chars().peekable();

    while let Some(current) = iter.next() {
        let mut count = 1;
        while let Some(next) = iter.peek() {
            if *next == current {
                count += 1;
                iter.next();
            } else {
                break;
            }
        }
        result.push_str(format!("{count}{current}").as_str());
    }
    result
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut string = String::from(input);
    for _ in 0..50 {
        string = look_and_say(string);
    }
    string.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut string = String::from("1");
        let result = "312211";

        for _ in 0..5 {
            string = look_and_say(string);
        }
        assert_eq!(string, result);
    }
}