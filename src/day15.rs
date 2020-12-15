use std::collections::HashMap;

fn compute(end: usize) -> usize {
    let input = vec![13, 16, 0, 12, 15, 1];
    let mut mem = HashMap::new();
    input.iter().enumerate().for_each(|(i, n)| {
        mem.insert(*n, (i, 0));
    });
    let mut last = *input.last().unwrap();
    for i in input.len()..end {
        last = mem.get(&last).unwrap().1;
        let entry = mem.entry(last).or_insert((i, 0));
        *entry = (i, i - entry.0);
    }

    return last;
}

pub fn puzzle1() -> usize {
    compute(2020)
}

pub fn puzzle2() -> usize {
    compute(30000000)
}
