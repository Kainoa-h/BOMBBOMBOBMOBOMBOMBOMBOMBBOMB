struct Solution {}
impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let col_count = grid[0].len();

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

        let flat_result: Vec<i32> = prefix_list
            .iter()
            .zip(postfix_list.iter())
            .map(|(&pre, &post)| ((pre * post) % 12345) as i32)
            .collect();

        flat_result
            .chunks(col_count)
            .map(|chunk| chunk.to_vec())
            .collect()
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

