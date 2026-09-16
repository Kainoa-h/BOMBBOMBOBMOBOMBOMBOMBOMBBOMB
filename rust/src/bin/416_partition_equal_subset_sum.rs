impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        if !sum.is_multiple_of(2) {
            return false;
        }
        let half = sum / 2;
        let mut dp = vec![false; half + 1];
        dp[0] = true;
        for n in nums {
            let n = n as usize;
            for i in (n..dp.len()).rev() {
                dp[i] |= dp[i - n];
            }
            if dp[half] {
                return true;
            }
        }

        dp[half]
    }
}

struct Solution {}
