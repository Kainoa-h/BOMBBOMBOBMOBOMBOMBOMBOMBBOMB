use std::cmp::Ordering;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut closest_sum = nums[0] + nums[1] + nums[2];
        nums.sort_unstable();

        for f in 0..n - 2 {
            let min = nums[f] + nums[f + 1] + nums[f + 2];
            if min > target {
                if target.abs_diff(min) < target.abs_diff(closest_sum) {
                    closest_sum = min;
                }
                break;
            }
            let max = nums[f] + nums[n - 2] + nums[n - 1];
            if max < target {
                if target.abs_diff(max) < target.abs_diff(closest_sum) {
                    closest_sum = max;
                }
                continue;
            }
            let (mut l, mut r) = (f + 1, n - 1);

            while l < r {
                let sum = nums[f] + nums[l] + nums[r];

                if target.abs_diff(sum) < target.abs_diff(closest_sum) {
                    closest_sum = sum;
                }

                match sum.cmp(&target) {
                    Ordering::Less => l += 1,
                    Ordering::Equal => return closest_sum,
                    Ordering::Greater => r -= 1,
                }
            }

        }

        closest_sum
    }
}

struct Solution {}
