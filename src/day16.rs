use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

use crate::common::read_lines;

fn parse_input() -> (
    HashMap<String, (usize, usize, usize, usize)>,
    Vec<Vec<usize>>,
) {
    lazy_static! {
        static ref RULE: Regex =
            Regex::new(r"^(?P<name>[a-z ]+): (?P<ll>\d+)-(?P<lh>\d+) or (?P<hl>\d+)-(?P<hh>\d+)$")
                .unwrap();
        static ref PASS: Regex = Regex::new(r"^(|your ticket:|nearby tickets:)$").unwrap();
    }

    let lines = read_lines("./inputs/day16");
    let mut rules: HashMap<String, (usize, usize, usize, usize)> = HashMap::new();
    let mut tickets: Vec<Vec<usize>> = Vec::new();

    let mut i = 0;

    while i < lines.len() {
        let rule = RULE.captures(&lines[i]);
        if rule.is_some() {
            let uw_rule = rule.unwrap();
            let name = uw_rule["name"].to_string();
            let ll = uw_rule["ll"].parse::<usize>().unwrap();
            let lh = uw_rule["lh"].parse::<usize>().unwrap();
            let hl = uw_rule["hl"].parse::<usize>().unwrap();
            let hh = uw_rule["hh"].parse::<usize>().unwrap();
            rules.insert(name, (ll, lh, hl, hh));
            i += 1;
            continue;
        }
        if PASS.is_match(&lines[i]) {
            i += 1;
            continue;
        }
        let ticket: Vec<usize> = lines[i]
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        tickets.push(ticket);
        i += 1;
    }

    (rules, tickets)
}

fn match_rule(rule: &(usize, usize, usize, usize), value: &usize) -> bool {
    return (rule.0 <= *value && *value <= rule.1) || (rule.2 <= *value && *value <= rule.3);
}

fn is_good(rules: &HashMap<String, (usize, usize, usize, usize)>, ticket: &Vec<usize>) -> bool {
    for u in ticket.iter() {
        let mut matched = false;
        for (_name, bornes) in rules.iter() {
            if match_rule(bornes, u) {
                matched = true;
                break;
            }
        }
        if !matched {
            return false;
        }
    }
    return true;
}

pub fn puzzle1() -> usize {
    let (rules, tickets) = parse_input();
    let mut unmatched: Vec<usize> = Vec::new();

    tickets.iter().flatten().for_each(|u| {
        let mut matched = false;
        for (_name, bornes) in rules.iter() {
            if (bornes.0 <= *u && *u <= bornes.1) || (bornes.2 <= *u && *u <= bornes.3) {
                matched = true;
                break;
            }
        }
        if !matched {
            unmatched.push(*u);
        }
    });
    unmatched.iter().sum()
}

pub fn puzzle2() -> usize {
    let (rules, tickets) = parse_input();
    let good_tickets: Vec<Vec<usize>> = tickets
        .iter()
        .filter(|t| is_good(&rules, t))
        .map(|t| t.to_vec())
        .collect();
    let mut rule_order: Vec<Vec<String>> = Vec::new();
    for i in 0..tickets[0].len() {
        let mut poss_rule: Vec<String> = rules.keys().map(|k| k.to_string()).collect();
        for ticket in good_tickets.iter() {
            if poss_rule.iter().len() == 1 {
                break;
            }
            let mut to_keep = Vec::new();
            for rule in poss_rule {
                if match_rule(rules.get(&rule).unwrap(), &ticket[i]) {
                    to_keep.push(rule);
                }
            }
            poss_rule = to_keep;
        }
        rule_order.push(poss_rule);
    }
    let len = rule_order.len();
    loop {
        if rule_order.iter().flatten().collect::<Vec<&String>>().len() == len {
            break;
        }
        let mut i = 0;
        while i < len {
            if rule_order[i].len() != 1 {
                i += 1;
                continue;
            }
            let mut new_order: Vec<Vec<String>> = Vec::new();
            let name = &rule_order[i][0];
            for (j, rsb) in rule_order.iter().enumerate() {
                if i == j {
                    new_order.push(vec![name.to_string()]);
                    continue;
                }
                new_order.push(
                    rsb.iter()
                        .filter(|r| **r != name.to_string())
                        .map(|r| r.to_string())
                        .collect(),
                )
            }
            rule_order = new_order;
            i += 1;
        }
    }
    rule_order
        .iter()
        .flatten()
        .enumerate()
        .filter(|(_, r)| (**r).starts_with("departure"))
        .map(|(i, _r)| tickets[0][i])
        .product()
}
