impl Solution {
    pub fn min_merged_integer(nums: Vec<i32>) -> i32 {
        let digits = |mut n: i32| -> u32 {
            let mut digits = 0;
            while n != 0 {
                n /= 10;
                digits += 1;
            }
            digits
        };
        nums.chunks(2)
            .map(|c| {
                let (a, b) = (c[0], c[1]);
                a * 10_i32.pow(digits(b)) + b
            })
            .min()
            .unwrap()
    }
}

struct Solution {}
