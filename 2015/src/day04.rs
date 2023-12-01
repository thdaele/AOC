use crypto::digest::Digest;
use crypto::md5::Md5;

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> u32 {
    bruteforce(input, |hash| hash[0..2] == [0; 2] && (hash[2] & 0xF0) == 0)
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> u32 {
    bruteforce(input, |hash| hash[0..3] == [0; 3])
}

fn bruteforce(secret: &str, is_valid: impl Fn(&[u8; 16]) -> bool) -> u32 {
    let mut hash = [0; 16];

    let mut hash_function = Md5::new();
    hash_function.input_str(secret);

    (1..=u32::MAX).map(|i| {
        let mut hash_function = hash_function;
        hash_function.input_str(&i.to_string());
        hash_function.result(&mut hash);

        (i, is_valid(&hash))
    }).find(|&(_, b)| b)
        .map(|(i, _)| i)
        .expect("result is bigger than u32")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(solve_part1("abcdef"), 609043);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part1("pqrstuv"), 1048970);
    }
}