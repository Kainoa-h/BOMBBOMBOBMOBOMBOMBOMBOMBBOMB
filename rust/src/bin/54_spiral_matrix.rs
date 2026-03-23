struct Solution {}
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut cells = rows * cols;
        let mut result: Vec<i32> = Vec::new();
        let mut row_idx = 0_usize;
        let mut col_idx = 0_usize;
        let (mut buffer_right, mut buffer_bot, mut buffer_left, mut buffer_top) =
            (0_usize, 0_usize, 0_usize, 1_usize);
        while cells > 0 {
            // right
            for i in (col_idx)..(cols - buffer_right) {
                result.push(matrix[row_idx][i]);
                cells -= 1;
            }
            if cells == 0 {
                break;
            }
            col_idx = cols - buffer_right - 1;
            buffer_right += 1;

            // down
            for i in (row_idx + 1)..(rows - buffer_bot) {
                result.push(matrix[i][col_idx]);
                cells -= 1;
            }
            if cells == 0 {
                break;
            }
            row_idx = rows - buffer_bot - 1;
            buffer_bot += 1;

            // left
            for i in (buffer_left..col_idx).rev() {
                result.push(matrix[row_idx][i]);
                cells -= 1;
            }
            if cells == 0 {
                break;
            }
            col_idx = buffer_left;
            buffer_left += 1;

            // up
            for i in (buffer_top..row_idx).rev() {
                result.push(matrix[i][col_idx]);
                cells -= 1;
            }
            if cells == 0 {
                break;
            }
            row_idx = buffer_top;
            buffer_top += 1;
            col_idx += 1;
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
