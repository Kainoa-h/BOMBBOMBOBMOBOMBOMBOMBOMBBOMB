impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut closest_sum = i32::MAX;
        nums.sort_unstable();

        let min = nums[0..3].iter().sum();
        if min > target {
            return min;
        }
        let max = nums[n - 3..n].iter().sum();
        if max < target {
            return max;
        }

        for f in 0..nums.len() - 2 {
            if f != 0 && nums[f] == nums[f - 1] {
                continue;
            }

            for l in f + 1..nums.len() - 1 {
                if l != f + 1 && nums[l] == nums[l - 1] {
                    continue;
                }

                for r in l + 1..nums.len() {
                    if r != l + 1 && nums[r] == nums[r - 1] {
                        continue;
                    }

                    let sum = nums[f] + nums[l] + nums[r];
                    let diff = (target - sum).abs();
                    if (target - closest_sum).abs() > diff {
                        closest_sum = sum;
                    }
                    if sum > target {
                        break;
                    }
                }
            }
        }

        closest_sum
    }
}

struct Solution {}
