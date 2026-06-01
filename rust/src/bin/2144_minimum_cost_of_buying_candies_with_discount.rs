struct Solution {}
impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable_by(|a, b| b.cmp(a));
        cost.chunks(3).map(|c| c.iter().take(2).sum::<i32>()).sum()
    }
}

fn main() {
    assert_eq!(Solution::minimum_cost(vec![1, 2, 3]), 5);
}
