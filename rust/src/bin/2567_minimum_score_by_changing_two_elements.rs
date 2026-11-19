impl Solution {
    pub fn minimize_sum(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        let change_first_two = nums[n - 1] - nums[2];
        let change_last_two = nums[n - 3] - nums[0];
        let change_outter_two = nums[n - 2] - nums[1];
        change_first_two.min(change_last_two).min(change_outter_two)
    }
}

struct Solution {}
