impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let cols = grid[0].len();
        let mut flattened = grid.into_iter().flatten().collect::<Vec<i32>>();
        let rot = k as usize % flattened.len();
        flattened.rotate_right(rot);
        flattened.chunks(cols).map(|c| c.to_vec()).collect()
    }
}

struct Solution {}
