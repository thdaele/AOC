type Gift = (u32, u32, u32);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Gift> {
    input.lines().map(|l| {
        let mut gift = l.trim().split('x').map(|d| d.parse().unwrap());
        (
            gift.next().unwrap(),
            gift.next().unwrap(),
            gift.next().unwrap(),
        )
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Gift]) -> u32 {
    input.iter().map(|&(l, w, h)| {
        let (s1, s2) = smallest_side((l, w, h));

        2 * l * w + 2 * w * h + 2 * h * l + s1 * s2
    }).sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Gift]) -> u32 {
    input.iter().map(|&(l, w, h)| {
        let (s1, s2) = smallest_side((l, w, h));

        2 * (s1 + s2) + l * w * h
    }).sum()
}

fn smallest_side((l, w, h): Gift) -> (u32, u32) {
    let mut vec = vec![l, w, h];
    vec.sort();

    (vec[0], vec[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(solve_part1(&input_generator("2x3x4")), 58);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part1(&input_generator("1x1x10")), 43);
    }

    #[test]
    fn example3() {
        assert_eq!(solve_part2(&input_generator("2x3x4")), 34);
    }

    #[test]
    fn example4() {
        assert_eq!(solve_part2(&input_generator("1x1x10")), 14);
    }
}