#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> usize {
    input.lines().map(line_length).map(|(m, c)| m - c).sum()
}

fn line_length(line: &str) -> (usize, usize) {
    let code_length = line.as_bytes().len();
    let mut string_length = 0;

    // Already move the outer quotes of the string since they don't count in string length
    let mut line = &line.as_bytes()[1..code_length - 1];
    while !line.is_empty() {
        let mut update = false;
        if line.len() >= 2 {
            let char = &line[0..2];
            match char {
                b"\\\\" | b"\\\"" => {
                    line = &line[2..];
                    update = true;
                },
                b"\\x" => {
                    line = &line[4..];
                    update = true;
                },
                _ => {}
            }
        }
        if !update {
            line = &line[1..];
        }
        string_length += 1;
    }
    (code_length, string_length)
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> usize {
    input.lines().map(encode_length).map(|(m, e)| e - m).sum()
}

fn encode_length(line: &str) -> (usize, usize) {
    let code_length = line.as_bytes().len();

    let encode_length = code_length
        + 2
        + line
        .as_bytes()
        .iter()
        .filter(|&ch| ch == &b'"' || ch == &b'\\')
        .count();

    (code_length, encode_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let string = "include_str!(\"test.txt\")";
        assert_eq!(solve_part1(string), 12);
    }

    #[test]
    fn example2() {
        let string = "include_str!(\"test.txt\")";
        assert_eq!(solve_part2(string), 19);
    }
}