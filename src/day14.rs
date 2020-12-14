use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

use crate::common::read_lines;

fn parse_input1() -> HashMap<usize, usize> {
    lazy_static! {
        static ref MASK: Regex = Regex::new(r"^mask = (?P<mask>[X01]{36})$").unwrap();
        static ref MEM: Regex = Regex::new(r"^mem\[(?P<id>[0-9]+)\] = (?P<entry>[0-9]+)$").unwrap();
    }
    let mut mem = HashMap::new();
    let mut mask: String = String::new();
    for line in read_lines("./inputs/day14").iter() {
        if MASK.is_match(line) {
            mask = MASK.captures(line).unwrap()["mask"].to_string();
            continue;
        }
        let mem_capt = MEM.captures(line).unwrap();
        let id = mem_capt["id"].parse::<usize>().unwrap();
        let entry = format!("{:36b}", mem_capt["entry"].parse::<usize>().unwrap());
        let mut val = 0;
        for (i, (m, v)) in mask.chars().zip(entry.chars()).enumerate() {
            match (m, v) {
                ('1', _) => val += 2_usize.pow(35 - i as u32),
                ('X', '1') => val += 2_usize.pow(35 - i as u32),
                _ => (),
            }
        }
        let mem_entry = mem.entry(id).or_insert(0);
        *mem_entry = val;
    }
    mem
}

fn parse_input2() -> HashMap<usize, usize> {
    lazy_static! {
        static ref MASK: Regex = Regex::new(r"^mask = (?P<mask>[X01]{36})$").unwrap();
        static ref MEM: Regex = Regex::new(r"^mem\[(?P<id>[0-9]+)\] = (?P<entry>[0-9]+)$").unwrap();
    }
    let mut mem = HashMap::new();
    let mut mask: String = String::new();
    for line in read_lines("./inputs/day14").iter() {
        if MASK.is_match(line) {
            mask = MASK.captures(line).unwrap()["mask"].to_string();
            continue;
        }
        let mem_capt = MEM.captures(line).unwrap();
        let id_bin = format!("{:36b}", mem_capt["id"].parse::<usize>().unwrap());
        let entry = mem_capt["entry"].parse::<usize>().unwrap();
        let mut ids = vec![0];
        for (i, (m, v)) in mask.chars().zip(id_bin.chars()).enumerate() {
            match (m, v) {
                ('1', _) => {
                    ids = ids
                        .iter()
                        .map(|e| *e + 2_usize.pow(35 - i as u32))
                        .collect()
                }
                ('0', '1') => {
                    ids = ids
                        .iter()
                        .map(|e| *e + 2_usize.pow(35 - i as u32))
                        .collect()
                }
                ('X', _) => {
                    let floatings: Vec<usize> = ids
                        .iter()
                        .cloned()
                        .map(|e| e + 2_usize.pow(35 - i as u32))
                        .collect();
                    ids.extend(floatings);
                }
                _ => (),
            }
        }
        for n_id in ids {
            let mem_entry = mem.entry(n_id).or_insert(0);
            *mem_entry = entry;
        }
    }
    mem
}

pub fn puzzle1() -> usize {
    parse_input1().values().sum()
}

pub fn puzzle2() -> usize {
    parse_input2().values().sum()
}
