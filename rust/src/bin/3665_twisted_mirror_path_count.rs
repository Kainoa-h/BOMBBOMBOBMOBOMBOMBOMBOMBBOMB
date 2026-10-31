const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![[0, 0]; cols]; rows];
        dp[0][0][0] = 1;
        for r in 0..rows {
            for c in 0..cols {
                if c > 0 {
                    if grid[r][c - 1] == 0 {
                        let sum = dp[r][c - 1].iter().sum::<i32>();
                        dp[r][c][0] = if sum >= MOD { sum - MOD } else { sum };
                    } else {
                        dp[r][c][0] = dp[r][c - 1][1];
                    }
                }
                if r > 0 {
                    if grid[r - 1][c] == 0 {
                        let sum = dp[r - 1][c].iter().sum::<i32>();
                        dp[r][c][1] = if sum >= MOD { sum - MOD } else { sum };
                    } else {
                        dp[r][c][1] = dp[r - 1][c][0];
                    }
                }
            }
        }
        let sum = dp[rows - 1][cols - 1].iter().sum::<i32>();
        if sum >= MOD { sum - MOD } else { sum }
    }
}

struct Solution {}
