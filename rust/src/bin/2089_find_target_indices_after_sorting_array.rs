impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut before_len = 0;
        let mut target_len = 0;
        for n in nums {
            before_len += i32::from(n < target);
            target_len += i32::from(n == target)
        }
        (before_len..before_len+target_len).collect()
    }
}

struct Solution {}
