#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let digits: Vec<u32> =  line.chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        result += digits.first().unwrap() * 10 + digits.last().unwrap();
    }
    result
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    for i in 0..lines.len() {
        for (index, digit) in digits.iter().enumerate() {
            let line = lines.get(i).unwrap();
            let str = format!("{}{}{}", digit, index + 1, digit);
            lines[i] = line.replace(digit, str.as_str());
        }
    }
    let string = lines.join("\n");
    solve_part1(string.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let string = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(solve_part1(string), 142);
    }

    #[test]
    fn example2() {
        let string = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(solve_part2(string), 281);
    }
}