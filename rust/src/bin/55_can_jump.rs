struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reachable = 0;
        for (i, &n) in nums.iter().enumerate() {
            if max_reachable < i {
                return false;
            }
            max_reachable = max_reachable.max(i + n as usize);

            if max_reachable + 1 >= nums.len() {
                return true;
            }
        }
        false
    }
}

fn main() {
    assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
}
