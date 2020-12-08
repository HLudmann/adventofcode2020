use crate::common::read_lines;

fn line_to_id(line: &String) -> usize {
    let chars = line.chars().collect::<Vec<char>>();
    let mut r_min = 0;
    let mut r_max = 127;
    let mut c_min = 0;
    let mut c_max = 7;

    for c in chars {
        match c {
            'F' => r_max = (r_max + r_min - 1) / 2,
            'B' => r_min = (r_max + r_min + 1) / 2,
            'L' => c_max = (c_max + c_min - 1) / 2,
            'R' => c_min = (c_max + c_min + 1) / 2,
            _ => panic!(),
        }
    }
    r_min * 8 + c_min
}

fn parse_input() -> Vec<usize> {
    read_lines("./inputs/day5")
        .iter()
        .map(|l| line_to_id(l))
        .collect()
}

pub fn puzzle1() -> usize {
    *parse_input().iter().max().unwrap()
}

pub fn puzzle2() -> usize {
    let mut seats = (0..=127 * 8 + 7)
        .map(|i| (i, true))
        .collect::<Vec<(usize, bool)>>();
    for id in parse_input().iter() {
        seats[*id] = (*id, false);
    }
    let possibilities = seats
        .iter()
        .filter(|s| s.1)
        .map(|s| s.0)
        .collect::<Vec<usize>>();
    for i in 1..possibilities.len() {
        if possibilities[i - 1] != possibilities[i] - 1 {
            return possibilities[i];
        }
    }
    return 0;
}
