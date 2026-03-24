struct Solution {}
impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_count = grid.len();
        let col_count = grid[0].len();
        let mut result_grid: Vec<Vec<i32>> = vec![vec![0; col_count]; row_count];

        let prefix_list: Vec<u64> = grid
            .iter()
            .flatten()
            .scan(1_u64, |acc, cell| {
                let current = *acc;
                *acc = (*acc * *cell as u64) % 12345;
                Some(current)
            })
            .collect();

        let mut postfix_list: Vec<u64> = grid
            .iter()
            .flatten()
            .rev()
            .scan(1_u64, |acc, cell| {
                let current = *acc;
                *acc = (*acc * *cell as u64) % 12345;
                Some(current)
            })
            .collect();
        postfix_list.reverse();
        let postfix_list = postfix_list;

        for (idx, cell) in result_grid.iter_mut().flatten().enumerate() {
            let prod = prefix_list[idx] * postfix_list[idx];
            *cell = (prod % 12345) as i32;
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

