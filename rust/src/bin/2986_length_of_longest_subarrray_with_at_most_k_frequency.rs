use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::<i32,i32>::new();
        let mut start = 0;
        let mut max = 0;
        for (end, &n) in nums.iter().enumerate() {
            *map.entry(n).or_default() += 1;
            
            while *map.get(&n).unwrap() > k {
                *map.get_mut(&nums[start]).unwrap() -= 1;
                start += 1;
            }

            max = max.max(end - start + 1);
        }

        max as i32
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::max_subarray_length(vec![1,2,3,1,2,3,1,2], 2), 6);
}
