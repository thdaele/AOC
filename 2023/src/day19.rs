use std::cmp::{max, min};
use std::ops::{Range};

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap;

struct Rule {
    workflow: String,
    value: u32,
    operator: String,
    variable: String,
    rule: Box<dyn Fn((&Part, u32, String)) -> Option<String>>
}

struct Workflow {
    name: String,
    rules: Vec<Rule>
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

#[aoc_generator(day19)]
fn parse(input: &str) -> (Vec<Part>, FxHashMap<String, Workflow>) {
    let (rules_str, parts_str) = input.split_once("\n\n").unwrap();
    let mut workflows = FxHashMap::default();
    for workflow in rules_str.lines() {
        let (workflow_name, rules_str) = workflow.split_once('{').unwrap();
        let workflow_name = workflow_name.to_string();
        let mut rules= vec![];
        let rules_str = &rules_str[0..rules_str.len() - 1]; // Remove the }
        for rule_str in rules_str.split(',') {
            if rule_str.contains(':') {
                let (condition, workflow) = rule_str.split_once(':').unwrap();
                let condition = condition.to_string();
                let variable = &condition[0..1];
                let operator = &condition[1..2];
                let value: u32 = condition[2..].parse().unwrap();

                let rule: Box<dyn Fn((&Part, u32, String)) -> Option<String>> = match variable {
                    "x" => match operator {
                        ">" => Box::new(|(part, value, workflow)| if part.x > value {Some(workflow)} else {None}),
                        "<" => Box::new(|(part, value, workflow)| if part.x < value {Some(workflow)} else {None}),
                        _ => unreachable!()
                    },
                    "m" => match operator {
                        ">" => Box::new(|(part, value, workflow)| if part.m > value {Some(workflow)} else {None}),
                        "<" => Box::new(|(part, value, workflow)| if part.m < value {Some(workflow)} else {None}),
                        _ => unreachable!()
                    },
                    "a" => match operator {
                        ">" => Box::new(|(part, value, workflow)| if part.a > value {Some(workflow)} else {None}),
                        "<" => Box::new(|(part, value, workflow)| if part.a < value {Some(workflow)} else {None}),
                        _ => unreachable!()
                    },
                    "s" => match operator {
                        ">" => Box::new(|(part, value, workflow)| if part.s > value {Some(workflow)} else {None}),
                        "<" => Box::new(|(part, value, workflow)| if part.s < value {Some(workflow)} else {None}),
                        _ => unreachable!()
                    },
                    _ => unreachable!()
                };
                rules.push(Rule{ value, rule, operator: operator.to_string(), variable: variable.to_string(), workflow: workflow.to_string() });
            } else {
                rules.push(Rule{value: 0, operator: "".to_string(), variable: "".to_string(), rule: Box::new(|(_, _, workflow)| Some(workflow)), workflow: rule_str.to_string()});
            }
        }
        workflows.insert(workflow_name.clone(), Workflow{name: workflow_name, rules});
    }


    let mut parts = vec![];
    for part_str in parts_str.lines() {
        let part_str = &part_str[1..part_str.len() - 1]; // Remove the {}
        let part_values = part_str.split(',');
        let mut values = vec![];
        for part in part_values {
            let (_, value) = part.split_once('=').unwrap();
            values.push(value.parse().unwrap());
        }
        // The order of the variables in the parts is xmas
        debug_assert!(values.len() == 4);
        parts.push(Part {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3]
        })
    }
    (parts, workflows)
}

#[aoc(day19, part1)]
fn part1(input: &(Vec<Part>, FxHashMap<String, Workflow>)) -> u32 {
    let (parts, workflows) = input;

    let mut sum = 0;
    for part in parts {
        let mut workflow = "in".to_string();
        while workflow != "R" && workflow != "A" {
            let rules = &workflows.get(&*workflow).unwrap().rules;
            for rule in rules {
                let function = &rule.rule;
                let result = function((part, rule.value, rule.workflow.clone()));
                if let Some(workflow_name) = result {
                    workflow = workflow_name;
                    break;
                }
            }
        }
        if workflow == "A" {
            sum += part.x + part.m + part.a + part.s;
        }
    }
    sum
}

fn intersect(first: &Range<u64>, second: &Range<u64>) -> Option<Range<u64>> {
    if first.start <= second.end && first.end >= second.start {
        return Some(max(first.start, second.start)..min(first.end, second.end))
    }
    None
}

