impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut cur_sum = nums[0];
        let mut best = nums[0];
        for n in nums.into_iter().skip(1) {
            cur_sum = n.max(cur_sum + n);
            best = best.max(cur_sum);
        }
        best
    }
}

struct Solution {}
