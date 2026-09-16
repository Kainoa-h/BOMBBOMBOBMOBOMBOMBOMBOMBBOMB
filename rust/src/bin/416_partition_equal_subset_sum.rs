impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 != 0 {
            return false;
        }
        let half = sum/2;
        let mut dp = vec![false; half as usize + 1];
        dp[0] = true;
        for n in nums {
            let n = n as usize;
            for i in (0..dp.len()).rev() {
                if dp[i] && let Some(x) = dp.get_mut(i + n) {
                    *x = true;
                }
            }
            if dp[half as usize] {
                break;
            }
        }

        dp[half as usize]
    }
}

struct Solution {}
