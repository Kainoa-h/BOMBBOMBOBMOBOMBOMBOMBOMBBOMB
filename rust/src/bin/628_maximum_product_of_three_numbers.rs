impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut first = -1001;
        let mut second = -1001;
        let mut third = -1001;
        let mut last = 1001;
        let mut second_last = 1001;
        for n in nums {
            third = third.max(second.min(n));
            second = second.max(first.min(n));
            first = first.max(n);

            second_last = second_last.min(last.max(n));
            last = last.min(n);
        }
        i32::max(first * second * third, first * last * second_last)
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::maximum_product(vec![-100, -98, -1, 2, 3, 4]),
        39200
    );
}
