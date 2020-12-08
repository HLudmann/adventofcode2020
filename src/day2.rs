use lazy_static::lazy_static;
use regex::Regex;

use crate::common::read_lines;

// pub fn parse_line(line: &str) -> (u64, u64, String, String) {
fn parse_line(line: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    }
    RE.captures(line)
        .and_then(|cap| Some(cap))
        .unwrap()
        .iter()
        .map(|m| m.unwrap().as_str().to_string())
        .collect()
}

fn test_parsed_line1(parsed: Vec<String>) -> bool {
    let min = parsed[1].parse::<usize>().unwrap();
    let max = parsed[2].parse::<usize>().unwrap();
    let req = parsed[3].chars().nth(0).unwrap();
    let cnt = parsed[4].chars().filter(|c| c == &req).count();

    min <= cnt && cnt <= max
}

pub fn puzzle1() -> String {
    let cnt = read_lines("./inputs/day2")
        .unwrap()
        .map(|l| parse_line(l.unwrap().as_str()))
        .filter(|p| test_parsed_line1(p.to_vec()))
        .count();
    format!("D2P1: {}", cnt)
}

fn test_parsed_line2(parsed: Vec<String>) -> bool {
    let pos1 = parsed[1].parse::<usize>().unwrap();
    let pos2 = parsed[2].parse::<usize>().unwrap();
    let req = parsed[3].chars().nth(0).unwrap();
    let char_vec: Vec<char> = parsed[4].chars().collect();

    (char_vec[pos1 - 1] == req) ^ (char_vec[pos2 - 1] == req)
}

pub fn puzzle2() -> String {
    let cnt = read_lines("./inputs/day2")
        .unwrap()
        .map(|l| parse_line(l.unwrap().as_str()))
        .filter(|p| test_parsed_line2(p.to_vec()))
        .count();
    format!("D2P2: {}", cnt)
}
