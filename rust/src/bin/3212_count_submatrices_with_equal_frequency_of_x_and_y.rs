struct Solution {}

#[derive(Clone, Copy, Default)]
struct Node {
    sum: i32,
    has_x: bool,
}

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut prefix_grid = vec![vec![Node::default(); cols + 1]; rows + 1];
        for r_idx in 0..rows {
            for c_idx in 0..cols {
                let char = grid[r_idx][c_idx];
                let val = match char {
                    'X' => 1,
                    'Y' => -1,
                    _ => 0,
                };

                let sum = prefix_grid[r_idx][c_idx + 1].sum + prefix_grid[r_idx + 1][c_idx].sum
                    - prefix_grid[r_idx][c_idx].sum
                    + val;
                let has_x = prefix_grid[r_idx][c_idx + 1].has_x
                    || prefix_grid[r_idx + 1][c_idx].has_x
                    || val == 1;
                prefix_grid[r_idx + 1][c_idx + 1] = Node { sum, has_x };

                if sum == 0 && has_x {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    assert_eq!(
        3,
        Solution::number_of_submatrices(vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']])
    )
}
