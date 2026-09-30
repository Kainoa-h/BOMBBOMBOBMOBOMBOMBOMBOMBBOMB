use std::collections::HashSet;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut dp = vec![HashSet::new(); cols + 1];
        dp[0].insert(0);

        for (r, row) in grid.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                let opr = if ch == '(' { 1 } else { -1 };
                let dist = ((rows - r) + (cols - c)) as isize;
                let mut new_set = HashSet::new();
                //above
                for &n in &dp[c + 1] {
                    let x = n + opr;
                    if x <= dist {
                        new_set.insert(x);
                    }
                }
                //left
                for &n in &dp[c] {
                    let x = n + opr;
                    if x <= dist {
                        new_set.insert(x);
                    }
                }
                dp[c + 1] = new_set;
            }
            dp[0].clear();
        }

        dp[cols].contains(&0)
    }
}

struct Solution {}
