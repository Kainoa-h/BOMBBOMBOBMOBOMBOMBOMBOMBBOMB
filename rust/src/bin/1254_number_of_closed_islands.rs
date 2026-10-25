impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        for r in 0..rows {
            Self::turn_into_water(r, 0, &mut grid);
            Self::turn_into_water(r, cols - 1, &mut grid);
        }
        for c in 1..cols - 1 {
            Self::turn_into_water(0, c, &mut grid);
            Self::turn_into_water(rows - 1, c, &mut grid);
        }

        let mut islands = 0;
        for r in 1..rows - 1 {
            for c in 1..cols - 1 {
                if grid[r][c] == 0 {
                    islands += 1;
                    Self::turn_into_water(r, c, &mut grid);
                }
            }
        }

        islands
    }

    fn turn_into_water(row: usize, col: usize, grid: &mut [Vec<i32>]) {
        const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        if grid[row][col] != 0 {
            return;
        }
        grid[row][col] = 1;
        let mut stack = vec![(row, col)];
        while let Some((r, c)) = stack.pop() {
            for (dr, dc) in DIRS {
                let (nr, nc) = (r.wrapping_add_signed(dr), c.wrapping_add_signed(dc));
                if let Some(cell) = grid.get_mut(nr).and_then(|x| x.get_mut(nc))
                    && *cell == 0
                {
                    *cell = 1;
                    stack.push((nr, nc));
                }
            }
        }
    }
}

struct Solution {}
