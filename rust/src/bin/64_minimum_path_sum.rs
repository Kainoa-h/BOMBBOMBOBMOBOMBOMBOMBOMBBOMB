impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        for i in 1..rows {
            grid[i][0] += grid[i - 1][0];
        }

        for i in 1..cols {
            grid[0][i] += grid[0][i - 1];
        }

        for r_idx in 1..rows {
            for c_idx in 1..cols {
                grid[r_idx][c_idx] += grid[r_idx - 1][c_idx].min(grid[r_idx][c_idx - 1]);
            }
        }

        grid[rows - 1][cols - 1]
    }
}

struct Solution {}
