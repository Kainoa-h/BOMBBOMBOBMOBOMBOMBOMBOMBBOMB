use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::<i32,i32>::new();
        let mut start = 0;
        let mut max = 0;
        for (idx, &n) in nums.iter().enumerate() {
            map.entry(n).and_modify(|x| *x+=1).or_insert(1);
            
            while *map.get(&n).unwrap() > k {
                map.entry(nums[start]).and_modify(|x| *x -= 1);
                start += 1;
            }

            max = max.max(idx - start + 1);
        }

        max as i32
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::max_subarray_length(vec![1,2,3,1,2,3,1,2], 2), 6);
}
