use crate::common::read_lines;

fn parse_input_withou_xs() -> (usize, Vec<usize>) {
    let lines = read_lines("./inputs/day13");
    let ts = lines[0].parse::<usize>().unwrap();
    let mut buses: Vec<usize> = lines[1]
        .split(",")
        .map(|b| match b {
            "x" => 0,
            _ => b.parse::<usize>().unwrap(),
        })
        .filter(|b| *b != 0)
        .collect();
    buses.sort();

    (ts, buses)
}

fn parse_input() -> (usize, Vec<usize>) {
    let lines = read_lines("./inputs/day13");
    let ts = lines[0].parse::<usize>().unwrap();
    let buses: Vec<usize> = lines[1]
        .split(",")
        .map(|b| match b {
            "x" => 0,
            _ => b.parse::<usize>().unwrap(),
        })
        .collect();

    (ts, buses)
}

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (x.min(y), x.max(y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b * gcd(a, b)
}

pub fn puzzle1() -> usize {
    let (ts, buses) = parse_input_withou_xs();

    for i in ts..ts + buses.last().unwrap() {
        for b in buses.iter() {
            if i % b == 0 {
                return (i - ts) * b;
            }
        }
    }
    return 0;
}

pub fn puzzle2() -> usize {
    let (_, buses) = parse_input();
    let i_max = buses.len() - 1;
    let mut ts = buses[0];
    let prod = buses.iter().filter(|b| **b != 0).product();

    loop {
        if ts > prod {
            break;
        }
        let mut step = 1;
        for (i, b) in buses.iter().enumerate() {
            if *b == 0 {
                continue;
            }
            if (ts + i) % b != 0 {
                break;
            }
            if i == i_max {
                return ts;
            }
            step = lcm(step, *b);
        }
        ts += step;
    }
    return 0;
}
