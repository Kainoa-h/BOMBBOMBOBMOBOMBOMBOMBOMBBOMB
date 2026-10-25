impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut turn_into_water = |row: usize, col: usize, grid: &mut Vec<Vec<i32>>| {
            let mut stack = vec![(row, col)];
            while let Some(cell) = stack.pop() {
                let (r, c) = (cell.0, cell.1);
                grid[r][c] = 1;
                if r != 0 && grid[r - 1][c] == 0 {
                    stack.push((r - 1, c));
                }
                if c != 0 && grid[r][c - 1] == 0 {
                    stack.push((r, c - 1));
                }
                if r != rows - 1 && grid[r + 1][c] == 0 {
                    stack.push((r + 1, c));
                }
                if c != cols - 1 && grid[r][c + 1] == 0 {
                    stack.push((r, c + 1));
                }
            }
        };

        for c in 0..cols {
            if grid[0][c] == 0 {
                turn_into_water(0, c, &mut grid);
            }
            if grid[rows - 1][c] == 0 {
                turn_into_water(rows - 1, c, &mut grid);
            }
        }
        for r in 0..rows {
            if grid[r][0] == 0 {
                turn_into_water(r, 0, &mut grid);
            }
            if grid[r][cols - 1] == 0 {
                turn_into_water(r, cols - 1, &mut grid);
            }
        }

        let mut islands = 0;
        for r in 1..rows - 1 {
            for c in 1..cols - 1 {
                let cell = grid[r][c];
                if cell == 0 {
                    islands += 1;
                    turn_into_water(r, c, &mut grid);
                }
            }
        }

        islands
    }
}

struct Solution {}
