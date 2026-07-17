impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn mut_neighours(grid: &mut Vec<Vec<char>>, row_idx: usize, col_idx: usize) {
            if let Some(x) = grid.get_mut(row_idx).and_then(|c| c.get_mut(col_idx))
                && *x == '1'
            {
                *x = '0';
                mut_neighours(grid, row_idx.saturating_sub(1), col_idx);
                mut_neighours(grid, row_idx + 1, col_idx);
                mut_neighours(grid, row_idx, col_idx.saturating_sub(1));
                mut_neighours(grid, row_idx, col_idx + 1);
            }
        }

        let mut count = 0;
        for row_idx in 0..grid.len() {
            for col_idx in 0..grid[0].len() {
                if grid[row_idx][col_idx] == '1' {
                    count += 1;
                    mut_neighours(&mut grid, row_idx, col_idx);
                }
            }
        }
        count
    }
}

struct Solution {}
