use crate::common::read_lines;

fn parse_input() -> Vec<Vec<Vec<bool>>> {
    let lines = read_lines("./inputs/day6");
    let mut groups: Vec<Vec<Vec<bool>>> = Vec::new();
    let mut group: Vec<Vec<bool>> = Vec::new();
    for l in lines.iter() {
        if l.len() == 0 {
            groups.push(group);
            group = Vec::new();
            continue;
        }
        let mut member = vec![false; 26];
        for c in l.chars() {
            member[c as usize - 'a' as usize] = true;
        }
        group.push(member);
    }
    groups.push(group);
    return groups;
}

fn count_one_true(group: &Vec<Vec<bool>>) -> usize {
    let mut cnt = 0;
    for i in 0..26 {
        for mem in group.iter() {
            if mem[i] {
                cnt += 1;
                break;
            }
        }
    }
    cnt
}

fn count_all_true(group: &Vec<Vec<bool>>) -> usize {
    let mut cnt = 26;
    for i in 0..26 {
        for mem in group.iter() {
            if !mem[i] {
                cnt -= 1;
                break;
            }
        }
    }
    cnt
}

pub fn puzzle1() -> String {
    format!(
        "D6P1: {:?}",
        parse_input()
            .iter()
            .map(|g| count_one_true(g))
            .sum::<usize>()
    )
}

pub fn puzzle2() -> String {
    format!(
        "D6P2: {:?}",
        parse_input()
            .iter()
            .map(|g| count_all_true(g))
            .sum::<usize>()
    )
}
