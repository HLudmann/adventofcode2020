use crate::common::read_lines;

fn parse_input() -> Vec<Vec<bool>> {
    read_lines("./inputs/day03")
        .iter()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect()
}

fn count_trees(matrix: &Vec<Vec<bool>>, i_shift: usize, j_shift: usize) -> usize {
    let j_max = matrix[0].len();
    let mut cnt = 0;
    let mut j = 0;
    for i in (i_shift..matrix.len()).step_by(i_shift) {
        j = (j + j_shift) % j_max;
        if matrix[i][j] {
            cnt += 1;
        }
    }
    cnt
}

pub fn puzzle1() -> usize {
    let mat = parse_input();
    count_trees(&mat, 1, 3)
}

pub fn puzzle2() -> usize {
    let mat = parse_input();
    count_trees(&mat, 1, 1)
        * count_trees(&mat, 1, 3)
        * count_trees(&mat, 1, 5)
        * count_trees(&mat, 1, 7)
        * count_trees(&mat, 2, 1)
}
