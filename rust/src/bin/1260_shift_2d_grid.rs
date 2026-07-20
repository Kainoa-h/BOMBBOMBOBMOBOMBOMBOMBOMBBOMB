impl Solution {
    pub fn shift_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let n = rows * cols;
        let k = k as usize % n;
        if k == 0 {
            return grid;
        }
        let mut rot = |mut start_inc: usize, mut end_inc: usize| {
            while start_inc < end_inc {
                let (l_r, l_c) = (start_inc / cols, start_inc % cols);
                let (r_r, r_c) = (end_inc / cols, end_inc % cols);
                (grid[l_r][l_c], grid[r_r][r_c]) = (grid[r_r][r_c], grid[l_r][l_c]);
                start_inc += 1;
                end_inc -= 1;
            }
        };
        rot(0, n-1);
        rot(0, k-1);
        rot(k, n-1);

        grid
    }
}

struct Solution {}
