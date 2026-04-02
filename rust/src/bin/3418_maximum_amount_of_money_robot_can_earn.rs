struct Solution {}

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![[i32::MIN / 2; 3]; coins[0].len() + 1];
        dp[1] = [0, 0, 0];

        for row in coins {
            for (col_idx, coin) in row.iter().enumerate() {
                dp[col_idx + 1][2] = i32::max(
                    i32::max(dp[col_idx + 1][2], dp[col_idx][2]) + coin,
                    i32::max(dp[col_idx + 1][1], dp[col_idx][1]),
                );
                dp[col_idx + 1][1] = i32::max(
                    i32::max(dp[col_idx + 1][1], dp[col_idx][1]) + coin,
                    i32::max(dp[col_idx + 1][0], dp[col_idx][0]),
                );
                dp[col_idx + 1][0] = dp[col_idx + 1][0].max(dp[col_idx][0]) + coin;
            }
        }
        dp.last().unwrap()[2]
    }
}

fn main() {
    let mut x;

    x = Solution::maximum_amount(vec![vec![0, 1, -1], vec![1, -2, 3], vec![2, -3, 4]]);
    println!("{}", x);
    assert_eq!(x, 8);

    x = Solution::maximum_amount(vec![vec![10, 10, 10], vec![10, 10, 10]]);
    println!("{}", x);
    assert_eq!(x, 40);

    x = Solution::maximum_amount(vec![
        vec![-7, 12, 12, 13],
        vec![-6, 19, 19, -6],
        vec![9, -2, -10, 16],
        vec![-4, 14, -10, -9],
    ]);
    println!("{}", x);
    assert_eq!(x, 60);
}
