impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let result = nums.iter().take(n/2)
            .enumerate()
            .map(|(i, x)| if n.is_multiple_of(i + 1) { x.pow(2) } else { 0 })
            .sum::<i32>();
        result + nums[n-1].pow(2)
    }
}

struct Solution {}
