impl Solution {
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let gcd = |mut a, mut b| -> i32 {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        };

        let mut max = nums[0];
        let mut prefix_gcd = Vec::with_capacity(nums.len());
        for x in nums {
            max = max.max(x);
            prefix_gcd.push(gcd(x, max));
        }

        prefix_gcd.sort_unstable();

        let mut left_ptr = 0;
        let mut right_ptr = prefix_gcd.len() - 1;
        let mut result = 0_i64;
        while left_ptr < right_ptr {
            let (a, b) = (prefix_gcd[left_ptr], prefix_gcd[right_ptr]);
            result += gcd(a, b) as i64;

            left_ptr += 1;
            right_ptr -= 1;
        }
        result
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::gcd_sum(vec![2, 6, 4]), 2);
}
