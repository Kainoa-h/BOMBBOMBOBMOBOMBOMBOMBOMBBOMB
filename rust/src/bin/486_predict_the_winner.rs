impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();

        if n & 1 == 0 {
            return true;
        }

        let mut dp = nums.clone();
        for left_bound in (0..n - 1).rev() {
            for right_bound in left_bound + 1..n {
                let pick_left = nums[left_bound] - dp[right_bound];
                let pick_right = nums[right_bound] - dp[right_bound -1];
                dp[right_bound] = i32::max(pick_left, pick_right);

            }
        }

        dp[n - 1] >= 0
    }
}

struct Solution {}

fn main() {}
