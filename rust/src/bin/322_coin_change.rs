use std::collections::HashSet;

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, mut amount: i32) -> i32 {
        coins.sort_unstable_by_key(|x| -x);
        fn change(amt: i32, coins: &[i32]) -> Option<i32> {
            if amt == 0 {
                return Some(1);
            }
            for &coin in coins {
                if amt >= coin && let Some(x) = change(amt - coin, coins) {
                    return Some(x + 1);
                }
            }
            None
        }

        change(amount, &coins).unwrap_or(-1)
    }
}

struct Solution {}
