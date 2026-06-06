struct Solution {}
impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let total_sum = nums.iter().sum::<i32>();
        let mut left_sum = 0;
        nums.iter()
            .map(|n| {
                let right_sum = total_sum - left_sum - n;
                let diff = (left_sum - right_sum).abs();
                left_sum += n;
                diff
            })
            .collect()
    }
}

fn main() {
    assert_eq!(
        Solution::left_right_difference(vec![10, 4, 8, 3]),
        vec![15, 1, 11, 22]
    );
    assert_eq!(Solution::left_right_difference(vec![1]), vec![0]);
}
