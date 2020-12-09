use crate::common::read_lines;

fn parse_input() -> Vec<usize> {
    read_lines("./inputs/day9")
        .iter()
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

fn test_number_preamble(preamble: Vec<usize>, nbr: usize) -> bool {
    for (i, a) in preamble.iter().enumerate() {
        for b in preamble[i..].iter() {
            if a + b == nbr {
                return true;
            }
        }
    }
    return false;
}

fn test_target(suite: Vec<usize>, target: usize) -> (bool, usize) {
    let mut acc: usize = 0;
    for (i, nbr) in suite.iter().enumerate() {
        if acc == target {
            return (true, i);
        }
        if acc > target {
            break;
        }
        acc += nbr;
    }
    return (false, 0);
}

fn find_mismatch(numbers: &Vec<usize>) -> usize {
    for (i, nbr) in numbers[25..].iter().enumerate() {
        if !test_number_preamble(numbers[i..i + 25].to_vec(), *nbr) {
            return *nbr;
        }
    }
    return 0;
}

pub fn puzzle1() -> usize {
    let numbers = parse_input();
    find_mismatch(&numbers)
}

pub fn puzzle2() -> usize {
    let numbers = parse_input();
    let target = find_mismatch(&numbers);

    for i in 0..numbers.len() {
        let (hit, len) = test_target(numbers[i..].to_vec(), target);
        if hit {
            let min = numbers[i..i + len].iter().min().unwrap();
            let max = numbers[i..i + len].iter().max().unwrap();
            return min + max;
        }
    }

    return 0;
}
