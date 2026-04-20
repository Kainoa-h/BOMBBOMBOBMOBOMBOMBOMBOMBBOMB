struct Solution {}
impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let last_idx = colors.len() - 1;

        for dist in (1..=last_idx).rev() {
            if colors.first() != colors.get(dist) || colors.last() != colors.get(last_idx - dist) {
                return dist as i32;
            }
        }

        0
    }
}

fn main() {
    assert_eq!(Solution::max_distance(vec![1, 1, 1, 6, 1, 1, 1]), 3);
    assert_eq!(Solution::max_distance(vec![1, 8, 3, 8, 3]), 4);
    assert_eq!(Solution::max_distance(vec![0, 1]), 1);
}
