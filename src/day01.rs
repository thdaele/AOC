#[aoc(day1, part1, Chars)]
pub fn part1_chars(input: &str) -> i32 {
    0
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::{part1_chars as part1, part2};

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
    }

    // ((( and (()(()( both result in floor 3.
    #[test]
    fn sample2() {
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
    }

    // ))((((( also results in floor 3.
    #[test]
    fn sample3() {
        assert_eq!(part1("))((((("), 3);
    }

    // ()) and ))( both result in floor -1 (the first basement level).
    #[test]
    fn sample4() {
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
    }

    // ))) and )())()) both result in floor -3.
    #[test]
    fn sample5() {
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    // ) causes him to enter the basement at character position 1.
    #[test]
    fn sample6() {
        assert_eq!(part2(")"), 1);
    }

    // ()()) causes him to enter the basement at character position 5.
    #[test]
    fn sample7() {
        assert_eq!(part2("()())"), 5);
    }
}