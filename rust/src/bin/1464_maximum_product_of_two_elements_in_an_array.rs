impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (first, second) = nums.iter().fold((0,0), |mut acc, &n| {
            acc.1 = acc.1.max(acc.0.min(n));
            acc.0 = acc.0.max(n);
            acc
        });
        (first - 1) * (second - 1)
    }
}
