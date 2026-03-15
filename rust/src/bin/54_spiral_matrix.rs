struct Solution {}
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let right_bound = matrix[0].len();
        let down_bound = matrix.len();

        let mut vertical_bounds_offset = 0;
        let mut horizontal_bounds_offset = 0;
        let mut result: Vec<i32> = Vec::new();
        let mut row_idx = 0_usize;
        let mut col_idx = 0_usize;
        result.push(matrix[0][0]);
        loop {
            // add to end of right
            for c_i in (col_idx + 1)..(right_bound - horizontal_bounds_offset) {
                result.push(matrix[row_idx][c_i]);
            }
            col_idx = right_bound - horizontal_bounds_offset - 1;

            // add to end of down
            let mut went_down = false;
            for r_i in (row_idx + 1)..(down_bound - vertical_bounds_offset) {
                went_down = true;
                result.push(matrix[r_i][col_idx]);
            }
            row_idx = down_bound - vertical_bounds_offset - 1;
            vertical_bounds_offset += 1;

            if !went_down {
                break;
            }

            // add to end of left
            let mut went_left = false;
            for c_i in ((horizontal_bounds_offset)..col_idx).rev() {
                went_left = true;
                result.push(matrix[row_idx][c_i]);
            }
            col_idx = horizontal_bounds_offset;
            horizontal_bounds_offset += 1;

            // add to end of up
            if went_left {
                for r_i in ((vertical_bounds_offset)..row_idx).rev() {
                    result.push(matrix[r_i][col_idx]);
                }
                row_idx = vertical_bounds_offset;
            }
        }
        result
    }
}

fn main() {
    assert_eq!(
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
        Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
    assert_eq!(
        vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
        Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ])
    );
    assert_eq!(
        vec![7, 9, 6],
        Solution::spiral_order(vec![vec![7], vec![9], vec![6]])
    );
}
