impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        let digit_sum = | mut num: i32| {
            let mut sum = 0;
            while num > 0 {
                sum += num % 10;
                num /= 10;
            }
            sum
        };

        nums.into_iter().enumerate().position(|(i, n)| digit_sum(n) == i as i32).map_or(-1, |x| x as i32)
    }
}

struct Solution {}
