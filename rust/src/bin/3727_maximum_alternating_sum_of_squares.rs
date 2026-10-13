impl Solution {
    pub fn max_alternating_sum(mut nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let (small, mid, large) = nums.select_nth_unstable_by_key(n / 2, |x| x.abs());
        let mut sum = large.iter().map(|&x| (x as i64).pow(2)).sum::<i64>();
        sum += (*mid as i64).pow(2);
        sum -= small.iter().map(|&x| (x as i64).pow(2)).sum::<i64>();
        sum
    }
}

struct Solution {}
