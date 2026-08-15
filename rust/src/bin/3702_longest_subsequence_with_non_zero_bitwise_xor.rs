impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let mut seen_non_zero = false;
        let xor = nums.iter().filter(|&&x| x != 0).fold(0, |acc, x| {
            seen_non_zero = true;
            acc ^ x
        });

        if !seen_non_zero {
            0
        } else if xor == 0 {
            nums.len() as i32 - 1
        } else {
            nums.len() as i32
        }
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::longest_subsequence(vec![1, 2, 3]), 2);
    assert_eq!(Solution::longest_subsequence(vec![0, 7]), 2);
    assert_eq!(Solution::longest_subsequence(vec![0, 0, 7]), 3);
    assert_eq!(Solution::longest_subsequence(vec![7, 6, 1, 9]), 4);
    assert_eq!(Solution::longest_subsequence(vec![7, 0]), 2);
}
