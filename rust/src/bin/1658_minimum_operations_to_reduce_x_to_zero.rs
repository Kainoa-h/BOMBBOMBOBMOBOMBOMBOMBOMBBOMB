impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let len = nums.len();
        let total = nums.iter().sum::<i32>();
        let target = total - x;
        if target < 0 {
            return -1;
        }
        if target == 0 {
            return len as i32;
        }
        let mut max_len = 0;
        let mut sum = 0;
        let mut start = 0;
        for (end, &n) in nums.iter().enumerate() {
            sum += n;
            while sum > target {
                sum -= nums[start];
                start += 1;
            }
            if sum == target {
                max_len = max_len.max(end - start + 1);
            }
        }

        if max_len == 0 {
            -1
        } else {
            (len - max_len) as i32
        }
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
    assert_eq!(Solution::min_operations(vec![1, 1], 3), -1);
}
