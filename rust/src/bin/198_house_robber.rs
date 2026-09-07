impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        for idx in 0..dp.len() {
            //underflow but wtv
            let prev_prev_now = dp.get(idx - 2).unwrap_or(&0) + nums[idx];
            let prev = *dp.get(idx-1).unwrap_or(&0);
            dp[idx] = prev_prev_now.max(prev);
        }
        dp[nums.len() - 1]
    }
}

struct Solution {}
