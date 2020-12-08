use crate::common::read_lines;

fn parse_input() -> Vec<u64> {
    read_lines("./inputs/day1")
        .iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect()
}

pub fn puzzle1() -> String {
    let inputs = parse_input();
    for (i, x) in inputs.iter().enumerate() {
        for y in inputs[i..].iter() {
            if x + y == 2020 {
                return format!("D1P1: {}", x * y);
            }
        }
    }
    return String::new();
}

pub fn puzzle2() -> String {
    let inputs = parse_input();
    for (i, x) in inputs.iter().enumerate() {
        for (j, y) in inputs[i..].iter().enumerate() {
            if x + y >= 2020 {
                continue;
            }
            for z in inputs[j..].iter() {
                if x + y + z == 2020 {
                    return format!("D1P2: {}", x * y * z);
                }
            }
        }
    }
    return String::new();
}
