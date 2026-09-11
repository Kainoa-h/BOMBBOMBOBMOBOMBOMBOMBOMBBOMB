impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            return 0;
        }

        fn move_down(grid: &[Vec<i32>], r: usize, c: usize) -> i32 {
            if r == grid.len() - 1 && c == grid[0].len() - 1 {
                return 1;
            }

            let mut count = 0;
            if let Some(&next) = grid.get(r + 1).and_then(|x| x.get(c))
                && next == 0
            {
                count += move_down(grid, r + 1, c);
            }
            if let Some(&next) = grid.get(r).and_then(|x| x.get(c + 1))
                && next == 0
            {
                count += move_down(grid, r, c + 1);
            }

            count
        }
        move_down(&obstacle_grid, 0, 0)
    }
}

struct Solution {}
