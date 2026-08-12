use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::<i32,i32>::new();
        let mut start = 0;
        let mut max = 0;
        for (end, &n) in nums.iter().enumerate() {
            let count = *map.entry(n).and_modify(|x|*x+=1).or_insert(1);

            if count > k {
                loop {
                    let val = nums[start];
                    if let Some(x) = map.get_mut(&val) {
                        *x -= 1;
                    }
                    start += 1;
                    
                    if val == n {
                        break;
                    }
                }
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
