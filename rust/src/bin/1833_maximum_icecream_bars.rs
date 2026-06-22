impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort_unstable();
        let mut bought = 0;
        for price in costs {
            if price > coins {
                break;
            }
            bought += 1;
            coins -= price;
        }
        bought
    }
}

struct Solution {}
