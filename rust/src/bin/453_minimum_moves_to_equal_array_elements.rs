impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let (sum, min) = nums
            .iter()
            .fold((0, nums[0]), |(sum, min), &x| (sum + x, min.min(x)));
        sum - (min * nums.len() as i32)
    }
}

struct Solution {}
