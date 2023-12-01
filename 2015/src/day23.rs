use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

enum Op {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(usize),
    Jie(char, usize),
    Jio(char, usize),
}

#[aoc_generator(day23)]
fn parse(input: &str) -> Vec<Op> {
    let mut result = vec![];
    for (idx, line) in input.lines().enumerate() {
        if let Some((first, offset)) = line.split_once(", ") {
            let (instruction, register) = first.split_once(' ').unwrap();
            let op = match instruction {
                "jie" => Op::Jie(register.parse().unwrap(), idx.wrapping_add(offset.parse::<i32>().unwrap() as usize)),
                "jio" => Op::Jio(register.parse().unwrap(), idx.wrapping_add(offset.parse::<i32>().unwrap() as usize)),
                _ => unreachable!()
            };
            result.push(op);
        } else {
            let (instruction, register) = line.split_once(' ').unwrap();
            let op = match instruction {
                "hlf" => Op::Hlf(register.parse().unwrap()),
                "tpl" => Op::Tpl(register.parse().unwrap()),
                "inc" => Op::Inc(register.parse().unwrap()),
                "jmp" => Op::Jmp(idx.wrapping_add(register.parse::<i32>().unwrap() as usize)),
                _ => unreachable!()
            };
            result.push(op);
        }
    }
    result
}

#[aoc(day23, part1)]
fn part1(input: &[Op]) -> usize {
    let registers = HashMap::from([
        ('a', 0),
        ('b', 0)
    ]);
    run(input, registers)
}

#[aoc(day23, part2)]
fn part2(input: &[Op]) -> usize {
    let registers = HashMap::from([
        ('a', 1),
        ('b', 0)
    ]);
    run(input, registers)
}

fn run(input: &[Op], mut registers: HashMap<char, usize>) -> usize {
    let mut pc = 0;

    while pc < input.len() {
        match input[pc] {
            Op::Hlf(reg) => {
                *registers.get_mut(&reg).unwrap() /= 2;
                pc += 1;
            },
            Op::Tpl(reg) => {
                *registers.get_mut(&reg).unwrap() *= 3;
                pc += 1;
            }
            Op::Inc(reg) => {
                *registers.get_mut(&reg).unwrap() += 1;
                pc += 1;
            }
            Op::Jmp(index) => {
                pc = index;
            },
            Op::Jie(reg, index) => {
                let value = *registers.get_mut(&reg).unwrap();
                pc = if value % 2 == 0 { index } else { pc + 1 };
            },
            Op::Jio(reg, index) => {
                let value = *registers.get_mut(&reg).unwrap();
                pc = if value == 1 { index } else { pc + 1 };
            },
        }
    }
    *registers.get_mut(&'b').unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "jio a, +16
inc a
inc a
tpl a
tpl a
tpl a
inc a
inc a
tpl a
inc a
inc a
tpl a
tpl a
tpl a
inc a
jmp +23
tpl a
inc a
inc a
tpl a
inc a
inc a
tpl a
tpl a
inc a
inc a
tpl a
inc a
tpl a
inc a
tpl a
inc a
inc a
tpl a
inc a
tpl a
tpl a
inc a
jio a, +8
inc b
jie a, +4
tpl a
inc a
jmp +2
hlf a
jmp -7";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 170);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 247);
    }
}