use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
    STORE(String, String),
    AND(String, String, String),
    OR(String, String, String),
    LSHIFT(String, String, String),
    RSHIFT(String, String, String),
    NOT(String, String)
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, Instruction> {
    let mut hash_map: HashMap<String, Instruction> = HashMap::new();
    for line in input.lines() {
        let mut tokens = line.split("->");

        let tokens_first_part :Vec<&str> = tokens.next().unwrap().trim().split(' ').collect();
        let register = tokens.next().unwrap().trim();
        if tokens_first_part.len() == 1 {
            let reg: String = register.parse().unwrap();
            hash_map.insert(reg.clone(), Instruction::STORE(tokens_first_part.get(0).unwrap().parse().unwrap(), reg));
        } else if tokens_first_part.len() == 2 {
            let reg: String = register.parse().unwrap();
            hash_map.insert(reg.clone(), Instruction::NOT(tokens_first_part.get(1).unwrap().parse().unwrap(), reg));
        } else {
            let reg: String = register.parse().unwrap();
            let instruction = match *tokens_first_part.get(1).unwrap() {
                "LSHIFT" => Instruction::LSHIFT(tokens_first_part.get(0).unwrap().parse().unwrap(),
                                                tokens_first_part.get(2).unwrap().parse().unwrap(),
                                                reg.clone()),
                "RSHIFT" => Instruction::RSHIFT(tokens_first_part.get(0).unwrap().parse().unwrap(),
                                                tokens_first_part.get(2).unwrap().parse().unwrap(),
                                                reg.clone()),
                "AND" => Instruction::AND(tokens_first_part.get(0).unwrap().parse().unwrap(),
                                          tokens_first_part.get(2).unwrap().parse().unwrap(),
                                          reg.clone()),
                "OR" => Instruction::OR(tokens_first_part.get(0).unwrap().parse().unwrap(),
                                        tokens_first_part.get(2).unwrap().parse().unwrap(),
                                        reg.clone()),
                _ => unreachable!()
            };
            hash_map.insert(reg, instruction);
        }
    }
    hash_map
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &HashMap<String, Instruction>) -> u16 {
    let mut memo: HashMap<String, u16> = HashMap::new();
    recursion(input, &mut memo, &String::from('a'))
}

pub fn recursion(input: &HashMap<String, Instruction>, memo: &mut HashMap<String, u16>, string: &String) -> u16 {
    match memo.get(string) {
        None => {}
        Some(result) => return *result
    }
    let instruction = match input.get(&*string) {
        None => return string.parse().unwrap(),
        Some(instruction) => instruction
    };

    let result: u16 = match instruction {
        Instruction::STORE(value, _) => recursion(input, memo,value),
        Instruction::AND(reg1, reg2, _) => recursion(input, memo,reg1) & recursion(input, memo,reg2),
        Instruction::OR(reg1, reg2, _) => recursion(input, memo,reg1) | recursion(input, memo,reg2),
        Instruction::NOT(reg1, _) => !recursion(input, memo,reg1),
        Instruction::LSHIFT(reg1, value, _) => recursion(input, memo,reg1) << value.parse::<u16>().unwrap(),
        Instruction::RSHIFT(reg1, value, _) => recursion(input, memo,reg1) >> value.parse::<u16>().unwrap(),
    };
    memo.insert(string.parse().unwrap(), result);
    result
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &HashMap<String, Instruction>) -> u16 {
    let mut memo: HashMap<String, u16> = HashMap::new();
    memo.insert(String::from("b"), 16076);

    recursion(input, &mut memo, &String::from('a'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let string = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> a";
        assert_eq!(solve_part1(&input_generator(string)), 65079);
    }
}