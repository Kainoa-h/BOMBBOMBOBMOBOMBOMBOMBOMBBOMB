impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn mut_neighours(grid: &mut Vec<Vec<char>>, row_idx: usize, col_idx: usize) {
            let top = (row_idx.saturating_sub(1), col_idx);
            let bot = (row_idx  + 1, col_idx);
            let right = (row_idx, col_idx +1);
            let left = (row_idx, col_idx.saturating_sub(1));
            
            if let Some(x) = grid.get_mut(top.0).and_then(|c| c.get_mut(top.1)) && *x == '1' {
                *x = '0';
                mut_neighours(grid, top.0, top.1);
            }

            if let Some(x) = grid.get_mut(bot.0).and_then(|c| c.get_mut(bot.1)) && *x == '1' {
                *x = '0';
                mut_neighours(grid, bot.0, bot.1);
            }

            if let Some(x) = grid.get_mut(right.0).and_then(|c| c.get_mut(right.1)) && *x == '1' {
                *x = '0';
                mut_neighours(grid, right.0, right.1);
            }

            if let Some(x) = grid.get_mut(left.0).and_then(|c| c.get_mut(left.1)) && *x == '1' {
                *x = '0';
                mut_neighours(grid, left.0, left.1);
            }
        }
        
        let mut count = 0;
        for row_idx in 0..grid.len() {
            for col_idx in 0..grid[0].len(){
                if grid[row_idx][col_idx] == '1' {
                    count += 1;
                    mut_neighours(&mut grid, row_idx, col_idx);
                }
            }
        }
        count
    }
}

struct Solution{}
