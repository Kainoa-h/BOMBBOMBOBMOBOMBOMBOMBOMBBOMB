struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max_profit = 0;

        for p in prices {
            min = min.min(p);
            max_profit = max_profit.max(p - min);
        }

        max_profit
    }
}

fn main() {
    println!("{}", Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    println!("{}", Solution::max_profit(vec![7, 6, 4, 3, 1]));
}
