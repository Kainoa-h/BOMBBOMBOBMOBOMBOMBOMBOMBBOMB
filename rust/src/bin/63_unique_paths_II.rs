impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let rows = obstacle_grid.len();
        let cols = obstacle_grid[0].len();
        if obstacle_grid[rows - 1][cols - 1] == 1 {
            return 0;
        }

        let mut dp = vec![vec![0; cols]; rows];
        dp[0][0] = 1;

        for r in 0..rows {
            for c in 0..cols {
                if r != 0 && obstacle_grid[r - 1][c] != 1 {
                    dp[r][c] += dp[r - 1][c];
                }
                if c != 0 && obstacle_grid[r][c - 1] != 1 {
                    dp[r][c] += dp[r][c - 1];
                }
            }
        }

        dp[rows - 1][cols - 1]
    }
}

struct Solution {}
