impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        nums.into_iter().enumerate().find_map(|(idx, mut num)| {
            let mut sum = 0;
            while num > 0 {
                sum += num % 10;
                num /= 10;
            }
            if sum as usize == idx {
                Some(idx as i32)
            } else {
                None
            }
        }).unwrap_or(-1)
    }
}

struct Solution {}
