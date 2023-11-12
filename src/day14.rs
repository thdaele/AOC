use std::cmp;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<(u32, u32, u32)> {
    let mut reindeer_stats: Vec<(u32, u32, u32)> = Vec::new();
    for line in input.lines() {
        let mut parts = line.trim().split(' ');
        parts.next(); // name
        parts.next(); // can
        parts.next(); // fly
        let speed = parts.next().unwrap().parse().unwrap();
        parts.next(); // km/s
        parts.next(); // for
        let time = parts.next().unwrap().parse().unwrap();
        parts.next(); // seconds,
        parts.next(); // but
        parts.next(); // then
        parts.next(); // must
        parts.next(); // rest
        parts.next(); // for
        let rest_time = parts.next().unwrap().parse().unwrap();
        reindeer_stats.push((speed, time, rest_time));
    }
    reindeer_stats
}

#[aoc(day14, part1)]
pub fn solve_part1(reindeer_stats: &[(u32, u32, u32)]) -> u32 {
    let mut max = u32::MIN;
    let total_time = 2503;
    for &(speed, time, rest_time) in reindeer_stats.iter() {
        let mult = total_time / (time + rest_time);
        let rest = total_time % (time + rest_time);
        let result = mult * speed * time + speed * cmp::min(time, rest);
        max = cmp::max(max, result);
    }
    max
}

#[aoc(day14, part2)]
pub fn solve_part2(reindeer_stats: &[(u32, u32, u32)]) -> u32 {
    let mut race_stats: Vec<(u32, u32, u32)> = Vec::new();
    let mut score: Vec<u32> = Vec::new();
    for &(_speed, time, _rest_time) in reindeer_stats {
        race_stats.push((0, time, 0));
        score.push(0);
    }
    for _time in 0..2503 {
        // Update the distance
        let mut max = u32::MIN;
        for (index, (speed, time, rest_time)) in reindeer_stats.iter().enumerate() {
            let (dst, time_left, rest_time_left) = race_stats.get_mut(index).unwrap();
            if *time_left > 0 {
                *time_left -= 1;
                if *time_left == 0 {
                    *rest_time_left = *rest_time;
                }
                *dst += speed;
            } else if *rest_time_left > 0 {
                *rest_time_left -= 1;
                if *rest_time_left == 0 {
                    *time_left = *time;
                }
            } else {
                unreachable!()
            }
            if *dst > max {
                max = *dst;
            }
        }
        let indices = race_stats
            .iter()
            .enumerate()
            .filter_map(|(index, &r)| (r.0 == max).then_some(index))
            .collect::<Vec<_>>();
        for index in indices {
            *score.get_mut(index).unwrap() += 1;
        }
    }
    *score.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let string = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        assert_eq!(solve_part1(&input_generator(string)), 2660)
    }

    #[test]
    fn example2() {
        let string = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        assert_eq!(solve_part2(&input_generator(string)), 1564)
    }
}