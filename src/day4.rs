use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

use crate::common::read_lines;

const BYR: &str = "byr";
const IYR: &str = "iyr";
const EYR: &str = "eyr";
const HGT: &str = "hgt";
const HCL: &str = "hcl";
const ECL: &str = "ecl";
const PID: &str = "pid";

fn parse_lines_to_passports() -> Vec<HashMap<String, String>> {
    let lines = read_lines("./inputs/day4").unwrap().map(|l| l.unwrap()).collect::<Vec<String>>();
    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let mut passport = HashMap::new();

    for line in lines {
        if line.len() == 0 {
            passports.push(passport);
            passport = HashMap::new();
            continue
        }
        let pairs = line.split(" ").collect::<Vec<&str>>();
        for pair in pairs {
            let keyval = pair.split(":").collect::<Vec<&str>>();
            passport.insert(keyval[0].to_string(), keyval[1].to_string());
        }
    }
    passports.push(passport);
    passports
}

fn passport_simple_test(passport: &HashMap<String, String>) -> bool {
    lazy_static! {
        static ref REQ_KEYS: Vec<&'static str> = vec![BYR, IYR, EYR, HGT, HCL, ECL, PID];
    }
    for key in REQ_KEYS.iter() {
        if !passport.contains_key(&key.to_string()) { return false }
    }
    true
}

fn passport_extended_test(passport: &HashMap<String, String>) -> bool {
    match passport.get(BYR).unwrap().parse::<usize>() {
        Ok(byr) => if byr < 1920 || byr > 2002 { return false },
        _ => return false
    }

    match passport.get(IYR).unwrap().parse::<usize>() {
        Ok(iyr) => if iyr < 2010 || iyr > 2020 { return false },
        _ => return false
    }

    match passport.get(EYR).unwrap().parse::<usize>() {
        Ok(eyr) => if eyr < 2020 || eyr > 2030 { return false },
        _ => return false
    }

    lazy_static! {
        static ref HGT_REF: Regex = Regex::new(r"^([0-9]{3}cm|[0-9]{2}in)$").unwrap();
        static ref HCL_REG: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref ECL_REG: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PID_REG: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }

    let hgt_str = passport.get(HGT).unwrap();
    if !HGT_REF.is_match(hgt_str) { return false }
    let hgt = match hgt_str[..=hgt_str.len()-3].to_string().parse::<usize>() {
        Ok(val) => val,
        _ => return false
    };
    match &hgt_str[hgt_str.len()-2..=hgt_str.len()-1] {
        "cm" => if hgt < 150 || hgt > 193 { return false }
        "in" => if hgt < 59 || hgt > 76 { return false }
        _ => return false
    }

    if !HCL_REG.is_match(passport.get(HCL).unwrap()) { return false }
    if !ECL_REG.is_match(passport.get(ECL).unwrap()) { return false }
    if !PID_REG.is_match(passport.get(PID).unwrap()) { return false }
    
    true
}

pub fn puzzle1() -> String {
    let cnt = parse_lines_to_passports().iter().filter(|p| passport_simple_test(*p)).count();
    format!("D4P1: {}", cnt)
}

pub fn puzzle2() -> String {
    let cnt = parse_lines_to_passports().iter().filter(|p| passport_simple_test(*p)).filter(|p| passport_extended_test(p)).count();
    format!("D4P2: {}", cnt)
}
