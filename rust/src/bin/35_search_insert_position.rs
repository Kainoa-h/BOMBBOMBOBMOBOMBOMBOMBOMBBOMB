impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = (right + left) / 2;

            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                left = mid.saturating_add(1);
            } else {
                right = mid.saturating_sub(1);
            }
        }
        left as i32 + i32::from(nums[left] < target)
    }
}

struct Solution {}
