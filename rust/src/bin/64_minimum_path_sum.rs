impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let cols = grid[0].len();
        grid.iter().skip(1).fold(
            grid[0]
                .iter()
                .scan(0, |acc, &x| {
                    *acc += x;
                    Some(*acc)
                })
                .collect::<Vec<i32>>(),
            |mut acc, row| {
                acc[0] += row[0];
                for i in 1..cols {
                    acc[i] = acc[i].min(acc[i - 1]) + row[i];
                }
                acc
            },
        )[cols - 1]
    }
}

struct Solution {}
