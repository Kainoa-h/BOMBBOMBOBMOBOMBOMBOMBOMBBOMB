impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let mut and = i32::MAX;
        for (idx, &n) in nums.iter().enumerate() {
            if idx as i32 != n {
                and &= n;
            }
        }
        if and == i32::MAX { 0 } else { and }
    }
}

struct Solution {}
