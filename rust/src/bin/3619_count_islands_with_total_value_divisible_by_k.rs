impl Solution {
    pub fn count_islands(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut stack = Vec::new();
        let mut islands = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != 0 {
                    let sum = Self::flood_and_sum(&mut grid, &mut stack, r, c);
                    islands += i32::from(sum % k as i64 == 0);
                }
            }
        }
        islands
    }

    fn flood_and_sum(
        grid: &mut [Vec<i32>],
        stack: &mut Vec<(usize, usize)>,
        row: usize,
        col: usize,
    ) -> i64 {
        const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];
        let mut sum = grid[row][col] as i64;
        grid[row][col] = 0;
        stack.clear();
        stack.push((row, col));
        while let Some((r, c)) = stack.pop() {
            for (dr, dc) in DIRS {
                let (nr, nc) = (r.wrapping_add_signed(dr), c.wrapping_add_signed(dc));
                if let Some(cell) = grid.get_mut(nr).and_then(|row| row.get_mut(nc))
                    && *cell != 0
                {
                    sum += *cell as i64;
                    *cell = 0;
                    stack.push((nr, nc));
                }
            }
        }
        sum
    }
}

struct Solution {}
