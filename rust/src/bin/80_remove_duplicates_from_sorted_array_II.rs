struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return nums.len() as i32;
        }

        let mut real_idx = 2;
        for i in 2..nums.len() {
            let before_real_current_num = nums[real_idx - 2];
            let curr_num = nums[i];
            if curr_num != before_real_current_num {
                nums[real_idx] = curr_num;
                real_idx += 1;
            }
        }
        real_idx as i32
    }
}

fn what(mut nums: Vec<i32>) {
    println!("{:?}", Solution::remove_duplicates(&mut nums));
    println!("{:?}", nums);
}

fn main() {
    // Input: nums = [1,1,1,2,2,3]
    // Output: 5, nums = [1,1,2,2,3,_]
    what(vec![1, 1, 1, 2, 2, 3]);

    // Input: nums = [0,0,1,1,1,1,2,3,3]
    // Output: 7, nums = [0,0,1,1,2,3,3,_,_]
    what(vec![0, 0, 1, 1, 1, 1, 2, 3, 3]);

    what(vec![1, 1, 1]);
    what(vec![1, 1]);
    what(vec![1]);
}
