use crate::common::read_lines;

pub fn puzzle1() -> String {
    let inputs = read_lines("./inputs/day1").unwrap().map(|l| l.unwrap().parse::<u64>().unwrap()).collect::<Vec<u64>>();
    for x in inputs.clone() {
        for y in inputs.clone() {
            if x + y == 2020 {
                return format!("D1P1: {a} + {b} = 2020\t{a} * {b} = {c}", a = x, b = y, c = x * y)
            }
        }
    }
    return String::new()
}

pub fn puzzle2() -> String {
    let inputs = read_lines("./inputs/day1").unwrap().map(|l| l.unwrap().parse::<u64>().unwrap()).collect::<Vec<u64>>();
    for x in inputs.clone() {
        for y in inputs.clone() {
            if x + y >= 2020 {
                continue;
            }
            for z in inputs.clone() {
                if x + y + z == 2020 {
                    return format!("D1P2: {a} + {b} + {c} = 2020\t{a} * {b} * {c} = {d}", a = x, b = y, c = z, d = x * y * z)
                }
            }
        }
    }
    return String::new()
}
