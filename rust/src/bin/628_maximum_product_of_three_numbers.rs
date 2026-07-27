impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut first = -1001;
        let mut second = -1001;
        let mut third = -1001;
        let mut last = 1001;
        let mut second_last = 1001;
        for n in nums {
            if n > first {
                third = second;
                second = first;
                first = n;
            } else if n > second {
                third = second;
                second = n;
            } else if n > third {
                third = n;
            } 
            if n < last {
                second_last = last;
                last = n;
            } else if n < second_last {
                second_last = n;
            }
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
