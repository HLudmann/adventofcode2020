use crate::common::read_lines;

fn parse_input() -> Vec<usize> {
    read_lines("./inputs/day1")
        .iter()
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

pub fn puzzle1() -> usize {
    let inputs = parse_input();
    for (i, x) in inputs.iter().enumerate() {
        for y in inputs[i..].iter() {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    return 0;
}

pub fn puzzle2() -> usize {
    let inputs = parse_input();
    for (i, x) in inputs.iter().enumerate() {
        for (j, y) in inputs[i..].iter().enumerate() {
            if x + y >= 2020 {
                continue;
            }
            for z in inputs[j..].iter() {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    return 0;
}
