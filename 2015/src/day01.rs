#[aoc(day1, part1, Chars)]
pub fn part1_chars(input: &str) -> i32 {
    input.chars().fold(0, |sum, c| match c {
        '(' => sum + 1,
        ')' => sum - 1,
        _ => unreachable!(),
    })
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut sum: u32 = 0;

    for (i, c) in input.as_bytes().iter().enumerate() {
        match c {
            b'(' => sum += 1,
            b')' => if let Some(s) = sum.checked_sub(1) {
                sum = s;
            } else {
                return i + 1;
            },
            _ => unreachable!(),
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::{part1_chars as part1, part2};

    #[test]
    fn sample1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
    }

    #[test]
    fn sample2() {
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
    }

    #[test]
    fn sample3() {
        assert_eq!(part1("))((((("), 3);
    }

    #[test]
    fn sample4() {
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
    }

    #[test]
    fn sample5() {
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    #[test]
    fn sample6() {
        assert_eq!(part2(")"), 1);
    }

    #[test]
    fn sample7() {
        assert_eq!(part2("()())"), 5);
    }
}