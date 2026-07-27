impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut first = 0;
        let mut second = 0;
        for n in nums {
            second = second.max(first.min(n));
            first = first.max(n);
        }
        (first - 1) * (second - 1)
    }
}
