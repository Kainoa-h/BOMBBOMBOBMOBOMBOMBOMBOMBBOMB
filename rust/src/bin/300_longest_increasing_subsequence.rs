impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1;nums.len()];
        let mut max = 1;
        for (idx, &n) in nums.iter().enumerate() {
            for bi in (0..idx).rev() {
                if nums[bi] < n {
                    dp[idx] = dp[idx].max(1 + dp[bi]);
                    max = max.max(dp[idx]);
                }
            }
        }
        max
    }
}

struct Solution {}
