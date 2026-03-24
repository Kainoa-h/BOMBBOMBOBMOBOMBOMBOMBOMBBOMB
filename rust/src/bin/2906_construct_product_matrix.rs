struct Solution {}
impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_count = grid.len();
        let col_count = grid[0].len();
        let mut prefix_list: Vec<u64> = Vec::new();
        let mut postfix_list: Vec<u64> = Vec::new();
        let mut result_grid: Vec<Vec<i32>> = vec![vec![0; col_count]; row_count];
        let mut accumulator = 1_u64;
        for row in grid.iter() {
            for cell in row.iter() {
                prefix_list.push(accumulator);
                accumulator *= *cell as u64;
                accumulator %= 12345;
            }
        }
        accumulator = 1;
        for row in grid.iter().rev() {
            for cell in row.iter().rev() {
                postfix_list.push(accumulator);
                accumulator *= *cell as u64;
                accumulator %= 12345;
            }
        }
        postfix_list.reverse();

        for (i, row) in result_grid.iter_mut().enumerate() {
            let x = i * col_count;
            for (j, cell) in row.iter_mut().enumerate() {
                let prod = prefix_list[x + j] * postfix_list[x + j];
                *cell = (prod % 12345) as i32;
            }
        }

        result_grid
    }
}

fn main() {
    assert_eq!(
        vec![vec![24, 12], vec![8, 6]],
        Solution::construct_product_matrix(vec![vec![1, 2], vec![3, 4]])
    );
    assert_eq!(
        vec![vec![2], vec![0], vec![0]],
        Solution::construct_product_matrix(vec![vec![12345], vec![2], vec![1]])
    );
}

