use std::collections::{HashMap, HashSet};
use itertools::Itertools;

// https://github.com/gobanos/cargo-aoc/issues/20
// #[aoc_generator(day9)]
pub fn input_generator(input: &str) -> (HashSet<&str>, HashMap<&str, HashMap<&str, usize>>) {
    let mut places = HashSet::new();
    let mut graph : HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    for line in input.lines() {
        let mut parts = line.trim().split(' ');
        let source = parts.next().unwrap();
        parts.next();
        let destination = parts.next().unwrap();
        parts.next();
        let distance = parts.next().unwrap().parse().unwrap();
        places.insert(source);
        places.insert(destination);

        insert_into_graph(source, destination, distance, &mut graph);
        insert_into_graph(destination, source, distance, &mut graph);
    }
    (places, graph)
}

fn insert_into_graph<'a>(src: &'a str, dst: &'a str, dist: usize, graph: &mut HashMap<&'a str, HashMap<&'a str, usize>>) {
    let sub_graph = match graph.get_mut(src) {
        None => {
            let sub_graph = HashMap::new();
            graph.insert(src, sub_graph);
            graph.get_mut(src).unwrap()
        },
        Some(sub_graph) => sub_graph
    };
    sub_graph.insert(dst, dist);
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> usize {
    let input = input_generator(input);
    let places = &input.0;
    let graph = &input.1;
    let mut result: Vec<usize> = Vec::new();
    for perm in places.iter().permutations(places.len()) {
        result.push(perm.iter()
            .zip(perm.iter().skip(1))
            .map(|(src, dst)| *graph.get(*src).unwrap().get(*dst).unwrap())
            .sum());
    }
    *result.iter().min().unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> usize {
    let input = input_generator(input);
    let places = &input.0;
    let graph = &input.1;
    let mut result: Vec<usize> = Vec::new();
    for perm in places.iter().permutations(places.len()) {
        result.push(perm.iter()
            .zip(perm.iter().skip(1))
            .map(|(src, dst)| *graph.get(*src).unwrap().get(*dst).unwrap())
            .sum());
    }
    *result.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let string = "London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141";

        assert_eq!(solve_part1(string), 605);
    }

    #[test]
    fn example2() {
        let string = "London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141";

        assert_eq!(solve_part2(string), 982);
    }
}