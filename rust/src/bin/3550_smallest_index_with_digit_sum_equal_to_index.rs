impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        let digit_sum = |num: i32| {
            std::iter::successors(Some(num), |&x| (x >= 10).then_some(x / 10))
                .map(|x| x % 10)
                .sum::<i32>()
        };

        nums.into_iter()
            .enumerate()
            .position(|(i, n)| digit_sum(n) == i as i32)
            .map_or(-1, |x| x as i32)
    }
}

struct Solution {}
