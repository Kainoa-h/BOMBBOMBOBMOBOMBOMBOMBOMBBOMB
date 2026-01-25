struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = prices[0];
        let mut max_profit = 0;

        for &p in prices.iter().skip(1) {
            if min > p {
                min = p;
                continue;
            }
            let diff = p - min;
            if diff > max_profit {
                max_profit = diff;
            }
        }

        max_profit
    }
}

fn main() {
    println!("{}", Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    println!("{}", Solution::max_profit(vec![7, 6, 4, 3, 1]));
}
