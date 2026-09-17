impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min = 0;
        let mut dp = i32::MIN;
        for n in nums {
            sum += n;
            dp = dp.max(sum - min);
            min = min.min(sum);
        }
        dp
    }
}

struct Solution {}
