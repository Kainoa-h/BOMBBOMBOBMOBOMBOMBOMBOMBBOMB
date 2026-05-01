struct Solution {}
impl Solution {
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let k = k as usize;
        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![-1; k + 1]; cols + 1]; rows + 1];

        dp[0][1][0] = 0;
        for row_idx in 1..dp.len() {
            for col_idx in 1..dp[0].len() {
                let score = grid[row_idx - 1][col_idx - 1];
                let cost;
                let till;
                if score != 0 {
                    cost = 1;
                    till = k;
                } else {
                    cost = 0;
                    till = k + 1;
                }
                for i in 0..till {
                    let l = dp[row_idx][col_idx - 1][i];
                    let u = dp[row_idx - 1][col_idx][i];
                    if l != -1 || u != -1 {
                        dp[row_idx][col_idx][i + cost] = l.max(u) + score;
                    }
                }
            }
        }

        dp[grid.len()][grid[0].len()].iter().max().copied().unwrap()
    }
}

fn main() {
    assert_eq!(Solution::max_path_score(vec![vec![0, 1], vec![2, 0]], 1), 2);
    assert_eq!(
        Solution::max_path_score(vec![vec![0, 1], vec![1, 2]], 1),
        -1
    );
}
