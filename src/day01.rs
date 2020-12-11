use crate::common::read_lines;

fn parse_input() -> Vec<usize> {
    read_lines("./inputs/day01")
        .iter()
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

pub fn puzzle1() -> usize {
    let inputs = parse_input();
    for (i, x) in inputs.iter().enumerate() {
        let y = 2020 - x;
        if y != *x && inputs[i..].contains(&y) {
            return x * y;
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
            let z = 2020 - x - y;
            if z != *x && z != *y && inputs[j..].contains(&z) {
                return x * y * z;
            }
        }
    }
    return 0;
}
