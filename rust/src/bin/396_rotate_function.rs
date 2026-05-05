struct Solution {}
impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        for i in 0..nums.len() {
            let mut sum = 0;
            for mult in 1..nums.len() as i32 {
                let idx = (i + mult as usize) % nums.len();
                sum += mult * nums[idx];
            }
            max = max.max(sum);
        }
        max
    }
}
fn main() {
    assert_eq!(Solution::max_rotate_function(vec![4, 3, 2, 6]), 26);
    assert_eq!(Solution::max_rotate_function(vec![100]), 0);
}
