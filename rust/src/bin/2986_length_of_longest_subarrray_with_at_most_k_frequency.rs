use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as u32;
        let mut max = 1;
        for head in 0..nums.len() - 1 {
            if nums.len() - head < max {
                break;
            }
            for tail in (head + 1..nums.len()).rev() {
                let len = tail - head + 1;
                if len <= max {
                    break;
                }

                let mut map = HashMap::<i32, u32>::new();
                let mut valid = true;
                for &n in &nums[head..=tail] {
                    if *map.entry(n).and_modify(|x| *x += 1).or_insert(1) > k {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    max = max.max(len);
                }
            }
        }

        max as i32
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::max_subarray_length(vec![1,2,3,1,2,3,1,2], 2), 6);
}
