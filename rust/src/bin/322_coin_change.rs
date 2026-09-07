impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, mut amount: i32) -> i32 {
        let mut coins_used = 0;
        coins.sort_unstable();
        while amount > 0
            && let Some(&coin) = coins.last()
        {
            if coin > amount {
                coins.pop();
                continue;
            }
            amount -= coin;
            coins_used +=1;
        }

        if amount == 0 { coins_used } else { -1 }
    }
}

struct Solution {}
