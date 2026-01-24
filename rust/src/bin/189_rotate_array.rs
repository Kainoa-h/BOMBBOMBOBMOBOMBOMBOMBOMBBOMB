struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            let last = nums[nums.len() - 1];
            for i in (1..nums.len()).rev() {
                nums[i] = nums[i - 1];
            }
            nums[0] = last;
        }
    }
}

fn what(mut nums: Vec<i32>, k: i32) {
    Solution::rotate(&mut nums, k);
    println!("{:?}", nums);
}

fn main() {
    // Input: nums = [1,2,3,4,5,6,7], k = 3
    // Output: [5,6,7,1,2,3,4]
    what(vec![1, 2, 3, 4, 5, 6, 7], 3);

    // Input: nums = [-1,-100,3,99], k = 2
    // Output: [3,99,-1,-100]
    what(vec![-1, -100, 3, 99], 2);
}
