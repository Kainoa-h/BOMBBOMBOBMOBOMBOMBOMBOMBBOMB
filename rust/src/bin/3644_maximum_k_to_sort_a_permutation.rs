impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let and = nums
            .iter()
            .enumerate()
            .filter(|&(i, &n)| i as i32 != n)
            .fold(i32::MAX, |acc, (_, &n)| acc & n);
        if and == i32::MAX { 0 } else { and }
    }
}

struct Solution {}
