struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.windows(2).map(|w| 0.max(w[1] - w[0])).sum()
    }
}

fn main() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}
