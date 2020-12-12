use lazy_static::lazy_static;
use regex::Regex;

use crate::common::read_lines;

fn parse_line() -> Vec<(char, usize)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<dir>[NSEWLRF])(?P<val>[0-9]+)$").unwrap();
    }
    read_lines("./inputs/day12")
        .iter()
        .map(|l| {
            let capt = RE.captures(l).unwrap();
            let dir = capt["dir"].chars().nth(0).unwrap();
            let val = capt["val"].parse::<usize>().unwrap();
            (dir, val)
        })
        .collect()
}

pub fn puzzle1() -> isize {
    let movements = parse_line();
    let mut pos: (isize, isize) = (0, 0);
    let mut dir: (isize, isize) = (1, 0);

    // println!("pos: {:?}\tdir: {:?}\tmv: none", pos, dir);
    for mv in movements.iter() {
        match mv {
            ('N', val) => pos.1 += *val as isize,
            ('S', val) => pos.1 -= *val as isize,
            ('E', val) => pos.0 += *val as isize,
            ('W', val) => pos.0 -= *val as isize,
            ('F', val) => {
                pos.0 += dir.0 * *val as isize;
                pos.1 += dir.1 * *val as isize;
            }
            (side, val) => match (side, (val / 90) % 4) {
                (_, 2) => dir = (-dir.0, -dir.1),
                ('L', 1) => dir = (-dir.1, dir.0),
                ('R', 3) => dir = (-dir.1, dir.0),
                ('L', 3) => dir = (dir.1, -dir.0),
                ('R', 1) => dir = (dir.1, -dir.0),
                _ => (),
            },
        }
        // println!("pos: {:?}\tdir: {:?}\tmv: {:?}", pos, dir, mv);
    }
    return pos.0.abs() + pos.1.abs();
}

pub fn puzzle2() -> isize {
    let movements = parse_line();
    let mut pos: (isize, isize) = (0, 0);
    let mut dir: (isize, isize) = (10, 1);

    // println!("pos: {:?}\tdir: {:?}\tmv: none", pos, dir);
    for mv in movements.iter() {
        match mv {
            ('N', val) => dir.1 += *val as isize,
            ('S', val) => dir.1 -= *val as isize,
            ('E', val) => dir.0 += *val as isize,
            ('W', val) => dir.0 -= *val as isize,
            ('F', val) => {
                pos.0 += dir.0 * *val as isize;
                pos.1 += dir.1 * *val as isize;
            }
            (side, val) => match (side, (val / 90) % 4) {
                (_, 2) => dir = (-dir.0, -dir.1),
                ('L', 1) => dir = (-dir.1, dir.0),
                ('R', 3) => dir = (-dir.1, dir.0),
                ('L', 3) => dir = (dir.1, -dir.0),
                ('R', 1) => dir = (dir.1, -dir.0),
                _ => (),
            },
        }
        // println!("pos: {:?}\tdir: {:?}\tmv: {:?}", pos, dir, mv);
    }
    return pos.0.abs() + pos.1.abs();
}
