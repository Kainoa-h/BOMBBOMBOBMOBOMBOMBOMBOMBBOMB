impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let (min, max) = nums
            .iter()
            .skip(1)
            .fold((&nums[0], &nums[0]), |(min, max), x| {
                (min.min(x), max.max(x))
            });
        (max - min) as i64 * k as i64
    }
}
struct Solution {}
