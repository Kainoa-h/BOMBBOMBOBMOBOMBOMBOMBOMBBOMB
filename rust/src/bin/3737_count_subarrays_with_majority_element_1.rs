impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let mut count = 0;
        for start_inc in 0..nums.len() {
            let mut balance = 0;
            for &x in nums.iter().skip(start_inc) {
                balance += if x == target { 1 } else { -1 };
                count += (balance > 0) as i32;
            }
        }

        count
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 2, 3], 2), 5);
}
