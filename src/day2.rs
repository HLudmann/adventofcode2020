use lazy_static::lazy_static;
use regex::Regex;

use crate::common::read_lines;

fn parse_input_by_line(line: &String) -> (usize, usize, char, String) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    }
    let capt = RE.captures(&line).unwrap();

    let u1: usize = capt[1].parse().unwrap();
    let u2: usize = capt[2].parse().unwrap();
    let req: char = capt[3].chars().nth(0).unwrap();
    let pwd: String = capt[4].to_string();

    (u1, u2, req, pwd)
}

fn check_pwd_method1(parsed: &(usize, usize, char, String)) -> bool {
    let cnt = parsed.3.chars().filter(|c| c == &parsed.2).count();

    parsed.0 <= cnt && cnt <= parsed.1
}

fn check_pwd_method2(parsed: &(usize, usize, char, String)) -> bool {
    let char_vec: Vec<char> = parsed.3.chars().collect();
    (char_vec[parsed.0 - 1] == parsed.2) ^ (char_vec[parsed.1 - 1] == parsed.2)
}

pub fn puzzle1() -> String {
    let cnt = read_lines("./inputs/day2")
        .iter()
        .map(|l| parse_input_by_line(l))
        .filter(|p| check_pwd_method1(p))
        .count();
    format!("D2P1: {}", cnt)
}

pub fn puzzle2() -> String {
    let cnt = read_lines("./inputs/day2")
        .iter()
        .map(|l| parse_input_by_line(l))
        .filter(|p| check_pwd_method2(p))
        .count();
    format!("D2P2: {}", cnt)
}
