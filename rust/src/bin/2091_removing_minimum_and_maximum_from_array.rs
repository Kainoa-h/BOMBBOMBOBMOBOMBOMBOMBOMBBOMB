impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let (mut max, mut max_idx) = (nums[0], 0);
        let (mut min, mut min_idx) = (nums[0], 0);
        for (idx, &n) in nums.iter().enumerate() {
            if n < min {
                min = n;
                min_idx = idx;
            } else if n > max {
                max = n;
                max_idx = idx;
            }
        }
        let (lower_idx, higher_idx) = (max_idx.min(min_idx), max_idx.max(min_idx));
        let delete_left = higher_idx + 1;
        let delete_right = nums.len() - lower_idx;
        let delete_both = lower_idx + 1 + nums.len() - higher_idx;

        delete_left.min(delete_right).min(delete_both) as i32
    }
}

struct Solution {}
