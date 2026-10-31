const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        let cols = grid[0].len();
        let mut dp_down = vec![0; cols];
        dp_down[0] = 1;
        for row in &grid {
            let mut from_left = 0;
            for (&cell, above) in row.iter().zip(dp_down.iter_mut()) {
                if cell == 0 {
                    let sum = *above + from_left;
                    let paths = if sum >= MOD { sum - MOD } else { sum };

                    *above = paths;
                    from_left = paths;
                } else {
                    std::mem::swap(above, &mut from_left);
                }
            }
        }
        dp_down[cols - 1]
    }
}

struct Solution {}
