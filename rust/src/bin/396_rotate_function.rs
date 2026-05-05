struct Solution {}
impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let incr = nums.iter().sum::<i32>();
        let mut prev = (1..nums.len()).fold(0, |acc, idx| acc + (idx as i32 * nums[idx]));
        let mut max = prev;

        // k=0, i=0
        // k=1, i=3,
        // k=2, i=2,
        // k=3, i=1
        for i in (1..nums.len()).rev() {
            prev = prev + incr - (nums[i] * nums.len() as i32);
            max = max.max(prev);
        }
        max
    }
}
fn main() {
    assert_eq!(Solution::max_rotate_function(vec![4, 3, 2, 6]), 26);
    assert_eq!(Solution::max_rotate_function(vec![100]), 0);
}
