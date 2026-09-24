impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        for (idx, mut num) in nums.into_iter().enumerate() {
            let mut sum = 0;
            while num > 0 {
                sum += num % 10;
                num /= 10;
            }
            if sum as usize == idx {
                return idx as i32;
            }
        }
        -1
    }
}
