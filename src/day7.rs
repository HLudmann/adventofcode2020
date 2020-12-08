use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::common::read_lines;

fn line_to_rule(line: String) -> (String, HashMap<String, usize>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([1-9]+) ([a-z]+ [a-z]+) bags?.?$").unwrap();
    }
    let mut bags_in_rule: HashMap<String, usize> = HashMap::new();
    let split = line.split(" bags contain ").collect::<Vec<&str>>();
    if split[1].contains("no other") {
        return (split[0].to_string(), bags_in_rule);
    }
    let bags = split[1].split(", ").collect::<Vec<&str>>();
    for bag in bags {
        let capt = RE
            .captures(bag)
            .and_then(|cap| Some(cap))
            .unwrap()
            .iter()
            .map(|m| m.unwrap().as_str().to_string())
            .collect::<Vec<String>>();
        let nbr = capt[1].parse::<usize>().unwrap();
        bags_in_rule.insert(capt[2].to_string(), nbr);
    }
    (split[0].to_string(), bags_in_rule)
}

fn can_contain_n_visited<'a>(
    rules: &'a HashMap<String, HashMap<String, usize>>,
    bag: &'a String,
) -> (bool, HashSet<&'a String>) {
    let mut visited: HashSet<&'a String> = HashSet::new();
    let content = rules.get(bag).unwrap();
    if content.is_empty() {
        return (false, visited);
    }
    if content.contains_key("shiny gold") {
        visited.insert(bag);
        return (true, visited);
    }
    for k in content.keys() {
        if visited.contains(k) {
            continue;
        }
        let (k_ok, k_visited) = can_contain_n_visited(rules, k);
        if k_ok {
            visited.extend(k_visited)
        }
    }
    if visited.is_empty() {
        return (false, visited);
    }
    visited.insert(bag);
    (true, visited)
}

fn count_bags_in_bag<'a>(
    rules: &'a HashMap<String, HashMap<String, usize>>,
    bag: &'a String,
) -> usize {
    let content = rules.get(bag).unwrap();
    if content.is_empty() {
        return 0;
    }
    content
        .iter()
        .map(|(k, v)| v * (1 + count_bags_in_bag(rules, k)))
        .sum::<usize>()
}

pub fn puzzle1() -> String {
    let mut rules: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for rule in read_lines("./inputs/day7")
        .unwrap()
        .map(|l| l.unwrap())
        .map(|l| line_to_rule(l))
    {
        rules.insert(rule.0, rule.1);
    }
    // println!("rules keys: {:?}", rules.keys());
    let mut can_contain: HashSet<&String> = HashSet::new();
    for bag in rules.keys() {
        if can_contain.contains(bag) {
            continue;
        }
        let (ok, visited) = can_contain_n_visited(&rules, bag);
        if ok {
            can_contain.extend(visited)
        }
    }

    format!("D7P1: {:?}", can_contain.len())
}

pub fn puzzle2() -> String {
    let mut rules: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for rule in read_lines("./inputs/day7")
        .unwrap()
        .map(|l| l.unwrap())
        .map(|l| line_to_rule(l))
    {
        rules.insert(rule.0, rule.1);
    }

    format!(
        "D7P2: {}",
        count_bags_in_bag(&rules, &"shiny gold".to_string())
    )
}
