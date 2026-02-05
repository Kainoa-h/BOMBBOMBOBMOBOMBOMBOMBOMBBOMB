struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0_usize;
        let mut r = numbers.len() - 1;

        while l < r {
            let sum = numbers[l] + numbers[r];
            if sum == target {
                return vec![(l + 1) as i32, (r + 1) as i32];
            }
            if sum > target {
                r -= 1;
            } else {
                l += 1;
            }
        }
        panic!()
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 26), vec![3, 4]);
    assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    assert_eq!(Solution::two_sum(vec![-1, 0, 4], -1), vec![1, 2]);
}
