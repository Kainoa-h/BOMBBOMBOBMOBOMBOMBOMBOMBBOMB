impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut squares = nums.iter().map(|&x| (x as i64).pow(2)).collect::<Vec<_>>();
        squares.sort_unstable();
        let (subs, adds) = squares.split_at(squares.len()/2);
        adds.iter().sum::<i64>() - subs.iter().sum::<i64>()
    }
}

struct Solution {}
