use std::ops::RangeInclusive;
use std::str::FromStr;

type Position = (usize, usize);

#[derive(Copy, Clone)]
enum Action {
    Off,
    On,
    Toggle
}

impl FromStr for Action {
    type Err = ();

    fn from_str(input: &str) -> Result<Action, Self::Err> {
        match input {
            "off" => Ok(Action::Off),
            "on" => Ok(Action::On),
            "toggle" => Ok(Action::Toggle),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Instruction {
    pos1: Position,
    pos2: Position,
    action: Action
}

impl Instruction {
    pub fn new(xmin: usize, ymin: usize, xmax: usize, ymax: usize, keyword: &str) -> Self {
        Self {
            pos1: (xmin, ymin),
            pos2: (xmax, ymax),
            action: Action::from_str(keyword).unwrap(),
        }
    }

    pub fn get_xrange(self) -> RangeInclusive<usize> {
        self.pos1.0..=self.pos2.0
    }

    pub fn get_yrange(self) -> RangeInclusive<usize> {
        self.pos1.1..=self.pos2.1
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    let mut v: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let mut tokens = line.trim().split(|c| c == ' ' || c == ',');
        let mut keyword = tokens.next().unwrap();
        if keyword == "turn" {
            keyword = tokens.next().unwrap();
        };
        let xmin: usize = tokens.next().unwrap().parse().unwrap();
        let ymin: usize = tokens.next().unwrap().parse().unwrap();
        tokens.next();
        let xmax: usize = tokens.next().unwrap().parse().unwrap();
        let ymax: usize = tokens.next().unwrap().parse().unwrap();

        v.push(Instruction::new(xmin , ymin , xmax, ymax, keyword))
    }
    v
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Instruction]) -> u64 {
    let mut lights = [[false; 1000]; 1000];

    for instruction in input.iter() {
        for row in &mut lights[instruction.get_xrange()] {
            for light in &mut row[instruction.get_yrange()] {
                *light = match instruction.action {
                    Action::Off => false,
                    Action::On => true,
                    Action::Toggle => !*light
                }
            }
        }
    }
    let mut sum: u64 = 0;
    for row in lights.iter() {
        sum += row.iter().fold(0, |acc, &x| if x {acc + 1} else {acc})
    }
    sum
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Instruction]) -> u32 {
    let mut lights = [[0u32; 1000]; 1000];

    for instruction in input.iter() {
        for row in &mut lights[instruction.get_xrange()] {
            for light in &mut row[instruction.get_yrange()] {
                *light = match instruction.action {
                    Action::Off => light.saturating_sub(1),
                    Action::On => *light + 1,
                    Action::Toggle => *light + 2
                }
            }
        }
    }
    let count = lights.iter().fold(0, |acc, row| acc + row.iter().sum::<u32>());
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let string = "turn on 0,0 through 999,999
        toggle 0,0 through 999,0
        turn off 499,499 through 500,500";
        assert_eq!(solve_part1(&input_generator(string)), 998996);
    }

    #[test]
    fn example2() {
        let string = "turn on 0,0 through 0,0";
        assert_eq!(solve_part2(&input_generator(string)), 1);
    }

    #[test]
    fn example3() {
        let string = "turn on 0,0 through 0,0\
        toggle 0,0 through 999,999";
        assert_eq!(solve_part2(&input_generator(string)), 2000001);
    }
}