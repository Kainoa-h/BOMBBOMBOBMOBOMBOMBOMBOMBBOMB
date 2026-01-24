struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = nums[0];
        let mut strength = 1;
        for n in nums.iter().skip(1) {
            if strength == 0 {
                candidate = *n;
                strength = 1;
            } else if candidate == *n {
                strength += 1;
            } else {
                strength -= 1;
            }
        }
        candidate
    }
}

fn what(nums: Vec<i32>) {
    println!("{:?}", Solution::majority_element(nums));
}

fn main() {
    // Input: nums = [3,2,3]
    // Output: 3
    what(vec![3, 2, 3]);
    // Input: nums = [2,2,1,1,1,2,2]
    // Output: 2
    what(vec![2, 2, 1, 1, 1, 2, 2]);
}
