use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = HashSet::<i32>::from_iter(nums);
        let starting_nums = set.iter().filter(|&x| set.contains(&(x - 1)));
        let mut longest = 0;
        for &s in starting_nums {
            let mut n = s + 1;
            while set.contains(&n) {
                n += 1;
            }
            longest = longest.max(n - s);
        }
        longest
    }
}

struct Solution {}