fn solve(mut part: (Range<u64>, Range<u64>, Range<u64>, Range<u64>), workflow: String, workflows: &FxHashMap<String, Workflow>) -> u64 {
    if workflow == "A" {
        return part.0.count() as u64 * part.1.count() as u64 * part.2.count() as u64 * part.3.count() as u64;
    } else if workflow == "R" {
        return 0;
    }
    let mut sum = 0;
    let rules = &workflows.get(&*workflow).unwrap().rules;
    for rule in rules {
        if part.0.is_empty() || part.1.is_empty() || part.2.is_empty() || part.3.is_empty() {
            return 0
        }
        if rule.operator == "<" {
            match rule.variable.as_str() {
                "x" => {
                    let result = intersect(&(1..rule.value as u64), &part.0);
                    if let Some(range) = result {
                        let mut part_clone = part.clone();
                        part_clone.0 = range;
                        sum += solve(part_clone, rule.workflow.clone(), workflows);
                        part.0 = intersect(&(rule.value as u64..4001), &part.0).unwrap();
                    }
                },
                "m" => {
                    let result = intersect(&(1..rule.value as u64), &part.1);
                    if let Some(range) = result {
                        let mut part_clone = part.clone();
                        part_clone.1 = range;
                        sum += solve(part_clone, rule.workflow.clone(), workflows);
                        part.1 = intersect(&(rule.value as u64..4001), &part.1).unwrap();
                    }
                },
                "a" => {
                    let result = intersect(&(1..rule.value as u64), &part.2);
                    if let Some(range) = result {
                        let mut part_clone = part.clone();
                        part_clone.2 = range;
                        sum += solve(part_clone, rule.workflow.clone(), workflows);
                        part.2 = intersect(&(rule.value as u64..4001), &part.2).unwrap();
                    }
                },
                "s" => {
                    let result = intersect(&(1..rule.value as u64), &part.3);
                    if let Some(range) = result {
                        let mut part_clone = part.clone();
                        part_clone.3 = range;
                        sum += solve(part_clone, rule.workflow.clone(), workflows);
                        part.3 = intersect(&(rule.value as u64..4001), &part.3).unwrap();
                    }
                },
                _ => unreachable!()
            }
        } else if rule.operator == ">" {
            match rule.variable.as_str() {
                "x" => {
                    let result = intersect(&(rule.value as u64 + 1..4001), &part.0);
                    if let Some(range) = result {
                        let mut part_clone = part.clone();
                        part_clone.0 = range;
                        sum += solve(part_clone, rule.workflow.clone(), workflows);
                        part.0 = intersect(&(1..rule.value as u64 + 1), &part.0).unwrap();
                    }
                },
                "m" => {
                    let result = intersect(&(rule.value as u64 + 1..4001), &part.1);
                    if let Some(range) = result {
                        let mut part_clone = part.clone();
                        part_clone.1 = range;
                        sum += solve(part_clone, rule.workflow.clone(), workflows);
                        part.1 = intersect(&(1..rule.value as u64 + 1), &part.1).unwrap();
                    }
                },
                "a" => {
                    let result = intersect(&(rule.value as u64 + 1..4001), &part.2);
                    if let Some(range) = result {
                        let mut part_clone = part.clone();
                        part_clone.2 = range;
                        sum += solve(part_clone, rule.workflow.clone(), workflows);
                        part.2 = intersect(&(1..rule.value as u64 + 1), &part.2).unwrap();
                    }
                },
                "s" => {
                    let result = intersect(&(rule.value as u64 + 1..4001), &part.3);
                    if let Some(range) = result {
                        let mut part_clone = part.clone();
                        part_clone.3 = range;
                        sum += solve(part_clone, rule.workflow.clone(), workflows);
                        part.3 = intersect(&(1..rule.value as u64 + 1), &part.3).unwrap();
                    }
                },
                _ => unreachable!()
            }
        } else {
            sum += solve(part.clone(), rule.workflow.clone(), workflows);
        }
    }
    sum
}

#[aoc(day19, part2)]
fn part2(input: &(Vec<Part>, FxHashMap<String, Workflow>)) -> u64 {
    let (_, workflows) = input;

    let part = (1..4001, 1..4001, 1..4001, 1..4001);
    solve(part, "in".to_string(), workflows)
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 19114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 167409079868000);
    }
}