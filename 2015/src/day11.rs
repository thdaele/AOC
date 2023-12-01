#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> String {
    let mut password = input.chars().collect::<Vec<char>>();
    let mut password = increment(&mut password);
    while !is_valid(password) {
        password = increment(password);
    }
    password.iter().collect()
}

pub fn increment(password: &mut Vec<char>) -> &mut Vec<char> {
    for current in password.iter_mut().rev() {
        if *current == 'z' {
            *current = 'a';
        } else {
            *current = (*current as u8 + 1) as char;
            break;
        }
    }
    password
}

pub fn increasing_letters(password: &[char]) -> bool {
    password.iter()
        .zip(password.iter().skip(1))
        .zip(password.iter().skip(2))
        .any(|((&a, &b), &c)| a as u8 == b as u8 - 1 && b as u8 == c as u8 - 1)
}

pub fn confusing_letters(password: &[char]) -> bool {
    password.iter()
        .any(|c| matches!(c, 'i' | 'o' | 'l'))
}

pub fn two_pairs(password: &[char]) -> bool {
    let mut iter = password.iter().peekable();

    let mut pairs = 0;
    while let Some(current) = iter.next() {
        if let Some(next) = iter.peek() {
            if *next == current {
                pairs += 1;
                iter.next();
            }
        }
    }
    pairs >= 2
}

#[inline]
pub fn is_valid(password: &[char]) -> bool {
    increasing_letters(password) && !confusing_letters(password) && two_pairs(password)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> String {
    solve_part1(&solve_part1(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!(increasing_letters(&"hijklmmn".chars().collect::<Vec<char>>()));
        assert!(confusing_letters(&"hijklmmn".chars().collect::<Vec<char>>()));
    }

    #[test]
    fn example2() {
        assert!(two_pairs(&"abbceffg".chars().collect::<Vec<char>>()));
    }

    #[test]
    fn example3() {
        assert!(!two_pairs(&"abbcegjk".chars().collect::<Vec<char>>()));
    }

    #[test]
    fn example4() {
        assert_eq!(solve_part1("abcdefgh"), "abcdffaa")
    }

    #[test]
    fn example5() {
        assert_eq!(solve_part1("ghijklmn"), "ghjaabcc")
    }
}