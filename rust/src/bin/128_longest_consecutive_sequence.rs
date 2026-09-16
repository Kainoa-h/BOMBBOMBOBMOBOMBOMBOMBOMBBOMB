use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = HashSet::<i32>::from_iter(nums);
        set.iter()
            .filter(|&x| !set.contains(&(x - 1)))
            .map(|x| {
                (x + 1..i32::MAX)
                    .take_while(|n| set.contains(n))
                    .last()
                    .map_or(1, |r| r - x + 1)
            })
            .max()
            .unwrap_or(0)
    }
}

struct Solution {}
