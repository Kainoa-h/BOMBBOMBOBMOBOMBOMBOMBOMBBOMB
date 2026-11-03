impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let (sum, max) = nums
            .iter()
            .fold((0, 0), |(sum, max), &x| (sum + x, max.max(x)));

        max * nums.len() as i32 - sum
    }
}

struct Solution {}
