impl Solution {
    pub fn min_merged_integer(nums: Vec<i32>) -> i32 {
        let digits = |n: i32| -> u32 { n.checked_ilog10().unwrap_or(0) + 1 };
        nums.chunks_exact(2)
            .map(|c| {
                let (a, b) = (c[0], c[1]);
                a * 10_i32.pow(digits(b)) + b
            })
            .min()
            .unwrap()
    }
}

struct Solution {}
