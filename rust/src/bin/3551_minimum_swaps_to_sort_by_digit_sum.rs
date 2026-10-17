impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut sorted = nums.to_vec();
        sorted.sort_by_cached_key(|&x| {
            std::iter::successors(Some(x), |&n| {
                let next = n / 10;
                (next > 0).then_some(next)
            })
            .map(|n| n % 10)
            .sum::<i32>()
        });
        nums.iter().zip(sorted.iter()).filter(|(a, b)| a != b).count() as i32 / 2
    }
}

struct Solution {}
