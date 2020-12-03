use crate::common::read_lines;

fn parse_line(line: String) -> Vec<bool> {
    line.chars().map(|c| c == '#').collect()
}

fn inputs_to_matrix() -> Vec<Vec<bool>> {
    read_lines("./inputs/day3").unwrap().map(|line| parse_line(line.unwrap())).collect()
}

fn count_trees(matrix: &Vec<Vec<bool>>, i_shift: usize, j_shift: usize) -> usize {
    let j_max = matrix[0].len();
    let mut cnt = 0;
    let mut j = 0;
    for i in (i_shift..=matrix.len()-1).step_by(i_shift) {
        j = (j + j_shift)%j_max;
        if matrix[i][j] {
            cnt += 1;
        }
    }
    cnt
}

pub fn puzzle1() -> String {
    let mat = inputs_to_matrix();
    format!("D3P1: {}", count_trees(&mat, 1, 3))
}

pub fn puzzle2() -> String {
    let mat = inputs_to_matrix();
    let cnt11 = count_trees(&mat, 1, 1);
    let cnt13 = count_trees(&mat, 1, 3);
    let cnt15 = count_trees(&mat, 1, 5);
    let cnt17 = count_trees(&mat, 1, 7);
    let cnt21 = count_trees(&mat, 2, 1);
    format!("D3P2: {}", cnt11 * cnt13 * cnt15 * cnt17 * cnt21)
}
