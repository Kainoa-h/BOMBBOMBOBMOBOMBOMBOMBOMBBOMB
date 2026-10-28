use std::collections::HashSet;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut dp = grid[0]
            .iter()
            .scan(0, |acc, &x| {
                *acc ^= x;
                Some(HashSet::from([*acc as i16]))
            })
            .collect::<Vec<_>>();
        for row in grid.iter().skip(1) {
            for (idx, &cell) in row.iter().enumerate() {
                let cell = cell as i16;
                let mut set = HashSet::new();
                if idx != 0 {
                    for &n in &dp[idx - 1] {
                        set.insert(n ^ cell);
                    }
                }
                for &n in &dp[idx] {
                    set.insert(n ^ cell);
                }
                dp[idx] = set;
            }
        }

        *dp[cols - 1].iter().min().unwrap() as i32
    }
}

struct Solution {}
