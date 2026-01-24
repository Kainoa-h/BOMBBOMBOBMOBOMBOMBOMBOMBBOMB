struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..nums.len() {
            if nums[i] == val {
                nums[i] = -1;
                stack.push(i);
            }
        }
        let left = (nums.len() - stack.len()) as i32;

        let mut candidate_idx = nums.len();
        while let Some(replace_idx) = stack.pop() {
            let candidate = loop {
                match candidate_idx.checked_sub(1) {
                    Some(x) => candidate_idx = x,
                    None => break 0,
                };
                let x = nums[candidate_idx];
                if x != -1 {
                    break x;
                }
            };

            if candidate_idx <= replace_idx {
                candidate_idx += 1;
                continue;
            }

            nums[replace_idx] = candidate;
        }

        left
    }
}

fn what(mut nums: Vec<i32>, val: i32) {
    println!("{:?}", Solution::remove_element(&mut nums, val));
    println!("{:?}", nums);
}

fn main() {
    // Input: nums = [3,2,2,3], val = 3
    // Output: 2, nums = [2,2,_,_]
    what(vec![3, 2, 2, 3], 3);

    // Input: nums = [0,1,2,2,3,0,4,2], val = 2
    // Output: 5, nums = [0,1,4,0,3,_,_,_]
    what(vec![0, 1, 2, 2, 3, 0, 4, 2], 2);

    what(vec![1], 1);
    what(vec![0, 1, 2, 2, 3, 0, 4, 2], 22);
}
