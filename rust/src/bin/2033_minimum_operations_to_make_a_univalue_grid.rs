struct Solution {}
impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let remainder = grid[0][0] % x;
        let mut flat: Vec<i32> = Vec::with_capacity(grid.len() * grid[0].len());
        for &v in grid.iter().flatten() {
            if v % x != remainder {
                return -1;
            }
            flat.push(v);
        }
        flat.sort_unstable();
        let median = flat[flat.len() / 2];
        flat.iter().map(|v| (median - v).abs() / x).sum()
    }
}

fn main() {
    assert_eq!(Solution::min_operations(vec![vec![2, 4], vec![6, 8]], 2), 4);
    assert_eq!(Solution::min_operations(vec![vec![1, 5], vec![2, 3]], 1), 5);
    assert_eq!(
        Solution::min_operations(vec![vec![1, 2], vec![3, 4]], 2),
        -1
    );
}
