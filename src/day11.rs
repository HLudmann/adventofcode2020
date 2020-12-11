use crate::common::read_lines;

fn parse_input() -> Vec<Vec<u8>> {
    read_lines("./inputs/day11")
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'L' => 1,
                    _ => 0,
                })
                .collect::<Vec<u8>>()
        })
        .collect()
}

fn seats_round_rules1(layout: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new_layout = vec![vec![0; layout[0].len()]; layout.len()];
    let i_max = layout.len() - 1;
    let j_max = layout[0].len() - 1;
    for i in 0..=i_max {
        for j in 0..=j_max {
            if layout[i][j] == 0 {
                continue;
            }
            let l_min = i.max(1) - 1;
            let l_max = (i + 1).min(i_max);
            let k_min = j.max(1) - 1;
            let k_max = (j + 1).min(j_max);
            let mut adja_seated = 0;
            for l in l_min..=l_max {
                for k in k_min..=k_max {
                    if l == i && k == j {
                        continue;
                    }
                    if layout[l][k] == 2 {
                        adja_seated += 1
                    }
                }
            }
            match (layout[i][j], adja_seated) {
                (1, 0) => new_layout[i][j] = 2,
                (2, n) if n < 4 => new_layout[i][j] = 2,
                _ => new_layout[i][j] = 1,
            }
        }
    }
    new_layout
}

fn seats_round_rules2(layout: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new_layout = vec![vec![0; layout[0].len()]; layout.len()];
    let i_max = layout.len() - 1;
    let j_max = layout[0].len() - 1;
    for i in 0..=i_max {
        for j in 0..=j_max {
            if layout[i][j] == 0 {
                continue;
            }
            let mut adja_seated = 0;
            // UP
            if i != 0 {
                for l in 1..=i {
                    match layout[i - l][j] {
                        2 => {
                            adja_seated += 1;
                            break;
                        }
                        1 => break,
                        _ => continue,
                    }
                }
                // LEFT
                if j != 0 {
                    for m in 1..=i.min(j) {
                        match layout[i - m][j - m] {
                            2 => {
                                adja_seated += 1;
                                break;
                            }
                            1 => break,
                            _ => continue,
                        }
                    }
                }
                // RIGHT
                if j != j_max {
                    for m in 1..=i.min(j_max - j) {
                        match layout[i - m][j + m] {
                            2 => {
                                adja_seated += 1;
                                break;
                            }
                            1 => break,
                            _ => continue,
                        }
                    }
                }
            }
            // DOWN
            if i != i_max {
                for l in i + 1..=i_max {
                    match layout[l][j] {
                        2 => {
                            adja_seated += 1;
                            break;
                        }
                        1 => break,
                        _ => continue,
                    }
                }
                // LEFT
                if j != 0 {
                    for m in 1..=(i_max - i).min(j) {
                        match layout[i + m][j - m] {
                            2 => {
                                adja_seated += 1;
                                break;
                            }
                            1 => break,
                            _ => continue,
                        }
                    }
                }
                // RIGHT
                if j != j_max {
                    for m in 1..=(i_max - i).min(j_max - j) {
                        match layout[i + m][j + m] {
                            2 => {
                                adja_seated += 1;
                                break;
                            }
                            1 => break,
                            _ => continue,
                        }
                    }
                }
            }
            // LEFT
            if j != 0 {
                for k in 1..=j {
                    match layout[i][j - k] {
                        2 => {
                            adja_seated += 1;
                            break;
                        }
                        1 => break,
                        _ => continue,
                    }
                }
            }
            // RIGHT
            if j != j_max {
                for k in j + 1..=j_max {
                    match layout[i][k] {
                        2 => {
                            adja_seated += 1;
                            break;
                        }
                        1 => break,
                        _ => continue,
                    }
                }
            }
            match (layout[i][j], adja_seated) {
                (1, 0) => new_layout[i][j] = 2,
                (2, n) if n < 5 => new_layout[i][j] = 2,
                _ => new_layout[i][j] = 1,
            }
        }
    }
    new_layout
}

fn count_seated(layout: &Vec<Vec<u8>>) -> usize {
    layout
        .iter()
        .map(|l| l.iter().filter(|i| **i == 2).count())
        .sum()
}

#[allow(dead_code)]
fn print_layout(layout: &Vec<Vec<u8>>) {
    layout.iter().for_each(|l| {
        println!(
            "{}",
            l.iter()
                .map(|c| {
                    match c {
                        2 => '#',
                        1 => 'L',
                        _ => '.',
                    }
                })
                .collect::<String>()
        )
    });
    println!("");
}

pub fn puzzle1() -> usize {
    let mut layout = parse_input();
    loop {
        let new_layout = seats_round_rules1(&layout);
        if new_layout == layout {
            break;
        }
        layout = new_layout;
    }
    count_seated(&layout)
}

pub fn puzzle2() -> usize {
    let mut layout = parse_input();
    loop {
        let new_layout = seats_round_rules2(&layout);
        if new_layout == layout {
            break;
        }
        layout = new_layout;
    }
    count_seated(&layout)
}
