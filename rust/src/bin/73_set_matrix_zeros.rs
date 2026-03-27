struct Solution {}
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let zero_row = !matrix[0].iter().all(|&x| x != 0);
        let zero_col = !matrix.iter().all(|row| row[0] != 0);

        for row_idx in 1..rows {
            for col_idx in 1..cols {
                if matrix[row_idx][col_idx] == 0 {
                    matrix[0][col_idx] = 0;
                    matrix[row_idx][0] = 0;
                }
            }
        }
        for row in matrix.iter_mut().skip(1) {
            if row[0] == 0 {
                row.iter_mut().for_each(|c| *c = 0);
            }
        }

        for col_idx in 1..cols {
            if matrix[0][col_idx] == 0 {
                matrix.iter_mut().for_each(|row| row[col_idx] = 0);
            }
        }
        if zero_row {
            matrix[0].iter_mut().for_each(|c| *c = 0);
        }
        if zero_col {
            matrix.iter_mut().for_each(|row| row[0] = 0);
        }
    }
}

fn what(ans: Vec<Vec<i32>>, mut matrix: Vec<Vec<i32>>) {
    println!("before: {:?}\n", matrix);
    Solution::set_zeroes(&mut matrix);
    println!("after: {:?}\n", matrix);
    assert_eq!(ans, matrix);
}

fn main() {
    what(
        vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
        vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]],
    );
}
