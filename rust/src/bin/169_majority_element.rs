use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let limit = (nums.len() / 2) as i32;
        for n in nums {
            let r = map.entry(n).or_insert(0);
            *r += 1;
            if *r > limit {
                return n;
            }
        }
        -1
    }
}

fn what(nums: Vec<i32>) {
    println!("{:?}", Solution::majority_element(nums));
}

fn main() {
    // Input: nums = [3,2,3]
    // Output: 3
    what(vec![3, 2, 3]);
    // Input: nums = [2,2,1,1,1,2,2]
    // Output: 2
    what(vec![2, 2, 1, 1, 1, 2, 2]);
}
