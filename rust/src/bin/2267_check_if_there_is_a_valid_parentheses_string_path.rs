impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let (rows, cols) = (grid.len(), grid[0].len());
        if (rows + cols - 1) % 2 != 0 || grid[0][0] == ')' || grid[rows - 1][cols - 1] == '(' {
            return false;
        }
        let mut dp = vec![0_u128; cols + 1];
        for (r, row) in grid.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                let new_mask = if r == 0 && c == 0 {
                    1
                } else {
                    dp[c + 1] | dp[c]
                };
                dp[c + 1] = if ch == '(' {
                    new_mask << 1
                } else {
                    new_mask >> 1
                }
            }
        }

        dp[cols] & 1 == 1
    }
}

struct Solution {}
