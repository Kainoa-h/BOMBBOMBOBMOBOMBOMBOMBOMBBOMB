impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut squares = nums.iter().map(|&x| (x as i64).pow(2)).collect::<Vec<_>>();
        squares.sort_unstable_by_key(|x| -x);
        let (mut left, mut right) = (0, n - 1);
        let mut sum = 0;
        if !nums.len().is_multiple_of(2) {
            sum = squares[n/2];
        }
        while left < right {
            sum += squares[left];
            sum -= squares[right];
            left += 1;
            right -= 1;
        }
        sum
    }
}

struct Solution {}
