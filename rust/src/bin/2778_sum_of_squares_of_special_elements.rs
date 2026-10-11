impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.iter()
            .enumerate()
            .map(|(i, x)| if n.is_multiple_of(i + 1) { x.pow(2) } else { 0 })
            .sum()
    }
}

struct Solution {}
