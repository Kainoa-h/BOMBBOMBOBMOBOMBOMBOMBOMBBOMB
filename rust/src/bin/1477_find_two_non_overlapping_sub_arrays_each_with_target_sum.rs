impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        let mut dp = vec![n + 1; n + 1];
        let mut current_count = 0;
        let mut left_idx = 0;
        let mut result = n + 1;
        for right_idx in 0..n {
            current_count += arr[right_idx];
            dp[right_idx + 1] = dp[right_idx];

            while current_count > target {
                current_count -= arr[left_idx];
                left_idx += 1;
            }

            if current_count == target {
                let len = right_idx - left_idx + 1;
                dp[right_idx + 1] = dp[right_idx].min(len);
                result = result.min(len + dp[left_idx]);
            }
        }
        if result == n + 1 { -1 } else { result as i32 }
    }
}

struct Solution {}
