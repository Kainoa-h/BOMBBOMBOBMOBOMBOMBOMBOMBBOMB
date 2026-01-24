struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut real_idx = 1;
        let mut prev_num = nums[0];
        for i in 1..nums.len() {
            let curr_num = nums[i];
            if curr_num != prev_num {
                nums[real_idx] = curr_num;
                real_idx += 1;
                prev_num = curr_num;
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
    // Input: nums = [1,1,2]
    // Output: 2, nums = [1,2,_]
    what(vec![1, 1, 2]);

    // Input: nums = [0,0,1,1,1,2,2,3,3,4]
    // Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
    what(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]);

    what(vec![1, 1]);
    what(vec![1]);
}
