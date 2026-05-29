struct Solution {}
impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        for n in nums {
            let mut sum = 0;
            let mut n = n;
            while n != 0 {
                sum += n % 10;
                n /= 10;
            }
            min = min.min(sum);
        }
        min
    }
}

fn main() {
    assert_eq!(1, Solution::min_element(vec![10, 12, 13, 14]));
}
