use crate::common::read_lines;

fn parse_input() -> Vec<(usize, isize)> {
    let lines = read_lines("./inputs/day8");
    let mut instructions: Vec<(usize, isize)> = Vec::new();
    for line in lines.iter() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let arg = split[1].parse::<isize>().unwrap();
        let ope: usize = match split[0] {
            "acc" => 1,
            "jmp" => 2,
            _ => 0,
        };

        instructions.push((ope, arg))
    }

    instructions
}

fn follow_until_loop_or_end(instructions: Vec<(usize, isize)>) -> (bool, isize) {
    let len = instructions.len();
    let mut acc: isize = 0;
    let mut order: Vec<usize> = Vec::new();
    let mut i: usize = 0;

    loop {
        if i >= len {
            return (false, acc);
        }
        if order.contains(&i) {
            return (true, acc);
        }
        order.push(i);
        match instructions[i] {
            (1, val) => {
                acc += val;
                i += 1
            }
            (2, val) if val > 0 => i += val.abs() as usize,
            (2, val) => i -= val.abs() as usize,
            _ => i += 1,
        }
    }
}

fn flip_occ_n(instructions: &Vec<(usize, isize)>, n: usize) -> Vec<(usize, isize)> {
    let mut p = 0;
    let mut flipped: Vec<(usize, isize)> = Vec::new();
    for i in 0..instructions.len() {
        if instructions[i].0 != 1 {
            p += 1
        }
        if p == n {
            match instructions[i] {
                (0, val) => flipped.push((2, val)),
                (2, val) => flipped.push((0, val)),
                _ => (),
            }
            instructions[i + 1..].iter().for_each(|t| flipped.push(*t));
            return flipped;
        }
        flipped.push(instructions[i]);
    }

    flipped
}

pub fn puzzle1() -> isize {
    follow_until_loop_or_end(parse_input()).1
}

pub fn puzzle2() -> isize {
    let instructions = parse_input();
    let n_max = instructions.iter().filter(|t| t.0 != 1).count();

    for n in 1..=n_max {
        let mod_inst = flip_occ_n(&instructions, n);
        let (looped, acc) = follow_until_loop_or_end(mod_inst);
        if !looped {
            return acc;
        }
    }
    return 0;
}
