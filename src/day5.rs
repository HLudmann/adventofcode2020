use crate::common::read_lines;

fn parse_input() -> Vec<usize> {
    read_lines("./inputs/day5")
        .iter()
        .map(|l| {
            let bin = l
                .replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1");
            usize::from_str_radix(bin.as_str(), 2).unwrap()
        })
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
