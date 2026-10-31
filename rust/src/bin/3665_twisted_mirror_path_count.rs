const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut jump_map = vec![vec![Vec::with_capacity(2); cols]; rows];
        for r in 0..rows {
            for c in 0..cols {
                let cell = grid[r][c];
                if cell == 1 {
                    continue;
                }
                if let Some(next) = Self::find_next_cell(&grid, r, c + 1, (0, 1)) {
                    jump_map[r][c].push(next);
                }
                if let Some(next) = Self::find_next_cell(&grid, r + 1, c, (1, 0)) {
                    jump_map[r][c].push(next);
                }
            }
        }

        let mut dp = vec![vec![0_i32; cols]; rows];
        dp[0][0] = 1;
        for r in 0..rows {
            for c in 0..cols {
                for &(jr, jc) in &jump_map[r][c] {
                    dp[jr][jc] = (dp[jr][jc] + dp[r][c]) % MOD;
                }
            }
        }
        dp[rows - 1][cols - 1]
    }

    fn find_next_cell(
        grid: &[Vec<i32>],
        r: usize,
        c: usize,
        d: (usize, usize),
    ) -> Option<(usize, usize)> {
        let cell = *grid.get(r).and_then(|x| x.get(c))?;
        if cell == 0 {
            return Some((r, c));
        }

        Self::find_next_cell(grid, r + d.1, c + d.0, (d.1, d.0))
    }
}

struct Solution {}
