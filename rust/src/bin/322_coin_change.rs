impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![None::<i32>; amount as usize + 1];
        dp[0] = Some(0);

        for amt in 1..=amount as usize {
            for &coin in &coins {
                let coin = coin as usize;
                if amt >= coin && let Some(from) = dp[amt - coin] {
                    dp[amt] = Some(dp[amt].map_or(from + 1, |x| x.min(from + 1)));
                }
            }
        }

        dp[amount as usize].unwrap_or(-1)
    }
}

struct Solution {}
