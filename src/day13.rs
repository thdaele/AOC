use std::cmp;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

// https://github.com/gobanos/cargo-aoc/issues/20
// #[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (HashSet<&str>, HashMap<&str, HashMap<&str, i32>>) {
    let mut table = HashSet::new();
    let mut happiness: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    for line in input.lines() {
        let mut parts = line.trim().split(' ');
        let person1 = parts.next().unwrap();
        parts.next(); // would
        let sign = parts.next().unwrap(); // gain
        let sign = match sign {
            "gain" => 1,
            "lose" => -1,
            _ => unreachable!()
        };
        let mut value = parts.next().unwrap().parse().unwrap();
        value *= sign;
        parts.next(); // happiness
        parts.next(); // units
        parts.next(); // by
        parts.next(); // sitting
        parts.next(); // next
        parts.next(); // to
        let person2 = parts.next().unwrap();
        let person2= &person2[..person2.len() - 1]; // Cut of the point at the end

        table.insert(person1);
        table.insert(person2);

        insert_into_happiness(person1, person2, value, &mut happiness);
    }
    (table, happiness)
}

fn insert_into_happiness<'a>(person1: &'a str, person2: &'a str, value: i32, happiness: &mut HashMap<&'a str, HashMap<&'a str, i32>>) {
    let sub_graph = match happiness.get_mut(person1) {
        None => {
            let sub_graph = HashMap::new();
            happiness.insert(person1, sub_graph);
            happiness.get_mut(person1).unwrap()
        },
        Some(sub_graph) => sub_graph
    };
    sub_graph.insert(person2, value);
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let input = input_generator(input);
    let table = &input.0;
    let happiness = &input.1;
    let mut max = i32::MIN;
    for perm in table.iter().permutations(table.len()) {
        let mut total = 0;
        for i in 0..table.len() {
            let curr = perm[i];
            let next = perm[(i+1) % perm.len()];
            total += happiness[curr][next];
            total += happiness[next][curr];
        }
        max = cmp::max(max, total);
    }
    max
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut input = input_generator(input);
    let table = &mut input.0;
    let mut happiness = input.1;

    table.insert("self");
    for person in table.iter() {
        insert_into_happiness("self", person, 0, &mut happiness);
        insert_into_happiness(person, "self", 0, &mut happiness);
    }

    let mut max = i32::MIN;
    for perm in table.iter().permutations(table.len()) {
        let mut total = 0;
        for i in 0..table.len() {
            let curr = perm[i];
            let next = perm[(i+1) % perm.len()];
            total += happiness[curr][next];
            total += happiness[next][curr];
        }
        max = cmp::max(max, total);
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let string = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";
        assert_eq!(solve_part1(string), 330);
    }
}