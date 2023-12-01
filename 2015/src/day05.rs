#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    input.lines().filter(|l| is_nice(l.trim())).count()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    input.lines().filter(|l| is_really_nice(l.trim())).count()
}

fn three_vowels(string: &str) -> bool {
    string.chars()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')).count() >= 3
}

fn twice_in_a_row(string: &str) -> bool {
    string.chars()
        .zip(string.chars().skip(1))
        .any(|(a, b)| a == b)
}

fn no_forbidden_strings(string: &str) -> bool {
    string.chars()
        .zip(string.chars().skip(1))
        .all(|ab| !matches!(ab, ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')))
}

fn is_nice(string: &str) -> bool {
    three_vowels(string) && twice_in_a_row(string) && no_forbidden_strings(string)
}

fn two_pairs(string: &str) -> bool {
    if string.len() < 4 {
        return false;
    }

    let pair = &string[0..2];
    let remain = &string[2..];

    remain.contains(pair) || two_pairs(&string[1..])
}

fn repeat_separated(string: &str) -> bool {
    string
        .chars()
        .zip(string.chars().skip(2))
        .any(|(a, b)| a == b)
}

fn is_really_nice(string: &str) -> bool {
    two_pairs(string) && repeat_separated(string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!(is_nice("ugknbfddgicrmopn"))
    }

    #[test]
    fn example2() {
        assert!(is_nice("aaa"));
    }

    #[test]
    fn example3() {
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!twice_in_a_row("jchzalrnumimnmhp"));
    }

    #[test]
    fn example4() {
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!no_forbidden_strings("haegwjzuvuyypxyu"));
    }

    #[test]
    fn example5() {
        assert!(!is_nice("dvszwmarrgswjxmb"));
        assert!(!three_vowels("dvszwmarrgswjxmb"));
    }

    #[test]
    fn example6() {
        assert!(is_really_nice("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn example7() {
        assert!(is_really_nice("xxyxx"));
    }

    #[test]
    fn example8() {
        assert!(!is_really_nice("uurcxstgmygtbstg"));
        assert!(!repeat_separated("uurcxstgmygtbstg"));
    }

    #[test]
    fn example9() {
        assert!(!is_really_nice("ieodomkazucvgmuy"));
        assert!(!two_pairs("ieodomkazucvgmuy"));
    }
}