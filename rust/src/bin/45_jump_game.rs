struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut max_reachable = 0;
        let mut current_range = 0;
        let mut jumps = 0;
        for (i, &n) in nums.iter().take(nums.len().saturating_sub(1)).enumerate() {
            max_reachable = max_reachable.max(i + n as usize);
            if i == current_range {
                jumps += 1;
                current_range = max_reachable;
                if current_range + 1 >= nums.len() {
                    return jumps;
                }
            }
        }
        jumps
    }
}

fn main() {
    assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    assert_eq!(Solution::jump(vec![3, 2, 1, 0, 4]), 2);
    assert_eq!(Solution::jump(vec![0]), 0);
    assert_eq!(Solution::jump(vec![1, 0]), 1);
}
