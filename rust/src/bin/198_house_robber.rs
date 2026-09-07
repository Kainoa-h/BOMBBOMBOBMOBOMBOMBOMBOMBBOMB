impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = (0, 0);
        for n in nums {
            let a = dp.0 + n;
            let b = dp.1;
            dp = (dp.1, a.max(b));
        }
        dp.1
    }
}

struct Solution {}
