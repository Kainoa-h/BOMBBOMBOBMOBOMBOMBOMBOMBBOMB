impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let (&first, rest) = nums.split_first().unwrap();
        let mut max = first;
        let mut min = first;
        for &n in rest {
            max = max.max(n);
            min = min.min(n);
        }
        while min != 0 {
            (max, min) = (min, max % min)
        }
        max
    }
}

struct Solution {}
