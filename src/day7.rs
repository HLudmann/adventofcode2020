use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::common::read_lines;

fn line_to_rule(line: &String) -> (String, HashMap<String, usize>) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?P<nbr>[1-9]+) (?P<color>[a-z]+ [a-z]+) bags?.?$").unwrap();
    }
    let mut bags_in_rule: HashMap<String, usize> = HashMap::new();
    let split = line.split(" bags contain ").collect::<Vec<&str>>();
    if split[1].contains("no other") {
        return (split[0].to_string(), bags_in_rule);
    }
    let bags = split[1].split(", ").collect::<Vec<&str>>();
    for bag in bags {
        let capt = RE.captures(bag).unwrap();
        let nbr = capt["nbr"].parse::<usize>().unwrap();
        let color = capt["color"].to_string();
        bags_in_rule.insert(color, nbr);
    }
    (split[0].to_string(), bags_in_rule)
}

fn parse_input() -> HashMap<String, HashMap<String, usize>> {
    read_lines("./inputs/day7")
        .iter()
        .map(|l| line_to_rule(l))
        .collect()
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

pub fn puzzle1() -> usize {
    let rules = parse_input();
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

    can_contain.len()
}

pub fn puzzle2() -> usize {
    count_bags_in_bag(&parse_input(), &"shiny gold".to_string())
}
