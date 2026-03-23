struct Solution {}
impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let rows = matrix.len();
        let cols = matrix[0].len();
        let (mut row_idx, mut col_idx) = (0_usize, 0_usize);
        let (mut vel_r, mut vel_c) = (0_isize, 1_isize);
        result.push(matrix[0][0]);
        matrix[0][0] = 101;
        for _ in 1..(rows * cols) {
            let (maybe_row_idx, maybe_col_idx) =
                (row_idx as isize + vel_r, col_idx as isize + vel_c);
            if 0 > maybe_row_idx
                || (rows as isize) <= maybe_row_idx
                || 0 > maybe_col_idx
                || (cols as isize) <= maybe_col_idx
                || matrix[maybe_row_idx as usize][maybe_col_idx as usize] == 101
            {
                (vel_r, vel_c) = (vel_c, -vel_r);
            }
            (row_idx, col_idx) = (
                row_idx.checked_add_signed(vel_r).unwrap(),
                col_idx.checked_add_signed(vel_c).unwrap(),
            );
            result.push(matrix[row_idx][col_idx]);
            matrix[row_idx][col_idx] = 101;
        }

        result
    }
}

fn main() {
    println!("3x3");
    assert_eq!(
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
        Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
    println!("3x4");
    assert_eq!(
        vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
        Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ])
    );
    println!("1x0");
    assert_eq!(
        vec![7, 9, 6],
        Solution::spiral_order(vec![vec![7], vec![9], vec![6]])
    );
}
