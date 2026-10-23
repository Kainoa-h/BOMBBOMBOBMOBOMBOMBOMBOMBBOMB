impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let and = nums.iter().enumerate().fold(i32::MAX, |acc, (i, &n)| {
            let mask = -i32::from(i as i32 == n);
            acc & (n | mask)
        });
        if and == i32::MAX { 0 } else { and }
    }
}

struct Solution {}

use std::hint::black_box;
fn main() {
    assert_eq!(Solution::sort_permutation(black_box(vec![3, 2, 1, 0])), 0);
    assert_eq!(Solution::sort_permutation(black_box(vec![0, 1, 3, 2])), 2);
    assert_eq!(Solution::sort_permutation(black_box((0..10_000).collect())), 0);
}
