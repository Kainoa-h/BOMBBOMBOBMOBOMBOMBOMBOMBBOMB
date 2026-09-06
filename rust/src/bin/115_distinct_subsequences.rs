impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut dp = vec![0_i64; t.len()+1];
        dp[0] = 1;

        for ch in s.into_bytes() {
            for idx in (0..t.len()).rev() {
                if ch == t.as_bytes()[idx] {
                    dp[idx + 1] += dp[idx];
                }
            }
        }

        dp[t.len()] as i32
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        5,
        Solution::num_distinct("babgbag".to_owned(), "bag".to_owned())
    );
    assert_eq!(
        3,
        Solution::num_distinct("rabbbit".to_owned(), "rabbit".to_owned())
    );
}
