const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut dp = vec![[0, 0]; cols];
        dp[0][1] = 1;
        for r in 0..rows {
            for c in 0..cols {
                if r > 0 {
                    if grid[r - 1][c] == 0 {
                        let sum = dp[c].iter().sum::<i32>();
                        dp[c][1] = if sum >= MOD { sum - MOD } else { sum };
                    } else {
                        dp[c][1] = dp[c][0];
                    }
                }
                if c > 0 {
                    if grid[r][c - 1] == 0 {
                        let sum = dp[c - 1].iter().sum::<i32>();
                        dp[c][0] = if sum >= MOD { sum - MOD } else { sum };
                    } else {
                        dp[c][0] = dp[c - 1][1];
                    }
                }
            }
        }
        let sum = dp[cols - 1].iter().sum::<i32>();
        if sum >= MOD { sum - MOD } else { sum }
    }
}

struct Solution {}
