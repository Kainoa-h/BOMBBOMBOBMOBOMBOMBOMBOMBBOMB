impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut coins = coins as usize;
        let mut counting = [0; 100_001];
        for cost in costs {
            counting[cost as usize] += 1;
        }

        let mut count = 0;
        for (cost, stock) in counting.iter_mut().enumerate().skip(1) {
            while *stock > 0 {
                if coins < cost {
                    return count;
                }
                *stock -= 1;
                coins -= cost;
                count += 1;
            }
        }
        count
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7), 4);
}
