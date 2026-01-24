struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k % nums.len() as i32;
        let len = nums.len();
        let mut clone = nums.clone();
        let (new_end_block, new_start_block) = clone.split_at_mut(len - k as usize);

        let mut idx = 0;
        for v in new_start_block {
            nums[idx] = *v;
            idx += 1;
        }
        for v in new_end_block {
            nums[idx] = *v;
            idx += 1;
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
