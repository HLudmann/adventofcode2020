use crate::common::read_lines;
use std::collections::HashMap;

fn parse_input() -> Vec<usize> {
    let mut parsed = read_lines("./inputs/day10")
        .iter()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    parsed.sort();
    parsed.insert(0, 0);
    parsed.push(parsed.last().unwrap() + 3);
    parsed
}

fn count_diff(input: &Vec<usize>) -> (usize, usize, usize) {
    let mut diff1 = 0;
    let mut diff2 = 0;
    let mut diff3 = 0;
    for (a, b) in input[1..].iter().zip(input[..input.len()].iter()) {
        match a - b {
            1 => diff1 += 1,
            2 => diff2 += 1,
            3 => diff3 += 1,
            _ => panic!(),
        }
    }
    (diff1, diff2, diff3)
}

fn count_paths(input: &Vec<usize>) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();
    let len = input.len();
    for (i, n) in input[..len - 2].iter().enumerate() {
        let val = *map.entry(*n).or_insert(1);
        for m in input[i + 1..].iter() {
            if m - n > 3 {
                break;
            }
            let m_val = map.entry(*m).or_insert(0);
            *m_val += val;
        }
        map.remove(n);
    }
    *map.get(&input[len - 2]).unwrap()
}

pub fn puzzle1() -> usize {
    let input = parse_input();
    let (diff1, _, diff3) = count_diff(&input);
    return diff1 * diff3;
}

pub fn puzzle2() -> usize {
    let input = parse_input();
    return count_paths(&input);
}
