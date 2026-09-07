impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |acc, n| (acc.1, acc.1.max(acc.0 + n)))
            .1
    }
}

struct Solution {}
