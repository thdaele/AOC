use std::collections::VecDeque;
use std::str::FromStr;

use aoc_runner_derive::aoc;
use num::integer::lcm;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Pulse {
    High,
    Low
}

#[derive(PartialEq, Eq, Clone)]
enum Module {
    Broadcaster,
    FlipFlop,
    Conjunction
}

#[derive(Debug, PartialEq, Eq)]
struct ParseConfigurationError;

#[derive(Clone)]
struct Configuration {
    name: String,
    module: Module,
    destination: Vec<String>,
    flip_flop_state: Option<bool>,
    conjunction_state: Option<FxHashMap<String, Pulse>>
}

impl Configuration {
    fn forward(&mut self, source: String, pulse: Pulse) -> Option<Pulse> {
        if let Some(state) = self.flip_flop_state.as_mut() {
            if pulse == Pulse::Low {
                *state = !*state;
                return if *state {Some(Pulse::High)} else {Some(Pulse::Low)}
            }
            return None
        } else if let Some(map) = self.conjunction_state.as_mut() {
            map.entry(source).and_modify(|saved_pulse| *saved_pulse = pulse);

            let all = map.iter().all(|(_, saved_pulse)| *saved_pulse == Pulse::High);
            return if all {Some(Pulse::Low)} else {Some(Pulse::High)}
        }
        Some(pulse)
    }
    fn init(&mut self, sources: &[String]) {
        if self.module == Module::Conjunction {
            let mut map: FxHashMap<String, Pulse> = FxHashMap::default();
            sources.iter().for_each(|source| {
                map.insert(source.clone(), Pulse::Low);
            });

            self.conjunction_state = Some(map);
        }
    }
}

impl FromStr for Configuration {
    type Err = ParseConfigurationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (def, dest) = s.split_once(" -> ").ok_or(ParseConfigurationError)?;
        let (name, module) = match &def[0..1] {
            "%" => (def[1..].to_string(), Module::FlipFlop),
            "&" => (def[1..].to_string(), Module::Conjunction),
            _ => (def[0..].to_string(), Module::Broadcaster),
        };
        let flip_flop_state = if module == Module::FlipFlop {Some(false)} else {None};
        let destination = dest.split(", ").map(|dest| dest.to_string()).collect();
        Ok(Configuration{name, module, destination, flip_flop_state , conjunction_state: None })
    }
}


// #[aoc_generator(day20)]
fn parse(input: &str) -> FxHashMap<String, Configuration> {
    let mut map: FxHashMap<String, Vec<String>> = FxHashMap::default();
    let mut result: FxHashMap<String, Configuration> = input.lines().map(|line| {
        let conf = line.parse::<Configuration>().unwrap();

        for dest in &conf.destination {
            let sources = map.entry(dest.clone()).or_default();
            sources.push(conf.name.clone());
        }

        (conf.name.clone(), conf)
    }).collect();

    result.iter_mut().for_each(|(k, v)| {
        if let Some(vec) = map.get(k) {
            v.init(vec);
        }
    });

    result
}

#[aoc(day20, part1)]
fn part1(input: &str) -> u32 {
    let input = &mut parse(input);
    let mut low_pulse = 0;
    let mut high_pulse = 0;
    let mut queue = VecDeque::new();
    for _ in 0..1000 {
        // Low pulse from button to broadcaster
        queue.push_back(("broadcaster".to_string(), Pulse::Low, "button".to_string()));

        while let Some((dest, pulse, source)) = queue.pop_front() {
            match pulse {
                Pulse::Low => low_pulse += 1,
                Pulse::High => high_pulse += 1
            }

            if let Some(conf) = input.get_mut(dest.as_str()) {
                let new_pulse = conf.forward(source, pulse);
                let source = dest;

                if let Some(new_pulse) = new_pulse {
                    conf.destination.iter().for_each(|dest| queue.push_back((dest.clone(), new_pulse, source.clone())));
                }
            }
            // If not in hashmap, like output in the example we can just ignore it
        }
    }
    low_pulse * high_pulse
}

#[aoc(day20, part2)]
fn part2(input: &str) -> u64 {
    let input = &mut parse(input);
    let mut queue = VecDeque::new();
    let mut last_seen: FxHashMap<String, u64> = FxHashMap::default();
    let (parent, _) = input.iter().find(|(_, v)| v.destination.contains(&"rx".to_string())).unwrap();
    // Works bc the nodes in search only have a single incoming connection
    let search: FxHashSet<String> = input.iter().filter_map(|(key, v)| {
        if v.destination.contains(parent) {
            return Some(key.clone())
        }
        None
    }).collect();
    let mut cycles = vec![];
    for i in 0..u64::MAX {
        // Low pulse from button to broadcaster
        queue.push_back(("broadcaster".to_string(), Pulse::Low, "button".to_string()));
        while let Some((dest, pulse, source)) = queue.pop_front() {
            if pulse == Pulse::Low {
                if dest == "rx" {
                    return i
                }
                if search.contains(&dest) && last_seen.get(&dest).is_some() {
                    cycles.push(i - last_seen.get(&dest).unwrap());
                }
                last_seen.insert(dest.clone(), i);
            }
            if search.len() == cycles.len() {
                break
            }

            if let Some(conf) = input.get_mut(dest.as_str()) {
                let new_pulse = conf.forward(source, pulse);
                let source = dest.clone();

                if let Some(new_pulse) = new_pulse {
                    conf.destination.iter().for_each(|dest| queue.push_back((dest.clone(), new_pulse, source.clone())));
                }
            }
        }
    }
    cycles.iter().copied().filter(|v| *v != 0).reduce(lcm).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const EXAMPLE_2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(EXAMPLE_1), 32000000);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part1(EXAMPLE_2), 11687500);
    }
}