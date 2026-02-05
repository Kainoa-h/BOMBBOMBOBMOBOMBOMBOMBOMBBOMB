struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for out_idx in 0..numbers.len() {
            for in_idx in (out_idx + 1)..numbers.len() {
                let sum = numbers[out_idx] + numbers[in_idx];
                if sum > target {
                    break;
                }

                if sum == target {
                    return vec![(out_idx + 1) as i32, (in_idx + 1) as i32];
                }
            }
        }
        panic!("invalid");
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    assert_eq!(Solution::two_sum(vec![-1, 0, 4], -1), vec![1, 2]);
}
